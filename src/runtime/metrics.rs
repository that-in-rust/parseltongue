//! Runtime Performance Metrics - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Metrics Configuration
//! Layer 3: Runtime Statistics
//! Layer 4: Performance Analysis
//! Layer 5: Resource Management

use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::{Context, Result};
use tokio::sync::RwLock;
use tracing::{debug, info};
use serde::Serialize;
use tokio::sync::Mutex;
use metrics::{Gauge, Histogram};
use metrics_exporter_prometheus::PrometheusBuilder;

use crate::metrics::{MetricsManager, TaskMetrics};

// Layer 1: Core Types
#[derive(Debug)]
pub struct RuntimeMetrics {
    state: Arc<RwLock<RuntimeState>>,
    task_metrics: TaskMetrics,
    #[cfg(feature = "metrics")]
    metrics_manager: Arc<MetricsManager>,
    requests: Gauge,
    response_time: Histogram,
    exporter: Arc<Mutex<metrics_exporter_prometheus::PrometheusReturn>>,
}

#[derive(Debug, Default, Serialize)]
struct RuntimeState {
    start_time: Instant,
    worker_count: usize,
    active_workers: usize,
    total_tasks: usize,
    failed_tasks: usize,
    total_duration: Duration,
}

// Layer 2: Implementation
impl RuntimeMetrics {
    pub fn new(worker_count: usize) -> Self {
        let builder = PrometheusBuilder::new();
        let exporter = builder
            .set_buckets_for_metric("response_time_seconds", &[0.1, 0.5, 1.0, 5.0])
            .install()
            .expect("Failed to install Prometheus metrics exporter");

        Self {
            state: Arc::new(RwLock::new(RuntimeState {
                start_time: Instant::now(),
                worker_count,
                ..Default::default()
            })),
            task_metrics: TaskMetrics::new(),
            #[cfg(feature = "metrics")]
            metrics_manager: Arc::new(MetricsManager::new()),
            requests: metrics::register_gauge!("requests_total"),
            response_time: metrics::register_histogram!("response_time_seconds"),
            exporter: Arc::new(Mutex::new(exporter)),
        }
    }

    // Layer 3: Metrics Recording
    pub async fn record_worker_start(&self) -> Result<()> {
        let mut state = self.state.write().await;
        state.active_workers += 1;
        debug!("Worker started ({} active)", state.active_workers);
        Ok(())
    }

    pub async fn record_worker_stop(&self) -> Result<()> {
        let mut state = self.state.write().await;
        state.active_workers = state.active_workers.saturating_sub(1);
        debug!("Worker stopped ({} active)", state.active_workers);
        Ok(())
    }

    pub async fn record_task_completion(&self, duration: Duration) -> Result<()> {
        let mut state = self.state.write().await;
        state.total_tasks += 1;
        state.total_duration += duration;

        self.task_metrics.record_operation(duration).await?;

        #[cfg(feature = "metrics")]
        self.metrics_manager.record_file_processed(0, duration).await?;

        Ok(())
    }

    pub async fn record_task_failure(&self, context: &str) -> Result<()> {
        let mut state = self.state.write().await;
        state.failed_tasks += 1;

        self.task_metrics.record_error(context).await?;

        #[cfg(feature = "metrics")]
        self.metrics_manager.record_error(context).await?;

        Ok(())
    }

    // Layer 4: Statistics
    pub async fn get_statistics(&self) -> Result<RuntimeStatistics> {
        let state = self.state.read().await;
        let task_stats = self.task_metrics.get_statistics().await?;

        Ok(RuntimeStatistics {
            uptime: state.start_time.elapsed(),
            worker_count: state.worker_count,
            active_workers: state.active_workers,
            total_tasks: state.total_tasks,
            failed_tasks: state.failed_tasks,
            avg_task_duration: task_stats.avg_duration,
        })
    }

    // Layer 5: Resource Management
    pub async fn shutdown(&self) -> Result<()> {
        let state = self.state.read().await;
        info!(
            "Runtime metrics: {} workers, {} tasks ({} failed), {} total duration",
            state.worker_count,
            state.total_tasks,
            state.failed_tasks,
            state.total_duration.as_secs(),
        );

        self.task_metrics.shutdown().await?;

        #[cfg(feature = "metrics")]
        self.metrics_manager.shutdown().await?;

        Ok(())
    }

    pub async fn record_request(&self) {
        self.requests.increment(1.0);
    }

    pub async fn record_response_time(&self, duration: f64) {
        self.response_time.observe(duration);
    }

    pub async fn get_metrics(&self) -> Result<String> {
        let exporter = self.exporter.lock().await;
        let metrics = exporter.render();
        Ok(metrics)
    }
}

#[derive(Debug, Serialize)]
pub struct RuntimeStatistics {
    pub uptime: Duration,
    pub worker_count: usize,
    pub active_workers: usize,
    pub total_tasks: usize,
    pub failed_tasks: usize,
    pub avg_task_duration: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_runtime_metrics() -> Result<()> {
        let metrics = RuntimeMetrics::new(2);
        
        metrics.record_worker_start().await?;
        metrics.record_task_completion(Duration::from_millis(100)).await?;
        sleep(Duration::from_millis(10)).await;
        metrics.record_worker_stop().await?;
        
        let stats = metrics.get_statistics().await?;
        assert_eq!(stats.total_tasks, 1);
        assert_eq!(stats.worker_count, 2);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_metrics_recording() -> Result<()> {
        let metrics = RuntimeMetrics::new(4);
        metrics.record_request().await;
        metrics.record_response_time(0.5).await;

        let collected = metrics.get_metrics().await?;
        assert!(collected.contains("requests_total"));
        assert!(collected.contains("response_time_seconds"));

        Ok(())
    }
}

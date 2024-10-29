//! Runtime Metrics - Pyramidal Structure
//! Layer 1: Core Metric Types
//! Layer 2: Metric Collection
//! Layer 3: Metric Aggregation
//! Layer 4: Export & Reporting
//! Layer 5: Resource Management

use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use metrics::{Counter, Gauge, Histogram};
use serde::Serialize;
use anyhow::Result;

// Layer 1: Core Types
#[derive(Debug)]
pub struct RuntimeMetrics {
    state: Arc<RwLock<MetricsState>>,
    counters: RuntimeCounters,
    gauges: RuntimeGauges,
    histograms: RuntimeHistograms,
}

#[derive(Debug, Default, Serialize)]
struct MetricsState {
    start_time: Option<Instant>,
    total_tasks: u64,
    active_tasks: u64,
    failed_tasks: u64,
}

// Layer 2: Metric Groups
#[derive(Debug)]
struct RuntimeCounters {
    tasks_started: Counter,
    tasks_completed: Counter,
    tasks_failed: Counter,
    bytes_processed: Counter,
}

#[derive(Debug)]
struct RuntimeGauges {
    active_tasks: Gauge,
    memory_usage: Gauge,
    worker_count: Gauge,
}

#[derive(Debug)]
struct RuntimeHistograms {
    task_duration: Histogram,
    queue_time: Histogram,
    processing_time: Histogram,
}

// Layer 3: Implementation
impl RuntimeMetrics {
    pub fn new() -> Self {
        let state = Arc::new(RwLock::new(MetricsState {
            start_time: Some(Instant::now()),
            ..Default::default()
        }));

        Self {
            state,
            counters: RuntimeCounters::new(),
            gauges: RuntimeGauges::new(),
            histograms: RuntimeHistograms::new(),
        }
    }

    // Layer 4: Metric Recording
    pub async fn record_task_start(&self) {
        self.counters.tasks_started.increment(1);
        let mut state = self.state.write().await;
        state.total_tasks += 1;
        state.active_tasks += 1;
        self.gauges.active_tasks.set(state.active_tasks as f64);
    }

    pub async fn record_task_completion(&self, duration: Duration) {
        self.counters.tasks_completed.increment(1);
        let mut state = self.state.write().await;
        state.active_tasks = state.active_tasks.saturating_sub(1);
        self.gauges.active_tasks.set(state.active_tasks as f64);
        self.histograms.task_duration.record(duration.as_secs_f64());
    }

    pub async fn record_task_failure(&self) {
        self.counters.tasks_failed.increment(1);
        let mut state = self.state.write().await;
        state.failed_tasks += 1;
        state.active_tasks = state.active_tasks.saturating_sub(1);
        self.gauges.active_tasks.set(state.active_tasks as f64);
    }

    // Layer 5: Metric Export
    pub async fn export_metrics(&self) -> Result<String> {
        let state = self.state.read().await;
        let metrics = serde_json::to_string_pretty(&*state)?;
        Ok(metrics)
    }
}

// Implementation for metric groups
impl RuntimeCounters {
    fn new() -> Self {
        Self {
            tasks_started: Counter::noop(),
            tasks_completed: Counter::noop(),
            tasks_failed: Counter::noop(),
            bytes_processed: Counter::noop(),
        }
    }
}

impl RuntimeGauges {
    fn new() -> Self {
        Self {
            active_tasks: Gauge::noop(),
            memory_usage: Gauge::noop(),
            worker_count: Gauge::noop(),
        }
    }
}

impl RuntimeHistograms {
    fn new() -> Self {
        Self {
            task_duration: Histogram::noop(),
            queue_time: Histogram::noop(),
            processing_time: Histogram::noop(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_metrics_recording() {
        let metrics = RuntimeMetrics::new();
        
        metrics.record_task_start().await;
        sleep(Duration::from_millis(100)).await;
        metrics.record_task_completion(Duration::from_millis(100)).await;
        
        let exported = metrics.export_metrics().await.unwrap();
        assert!(exported.contains("total_tasks"));
    }
}

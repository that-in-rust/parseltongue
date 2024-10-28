//! Runtime Performance Metrics
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Metrics Aggregation
//! - MetricsAggregator  (combines all runtime metrics)
//!   ├── Performance summaries
//!   ├── Resource utilization
//!   └── System health
//! 
//! Level 3: Component Metrics
//! - WorkerMetrics     (worker performance)
//!   ├── Task throughput
//!   ├── Queue depths
//!   └── Processing latency
//! 
//! Level 2: Resource Metrics
//! - ResourceMetrics   (resource usage)
//!   ├── Memory tracking
//!   ├── CPU utilization
//!   └── I/O statistics
//! 
//! Level 1 (Base): Core Metric Types
//! - MetricTypes      (foundational types)
//!   ├── Counters
//!   ├── Gauges
//!   └── Histograms

use std::sync::Arc;
use metrics::{Counter, Gauge, Histogram};
use tokio::sync::RwLock;
use crate::core::{error::Result, types::*};

// Design Choice: Using atomic metrics for lock-free updates
#[derive(Debug)]
pub struct RuntimeMetrics {
    /// Tasks processed
    pub tasks_completed: Counter,
    /// Active workers
    pub active_workers: Gauge,
    /// Task latency
    pub task_latency: Histogram,
    /// Memory usage
    pub memory_usage: Gauge,
    /// CPU usage
    pub cpu_usage: Gauge,
}

// ===== Level 2: Resource Metrics =====
// Design Choice: Using RwLock for infrequent updates

/// Resource utilization metrics
pub struct ResourceMetrics {
    /// Memory metrics
    memory: Arc<MemoryMetrics>,
    /// CPU metrics
    cpu: Arc<CpuMetrics>,
    /// I/O metrics
    io: Arc<IoMetrics>,
}

// ===== Level 3: Component Metrics =====
// Design Choice: Using channels for metric updates

/// Worker-specific metrics
pub struct WorkerMetrics {
    /// Tasks started
    pub tasks_started: Counter,
    /// Tasks completed
    pub tasks_completed: Counter,
    /// Tasks failed
    pub tasks_failed: Counter,
    /// Queue depth
    pub queue_depth: Gauge,
    /// Processing time
    pub processing_time: Histogram,
}

// ===== Level 4: Metrics Aggregation =====
// Design Choice: Using builder pattern for configuration

/// Metrics aggregator implementation
pub struct MetricsAggregator {
    /// Runtime metrics
    runtime: Arc<RuntimeMetrics>,
    /// Resource metrics
    resources: Arc<ResourceMetrics>,
    /// Worker metrics
    workers: Arc<RwLock<Vec<Arc<WorkerMetrics>>>>,
}

impl MetricsAggregator {
    /// Creates new metrics aggregator
    pub fn new() -> Self {
        Self {
            runtime: Arc::new(RuntimeMetrics::new()),
            resources: Arc::new(ResourceMetrics::new()),
            workers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Records task completion
    pub fn record_task_completion(&self, duration: Duration) {
        self.runtime.tasks_completed.increment(1);
        self.runtime.task_latency.record(duration);
    }

    /// Updates resource metrics
    pub async fn update_resources(&self) -> Result<()> {
        // Update memory metrics
        let mem_info = sys_info::mem_info()?;
        self.runtime.memory_usage.set(mem_info.total as f64);

        // Update CPU metrics
        let cpu_load = sys_info::loadavg()?;
        self.runtime.cpu_usage.set(cpu_load.one);

        Ok(())
    }

    /// Registers worker metrics
    pub async fn register_worker(&self, metrics: Arc<WorkerMetrics>) {
        self.runtime.active_workers.increment(1.0);
        self.workers.write().await.push(metrics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_metrics_aggregation() {
        let aggregator = MetricsAggregator::new();
        
        // Record some metrics
        aggregator.record_task_completion(Duration::from_millis(100));
        
        // Update resources
        aggregator.update_resources().await.unwrap();
        
        // Register worker
        let worker_metrics = Arc::new(WorkerMetrics::new("test-worker"));
        aggregator.register_worker(worker_metrics).await;
        
        // Verify metrics
        assert_eq!(aggregator.runtime.tasks_completed.get(), 1);
        assert_eq!(aggregator.runtime.active_workers.get(), 1.0);
    }
}

//! Task Performance Metrics
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Task Metrics
//! - TaskMetricsManager (manages task metrics)
//!   ├── Performance tracking
//!   ├── Resource utilization
//!   └── Task statistics
//! 
//! Level 3: Metric Types
//! - TaskMetrics      (task performance)
//! - ResourceMetrics  (resource usage)
//! - QueueMetrics    (queue stats)
//! 
//! Level 2: Implementation
//! - MetricCollector  (collects metrics)
//! - MetricReporter   (reports metrics)
//! - MetricStorage    (stores metrics)
//! 
//! Level 1 (Base): Core Types
//! - TaskStats       (statistics)
//! - MetricValue     (value types)
//! - MetricError     (error types)

use std::sync::Arc;
use tokio::sync::RwLock;
use metrics::{Counter, Gauge, Histogram};
use crate::core::error::Result;

// Design Choice: Using atomic metrics for performance
pub struct TaskMetrics {
    tasks_started: Counter,
    tasks_completed: Counter,
    tasks_failed: Counter,
    active_tasks: Gauge,
    queue_depth: Gauge,
    task_duration: Histogram,
}

impl TaskMetrics {
    pub fn new() -> Self {
        Self {
            tasks_started: Counter::new(),
            tasks_completed: Counter::new(),
            tasks_failed: Counter::new(),
            active_tasks: Gauge::new(),
            queue_depth: Gauge::new(),
            task_duration: Histogram::new(),
        }
    }

    pub fn record_start(&self) {
        self.tasks_started.increment(1);
        self.active_tasks.increment(1.0);
    }

    pub fn record_complete(&self, duration: std::time::Duration) {
        self.tasks_completed.increment(1);
        self.active_tasks.decrement(1.0);
        self.task_duration.record(duration.as_secs_f64());
    }

    pub fn record_failure(&self) {
        self.tasks_failed.increment(1);
        self.active_tasks.decrement(1.0);
    }

    pub fn update_queue_depth(&self, depth: usize) {
        self.queue_depth.set(depth as f64);
    }
}

// Design Choice: Using builder for metric configuration
#[derive(Debug, Clone)]
pub struct TaskMetricsConfig {
    pub enabled: bool,
    pub histogram_buckets: Vec<f64>,
    pub export_interval: std::time::Duration,
}

impl Default for TaskMetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            histogram_buckets: vec![0.001, 0.01, 0.1, 1.0, 10.0],
            export_interval: std::time::Duration::from_secs(10),
        }
    }
}

// Design Choice: Using async traits for metric collection
#[async_trait::async_trait]
pub trait TaskMetricCollector: Send + Sync {
    async fn collect(&self) -> Result<TaskMetricSnapshot>;
    async fn reset(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct TaskMetricSnapshot {
    pub total_tasks: u64,
    pub active_tasks: u64,
    pub failure_rate: f64,
    pub avg_duration: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_task_metrics() {
        let metrics = TaskMetrics::new();
        
        metrics.record_start();
        sleep(std::time::Duration::from_millis(100)).await;
        metrics.record_complete(std::time::Duration::from_millis(100));
        
        assert_eq!(metrics.tasks_completed.get(), 1);
        assert_eq!(metrics.active_tasks.get(), 0.0);
    }
}

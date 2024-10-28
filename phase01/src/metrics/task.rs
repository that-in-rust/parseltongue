//! Task Metrics - Pyramidal Structure
//! Layer 1: Task Interface
//! Layer 2: Performance Tracking
//! Layer 3: Statistics Collection
//! Layer 4: Analysis
//! Layer 5: Export

use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use anyhow::Result;

// Layer 1: Core Types
#[derive(Clone)]
pub struct TaskMetrics {
    inner: Arc<RwLock<TaskStats>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct TaskStats {
    start_time: Option<Instant>,
    task_count: usize,
    total_duration: Duration,
    min_duration: Option<Duration>,
    max_duration: Option<Duration>,
    error_count: usize,
}

// Layer 2: Implementation
impl TaskMetrics {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(TaskStats::default())),
        }
    }

    // Layer 3: Metric Collection
    pub async fn record_task(&self, duration: Duration, success: bool) -> Result<()> {
        let mut stats = self.inner.write().await;
        
        if stats.start_time.is_none() {
            stats.start_time = Some(Instant::now());
        }

        stats.task_count += 1;
        stats.total_duration += duration;

        // Update min duration
        if let Some(min) = stats.min_duration {
            if duration < min {
                stats.min_duration = Some(duration);
            }
        } else {
            stats.min_duration = Some(duration);
        }

        // Update max duration
        if let Some(max) = stats.max_duration {
            if duration > max {
                stats.max_duration = Some(duration);
            }
        } else {
            stats.max_duration = Some(duration);
        }

        if !success {
            stats.error_count += 1;
        }

        Ok(())
    }

    // Layer 4: Analysis
    pub async fn get_average_duration(&self) -> Result<Option<Duration>> {
        let stats = self.inner.read().await;
        if stats.task_count > 0 {
            Ok(Some(stats.total_duration / stats.task_count as u32))
        } else {
            Ok(None)
        }
    }

    pub async fn get_error_rate(&self) -> f64 {
        let stats = self.inner.read().await;
        if stats.task_count > 0 {
            stats.error_count as f64 / stats.task_count as f64
        } else {
            0.0
        }
    }

    // Layer 5: Export
    pub async fn export_json(&self) -> Result<String> {
        let stats = self.inner.read().await;
        Ok(serde_json::to_string_pretty(&*stats)?)
    }
}

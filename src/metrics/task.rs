//! Task Metrics - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Metrics Configuration
//! Layer 3: Task Statistics
//! Layer 4: Performance Analysis
//! Layer 5: Resource Management

use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::Result;
use tokio::sync::RwLock;
use tracing::{debug, info};
use serde::Serialize;

// Layer 1: Core Types
#[derive(Debug)]
pub struct TaskMetrics {
    state: Arc<RwLock<TaskState>>,
}

#[derive(Debug, Default, Serialize)]
struct TaskState {
    operation_count: usize,
    error_count: usize,
    total_duration: Duration,
    min_duration: Option<Duration>,
    max_duration: Option<Duration>,
    avg_duration: Option<Duration>,
    last_operation: Option<Instant>,
}

// Layer 2: Implementation
impl TaskMetrics {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(TaskState::default())),
        }
    }

    // Layer 3: Metrics Recording
    pub async fn record_operation(&self, duration: Duration) -> Result<()> {
        let mut state = self.state.write().await;
        state.operation_count += 1;
        state.total_duration += duration;
        state.last_operation = Some(Instant::now());

        // Update min duration
        state.min_duration = Some(match state.min_duration {
            Some(min) => min.min(duration),
            None => duration,
        });

        // Update max duration
        state.max_duration = Some(match state.max_duration {
            Some(max) => max.max(duration),
            None => duration,
        });

        // Update average duration
        state.avg_duration = Some(state.total_duration / state.operation_count as u32);

        debug!(
            "Recorded operation: {}ms (avg: {}ms)",
            duration.as_millis(),
            state.avg_duration.unwrap().as_millis()
        );

        Ok(())
    }

    pub async fn record_error(&self, context: &str) -> Result<()> {
        let mut state = self.state.write().await;
        state.error_count += 1;
        debug!("Recorded error: {}", context);
        Ok(())
    }

    // Layer 4: Statistics
    pub async fn get_statistics(&self) -> Result<TaskStatistics> {
        let state = self.state.read().await;
        Ok(TaskStatistics {
            operation_count: state.operation_count,
            error_count: state.error_count,
            avg_duration: state.avg_duration,
            min_duration: state.min_duration,
            max_duration: state.max_duration,
        })
    }

    // Layer 5: Resource Management
    pub async fn shutdown(&self) -> Result<()> {
        let state = self.state.read().await;
        info!(
            "Task metrics: {} operations, {} errors, avg duration: {:?}",
            state.operation_count,
            state.error_count,
            state.avg_duration,
        );
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct TaskStatistics {
    pub operation_count: usize,
    pub error_count: usize,
    pub avg_duration: Option<Duration>,
    pub min_duration: Option<Duration>,
    pub max_duration: Option<Duration>,
}

impl Default for TaskMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_task_metrics() -> Result<()> {
        let metrics = TaskMetrics::new();
        
        metrics.record_operation(Duration::from_millis(100)).await?;
        sleep(Duration::from_millis(10)).await;
        metrics.record_operation(Duration::from_millis(200)).await?;
        
        let stats = metrics.get_statistics().await?;
        assert_eq!(stats.operation_count, 2);
        assert!(stats.avg_duration.unwrap() >= Duration::from_millis(100));
        
        Ok(())
    }
}

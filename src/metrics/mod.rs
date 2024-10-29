//! Metrics Core - Pyramidal Structure
//! Layer 1: Core Types & Exports
//! Layer 2: Metrics Configuration
//! Layer 3: Metrics Collection
//! Layer 4: Export & Reporting
//! Layer 5: Resource Management

pub mod console;
pub mod task;

use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::Result;
use tokio::sync::RwLock;
use tracing::{debug, info};
use serde::Serialize;

use crate::Config;

// Layer 1: Core Types
#[derive(Debug)]
pub struct MetricsManager {
    state: Arc<RwLock<MetricsState>>,
    #[cfg(feature = "metrics")]
    console: console::ConsoleMetrics,
    task: task::TaskMetrics,
}

#[derive(Debug, Default, Serialize)]
struct MetricsState {
    start_time: Option<Instant>,
    total_bytes: u64,
    processed_files: usize,
    errors: usize,
    duration: Option<Duration>,
}

// Layer 2: Implementation
impl MetricsManager {
    pub fn new() -> Self {
        let state = Arc::new(RwLock::new(MetricsState {
            start_time: Some(Instant::now()),
            ..Default::default()
        }));

        Self {
            state,
            #[cfg(feature = "metrics")]
            console: console::ConsoleMetrics::new(),
            task: task::TaskMetrics::new(),
        }
    }

    // Layer 3: Metrics Recording
    pub async fn record_file_processed(&self, size: u64, duration: Duration) -> Result<()> {
        let mut state = self.state.write().await;
        state.total_bytes += size;
        state.processed_files += 1;
        
        self.task.record_operation(duration).await?;
        
        #[cfg(feature = "metrics")]
        self.console.record_file_processed(size).await?;

        Ok(())
    }

    pub async fn record_error(&self, context: &str) -> Result<()> {
        let mut state = self.state.write().await;
        state.errors += 1;
        
        self.task.record_error(context).await?;
        
        #[cfg(feature = "metrics")]
        self.console.record_error().await?;

        Ok(())
    }

    // Layer 4: Metrics Export
    pub async fn export_metrics(&self) -> Result<String> {
        let mut state = self.state.write().await;
        if let Some(start) = state.start_time {
            state.duration = Some(start.elapsed());
        }
        
        let metrics = serde_json::to_string_pretty(&*state)?;
        Ok(metrics)
    }

    // Layer 5: Resource Management
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down metrics manager");
        
        #[cfg(feature = "metrics")]
        self.console.shutdown().await?;
        
        self.task.shutdown().await?;
        
        Ok(())
    }
}

impl Default for MetricsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_metrics_recording() -> Result<()> {
        let metrics = MetricsManager::new();
        
        metrics.record_file_processed(100, Duration::from_millis(50)).await?;
        sleep(Duration::from_millis(10)).await;
        
        let exported = metrics.export_metrics().await?;
        assert!(exported.contains("total_bytes"));
        assert!(exported.contains("processed_files"));
        
        Ok(())
    }
}

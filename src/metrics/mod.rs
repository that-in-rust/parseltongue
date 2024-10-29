//! Metrics Core - Pyramidal Structure
//! Layer 1: Core Types & Exports
//! Layer 2: Metrics Configuration
//! Layer 3: Metrics Collection
//! Layer 4: Export & Reporting
//! Layer 5: Resource Management

pub mod console;
pub mod task;

use std::sync::Arc;
use std::time::{Duration, SystemTime};
use anyhow::{Context, Result};
use tokio::sync::RwLock;
use tracing::{debug, info};
use serde::{Serialize, Deserialize};

// Layer 1: Core Types
#[derive(Debug)]
pub struct MetricsManager {
    state: Arc<RwLock<MetricsState>>,
    #[cfg(feature = "metrics")]
    console: console::ConsoleMetrics,
    task: task::TaskMetrics,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct MetricsState {
    #[serde(with = "time_serde")]
    start_time: SystemTime,
    total_bytes: u64,
    processed_files: usize,
    errors: usize,
    #[serde(with = "duration_serde")]
    duration: Option<Duration>,
}

// Layer 2: Implementation
impl MetricsManager {
    pub fn new() -> Self {
        let state = Arc::new(RwLock::new(MetricsState {
            start_time: SystemTime::now(),
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
        
        self.task.record_operation(duration).await
            .context("Failed to record task metrics")?;
        
        #[cfg(feature = "metrics")]
        self.console.record_file_processed(size).await
            .context("Failed to record console metrics")?;

        Ok(())
    }

    pub async fn record_error(&self, context: &str) -> Result<()> {
        let mut state = self.state.write().await;
        state.errors += 1;
        
        self.task.record_error(context).await
            .context("Failed to record task error")?;
        
        #[cfg(feature = "metrics")]
        self.console.record_error().await
            .context("Failed to record console error")?;

        Ok(())
    }

    // Layer 4: Metrics Export
    pub async fn export_metrics(&self) -> Result<String> {
        let mut state = self.state.write().await;
        state.duration = Some(state.start_time.elapsed()
            .context("Failed to calculate duration")?);
        
        serde_json::to_string_pretty(&*state)
            .context("Failed to serialize metrics")
    }

    // Layer 5: Resource Management
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down metrics manager");
        
        #[cfg(feature = "metrics")]
        self.console.shutdown().await
            .context("Failed to shutdown console metrics")?;
        
        self.task.shutdown().await
            .context("Failed to shutdown task metrics")?;
        
        Ok(())
    }
}

// Custom serialization modules
mod time_serde {
    use super::*;
    use serde::{Serializer, Deserializer};
    use std::time::UNIX_EPOCH;

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + Duration::from_secs(secs))
    }
}

mod duration_serde {
    use super::*;
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match duration {
            Some(d) => serializer.serialize_u64(d.as_millis() as u64),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms: Option<u64> = Option::deserialize(deserializer)?;
        Ok(ms.map(Duration::from_millis))
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

    #[tokio::test]
    async fn test_error_recording() -> Result<()> {
        let metrics = MetricsManager::new();
        metrics.record_error("test error").await?;
        
        let exported = metrics.export_metrics().await?;
        assert!(exported.contains("errors"));
        
        Ok(())
    }
}

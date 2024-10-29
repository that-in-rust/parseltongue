//! Runtime Core - Pyramidal Structure
//! Layer 1: Core Types & Exports
//! Layer 2: Runtime Configuration
//! Layer 3: Worker Management
//! Layer 4: Resource Control
//! Layer 5: Shutdown Handling

pub mod worker;
pub mod shutdown;
pub mod metrics;

use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::sync::Semaphore;
use tracing::{debug, info};

use crate::Config;
use worker::WorkerPool;
use shutdown::ShutdownManager;
use metrics::RuntimeMetrics;

// Layer 1: Core Types
#[derive(Debug)]
pub struct RuntimeManager {
    config: RuntimeConfig,
    workers: Arc<WorkerPool>,
    shutdown: Arc<ShutdownManager>,
    metrics: Arc<RuntimeMetrics>,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub worker_threads: usize,
    pub shutdown_timeout: std::time::Duration,
}

// Layer 2: Implementation
impl RuntimeManager {
    pub fn new(config: &Config) -> Result<Self> {
        let runtime_config = RuntimeConfig {
            worker_threads: config.workers,
            shutdown_timeout: config.shutdown_timeout,
        };

        let metrics = Arc::new(RuntimeMetrics::new(config.workers));
        let shutdown = Arc::new(ShutdownManager::new(runtime_config.clone()));
        let workers = Arc::new(WorkerPool::new(
            config.workers,
            Arc::clone(&metrics),
            Arc::clone(&shutdown),
        ));

        Ok(Self {
            config: runtime_config,
            workers,
            shutdown,
            metrics,
        })
    }

    // Layer 3: Task Management
    pub async fn spawn<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<T>> + Send + 'static,
        T: Send + 'static,
    {
        self.workers.spawn(f).await
    }

    // Layer 4: Status & Metrics
    pub async fn active_workers(&self) -> usize {
        self.workers.active_count().await
    }

    pub async fn get_metrics(&self) -> Result<metrics::RuntimeStatistics> {
        self.metrics.get_statistics().await
    }

    // Layer 5: Shutdown
    pub async fn shutdown(self) -> Result<()> {
        info!("Initiating runtime shutdown");
        
        // Shutdown sequence
        self.shutdown.initiate().await
            .context("Failed to initiate shutdown")?;
        
        self.workers.shutdown().await
            .context("Failed to shutdown worker pool")?;
        
        self.metrics.shutdown().await
            .context("Failed to shutdown metrics")?;

        info!("Runtime shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_runtime_lifecycle() -> Result<()> {
        let config = Config::builder()
            .workers(2)
            .shutdown_timeout(Duration::from_secs(1))
            .build()?;

        let runtime = RuntimeManager::new(&config)?;
        
        // Test task spawning
        let result = runtime.spawn(|| async {
            sleep(Duration::from_millis(100)).await;
            Ok::<_, anyhow::Error>(42)
        }).await?;
        
        assert_eq!(result, 42);
        
        runtime.shutdown().await?;
        Ok(())
    }
}

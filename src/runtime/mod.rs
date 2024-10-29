//! Runtime Core - Pyramidal Structure
//! Layer 1: Module Organization & Exports
//! Layer 2: Runtime Configuration
//! Layer 3: Runtime Management
//! Layer 4: Resource Coordination
//! Layer 5: Metrics & Monitoring

pub mod worker;
pub mod shutdown;
pub mod metrics;

use std::sync::Arc;
use tokio::runtime::Runtime;
use anyhow::{Context, Result};
use tracing::{info, warn};

use crate::Config;
use worker::WorkerPool;
use shutdown::ShutdownManager;
use metrics::RuntimeMetrics;

// Layer 1: Core Runtime Types
#[derive(Debug)]
pub struct RuntimeManager {
    runtime: Runtime,
    workers: Arc<WorkerPool>,
    shutdown: Arc<ShutdownManager>,
    #[cfg(feature = "metrics")]
    metrics: Arc<RuntimeMetrics>,
}

// Layer 2: Configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub worker_threads: usize,
    pub shutdown_timeout: std::time::Duration,
}

// Layer 3: Implementation
impl RuntimeManager {
    pub fn new(config: &Config) -> Result<Self> {
        let runtime_config = RuntimeConfig {
            worker_threads: config.workers,
            shutdown_timeout: config.shutdown_timeout,
        };

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(runtime_config.worker_threads)
            .enable_all()
            .build()
            .context("Failed to create Tokio runtime")?;

        let workers = Arc::new(WorkerPool::new(runtime_config.clone())?);
        let shutdown = Arc::new(ShutdownManager::new(runtime_config.clone()));
        
        #[cfg(feature = "metrics")]
        let metrics = Arc::new(RuntimeMetrics::new());

        Ok(Self {
            runtime,
            workers,
            shutdown,
            #[cfg(feature = "metrics")]
            metrics,
        })
    }

    // Layer 4: Runtime Control
    pub async fn start(&self) -> Result<()> {
        info!("Starting runtime with {} workers", self.workers.count());
        self.workers.start().await?;
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<()> {
        info!("Initiating runtime shutdown");
        self.shutdown.initiate().await?;
        self.workers.shutdown().await?;
        Ok(())
    }

    // Layer 5: Resource Access
    pub fn workers(&self) -> Arc<WorkerPool> {
        Arc::clone(&self.workers)
    }

    pub fn shutdown_manager(&self) -> Arc<ShutdownManager> {
        Arc::clone(&self.shutdown)
    }

    #[cfg(feature = "metrics")]
    pub fn metrics(&self) -> Arc<RuntimeMetrics> {
        Arc::clone(&self.metrics)
    }
}

impl Drop for RuntimeManager {
    fn drop(&mut self) {
        if !self.shutdown.is_complete() {
            warn!("RuntimeManager dropped before clean shutdown");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_runtime_lifecycle() {
        let config = Config::builder()
            .workers(2)
            .shutdown_timeout(Duration::from_secs(1))
            .build()
            .unwrap();

        let runtime = RuntimeManager::new(&config).unwrap();
        runtime.start().await.unwrap();
        runtime.shutdown().await.unwrap();
    }
}

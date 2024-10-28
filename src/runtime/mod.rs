//! Runtime Module - Pyramidal Structure
//! Layer 1: Public Interface
//! Layer 2: Runtime Configuration
//! Layer 3: Worker Management
//! Layer 4: Task Scheduling
//! Layer 5: Resource Management

use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::{oneshot, Semaphore};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering}; // Add for atomic operations
use anyhow::Result;
use tracing::{info, warn};
use tokio::time::sleep;  // Add for sleep operations

// Layer 1: Core Types
pub struct RuntimeManager {
    runtime: Runtime,
    shutdown_tx: oneshot::Sender<()>,
    worker_pool: Arc<Semaphore>,
    config: RuntimeConfig,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub worker_threads: usize,
    pub shutdown_timeout: Duration,
}

// Layer 2: Implementation
impl RuntimeManager {
    pub fn new(config: RuntimeConfig) -> Result<Self> {
        info!("Initializing runtime with {} worker threads", config.worker_threads);

        let runtime = Builder::new_multi_thread()
            .worker_threads(config.worker_threads)
            .enable_all()
            .thread_name("zip-worker")
            .thread_stack_size(3 * 1024 * 1024) // 3MB stack
            .build()?;

        let (shutdown_tx, _) = oneshot::channel();
        let worker_pool = Arc::new(Semaphore::new(config.worker_threads));

        Ok(Self {
            runtime,
            shutdown_tx,
            worker_pool,
            config,
        })
    }

    // Layer 3: Task Management
    pub async fn spawn_task<F, T>(&self, task: F) -> Result<T>
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let _permit = self.worker_pool.acquire().await?;
        let result = self.runtime.spawn(task).await?;
        Ok(result)
    }

    // Layer 4: Worker Management
    pub fn get_worker_count(&self) -> usize {
        self.config.worker_threads
    }

    pub async fn is_overloaded(&self) -> bool {
        self.worker_pool.available_permits() == 0
    }

    // Layer 5: Cleanup
    pub async fn shutdown(self) -> Result<()> {
        info!("Initiating runtime shutdown");

        // Signal shutdown
        let _ = self.shutdown_tx.send(());

        // Wait for tasks to complete
        let timeout = self.config.shutdown_timeout;
        tokio::time::timeout(timeout, async {
            while self.worker_pool.available_permits() != self.config.worker_threads {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .map_err(|_| {
            warn!("Shutdown timeout after {:?}", timeout);
            anyhow::anyhow!("Shutdown timeout")
        })?;

        info!("Runtime shutdown complete");
        Ok(())
    }
}

pub mod worker;
pub mod shutdown;

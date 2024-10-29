// Level 4: Runtime Management and Coordination
// - Manages Tokio runtime configuration and lifecycle
// - Coordinates worker pools and resource limits
// - Handles graceful shutdown across all subsystems

use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::{broadcast, Semaphore};
use crate::error::Result;
use crate::config::Config;

pub mod shutdown;
pub mod worker;

pub struct RuntimeManager {
    runtime: Runtime,
    worker_pool: Arc<worker::WorkerPool>,
    shutdown_signal: broadcast::Sender<()>,
    backpressure: Arc<Semaphore>,
}

impl RuntimeManager {
    pub fn new(config: &Config) -> Result<Self> {
        // Level 3: Configure and build Tokio runtime
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(config.workers)
            .thread_name("parseltongue-worker")
            .thread_stack_size(3 * 1024 * 1024)
            .enable_all()
            .build()?;

        // Level 3: Initialize worker pool
        let worker_pool = Arc::new(worker::WorkerPool::new(config.workers));
        
        // Level 3: Setup shutdown coordination
        let (shutdown_signal, _) = broadcast::channel(1);
        
        // Level 3: Configure backpressure control
        let backpressure = Arc::new(Semaphore::new(config.workers * 2));

        Ok(Self {
            runtime,
            worker_pool,
            shutdown_signal,
            backpressure,
        })
    }

    // Level 2: Task spawning with backpressure
    pub async fn spawn<F>(&self, task: F) -> Result<()> 
    where
        F: Future<Output = Result<()>> + Send + 'static,
    {
        let permit = self.backpressure.acquire().await?;
        let worker_pool = self.worker_pool.clone();
        let mut shutdown = self.shutdown_signal.subscribe();

        self.runtime.spawn(async move {
            tokio::select! {
                res = task => {
                    if let Err(e) = res {
                        tracing::error!("Task error: {:?}", e);
                    }
                }
                _ = shutdown.recv() => {
                    tracing::info!("Task cancelled due to shutdown");
                }
            }
            drop(permit);
        });

        Ok(())
    }

    // Level 2: Graceful shutdown
    pub async fn shutdown(self) -> Result<()> {
        // Signal all tasks to stop
        let _ = self.shutdown_signal.send(());
        
        // Wait for worker pool cleanup
        self.worker_pool.shutdown().await?;
        
        // Shutdown runtime
        self.runtime.shutdown_timeout(Duration::from_secs(5));
        
        Ok(())
    }
} 
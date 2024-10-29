// Level 4: Runtime Management & Coordination
// - Manages overall runtime configuration and lifecycle
// - Coordinates worker pools and shutdown
// - Handles resource limits and backpressure

use tokio::runtime::Runtime;
use std::sync::Arc;

pub mod worker;
pub mod shutdown;

pub struct RuntimeManager {
    runtime: Arc<Runtime>,
    worker_pool: worker::WorkerPool,
    shutdown: shutdown::ShutdownManager,
}

impl RuntimeManager {
    // Level 3: Runtime Configuration
    pub fn new(worker_threads: usize) -> Self {
        let runtime = Arc::new(Runtime::builder()
            .worker_threads(worker_threads)
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime"));
            
        let worker_pool = worker::WorkerPool::new(worker_threads * 2);
        let shutdown = shutdown::ShutdownManager::new();
        
        Self { runtime, worker_pool, shutdown }
    }

    // Level 2: Task Management
    pub fn spawn_worker<F>(&self, future: F) 
    where 
        F: std::future::Future<Output = ()> + Send + 'static 
    {
        self.worker_pool.spawn(future);
    }

    // Level 1: Lifecycle Management
    pub async fn shutdown(self) {
        self.shutdown.shutdown();
        self.worker_pool.shutdown().await;
        // Runtime shutdown happens implicitly when dropped
    }
} 
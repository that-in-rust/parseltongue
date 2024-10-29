// Level 4: Runtime Management
// - Coordinates async tasks
// - Manages worker pools
// - Handles shutdown
// - Tracks metrics

use tokio::runtime::Runtime;
use std::sync::Arc;
use crate::core::error::Result;
use metrics::{counter, gauge};

pub struct RuntimeManager {
    runtime: Arc<Runtime>,
    worker_pool: Arc<crate::core::runtime::worker::WorkerPool>,
}

impl RuntimeManager {
    pub fn new(worker_threads: usize) -> Result<Self> {
        let runtime = Arc::new(Runtime::builder()
            .worker_threads(worker_threads)
            .enable_all()
            .build()?);
            
        let worker_pool = Arc::new(crate::core::runtime::worker::WorkerPool::new(worker_threads));
        
        counter!("runtime.init").increment(1);
        gauge!("runtime.workers").set(worker_threads as f64);
        
        Ok(Self { runtime, worker_pool })
    }

    pub fn spawn<F>(&self, future: F) -> Result<()> 
    where 
        F: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let worker_pool = self.worker_pool.clone();
        self.runtime.spawn(async move {
            worker_pool.spawn(|| future).await
        });
        Ok(())
    }
} 
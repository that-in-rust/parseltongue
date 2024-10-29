// Level 4: Worker Pool Management
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::core::error::Result;

pub struct WorkerPool {
    sem: Arc<Semaphore>,
    count: Arc<AtomicUsize>,
}

impl WorkerPool {
    pub fn new(limit: usize) -> Self {
        Self {
            sem: Arc::new(Semaphore::new(limit)),
            count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub async fn spawn<F, Fut>(&self, future: F) -> Result<()> 
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let sem = self.sem.clone();
        let count = self.count.clone();
        
        tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            count.fetch_add(1, Ordering::SeqCst);
            let result = future().await;
            count.fetch_sub(1, Ordering::SeqCst);
            result
        });
        
        Ok(())
    }
} 
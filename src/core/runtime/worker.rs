// Level 4: Worker Pool Management
// - Manages worker thread lifecycle
// - Implements task scheduling
// - Handles backpressure
// - Collects worker metrics

use tokio::sync::Semaphore;
use std::sync::Arc;
use crate::core::error::Result;

// Level 3: Worker Pool
pub struct WorkerPool {
    semaphore: Arc<Semaphore>,
    task_count: std::sync::atomic::AtomicU64,
}

impl WorkerPool {
    // Level 2: Pool Management
    pub fn new(max_workers: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_workers)),
            task_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    // Level 1: Task Operations
    pub fn spawn<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let sem = self.semaphore.clone();
        let count = &self.task_count;
        
        tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            future.await;
            count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        });
    }

    pub async fn shutdown(&self) {
        // Wait for all tasks to complete
        while self.task_count.load(std::sync::atomic::Ordering::SeqCst) > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
} 
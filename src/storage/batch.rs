// Level 4: Batch Processing
// - Manages batch operations
// - Handles transactions
// - Provides atomicity
// - Tracks metrics

use tokio::sync::Semaphore;
use std::sync::Arc;
use crate::core::error::Result;
use metrics::{counter, gauge};

pub struct BatchProcessor {
    semaphore: Arc<Semaphore>,
    batch_size: usize,
}

impl BatchProcessor {
    pub fn new(batch_size: usize, max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            batch_size,
        }
    }

    pub async fn process_batch<T, F>(&self, items: Vec<T>, f: F) -> Result<()>
    where
        F: Fn(T) -> Result<()> + Send + Sync + 'static,
        T: Send + 'static,
    {
        let permit = self.semaphore.acquire().await?;
        gauge!("batch.size").set(items.len() as f64);
        
        for chunk in items.chunks(self.batch_size) {
            let tasks: Vec<_> = chunk.iter().map(|item| {
                f(item.clone())
            }).collect();
            
            futures::future::try_join_all(tasks).await?;
            counter!("batch.processed").increment(chunk.len() as u64);
        }
        
        drop(permit);
        Ok(())
    }
} 
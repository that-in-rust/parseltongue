// Level 4: Resource Management
// - Manages system resources
// - Handles cleanup
// - Tracks allocations
// - Provides metrics

use tokio::sync::Semaphore;
use std::sync::Arc;
use metrics::{counter, gauge};
use crate::core::error::Result;

pub struct ResourceManager {
    memory_limit: usize,
    semaphore: Arc<Semaphore>,
    current_usage: std::sync::atomic::AtomicUsize,
}

impl ResourceManager {
    pub fn new(memory_limit: usize) -> Self {
        let semaphore = Arc::new(Semaphore::new(memory_limit));
        gauge!("resource.memory.limit").set(memory_limit as f64);
        
        Self {
            memory_limit,
            semaphore,
            current_usage: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub async fn allocate(&self, size: usize) -> Result<ResourceGuard> {
        let permits = (size + 1023) / 1024; // Round up to nearest KB
        let _permit = self.semaphore.acquire_many(permits as u32).await?;
        
        self.current_usage.fetch_add(size, std::sync::atomic::Ordering::SeqCst);
        gauge!("resource.memory.used").set(self.current_usage.load(std::sync::atomic::Ordering::SeqCst) as f64);
        
        Ok(ResourceGuard::new(self.current_usage.clone(), size))
    }
}

pub struct ResourceGuard {
    usage_counter: std::sync::atomic::AtomicUsize,
    size: usize,
}

impl ResourceGuard {
    fn new(usage_counter: std::sync::atomic::AtomicUsize, size: usize) -> Self {
        Self { usage_counter, size }
    }
}

impl Drop for ResourceGuard {
    fn drop(&mut self) {
        self.usage_counter.fetch_sub(self.size, std::sync::atomic::Ordering::SeqCst);
        counter!("resource.deallocations").increment(1);
    }
} 
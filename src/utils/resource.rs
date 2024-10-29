// Level 4: Resource Management
// - Manages resource lifecycle
// - Implements pooling
// - Handles limits
// - Provides metrics

use tokio::sync::{Semaphore, OwnedSemaphorePermit};
use std::sync::Arc;
use crate::core::error::Result;

// Level 3: Resource Pool
pub struct ResourcePool<T> {
    resources: Vec<Arc<T>>,
    semaphore: Arc<Semaphore>,
}

impl<T> ResourcePool<T> {
    // Level 2: Pool Operations
    pub async fn acquire(&self) -> Result<(Arc<T>, OwnedSemaphorePermit)> {
        let permit = self.semaphore.clone()
            .acquire_owned()
            .await
            .map_err(|_| crate::core::error::Error::Processing {
                msg: "Failed to acquire resource".into()
            })?;
            
        let resource = self.resources[permit.as_u32() as usize].clone();
        Ok((resource, permit))
    }

    // Level 1: Lifecycle Management
    pub fn new(resources: Vec<T>, max_concurrent: usize) -> Self {
        Self {
            resources: resources.into_iter().map(Arc::new).collect(),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }
} 
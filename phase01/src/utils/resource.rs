//! Resource Management Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Resource Coordination
//! - ResourceManager   (manages resources)
//! - ResourceMetrics   (tracks resource usage)
//! - LoadBalancer      (balances resources)
//! 
//! Level 3: Resource Management
//! - ResourcePool      (manages resource pools)
//! - ResourceMonitor   (monitors resources)
//! - ResourceLimiter   (enforces limits)
//! 
//! Level 2: Resource Implementation
//! - Resource          (resource implementation)
//! - ResourceState     (resource lifecycle)
//! - ResourceMetrics   (resource stats)
//! 
//! Level 1 (Base): Core Resource Types
//! - ResourceConfig    (resource configuration)
//! - ResourceMetrics   (resource metrics)
//! - ResourceError     (resource errors)

use std::sync::Arc;
use tokio::sync::{Semaphore, Mutex};
use metrics::{Counter, Gauge};
use crate::core::{error::{Error, Result}, types::*};

// ===== Level 1: Core Resource Types =====
// Design Choice: Using generics for type safety

/// Resource configuration
#[derive(Debug, Clone)]
pub struct ResourceConfig {
    /// Maximum resources
    pub max_resources: usize,
    /// Resource timeout
    pub timeout: std::time::Duration,
    /// Enable metrics
    pub metrics_enabled: bool,
}

// ===== Level 2: Resource Implementation =====
// Design Choice: Using async traits for resources

/// Resource pool implementation
pub struct ResourcePool<T> {
    /// Available resources
    resources: Arc<Mutex<Vec<T>>>,
    /// Resource semaphore
    semaphore: Arc<Semaphore>,
    /// Pool configuration
    config: ResourceConfig,
    /// Pool metrics
    metrics: ResourceMetrics,
}

impl<T> ResourcePool<T>
where
    T: Send + 'static,
{
    /// Creates new resource pool
    pub fn new(config: ResourceConfig) -> Self {
        let resources = Arc::new(Mutex::new(Vec::with_capacity(config.max_resources)));
        let semaphore = Arc::new(Semaphore::new(config.max_resources));
        let metrics = ResourceMetrics::new();

        Self {
            resources,
            semaphore,
            config,
            metrics,
        }
    }

    /// Acquires resource from pool
    pub async fn acquire(&self) -> Result<ResourceGuard<T>> {
        let _permit = self.semaphore.acquire().await?;
        
        let mut resources = self.resources.lock().await;
        let resource = resources.pop()
            .ok_or_else(|| Error::ResourceLimit("No resources available".into()))?;

        self.metrics.active_resources.increment(1.0);
        
        Ok(ResourceGuard {
            resource: Some(resource),
            pool: self.clone(),
        })
    }

    /// Returns resource to pool
    async fn release(&self, resource: T) {
        self.resources.lock().await.push(resource);
        self.metrics.active_resources.decrement(1.0);
    }
}

// ===== Level 3: Resource Management =====
// Design Choice: Using RAII for resource management

/// Resource guard for automatic return
pub struct ResourceGuard<T> {
    /// Managed resource
    resource: Option<T>,
    /// Owner pool
    pool: ResourcePool<T>,
}

impl<T> Drop for ResourceGuard<T>
where
    T: Send + 'static,
{
    fn drop(&mut self) {
        if let Some(resource) = self.resource.take() {
            let pool = self.pool.clone();
            
            tokio::spawn(async move {
                pool.release(resource).await;
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resource_pool() {
        let config = ResourceConfig {
            max_resources: 10,
            timeout: std::time::Duration::from_secs(1),
            metrics_enabled: true,
        };

        let pool = ResourcePool::new(config);
        
        // Add test resource
        pool.resources.lock().await.push(42);
        
        // Test resource acquisition
        let guard = pool.acquire().await.unwrap();
        assert_eq!(*guard.resource.as_ref().unwrap(), 42);
        
        // Resource should be returned to pool on drop
        drop(guard);
        
        // Allow time for async release
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        assert_eq!(pool.metrics.active_resources.get(), 0.0);
    }
}


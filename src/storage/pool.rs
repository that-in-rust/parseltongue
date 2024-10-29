//! Connection Pool Management - Pyramidal Structure
//! Layer 1: Pool Types & Configuration
//! Layer 2: Pool Management
//! Layer 3: Connection Handling
//! Layer 4: Resource Management
//! Layer 5: Metrics Integration

use std::sync::Arc;
use tokio::sync::{Semaphore, OwnedSemaphorePermit};
use anyhow::{Context, Result};
use tracing::{debug, warn};

use crate::metrics::MetricsManager;
use super::sled::SledStorage;

// Layer 1: Core Types
#[derive(Debug)]
pub struct ConnectionPool {
    storage: Arc<SledStorage>,
    semaphore: Arc<Semaphore>,
    metrics: Arc<MetricsManager>,
    config: PoolConfig,
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_connections: usize,
    pub acquire_timeout: std::time::Duration,
}

// Layer 2: Pool Implementation
impl ConnectionPool {
    pub fn new(storage: Arc<SledStorage>, config: PoolConfig, metrics: Arc<MetricsManager>) -> Self {
        Self {
            storage,
            semaphore: Arc::new(Semaphore::new(config.max_connections)),
            metrics,
            config,
        }
    }

    // Layer 3: Connection Management
    pub async fn acquire(&self) -> Result<PoolGuard> {
        let start = std::time::Instant::now();
        
        let permit = tokio::time::timeout(
            self.config.acquire_timeout,
            self.semaphore.acquire_owned()
        ).await
        .context("Connection acquire timeout")?
        .context("Failed to acquire connection")?;

        self.metrics.record_pool_acquire(start.elapsed()).await?;

        Ok(PoolGuard {
            storage: Arc::clone(&self.storage),
            metrics: Arc::clone(&self.metrics),
            _permit: permit,
        })
    }

    // Layer 4: Pool Status
    pub fn available_connections(&self) -> usize {
        self.semaphore.available_permits()
    }

    pub fn max_connections(&self) -> usize {
        self.config.max_connections
    }

    // Layer 5: Cleanup
    pub async fn shutdown(&self) -> Result<()> {
        debug!("Shutting down connection pool");
        Ok(())
    }
}

// Connection Guard
#[derive(Debug)]
pub struct PoolGuard {
    storage: Arc<SledStorage>,
    metrics: Arc<MetricsManager>,
    _permit: OwnedSemaphorePermit,
}

impl PoolGuard {
    pub async fn execute<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&SledStorage) -> Result<T>,
    {
        let start = std::time::Instant::now();
        let result = f(&self.storage);
        self.metrics.record_operation(start.elapsed()).await?;
        result
    }
}

impl Drop for PoolGuard {
    fn drop(&mut self) {
        debug!("Releasing connection back to pool");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::storage::StorageConfig;

    #[tokio::test]
    async fn test_pool_operations() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config = StorageConfig {
            path: temp_dir.path().to_path_buf(),
            max_concurrent_ops: 2,
            batch_size: 8192,
        };

        let storage = Arc::new(SledStorage::new(&config)?);
        let metrics = Arc::new(MetricsManager::new());
        
        let pool = ConnectionPool::new(
            storage,
            PoolConfig {
                max_connections: 2,
                acquire_timeout: std::time::Duration::from_secs(1),
            },
            metrics,
        );

        let guard = pool.acquire().await?;
        assert_eq!(pool.available_connections(), 1);
        
        drop(guard);
        assert_eq!(pool.available_connections(), 2);
        
        Ok(())
    }
}

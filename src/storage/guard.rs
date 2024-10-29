use std::sync::Arc;
use anyhow::{Context, Result};
use bytes::Bytes;
use tokio::sync::Semaphore;
use tracing::{debug, warn};

use super::sled::SledStorage;

//! Storage Resource Guards - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Guard Configuration
//! Layer 3: Resource Management
//! Layer 4: Storage Operations
//! Layer 5: Cleanup & Safety

// Layer 1: Core Types
#[derive(Debug)]
pub struct StorageGuard {
    storage: Arc<SledStorage>,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

// Layer 2: Implementation
impl StorageGuard {
    pub async fn new(
        storage: Arc<SledStorage>,
        semaphore: Arc<Semaphore>,
    ) -> Result<Self> {
        let permit = semaphore.try_acquire_owned()
            .context("Failed to acquire storage permit")?;

        Ok(Self {
            storage,
            _permit: permit,
        })
    }

    // Layer 3: Storage Operations
    pub async fn store(&self, key: &str, value: Bytes) -> Result<()> {
        debug!("Storing key with guard: {}", key);
        self.storage.insert(key, value).await
    }

    pub async fn get(&self, key: &str) -> Result<Option<Bytes>> {
        debug!("Retrieving key with guard: {}", key);
        self.storage.get(key).await
    }

    // Layer 4: Batch Operations
    pub async fn store_batch(&self, entries: Vec<(String, Bytes)>) -> Result<()> {
        debug!("Storing batch of {} entries", entries.len());
        self.storage.insert_batch(entries).await
    }

    // Layer 5: Resource Management
    pub async fn flush(&self) -> Result<()> {
        debug!("Flushing storage");
        self.storage.flush().await
    }
}

impl Drop for StorageGuard {
    fn drop(&mut self) {
        debug!("Storage guard dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::storage::StorageConfig;

    #[tokio::test]
    async fn test_storage_guard() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config = StorageConfig {
            path: temp_dir.path().to_path_buf(),
            max_concurrent_ops: 2,
            batch_size: 8192,
        };

        let storage = Arc::new(SledStorage::new(&config)?);
        let semaphore = Arc::new(Semaphore::new(1));
        
        let guard = StorageGuard::new(
            Arc::clone(&storage),
            Arc::clone(&semaphore),
        ).await?;
        
        let key = "test_key";
        let value = Bytes::from("test_value");
        
        guard.store(key, value.clone()).await?;
        let retrieved = guard.get(key).await?;
        
        assert_eq!(retrieved.unwrap(), value);
        Ok(())
    }
}

//! Storage Core - Pyramidal Structure
//! Layer 1: Core Types & Exports
//! Layer 2: Storage Configuration
//! Layer 3: Storage Management
//! Layer 4: Resource Control
//! Layer 5: Error Handling

pub mod sled;
pub mod guard;

use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::sync::Semaphore;
use tracing::{debug, error, info};

use crate::Config;
use sled::SledStorage;
use guard::StorageGuard;

// Layer 1: Core Types
#[derive(Debug)]
pub struct StorageManager {
    storage: Arc<SledStorage>,
    semaphore: Arc<Semaphore>,
    config: StorageConfig,
}

// Layer 2: Configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub path: PathBuf,
    pub max_concurrent_ops: usize,
    pub batch_size: usize,
}

// Layer 3: Implementation
impl StorageManager {
    pub async fn new(config: &Config) -> Result<Self> {
        let storage_config = StorageConfig {
            path: config.output_dir.join("db"),
            max_concurrent_ops: config.workers,
            batch_size: config.buffer_size,
        };

        let storage = SledStorage::new(&storage_config)
            .context("Failed to initialize storage")?;

        let semaphore = Arc::new(Semaphore::new(storage_config.max_concurrent_ops));

        Ok(Self {
            storage: Arc::new(storage),
            semaphore,
            config: storage_config,
        })
    }

    // Layer 4: Storage Operations
    pub async fn store(&self, key: &str, value: bytes::Bytes) -> Result<()> {
        debug!("Storing key: {}", key);

        let guard = StorageGuard::new(
            Arc::clone(&self.storage),
            Arc::clone(&self.semaphore),
        ).await?;

        guard.store(key, value).await
    }

    pub async fn get(&self, key: &str) -> Result<Option<bytes::Bytes>> {
        debug!("Retrieving key: {}", key);

        let guard = StorageGuard::new(
            Arc::clone(&self.storage),
            Arc::clone(&self.semaphore),
        ).await?;

        guard.get(key).await
    }

    // Layer 5: Cleanup
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down storage manager");
        self.storage.flush().await?;
        Ok(())
    }
}

impl Drop for StorageManager {
    fn drop(&mut self) {
        debug!("Storage manager dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_storage_manager() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config = Config::builder()
            .output_dir(temp_dir.path())
            .workers(2)
            .buffer_size(8192)
            .build()?;

        let manager = StorageManager::new(&config).await?;
        
        let key = "test_key";
        let value = bytes::Bytes::from("test_value");
        
        manager.store(key, value.clone()).await?;
        let retrieved = manager.get(key).await?;
        
        assert_eq!(retrieved.unwrap(), value);
        Ok(())
    }
}

//! Storage Module - Pyramidal Structure
//! Layer 1: Public Interface & Types
//! Layer 2: Storage Management
//! Layer 3: Connection Pooling
//! Layer 4: Error Handling
//! Layer 5: Resource Management

use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::Semaphore;
use bytes::Bytes;

mod sled;
mod guard;
pub mod pool; // For tokio::sync::Semaphore-based connection pooling

pub use self::sled::SledStorage;

// Layer 1: Core Types
#[derive(Clone)]
pub struct StorageManager {
    inner: Arc<StorageInner>,
}

struct StorageInner {
    storage: SledStorage,
    pool: Arc<Semaphore>,
}

// Layer 2: Configuration
#[derive(Debug)]
pub struct StorageConfig {
    pub path: PathBuf,
    pub pool_size: usize,
    pub batch_size: usize,
}

// Layer 3: Implementation
impl StorageManager {
    pub async fn new(config: StorageConfig) -> Result<Self> {
        let storage = SledStorage::new(&config.path)?;
        let pool = Arc::new(Semaphore::new(config.pool_size));

        Ok(Self {
            inner: Arc::new(StorageInner { storage, pool }),
        })
    }

    // Layer 4: Storage Operations
    pub async fn store(&self, key: &str, value: Bytes) -> Result<()> {
        let _permit = self.inner.pool.acquire().await?;
        self.inner.storage.insert(key, value).await
    }

    pub async fn get(&self, key: &str) -> Result<Option<Bytes>> {
        let _permit = self.inner.pool.acquire().await?;
        self.inner.storage.get(key).await
    }

    // Layer 5: Batch Operations
    pub async fn store_batch(&self, entries: Vec<(String, Bytes)>) -> Result<()> {
        let _permit = self.inner.pool.acquire().await?;
        self.inner.storage.insert_batch(entries).await
    }
}

// Error Types
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Storage initialization failed: {0}")]
    InitError(String),
    
    #[error("Storage operation failed: {0}")]
    OperationError(String),
    
    #[error("Invalid key: {0}")]
    InvalidKey(String),
}

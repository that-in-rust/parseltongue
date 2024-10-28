//! Storage Layer Coordination
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Storage Orchestration
//! - StorageManager   (coordinates storage)
//!   ├── Connection management
//!   ├── Transaction handling
//!   └── Resource cleanup
//! 
//! Level 3: Operation Management
//! - BatchProcessor   (batch operations)
//!   ├── Write batching
//!   ├── Read batching
//!   └── Error handling
//! 
//! Level 2: Storage Implementation
//! - DatabaseStorage  (sled implementation)
//!   ├── ACID guarantees
//!   ├── Concurrent access
//!   └── Resource pooling
//! 
//! Level 1 (Base): Core Storage Types
//! - StorageConfig   (configuration)
//! - StorageMetrics  (metrics collection)
//! - ConnectionPool  (connection management)

pub mod sled;
pub mod guard;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{Semaphore, RwLock};
use bytes::Bytes;
use metrics::{Counter, Gauge, Histogram};
use crate::core::{error::{Error, Result}, types::*};

// Re-export main types
pub use sled::DatabaseStorage;

// Design Choice: Using builder pattern for configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Base storage path
    pub path: PathBuf,
    /// Connection pool size
    pub pool_size: usize,
    /// Batch size for operations
    pub batch_size: usize,
    /// Index configuration
    pub index_config: IndexConfig,
}

impl StorageConfig {
    /// Creates new storage configuration
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            pool_size: 4,
            batch_size: 1000,
            index_config: IndexConfig::default(),
        }
    }

    /// Sets the storage path
    pub fn with_path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = path.as_ref().to_path_buf();
        self
    }
}

/// Storage metrics collection
#[derive(Debug)]
struct StorageMetrics {
    operations: Counter,
    active_connections: Gauge,
    operation_duration: Histogram,
    batch_size: Histogram,
}

impl StorageMetrics {
    fn new() -> Self {
        Self {
            operations: Counter::new(),
            active_connections: Gauge::new(),
            operation_duration: Histogram::new(),
            batch_size: Histogram::new(),
        }
    }
}

// ===== Level 2: Storage Traits =====
// Design Choice: Using async traits for storage operations

/// Async storage interface
#[async_trait::async_trait]
pub trait AsyncStorage: Send + Sync + 'static {
    /// Store data with key
    async fn store(&self, key: &str, data: Bytes) -> Result<()>;
    
    /// Batch store multiple entries
    async fn batch_store(&self, entries: Vec<(String, Bytes)>) -> Result<()>;
    
    /// Retrieve data by key
    async fn get(&self, key: &str) -> Result<Option<Bytes>>;
}

// ===== Level 3: Operation Management =====
// Design Choice: Using connection pooling and batch processing

/// Storage manager implementation
pub struct StorageManager {
    /// Storage backend
    storage: Arc<DatabaseStorage>,
    /// Index manager
    index: Arc<IndexManager>,
    /// Connection pool
    pool: Arc<Semaphore>,
    /// Batch processor
    batch_processor: Arc<BatchProcessor>,
    /// Storage metrics
    metrics: StorageMetrics,
}

impl StorageManager {
    /// Creates new storage manager
    pub async fn new(config: StorageConfig) -> Result<Self> {
        let storage = Arc::new(DatabaseStorage::new(&config)?);
        let index = Arc::new(IndexManager::new(storage.clone(), config.index_config)?);
        let pool = Arc::new(Semaphore::new(config.pool_size));
        let batch_processor = Arc::new(BatchProcessor::new(config.batch_size));
        let metrics = StorageMetrics::new();

        Ok(Self {
            storage,
            index,
            pool,
            batch_processor,
            metrics,
        })
    }

    /// Opens storage at path
    pub async fn open(path: impl AsRef<Path>) -> Result<Self> {
        let config = StorageConfig::new(path);
        Self::new(config).await
    }

    /// Stores data with key
    pub async fn store(&self, key: String, data: Bytes) -> Result<()> {
        let _permit = self.pool.acquire().await?;
        let start = std::time::Instant::now();

        self.metrics.active_connections.increment(1.0);
        
        let result = self.storage.store(&key, data.clone()).await;
        
        if result.is_ok() {
            self.index.update(&key, IndexEntry::new(&key, &data)).await?;
            self.metrics.operations.increment(1);
            self.metrics.operation_duration.record(start.elapsed());
        }

        self.metrics.active_connections.decrement(1.0);
        result
    }

    /// Batch stores multiple entries
    pub async fn batch_store(&self, entries: Vec<(String, Bytes)>) -> Result<()> {
        let _permit = self.pool.acquire().await?;
        let start = std::time::Instant::now();

        self.metrics.active_connections.increment(1.0);
        self.metrics.batch_size.record(entries.len() as f64);

        let result = self.batch_processor.process(entries, |batch| {
            async move {
                self.storage.batch_store(batch).await?;
                Ok(())
            }
        }).await;

        self.metrics.active_connections.decrement(1.0);
        self.metrics.operation_duration.record(start.elapsed());
        
        result
    }
}

// ===== Level 4: Storage Orchestration =====
// Design Choice: Using batch processing for efficiency

/// Batch processor implementation
struct BatchProcessor {
    batch_size: usize,
}

impl BatchProcessor {
    fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }

    async fn process<T, F, Fut>(&self, items: Vec<T>, f: F) -> Result<()>
    where
        F: Fn(Vec<T>) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        for chunk in items.chunks(self.batch_size) {
            f(chunk.to_vec()).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_storage_operations() {
        let temp_dir = TempDir::new().unwrap();
        
        // Test both ways of creating storage
        let storage1 = StorageManager::open(temp_dir.path()).await.unwrap();
        
        let config = StorageConfig::new(temp_dir.path())
            .with_path(temp_dir.path().join("storage"));
        let storage2 = StorageManager::new(config).await.unwrap();

        // Test operations
        let key = "test-key".to_string();
        let data = Bytes::from("test-data");
        
        assert!(storage1.store(key.clone(), data.clone()).await.is_ok());
        assert!(storage2.store(key, data).await.is_ok());
    }
}

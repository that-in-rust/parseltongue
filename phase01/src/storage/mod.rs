//! Storage Layer Coordination
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Storage Orchestration
//! - StorageManager    (coordinates storage operations)
//! - StorageMetrics    (aggregates storage metrics)
//! - ConnectionPool    (manages DB connections)
//! 
//! Level 3: Operation Management
//! - BatchProcessor    (handles batch operations)
//! - TransactionManager (manages transactions)
//! - IndexCoordinator  (coordinates indexing)
//! 
//! Level 2: Storage Traits
//! - AsyncStorage      (async storage interface)
//! - StorageBackend    (storage implementation)
//! - IndexManager      (index management)
//! 
//! Level 1 (Base): Core Storage Types
//! - StorageConfig    (storage configuration)
//! - StorageError     (storage-specific errors)
//! - ConnectionConfig (connection settings)

use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::core::{error::Result, types::*};

pub mod db;
pub mod index;

// Re-export main types
pub use db::DatabaseStorage;
pub use index::IndexManager;

// ===== Level 1: Core Storage Types =====
// Design Choice: Using builder pattern for configuration

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Base storage path
    pub path: std::path::PathBuf,
    /// Connection pool size
    pub pool_size: usize,
    /// Batch size for operations
    pub batch_size: usize,
    /// Index configuration
    pub index_config: IndexConfig,
}

// ===== Level 2: Storage Traits =====
// Design Choice: Using async traits for storage operations

/// Async storage interface
#[async_trait::async_trait]
pub trait AsyncStorage: Send + Sync + 'static {
    /// Store data with key
    async fn store(&self, key: &str, data: bytes::Bytes) -> Result<()>;
    
    /// Batch store multiple entries
    async fn batch_store(&self, entries: Vec<(String, bytes::Bytes)>) -> Result<()>;
    
    /// Retrieve data by key
    async fn get(&self, key: &str) -> Result<Option<bytes::Bytes>>;
}

// ===== Level 3: Operation Management =====
// Design Choice: Using connection pooling

/// Storage manager implementation
pub struct StorageManager {
    /// Storage backend
    storage: Arc<dyn AsyncStorage>,
    /// Connection pool
    pool: Arc<Semaphore>,
    /// Storage metrics
    metrics: StorageMetrics,
}

impl StorageManager {
    /// Creates new storage manager
    pub async fn new(config: StorageConfig) -> Result<Self> {
        let storage = Arc::new(DatabaseStorage::new(config.clone())?);
        let pool = Arc::new(Semaphore::new(config.pool_size));
        let metrics = StorageMetrics::new();

        Ok(Self {
            storage,
            pool,
            metrics,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_storage_manager() {
        let temp_dir = TempDir::new().unwrap();
        
        let config = StorageConfig {
            path: temp_dir.path().to_path_buf(),
            pool_size: 4,
            batch_size: 100,
            index_config: IndexConfig::default(),
        };

        let manager = StorageManager::new(config).await;
        assert!(manager.is_ok());
    }
}

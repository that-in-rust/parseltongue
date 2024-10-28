//! Database Storage Implementation
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Database Operations
//! - DatabaseStorage   (main storage implementation)
//! - BatchProcessor    (batch operation handling)
//! - MetricsCollector  (storage metrics)
//! 
//! Level 3: Transaction Management
//! - TransactionManager (manages transactions)
//! - TransactionMetrics (transaction monitoring)
//! - ErrorRecovery     (handles failures)
//! 
//! Level 2: Connection Management
//! - ConnectionPool    (manages connections)
//! - ConnectionMetrics (connection monitoring)
//! - PoolConfig       (pool configuration)
//! 
//! Level 1 (Base): Core Database Types
//! - DbConfig         (database configuration)
//! - DbError          (database errors)
//! - ConnectionInfo   (connection details)

use std::sync::Arc;
use sled::{Db, Tree};
use tokio::sync::Semaphore;
use bytes::Bytes;
use crate::core::{error::{Error, Result}, types::*};
use super::{AsyncStorage, StorageConfig};

// ===== Level 1: Core Database Types =====
// Design Choice: Using sled for embedded storage

/// Database storage implementation
pub struct DatabaseStorage {
    /// Sled database instance
    db: Arc<Db>,
    /// Main data tree
    data_tree: Arc<Tree>,
    /// Connection pool
    pool: Arc<Semaphore>,
    /// Storage metrics
    metrics: DbMetrics,
}

impl DatabaseStorage {
    /// Creates new database storage
    pub fn new(config: StorageConfig) -> Result<Self> {
        let db = sled::open(&config.path)?;
        let data_tree = db.open_tree("data")?;
        let pool = Arc::new(Semaphore::new(config.pool_size));
        let metrics = DbMetrics::new();

        Ok(Self {
            db: Arc::new(db),
            data_tree: Arc::new(data_tree),
            pool,
            metrics,
        })
    }

    /// Executes operation with connection from pool
    async fn with_connection<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce() -> Result<R> + Send + 'static,
        R: Send + 'static,
    {
        let _permit = self.pool.acquire().await?;
        
        tokio::task::spawn_blocking(f)
            .await
            .map_err(|e| Error::Runtime(e.to_string()))?
    }
}

#[async_trait::async_trait]
impl AsyncStorage for DatabaseStorage {
    async fn store(&self, key: &str, data: Bytes) -> Result<()> {
        let key = key.to_string();
        let data = data.to_vec();
        
        self.with_connection(move || {
            self.data_tree.insert(key.as_bytes(), data)?;
            self.data_tree.flush()?;
            Ok(())
        }).await
    }

    async fn batch_store(&self, entries: Vec<(String, Bytes)>) -> Result<()> {
        self.with_connection(move || {
            let batch = sled::Batch::default();
            
            for (key, data) in entries {
                batch.insert(key.as_bytes(), data.to_vec());
            }
            
            self.data_tree.apply_batch(batch)?;
            self.data_tree.flush()?;
            Ok(())
        }).await
    }

    async fn get(&self, key: &str) -> Result<Option<Bytes>> {
        let key = key.to_string();
        
        self.with_connection(move || {
            Ok(self.data_tree.get(key.as_bytes())?
                .map(|v| Bytes::from(v.to_vec())))
        }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_basic_operations() {
        let temp_dir = TempDir::new().unwrap();
        
        let config = StorageConfig {
            path: temp_dir.path().to_path_buf(),
            pool_size: 4,
            batch_size: 100,
            index_config: IndexConfig::default(),
        };

        let storage = DatabaseStorage::new(config).unwrap();

        // Test store and retrieve
        let key = "test-key";
        let data = Bytes::from("test-data");
        
        storage.store(key, data.clone()).await.unwrap();
        let retrieved = storage.get(key).await.unwrap();
        
        assert_eq!(retrieved, Some(data));
    }
}


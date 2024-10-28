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
use tokio::sync::{Semaphore, RwLock};
use sled::{Db, Tree};
use bytes::Bytes;
use metrics::{Counter, Gauge, Histogram};
use crate::core::{error::{Error, Result}, types::*};
use super::{AsyncStorage, StorageConfig};

// ===== Level 1: Core Database Types =====
// Design Choice: Using sled for embedded storage

/// Database metrics collection
#[derive(Debug)]
struct DbMetrics {
    writes: Counter,
    reads: Counter,
    active_transactions: Gauge,
    transaction_duration: Histogram,
}

impl DbMetrics {
    fn new() -> Self {
        Self {
            writes: Counter::new(),
            reads: Counter::new(),
            active_transactions: Gauge::new(),
            transaction_duration: Histogram::new(),
        }
    }
}

// ===== Level 2: Connection Management =====
// Design Choice: Using connection pooling

/// Database storage implementation
pub struct DatabaseStorage {
    /// Sled database instance
    db: Arc<Db>,
    /// Main data tree
    data_tree: Arc<Tree>,
    /// Connection pool
    pool: Arc<Semaphore>,
    /// Transaction manager
    transactions: Arc<TransactionManager>,
    /// Database metrics
    metrics: DbMetrics,
}

impl DatabaseStorage {
    /// Creates new database storage
    pub fn new(config: StorageConfig) -> Result<Self> {
        let db = sled::open(&config.path)?;
        let data_tree = db.open_tree("data")?;
        let pool = Arc::new(Semaphore::new(config.pool_size));
        let transactions = Arc::new(TransactionManager::new());
        let metrics = DbMetrics::new();

        Ok(Self {
            db: Arc::new(db),
            data_tree: Arc::new(data_tree),
            pool,
            transactions,
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
        let start = std::time::Instant::now();
        
        self.metrics.active_transactions.increment(1.0);
        
        let result = tokio::task::spawn_blocking(f)
            .await
            .map_err(|e| Error::Runtime(e.to_string()))??;

        self.metrics.active_transactions.decrement(1.0);
        self.metrics.transaction_duration.record(start.elapsed());
        
        Ok(result)
    }
}

// ===== Level 3: Transaction Management =====
// Design Choice: Using ACID transactions

/// Transaction manager implementation
struct TransactionManager {
    active_transactions: Arc<RwLock<Vec<TransactionId>>>,
}

impl TransactionManager {
    fn new() -> Self {
        Self {
            active_transactions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    async fn begin(&self) -> TransactionId {
        let id = TransactionId::new();
        self.active_transactions.write().await.push(id);
        id
    }

    async fn commit(&self, id: TransactionId) {
        let mut transactions = self.active_transactions.write().await;
        if let Some(pos) = transactions.iter().position(|x| *x == id) {
            transactions.remove(pos);
        }
    }
}

// ===== Level 4: Database Operations =====
// Design Choice: Using async traits for storage operations

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

/// Transaction identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TransactionId(u64);

impl TransactionId {
    fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_database_operations() {
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

        // Test batch store
        let entries = vec![
            ("key1".to_string(), Bytes::from("data1")),
            ("key2".to_string(), Bytes::from("data2")),
        ];
        
        assert!(storage.batch_store(entries).await.is_ok());
    }
}

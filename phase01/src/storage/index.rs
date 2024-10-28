//! Storage Index Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Index Operations
//! - IndexManager     (manages indices)
//! - IndexMetrics     (tracks index performance)
//! - SearchOptimizer  (optimizes searches)
//! 
//! Level 3: Index Maintenance
//! - IndexUpdater     (handles updates)
//! - IndexCompactor   (manages compaction)
//! - IndexValidator   (validates integrity)
//! 
//! Level 2: Index Implementation
//! - IndexTree        (tree implementation)
//! - IndexIterator    (index traversal)
//! - IndexCache       (caching layer)
//! 
//! Level 1 (Base): Core Index Types
//! - IndexConfig      (index configuration)
//! - IndexEntry       (index entry type)
//! - IndexError       (index-specific errors)

use std::sync::Arc;
use tokio::sync::RwLock;
use sled::Tree;
use bytes::Bytes;
use metrics::{Counter, Gauge, Histogram};
use crate::core::{error::{Error, Result}, types::*};
use super::DatabaseStorage;

// ===== Level 1: Core Index Types =====
// Design Choice: Using RwLock for concurrent access

/// Index configuration
#[derive(Debug, Clone, Default)]
pub struct IndexConfig {
    /// Cache size
    pub cache_size: usize,
    /// Flush threshold
    pub flush_threshold: usize,
    /// Enable compression
    pub compression_enabled: bool,
}

/// Index metrics collection
#[derive(Debug)]
struct IndexMetrics {
    updates: Counter,
    searches: Counter,
    cache_hits: Counter,
    cache_misses: Counter,
}

impl IndexMetrics {
    fn new() -> Self {
        Self {
            updates: Counter::new(),
            searches: Counter::new(),
            cache_hits: Counter::new(),
            cache_misses: Counter::new(),
        }
    }
}

// ===== Level 2: Index Implementation =====
// Design Choice: Using LRU cache for performance

/// Index entry representation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexEntry {
    /// Entry path
    pub path: std::path::PathBuf,
    /// Entry size
    pub size: u64,
    /// Entry timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl IndexEntry {
    pub fn new(path: &str, data: &Bytes) -> Self {
        Self {
            path: path.into(),
            size: data.len() as u64,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Index manager implementation
pub struct IndexManager {
    /// Storage backend
    storage: Arc<DatabaseStorage>,
    /// Index tree
    tree: Arc<Tree>,
    /// Index cache
    cache: Arc<RwLock<lru::LruCache<String, IndexEntry>>>,
    /// Index metrics
    metrics: IndexMetrics,
}

impl IndexManager {
    /// Creates new index manager
    pub fn new(storage: Arc<DatabaseStorage>, config: IndexConfig) -> Result<Self> {
        let tree = storage.db().open_tree("index")?;
        let cache = Arc::new(RwLock::new(
            lru::LruCache::new(config.cache_size)
        ));
        let metrics = IndexMetrics::new();

        Ok(Self {
            storage,
            tree: Arc::new(tree),
            cache,
            metrics,
        })
    }

    // ===== Level 3: Index Maintenance =====
    // Design Choice: Using batched updates for efficiency

    /// Updates index entry
    pub async fn update(&self, key: &str, entry: IndexEntry) -> Result<()> {
        // Update cache
        self.cache.write().await.put(
            key.to_string(), 
            entry.clone()
        );

        // Update tree
        self.tree.insert(
            key.as_bytes(),
            bincode::serialize(&entry)?
        )?;

        self.metrics.updates.increment(1);
        Ok(())
    }

    /// Searches index
    pub async fn search(&self, prefix: &str) -> Result<Vec<IndexEntry>> {
        self.metrics.searches.increment(1);
        
        let mut results = Vec::new();
        
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(entry) = cache.get(prefix) {
                self.metrics.cache_hits.increment(1);
                results.push(entry.clone());
                return Ok(results);
            }
        }
        
        self.metrics.cache_misses.increment(1);

        // Search tree
        for item in self.tree.scan_prefix(prefix.as_bytes()) {
            let (_, value) = item?;
            let entry: IndexEntry = bincode::deserialize(&value)?;
            results.push(entry);
        }

        Ok(results)
    }

    // ===== Level 4: Index Operations =====
    // Design Choice: Using async compaction

    /// Compacts index
    pub async fn compact(&self) -> Result<()> {
        self.tree.flush()?;
        self.tree.compact_range::<&[u8], &[u8]>(None, None)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_index_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config = StorageConfig {
            path: temp_dir.path().to_path_buf(),
            pool_size: 4,
            batch_size: 100,
            index_config: IndexConfig::default(),
        };

        let storage = Arc::new(DatabaseStorage::new(config.clone()).unwrap());
        let index = IndexManager::new(storage, config.index_config).unwrap();

        // Test index operations
        let entry = IndexEntry::new("test/path", &Bytes::from("test"));
        
        index.update("test-key", entry.clone()).await.unwrap();
        let results = index.search("test").await.unwrap();
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path, entry.path);
    }
}

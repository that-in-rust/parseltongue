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
use sled::Tree;
use tokio::sync::RwLock;
use crate::core::{error::Result, types::*};

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

/// Index manager implementation
pub struct IndexManager {
    /// Index tree
    tree: Arc<Tree>,
    /// Index cache
    cache: Arc<RwLock<lru::LruCache<String, IndexEntry>>>,
    /// Index metrics
    metrics: IndexMetrics,
}

impl IndexManager {
    /// Creates new index manager
    pub fn new(tree: Tree, config: IndexConfig) -> Self {
        let cache = Arc::new(RwLock::new(
            lru::LruCache::new(config.cache_size)
        ));
        
        let metrics = IndexMetrics::new();

        Self {
            tree: Arc::new(tree),
            cache,
            metrics,
        }
    }

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
        let mut results = Vec::new();
        
        for item in self.tree.scan_prefix(prefix.as_bytes()) {
            let (_, value) = item?;
            let entry: IndexEntry = bincode::deserialize(&value)?;
            results.push(entry);
        }

        self.metrics.searches.increment(1);
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_index_operations() {
        let temp_dir = TempDir::new().unwrap();
        let db = sled::open(temp_dir.path()).unwrap();
        let tree = db.open_tree("index").unwrap();
        
        let config = IndexConfig {
            cache_size: 1000,
            flush_threshold: 100,
            compression_enabled: true,
        };

        let index = IndexManager::new(tree, config);

        // Test index operations
        let entry = IndexEntry {
            path: "test/path".into(),
            size: 100,
            timestamp: chrono::Utc::now(),
        };

        index.update("test-key", entry.clone()).await.unwrap();
        let results = index.search("test").await.unwrap();
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path, entry.path);
    }
}


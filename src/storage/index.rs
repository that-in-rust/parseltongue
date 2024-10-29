// Level 4: Storage Indexing
// - Manages file indices
// - Handles search operations
// - Provides caching
// - Tracks index metrics

use crate::core::error::Result;
use metrics::{counter, gauge};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct StorageIndex {
    entries: Arc<RwLock<HashMap<String, EntryMetadata>>>,
}

#[derive(Clone, Debug)]
pub struct EntryMetadata {
    pub size: u64,
    pub offset: u64,
    pub crc32: u32,
}

impl StorageIndex {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, path: String, metadata: EntryMetadata) -> Result<()> {
        let mut entries = self.entries.write().await;
        entries.insert(path, metadata);
        gauge!("storage.index.entries").set(entries.len() as f64);
        counter!("storage.index.inserts").increment(1);
        Ok(())
    }

    pub async fn get(&self, path: &str) -> Option<EntryMetadata> {
        let entries = self.entries.read().await;
        entries.get(path).cloned()
    }
} 
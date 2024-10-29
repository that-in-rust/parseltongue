//! Sled Database Implementation - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Database Configuration
//! Layer 3: Storage Operations
//! Layer 4: Transaction Management
//! Layer 5: Resource Management

use std::path::Path;
use anyhow::{Context, Result};
use bytes::Bytes;
use sled::{Db, IVec};
use tokio::sync::RwLock;
use tracing::{debug, warn};

use super::StorageConfig;

// Layer 1: Core Types
#[derive(Debug)]
pub struct SledStorage {
    db: RwLock<Db>,
    config: StorageConfig,
}

// Layer 2: Implementation
impl SledStorage {
    pub fn new(config: &StorageConfig) -> Result<Self> {
        debug!("Initializing sled database at: {}", config.path.display());

        let db_config = sled::Config::new()
            .path(&config.path)
            .cache_capacity(config.batch_size * 10)
            .flush_every_ms(Some(1000))
            .mode(sled::Mode::HighThroughput);

        let db = db_config.open()
            .context("Failed to open sled database")?;

        Ok(Self {
            db: RwLock::new(db),
            config: config.clone(),
        })
    }

    // Layer 3: Storage Operations
    pub async fn insert(&self, key: &str, value: Bytes) -> Result<()> {
        let normalized_key = self.normalize_key(key);
        let db = self.db.write().await;

        db.insert(normalized_key.as_bytes(), value.as_ref())
            .context("Failed to insert value")?;

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Bytes>> {
        let normalized_key = self.normalize_key(key);
        let db = self.db.read().await;

        Ok(db.get(normalized_key.as_bytes())
            .context("Failed to retrieve value")?
            .map(|ivec| Bytes::copy_from_slice(&ivec)))
    }

    // Layer 4: Batch Operations
    pub async fn insert_batch(&self, entries: Vec<(String, Bytes)>) -> Result<()> {
        let db = self.db.write().await;
        let mut batch = sled::Batch::default();

        for (key, value) in entries {
            let normalized_key = self.normalize_key(&key);
            batch.insert(normalized_key.as_bytes(), value.as_ref());
        }

        db.apply_batch(batch)
            .context("Failed to apply batch")?;

        Ok(())
    }

    // Layer 5: Resource Management
    pub async fn flush(&self) -> Result<()> {
        debug!("Flushing database to disk");
        let db = self.db.read().await;
        db.flush()
            .context("Failed to flush database")?;
        Ok(())
    }

    fn normalize_key(&self, key: &str) -> String {
        // Normalize path separators and remove leading/trailing slashes
        key.replace('\\', "/")
            .trim_start_matches('/')
            .trim_end_matches('/')
            .to_string()
    }
}

impl Drop for SledStorage {
    fn drop(&mut self) {
        if let Ok(db) = self.db.try_write() {
            if let Err(e) = db.flush() {
                warn!("Failed to flush database on drop: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_sled_storage() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config = StorageConfig {
            path: temp_dir.path().to_path_buf(),
            max_concurrent_ops: 2,
            batch_size: 8192,
        };

        let storage = SledStorage::new(&config)?;
        
        let key = "test/key";
        let value = Bytes::from("test value");
        
        storage.insert(key, value.clone()).await?;
        let retrieved = storage.get(key).await?;
        
        assert_eq!(retrieved.unwrap(), value);
        Ok(())
    }
}

//! Sled Storage Implementation - Pyramidal Structure
//! Layer 1: Storage Interface
//! Layer 2: Database Operations
//! Layer 3: Batch Processing
//! Layer 4: Compression
//! Layer 5: Metrics & Cleanup

use std::path::Path;
use anyhow::Result;
use bytes::Bytes;
use sled::{Db, IVec};
use tokio::task;

// Layer 1: Core Storage
pub struct SledStorage {
    db: Db,
}

// Layer 2: Implementation
impl SledStorage {
    pub fn new(path: &Path) -> Result<Self> {
        let db = sled::Config::new()
            .path(path)
            .mode(sled::Mode::HighThroughput)
            .cache_capacity(1024 * 1024 * 1024) // 1GB
            .flush_every_ms(Some(1000))
            .open()?;

        Ok(Self { db })
    }

    // Layer 3: Basic Operations
    pub async fn insert(&self, key: &str, value: Bytes) -> Result<()> {
        let db = self.db.clone();
        let key = key.to_string();
        
        task::spawn_blocking(move || {
            db.insert(key.as_bytes(), IVec::from(value.as_ref()))?;
            db.flush()?;
            Ok::<_, anyhow::Error>(())
        })
        .await??;

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Bytes>> {
        let db = self.db.clone();
        let key = key.to_string();

        let result = task::spawn_blocking(move || {
            Ok::<_, anyhow::Error>(
                db.get(key.as_bytes())?
                    .map(|v| Bytes::copy_from_slice(&v))
            )
        })
        .await??;

        Ok(result)
    }

    // Layer 4: Batch Operations
    pub async fn insert_batch(&self, entries: Vec<(String, Bytes)>) -> Result<()> {
        let db = self.db.clone();
        
        task::spawn_blocking(move || {
            let batch = entries.into_iter().map(|(k, v)| {
                (IVec::from(k.as_bytes()), IVec::from(v.as_ref()))
            });

            let mut batch_writer = sled::Batch::default();
            for (k, v) in batch {
                batch_writer.insert(k, v);
            }

            db.apply_batch(batch_writer)?;
            db.flush()?;
            Ok::<_, anyhow::Error>(())
        })
        .await??;

        Ok(())
    }

    // Layer 5: Cleanup
    pub async fn cleanup(&self) -> Result<()> {
        let db = self.db.clone();
        task::spawn_blocking(move || {
            db.flush()?;
            Ok::<_, anyhow::Error>(())
        })
        .await??;
        Ok(())
    }
}

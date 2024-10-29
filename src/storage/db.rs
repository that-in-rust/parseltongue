// Level 4: Database Operations
// - Manages RocksDB instance
// - Handles async operations
// - Provides CRUD interface
// - Implements metrics

use tokio::task;
use rocksdb::{DB, Options};
use std::path::Path;
use crate::core::error::{Error, Result};
use metrics::{counter, gauge};

pub struct Database {
    inner: DB,
    metrics_prefix: String,
}

impl Database {
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let mut opts = Options::default();
        opts.create_if_missing(true);
        
        let db = task::spawn_blocking(move || {
            DB::open(&opts, path)
        }).await??;

        Ok(Self {
            inner: db,
            metrics_prefix: "storage.db".to_string(),
        })
    }

    pub async fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        let key = key.to_vec();
        let value = value.to_vec();
        
        task::spawn_blocking(move || {
            self.inner.put(key, value)
        }).await??;

        counter!("storage.db.writes").increment(1);
        Ok(())
    }

    // ... Additional methods with metrics
} 
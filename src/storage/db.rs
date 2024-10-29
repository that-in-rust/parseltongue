// Level 4: Database Operations
// - Manages RocksDB instance
// - Provides async storage operations
// - Handles connection lifecycle

use rocksdb::{DB, Options};
use std::path::Path;
use crate::error::Result;
use tokio::task::spawn_blocking;
use std::sync::Arc;

pub struct Database {
    db: Arc<DB>,
}

impl Database {
    pub async fn open(path: &Path) -> Result<Self> {
        let path = path.to_path_buf();
        let db = spawn_blocking(move || {
            let mut opts = Options::default();
            opts.create_if_missing(true);
            DB::open(&opts, path)
        }).await??;
        
        Ok(Self { db: Arc::new(db) })
    }

    pub async fn store(&self, key: &str, value: &[u8]) -> Result<()> {
        let db = self.db.clone();
        let key = key.to_string();
        let value = value.to_vec();
        
        spawn_blocking(move || {
            db.put(key.as_bytes(), value)
        }).await??;
        
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let db = self.db.clone();
        let key = key.to_string();
        
        let result = spawn_blocking(move || {
            db.get(key.as_bytes())
        }).await??;
        
        Ok(result)
    }
} 
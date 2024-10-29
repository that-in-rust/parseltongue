// Level 4: Database Interactions
// - Manages connection to RocksDB
// - Provides async methods to store and retrieve data

use rocksdb::{DB, Options};
use std::path::Path;
use crate::error::{Result, Error};
use std::sync::Arc;
use tokio::task::spawn_blocking;

pub struct Database {
    db: Arc<DB>,
}

impl Database {
    // Level 3: Initialize the database
    pub async fn new(path: &Path) -> Result<Self> {
        let path = path.to_owned();
        let db = spawn_blocking(move || {
            let mut opts = Options::default();
            opts.create_if_missing(true);
            DB::open(&opts, path).map_err(Error::from)
        })
        .await??;

        Ok(Database { db: Arc::new(db) })
    }

    // Level 2: Store data asynchronously
    pub async fn store(&self, key: &str, value: &[u8]) -> Result<()> {
        let db = self.db.clone();
        let key = key.to_owned();
        let value = value.to_owned();

        spawn_blocking(move || db.put(key, value)).await??;

        Ok(())
    }

    // Level 2: Close the database gracefully
    pub async fn close(self) -> Result<()> {
        let db = Arc::try_unwrap(self.db)
            .map_err(|_| crate::error::Error::Generic("Database still has multiple owners".into()))?;
        spawn_blocking(move || db.flush()).await??;
        Ok(())
    }
} 
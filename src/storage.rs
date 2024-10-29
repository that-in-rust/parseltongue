// Level 4: Database Interactions
// - Manages RocksDB connections
// - Provides asynchronous store and retrieve methods

use std::path::Path;
use std::sync::Arc;
use rocksdb::{DB, Options};
use crate::error::Result;
use tokio::task;

pub struct Database {
    db: Arc<DB>,
}

impl Database {
    // Level 3: Open the database asynchronously
    pub async fn open(path: &Path) -> Result<Self> {
        let path = path.to_owned();
        let db = task::spawn_blocking(move || {
            let mut opts = Options::default();
            opts.create_if_missing(true);
            DB::open(&opts, &path)
        })
        .await??;

        Ok(Database { db: Arc::new(db) })
    }

    // Level 3: Store data asynchronously
    pub async fn store(&self, key: &str, value: &[u8]) -> Result<()> {
        let db = self.db.clone();
        let key = key.to_owned();
        let value = value.to_owned();

        task::spawn_blocking(move || {
            db.put(key.as_bytes(), &value)
        })
        .await??;

        Ok(())
    }

    // Level 3: Close the database gracefully
    pub async fn close(self) -> Result<()> {
        let db = Arc::try_unwrap(self.db).map_err(|_| crate::error::Error::Generic("Database still has multiple owners".into()))?;
        task::spawn_blocking(|| db.flush()).await??;
        Ok(())
    }
} 
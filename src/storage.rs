// Asynchronous database interactions using RocksDB
//
// This module defines the `Database` struct with async methods for data operations.
// Key design points:
//
// - Synchronous RocksDB operations are wrapped in `spawn_blocking` to prevent blocking the async runtime.
// - The API provides async `put` and `get` methods.

use rocksdb::{DB, Options};
use tokio::task;
use std::path::PathBuf;
use crate::error::Result;

pub struct Database {
    db: DB,
}

impl Database {
    // Opens a new RocksDB instance asynchronously.
    pub async fn open(path: PathBuf) -> Result<Self> {
        let db = task::spawn_blocking(move || {
            let mut opts = Options::default();
            opts.create_if_missing(true);
            DB::open(&opts, path)
        }).await??;
        Ok(Self { db })
    }

    // Asynchronously puts data into the database.
    pub async fn put(&self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let db = self.db.clone();
        task::spawn_blocking(move || db.put(key, value)).await??;
        Ok(())
    }

    // Asynchronously gets data from the database.
    pub async fn get(&self, key: Vec<u8>) -> Result<Option<Vec<u8>>> {
        let db = self.db.clone();
        let result = task::spawn_blocking(move || db.get(key)).await??;
        Ok(result)
    }

    // Closes the database.
    pub async fn close(self) -> Result<()> {
        // RocksDB handles cleanup on drop.
        drop(self.db);
        Ok(())
    }
} 
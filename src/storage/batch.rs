// Level 4: Batched Storage Operations
// - Manages write batches for RocksDB
// - Handles batch size limits
// - Provides atomic operations

use rocksdb::{WriteBatch, DB};
use std::sync::Arc;
use crate::error::Result;
use tokio::sync::Mutex;

pub struct BatchWriter {
    db: Arc<DB>,
    batch: Mutex<WriteBatch>,
    size: usize,
    max_size: usize,
}

impl BatchWriter {
    pub fn new(db: Arc<DB>, max_size: usize) -> Self {
        Self {
            db,
            batch: Mutex::new(WriteBatch::default()),
            size: 0,
            max_size,
        }
    }

    pub async fn add(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        let mut batch = self.batch.lock().await;
        batch.put(key, value);
        self.size += key.len() + value.len();

        if self.size >= self.max_size {
            self.flush().await?;
        }

        Ok(())
    }

    pub async fn flush(&mut self) -> Result<()> {
        let mut batch = self.batch.lock().await;
        if self.size > 0 {
            let db = self.db.clone();
            let old_batch = std::mem::replace(&mut *batch, WriteBatch::default());
            
            tokio::task::spawn_blocking(move || {
                db.write(old_batch)
            }).await??;
            
            self.size = 0;
        }
        Ok(())
    }
} 
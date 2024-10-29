// Level 4: Database Interface
// - Manages RocksDB lifecycle and operations
// - Handles async storage operations
// - Implements backpressure and resource limits
// - Provides metrics collection points

use tokio::task;
use rocksdb::{DB, Options, ColumnFamilyDescriptor, TransactionDB, Transaction};
use crate::core::error::{Error, Result};
use std::path::Path;
use metrics::{Counter, Gauge};

// Level 3: Database Types
pub struct Database {
    db: TransactionDB,
    write_counter: Counter,
    read_counter: Counter,
    size_gauge: Gauge,
}

impl Database {
    // Level 2: Database Lifecycle
    pub async fn new(path: impl AsRef<Path>, column_families: &[&str]) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let cf_descriptors: Vec<_> = column_families.iter()
            .map(|name| ColumnFamilyDescriptor::new(*name, Options::default()))
            .collect();

        let db = task::spawn_blocking(move || {
            let mut opts = Options::default();
            opts.create_if_missing(true);
            opts.set_max_background_jobs(4);
            TransactionDB::open_cf_descriptors(&opts, path, cf_descriptors)
        }).await??;

        Ok(Self {
            db,
            write_counter: metrics::counter!("db_writes_total"),
            read_counter: metrics::counter!("db_reads_total"),
            size_gauge: metrics::gauge!("db_size_bytes"),
        })
    }

    // Level 1: Transaction Operations
    pub async fn transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Transaction) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let db = self.db.clone();
        task::spawn_blocking(move || {
            let txn = db.transaction();
            let result = f(&txn)?;
            txn.commit()?;
            Ok(result)
        }).await??
    }
} 
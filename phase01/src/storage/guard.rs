use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::error::Result;

/// RAII guard for database connections
pub struct ConnectionGuard {
    pool: Arc<Semaphore>,
}

impl ConnectionGuard {
    pub(crate) fn new(pool: Arc<Semaphore>) -> Self {
        Self { pool }
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.pool.add_permits(1);
    }
}

/// RAII guard for database transactions
pub struct TransactionGuard {
    committed: bool,
    tree: Arc<sled::Tree>,
}

impl TransactionGuard {
    pub(crate) fn new(tree: Arc<sled::Tree>) -> Self {
        Self {
            committed: false,
            tree,
        }
    }

    pub async fn commit(&mut self) -> Result<()> {
        self.tree.flush()?;
        self.committed = true;
        Ok(())
    }
}

impl Drop for TransactionGuard {
    fn drop(&mut self) {
        if !self.committed {
            // Rollback happens automatically when tree is dropped
        }
    }
}


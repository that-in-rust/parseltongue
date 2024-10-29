//! Storage Core - Pyramidal Structure
//! Layer 1: Core Types & Exports
//! Layer 2: Storage Configuration
//! Layer 3: Storage Management
//! Layer 4: Resource Control
//! Layer 5: Error Handling

pub mod sled_storage;
pub mod guard;
pub mod pool;

pub use sled_storage::SledStorageManager;
pub use pool::StoragePool;

pub struct StorageManager {
    db: sled::Db,
    pool: StoragePool,
}

impl StorageManager {
    pub fn new(db_path: &std::path::Path, pool_size: usize) -> anyhow::Result<Self> {
        let db = sled::open(db_path)?;
        let pool = StoragePool::new(pool_size)?;
        Ok(Self { db, pool })
    }

    pub fn insert(&self, key: &[u8], value: &[u8]) -> anyhow::Result<()> {
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> anyhow::Result<Option<sled::IVec>> {
        Ok(self.db.get(key)?)
    }

    pub fn remove(&self, key: &[u8]) -> anyhow::Result<()> {
        self.db.remove(key)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn iterate(&self) -> sled::Iter {
        self.db.iter()
    }

    pub fn shutdown(&self) -> anyhow::Result<()> {
        self.db.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_storage_manager() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let storage = StorageManager::new(temp_dir.path(), 2)?;

        storage.insert(b"key1", b"value1")?;
        let value = storage.get(b"key1")?.unwrap();
        assert_eq!(value.as_ref(), b"value1");

        storage.remove(b"key1")?;
        assert!(storage.get(b"key1")?.is_none());

        Ok(())
    }
}

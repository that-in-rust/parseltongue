use anyhow::{Context, Result};
use sled::Db;
use std::path::Path;

pub struct DatabaseManager {
    db: Db,
}

impl DatabaseManager {
    pub fn new(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path).context("Failed to create database directory")?;
        let db = sled::open(path).context("Failed to open sled database")?;
        Ok(Self { db })
    }

    pub fn store(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.insert(key, value).context("Failed to insert into database")?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key).context("Failed to retrieve from database")?.map(|ivec| ivec.to_vec()))
    }
}

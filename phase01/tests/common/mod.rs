//! Common Test Utilities - Pyramidal Structure
//! Layer 1: Test Setup
//! Layer 2: Test Data Generation
//! Layer 3: Validation Helpers
//! Layer 4: Cleanup
//! Layer 5: Helper Functions

use std::path::PathBuf;
use anyhow::Result;
use tempfile::TempDir;
use zip::ZipWriter;
use std::fs::File;
use std::io::Write;
use rand::Rng;
use tokio::fs;

// Layer 1: Test Setup
pub struct TestContext {
    pub temp_dir: TempDir,
    pub input_zip: PathBuf,
    pub output_dir: PathBuf,
}

impl TestContext {
    pub async fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let input_zip = temp_dir.path().join("test.zip");
        let output_dir = temp_dir.path().join("output");

        fs::create_dir_all(&output_dir).await?;

        Ok(Self {
            temp_dir,
            input_zip,
            output_dir,
        })
    }

    // Layer 2: Test Data Generation
    pub fn create_test_zip(&self, entries: &[(&str, &[u8])]) -> Result<()> {
        let file = File::create(&self.input_zip)?;
        let mut zip = ZipWriter::new(file);

        for (name, content) in entries {
            zip.start_file(name, Default::default())?;
            zip.write_all(content)?;
        }

        zip.finish()?;
        Ok(())
    }

    // Layer 3: Validation Helpers
    pub async fn verify_output_structure(&self) -> Result<bool> {
        let db_dir = self.output_dir.join("db");
        let logs_dir = self.output_dir.join("logs");
        let metrics_dir = self.output_dir.join("metrics");

        Ok(
            tokio::fs::metadata(&db_dir).await?.is_dir() &&
            tokio::fs::metadata(&logs_dir).await?.is_dir() &&
            tokio::fs::metadata(&metrics_dir).await?.is_dir()
        )
    }

    // Layer 4: Cleanup
    pub async fn cleanup(self) -> Result<()> {
        tokio::fs::remove_dir_all(&self.output_dir).await?;
        Ok(())
    }
}

// Layer 5: Helper Functions
pub fn generate_random_data(size: usize) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

pub fn create_test_entries(count: usize, size: usize) -> Vec<(String, Vec<u8>)> {
    (0..count)
        .map(|i| (
            format!("file_{}.dat", i),
            generate_random_data(size)
        ))
        .collect()
}

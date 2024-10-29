//! Test Utilities - Pyramidal Structure
//! Layer 1: Core Test Types
//! Layer 2: Test Configuration
//! Layer 3: Test Fixtures
//! Layer 4: Helper Functions
//! Layer 5: Cleanup Management

use std::path::PathBuf;
use anyhow::Result;
use tempfile::{TempDir, NamedTempFile};
use zip::write::FileOptions;

// Layer 1: Core Types
pub struct TestContext {
    pub temp_dir: TempDir,
    pub test_zip: NamedTempFile,
    pub config: parseltongue::Config,
}

// Layer 2: Test Configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub file_count: usize,
    pub file_size: usize,
    pub workers: usize,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            file_count: 10,
            file_size: 1024,
            workers: 2,
        }
    }
}

// Layer 3: Test Setup
impl TestContext {
    pub async fn new(config: TestConfig) -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let test_zip = create_test_zip(&config)?;

        let app_config = parseltongue::Config::builder()
            .input_zip(test_zip.path())
            .output_dir(temp_dir.path())
            .workers(config.workers)
            .buffer_size(8192)
            .build()?;

        Ok(Self {
            temp_dir,
            test_zip,
            config: app_config,
        })
    }

    // Layer 4: Helper Methods
    pub fn output_path(&self) -> PathBuf {
        self.temp_dir.path().to_path_buf()
    }

    pub fn input_path(&self) -> PathBuf {
        self.test_zip.path().to_path_buf()
    }
}

// Layer 5: Test Utilities
fn create_test_zip(config: &TestConfig) -> Result<NamedTempFile> {
    let file = NamedTempFile::new()?;
    let mut zip = zip::ZipWriter::new(std::fs::File::create(file.path())?);
    let options = FileOptions::default();

    let content = vec![b'x'; config.file_size];
    for i in 0..config.file_count {
        zip.start_file(format!("test{}.txt", i), options)?;
        zip.write_all(&content)?;
    }

    zip.finish()?;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_context_creation() -> Result<()> {
        let config = TestConfig::default();
        let ctx = TestContext::new(config).await?;
        
        assert!(ctx.input_path().exists());
        assert!(ctx.output_path().exists());
        
        Ok(())
    }
}

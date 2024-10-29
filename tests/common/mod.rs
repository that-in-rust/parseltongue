// Level 4: Test Utilities
// - Provides test helpers
// - Manages test resources
// - Handles cleanup
// - Simulates failures

use tempfile::TempDir;
use std::path::PathBuf;
use crate::core::error::Result;

pub struct TestContext {
    pub temp_dir: TempDir,
    pub test_data: PathBuf,
}

impl TestContext {
    pub async fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let test_data = PathBuf::from("tests/data");
        Ok(Self { temp_dir, test_data })
    }
} 
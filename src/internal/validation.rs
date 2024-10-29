//! Path Validation - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Validation Rules
//! Layer 3: Path Processing
//! Layer 4: Error Handling
//! Layer 5: Safety Checks

use std::path::{Path, PathBuf};
use anyhow::Result;
use tracing::{debug, warn};

use crate::error::Error;

// Layer 1: Core Types
#[derive(Debug, Default)]
pub struct PathValidator {
    checks: Vec<Box<dyn Fn(&Path) -> Result<()> + Send + Sync>>,
}

// Layer 2: Implementation
impl PathValidator {
    pub fn new() -> Self {
        Self::default()
    }

    // Layer 3: Validation Rules
    pub fn exists<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        self.checks.push(Box::new(move |_| {
            if !path.exists() {
                anyhow::bail!("Path does not exist: {}", path.display());
            }
            Ok(())
        }));
        self
    }

    pub fn is_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        self.checks.push(Box::new(move |_| {
            if !path.is_file() {
                anyhow::bail!("Path is not a file: {}", path.display());
            }
            Ok(())
        }));
        self
    }

    pub fn is_zip<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        self.checks.push(Box::new(move |_| {
            if let Some(ext) = path.extension() {
                if ext != "zip" {
                    anyhow::bail!("File is not a ZIP: {}", path.display());
                }
            } else {
                anyhow::bail!("File has no extension: {}", path.display());
            }
            Ok(())
        }));
        self
    }

    pub fn parent_exists<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        self.checks.push(Box::new(move |_| {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    anyhow::bail!("Parent directory does not exist: {}", parent.display());
                }
            }
            Ok(())
        }));
        self
    }

    // Layer 4: Path Safety
    pub fn is_safe_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        self.checks.push(Box::new(move |_| {
            // Check for path traversal attempts
            if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
                anyhow::bail!("Path contains parent directory references: {}", path.display());
            }
            
            // Check for absolute paths
            if path.is_absolute() {
                anyhow::bail!("Path must be relative: {}", path.display());
            }
            
            Ok(())
        }));
        self
    }

    // Layer 5: Validation Execution
    pub fn validate(&self) -> Result<()> {
        for check in &self.checks {
            check(Path::new(""))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_path_validation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_file = temp_dir.path().join("test.zip");
        std::fs::write(&test_file, b"test")?;

        let validator = PathValidator::new()
            .exists(&test_file)
            .is_file(&test_file)
            .is_zip(&test_file)
            .parent_exists(&test_file);

        assert!(validator.validate().is_ok());
        Ok(())
    }

    #[test]
    fn test_unsafe_path() -> Result<()> {
        let validator = PathValidator::new()
            .is_safe_path("../test.zip");

        assert!(validator.validate().is_err());
        Ok(())
    }
} 
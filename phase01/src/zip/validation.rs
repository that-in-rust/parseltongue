//! ZIP Entry Validation
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Validation Orchestration
//! - ValidationManager (manages validation)
//! - ValidationMetrics (tracks validation)
//! - ResultAggregator  (combines results)
//! 
//! Level 3: Validation Types
//! - CrcValidator     (validates CRC32)
//! - PathValidator    (validates paths)
//! - SizeValidator    (validates sizes)
//! 
//! Level 2: Validation Implementation
//! - AsyncValidator   (async validation)
//! - ValidationState  (validation state)
//! - ResultCollector  (result collection)
//! 
//! Level 1 (Base): Core Validation Types
//! - ValidationConfig (validation config)
//! - ValidationResult (result types)
//! - ValidationError  (validation errors)

use std::sync::Arc;
use tokio::sync::Semaphore;
use crc32fast::Hasher;
use metrics::{Counter, Gauge};
use crate::core::{error::{Error, Result}, types::*};
use super::ZipEntry;

// ===== Level 1: Core Validation Types =====
// Design Choice: Using type-state pattern for validation states

/// Validation metrics collection
#[derive(Debug, Default)]
struct ValidationMetrics {
    validations_completed: Counter,
    validation_errors: Counter,
    active_validations: Gauge,
}

impl ValidationMetrics {
    fn new() -> Self {
        Self::default()
    }
}

/// Validation configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Enable CRC validation
    pub validate_crc: bool,
    /// Enable path validation
    pub validate_paths: bool,
    /// Maximum path length
    pub max_path_length: usize,
    /// Maximum concurrent validations
    pub max_concurrent: usize,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            validate_crc: true,
            validate_paths: true,
            max_path_length: 256,
            max_concurrent: 4,
        }
    }
}

// ===== Level 2: Validation Implementation =====
// Design Choice: Using async traits for concurrent validation

/// Entry validator implementation
pub struct EntryValidator {
    /// Validation configuration
    config: ValidationConfig,
    /// Validation semaphore
    semaphore: Arc<Semaphore>,
    /// Validation metrics
    metrics: ValidationMetrics,
}

impl EntryValidator {
    /// Creates new entry validator
    pub fn new(config: ValidationConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
        let metrics = ValidationMetrics::new();

        Self {
            config,
            semaphore,
            metrics,
        }
    }

    // ===== Level 3: Validation Types =====
    // Design Choice: Using separate validation functions for modularity

    /// Validates ZIP entry
    pub async fn validate(&self, entry: &ZipEntry) -> Result<()> {
        let _permit = self.semaphore.acquire().await?;
        self.metrics.active_validations.increment(1.0);

        let result = async {
            // Validate path
            if self.config.validate_paths {
                self.validate_path(entry)?;
            }

            // Validate CRC
            if self.config.validate_crc {
                self.validate_crc(entry)?;
            }

            Ok(())
        }.await;

        self.metrics.active_validations.decrement(1.0);
        
        match result {
            Ok(_) => {
                self.metrics.validations_completed.increment(1);
                Ok(())
            }
            Err(e) => {
                self.metrics.validation_errors.increment(1);
                Err(e)
            }
        }
    }

    /// Validates entry path
    fn validate_path(&self, entry: &ZipEntry) -> Result<()> {
        let path_str = entry.path.to_string_lossy();
        
        // Check path length
        if path_str.len() > self.config.max_path_length {
            return Err(Error::InvalidPath(entry.path.clone()));
        }

        // Check for path traversal
        if path_str.contains("..") {
            return Err(Error::InvalidPath(entry.path.clone()));
        }

        // Check for absolute paths
        if entry.path.is_absolute() {
            return Err(Error::InvalidPath(entry.path.clone()));
        }

        Ok(())
    }

    /// Validates entry CRC
    fn validate_crc(&self, entry: &ZipEntry) -> Result<()> {
        let mut hasher = Hasher::new();
        hasher.update(&entry.data);
        let computed_crc = hasher.finalize();

        if computed_crc != entry.crc32 {
            return Err(Error::ValidationFailed(format!(
                "CRC mismatch for {}: expected {:x}, got {:x}",
                entry.path.display(),
                entry.crc32,
                computed_crc
            )));
        }

        Ok(())
    }
}

// ===== Level 4: Validation Orchestration =====
// Design Choice: Using builder pattern for validation chain

/// Validation chain builder
pub struct ValidationChain {
    validators: Vec<Box<dyn Fn(&ZipEntry) -> Result<()> + Send + Sync>>,
}

impl ValidationChain {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn add_validator<F>(&mut self, validator: F) -> &mut Self 
    where
        F: Fn(&ZipEntry) -> Result<()> + Send + Sync + 'static,
    {
        self.validators.push(Box::new(validator));
        self
    }

    pub fn validate(&self, entry: &ZipEntry) -> Result<()> {
        for validator in &self.validators {
            validator(entry)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use bytes::Bytes;

    #[tokio::test]
    async fn test_path_validation() {
        let config = ValidationConfig {
            validate_paths: true,
            max_path_length: 10,
            ..Default::default()
        };

        let validator = EntryValidator::new(config);

        // Test valid path
        let entry = ZipEntry {
            path: PathBuf::from("test.txt"),
            data: Bytes::from("test"),
            crc32: 0xd87f7e0c,  // CRC32 of "test"
            size: 4,
        };

        assert!(validator.validate(&entry).await.is_ok());

        // Test invalid path
        let entry = ZipEntry {
            path: PathBuf::from("../test.txt"),
            data: Bytes::new(),
            crc32: 0,
            size: 0,
        };

        assert!(validator.validate(&entry).await.is_err());
    }

    #[tokio::test]
    async fn test_crc_validation() {
        let config = ValidationConfig {
            validate_crc: true,
            ..Default::default()
        };

        let validator = EntryValidator::new(config);

        let entry = ZipEntry {
            path: PathBuf::from("test.txt"),
            data: Bytes::from("test"),
            crc32: 0xd87f7e0c,  // Correct CRC32 for "test"
            size: 4,
        };

        assert!(validator.validate(&entry).await.is_ok());

        let entry = ZipEntry {
            path: PathBuf::from("test.txt"),
            data: Bytes::from("test"),
            crc32: 0x12345678,  // Incorrect CRC32
            size: 4,
        };

        assert!(validator.validate(&entry).await.is_err());
    }
}

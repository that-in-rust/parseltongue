//! CLI Configuration - Pyramidal Structure
//! Layer 1: Public Types & Constants
//!   - Core configuration types
//! Layer 2: Builder Pattern
//!   - Safe configuration construction
//! Layer 3: Validation Logic
//!   - Configuration validation rules
//! Layer 4: Error Handling
//!   - Configuration-specific errors
//! Layer 5: Default Implementation
//!   - Sensible defaults and constants

use std::path::PathBuf;
use std::time::Duration;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// Layer 1: Core Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub workers: usize,
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
    pub buffer_size: usize,
    pub shutdown_timeout: Duration,
    pub verbose: bool,
}

// Layer 2: Constants
pub const DEFAULT_WORKERS: usize = num_cpus::get();
pub const DEFAULT_BUFFER_SIZE: usize = 8 * 1024; // 8KB
pub const DEFAULT_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);
pub const MIN_BUFFER_SIZE: usize = 1024; // 1KB
pub const MAX_WORKERS: usize = 32;

// Layer 3: Builder Pattern
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    workers: Option<usize>,
    input_path: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    buffer_size: Option<usize>,
    shutdown_timeout: Option<Duration>,
    verbose: bool,
}

// Layer 4: Implementation
impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = Some(workers);
        self
    }

    pub fn input_path<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.input_path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn output_dir<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.output_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = Some(size);
        self
    }

    pub fn shutdown_timeout(mut self, timeout: Duration) -> Self {
        self.shutdown_timeout = Some(timeout);
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    // Layer 5: Validation & Construction
    pub fn build(self) -> Result<Config> {
        let workers = self.workers
            .unwrap_or(DEFAULT_WORKERS)
            .min(MAX_WORKERS);

        let buffer_size = self.buffer_size
            .unwrap_or(DEFAULT_BUFFER_SIZE)
            .max(MIN_BUFFER_SIZE);

        let config = Config {
            workers,
            input_path: self.input_path
                .ok_or_else(|| anyhow::anyhow!("input_path is required"))?,
            output_dir: self.output_dir
                .ok_or_else(|| anyhow::anyhow!("output_dir is required"))?,
            buffer_size,
            shutdown_timeout: self.shutdown_timeout.unwrap_or(DEFAULT_SHUTDOWN_TIMEOUT),
            verbose: self.verbose,
        };

        config.validate()?;
        Ok(config)
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    fn validate(&self) -> Result<()> {
        if !self.input_path.exists() {
            anyhow::bail!("Input path does not exist: {}", self.input_path.display());
        }
        if !self.input_path.is_file() {
            anyhow::bail!("Input path is not a file: {}", self.input_path.display());
        }
        if self.workers == 0 {
            anyhow::bail!("Worker count cannot be zero");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_builder() {
        let temp_dir = TempDir::new().unwrap();
        let input_file = temp_dir.path().join("test.zip");
        std::fs::write(&input_file, b"test").unwrap();

        let config = Config::builder()
            .input_path(input_file)
            .output_dir(temp_dir.path())
            .workers(2)
            .build();

        assert!(config.is_ok());
    }
}


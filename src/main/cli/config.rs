//! CLI Configuration - Pyramidal Structure
//! Layer 1: Public Types & Constants
//! Layer 2: Builder Pattern
//! Layer 3: Validation Logic
//! Layer 4: Error Handling
//! Layer 5: Default Implementation

use std::path::{Path, PathBuf};
use std::time::Duration;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// Layer 1: Core Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub input_zip: PathBuf,
    pub output_dir: PathBuf,
    pub workers: usize,
    pub buffer_size: usize,
    pub shutdown_timeout: Duration,
    pub verbose: bool,
}

// Layer 2: Builder Pattern
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    input_zip: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    workers: Option<usize>,
    buffer_size: Option<usize>,
    shutdown_timeout: Option<Duration>,
    verbose: bool,
}

// Layer 3: Builder Implementation
impl ConfigBuilder {
    pub fn input_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.input_zip = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn output_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.output_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn workers(mut self, count: usize) -> Self {
        self.workers = Some(count);
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

    // Layer 4: Build & Validation
    pub fn build(self) -> Result<Config> {
        let config = Config {
            input_zip: self.input_zip
                .context("Input ZIP path is required")?,
            output_dir: self.output_dir
                .context("Output directory path is required")?,
            workers: self.workers
                .unwrap_or_else(num_cpus::get),
            buffer_size: self.buffer_size
                .unwrap_or(8192),
            shutdown_timeout: self.shutdown_timeout
                .unwrap_or_else(|| Duration::from_secs(30)),
            verbose: self.verbose,
        };

        config.validate()?;
        Ok(config)
    }
}

// Layer 5: Config Implementation
impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<()> {
        if !self.input_zip.exists() {
            anyhow::bail!("Input file does not exist: {}", self.input_zip.display());
        }
        if !self.input_zip.is_file() {
            anyhow::bail!("Input path is not a file: {}", self.input_zip.display());
        }
        if self.workers == 0 {
            anyhow::bail!("Worker count must be greater than zero");
        }
        if self.buffer_size < 1024 {
            anyhow::bail!("Buffer size must be at least 1KB");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_config_builder() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let input_file = temp_dir.path().join("test.zip");
        File::create(&input_file)?;

        let config = Config::builder()
            .input_path(input_file)
            .output_dir(temp_dir.path())
            .workers(2)
            .buffer_size(8192)
            .shutdown_timeout(Duration::from_secs(30))
            .verbose(false)
            .build()?;

        assert!(config.validate().is_ok());
        Ok(())
    }
}


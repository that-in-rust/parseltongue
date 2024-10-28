//! CLI Configuration - Pyramidal Structure
//! Layer 1: Public Types
//! Layer 2: Configuration Validation
//! Layer 3: Default Implementation
//! Layer 4: Builder Pattern
//! Layer 5: Conversion Utilities

use std::path::PathBuf;
use std::time::Duration;
use anyhow::Result;

// Layer 1: Core Configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub input_zip: PathBuf,
    pub output_dir: PathBuf,
    pub workers: usize,
    pub buffer_size: usize,
    pub shutdown_timeout: Duration,
    pub verbose: bool,
}

// Layer 2: Validation
impl Config {
    pub fn validate(&self) -> Result<()> {
        if !self.input_zip.exists() {
            anyhow::bail!("Input ZIP file does not exist");
        }
        if !self.input_zip.is_file() {
            anyhow::bail!("Input ZIP path is not a file");
        }
        if self.workers == 0 {
            anyhow::bail!("Worker count must be greater than 0");
        }
        Ok(())
    }
}

// Layer 3: Defaults
impl Default for Config {
    fn default() -> Self {
        Self {
            input_zip: PathBuf::new(),
            output_dir: PathBuf::new(),
            workers: num_cpus::get(),
            buffer_size: 8 * 1024, // 8KB
            shutdown_timeout: Duration::from_secs(10),
            verbose: false,
        }
    }
}

// Layer 4: Builder
#[derive(Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_zip(mut self, path: PathBuf) -> Self {
        self.config.input_zip = path;
        self
    }

    pub fn output_dir(mut self, path: PathBuf) -> Self {
        self.config.output_dir = path;
        self
    }

    pub fn workers(mut self, count: usize) -> Self {
        self.config.workers = count;
        self
    }

    pub fn build(self) -> Result<Config> {
        self.config.validate()?;
        Ok(self.config)
    }
}

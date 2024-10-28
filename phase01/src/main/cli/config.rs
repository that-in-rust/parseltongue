//! CLI Configuration Types
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Configuration Integration
//! - ConfigBuilder     (builds full config)
//! - ConfigValidator   (validates settings)
//! 
//! Level 3: Config Components
//! - RuntimeConfig    (runtime settings)
//! - StorageConfig    (storage settings)
//! - MetricsConfig    (metrics settings)
//! 
//! Level 2: Config Implementation
//! - Config types     (concrete settings)
//! - Validation      (validation logic)
//! 
//! Level 1 (Base): Core Config Types
//! - Basic settings  (primitive types)
//! - Defaults        (default values)

use std::path::PathBuf;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use crate::core::error::Result;

// Design Choice: Using builder pattern for complex configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// Input ZIP file path
    pub input_path: PathBuf,
    /// Output directory path
    pub output_dir: PathBuf,
    /// Runtime configuration
    pub runtime: RuntimeConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
}

// Design Choice: Using separate configs for components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Number of worker threads
    pub workers: usize,
    /// Buffer size for streaming
    pub buffer_size: usize,
    /// Shutdown timeout
    pub shutdown_timeout: Duration,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            workers: num_cpus::get(),
            buffer_size: 64 * 1024,  // 64KB
            shutdown_timeout: Duration::from_secs(30),
            verbose: false,
        }
    }
}

// Design Choice: Using validation traits
pub trait ConfigValidator {
    fn validate(&self) -> Result<()>;
}

impl ConfigValidator for CliConfig {
    fn validate(&self) -> Result<()> {
        // Validate paths
        if !self.input_path.exists() {
            return Err(Error::InvalidPath(self.input_path.clone()));
        }
        
        // Validate runtime config
        if self.runtime.workers == 0 {
            return Err(Error::InvalidConfig("Workers must be > 0"));
        }
        
        Ok(())
    }
}

// Design Choice: Using builder for construction
#[derive(Default)]
pub struct ConfigBuilder {
    input_path: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    runtime: RuntimeConfig,
    storage: StorageConfig,
    metrics: MetricsConfig,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_path(mut self, path: PathBuf) -> Self {
        self.input_path = Some(path);
        self
    }

    pub fn output_dir(mut self, path: PathBuf) -> Self {
        self.output_dir = Some(path);
        self
    }

    pub fn build(self) -> Result<CliConfig> {
        let config = CliConfig {
            input_path: self.input_path.ok_or_else(|| Error::InvalidConfig("Missing input path"))?,
            output_dir: self.output_dir.ok_or_else(|| Error::InvalidConfig("Missing output dir"))?,
            runtime: self.runtime,
            storage: self.storage,
            metrics: self.metrics,
        };
        
        config.validate()?;
        Ok(config)
    }
}


//! Core Library Interface - Pyramidal Structure
//! Layer 1: Core Exports & Public Interface
//!   - Essential types and traits exposed to users
//! Layer 2: Module Organization
//!   - Submodule structure and visibility
//! Layer 3: Type Definitions & Configuration
//!   - Core types and their implementations
//! Layer 4: Feature Gates & Conditional Compilation
//!   - Optional functionality and platform-specific code
//! Layer 5: Resource Management & Cleanup
//!   - RAII patterns and resource handling

// Pyramid Structure: Public API -> Internal Modules -> Utilities
pub mod error;
pub mod prelude;
pub mod internal;

// Layer 1: Core Exports
pub mod zip;
pub mod storage;
pub mod runtime;
pub mod metrics;

// Re-exports for clean public API
pub use error::{Error, Result};
pub use prelude::*;

// Layer 2: Module Organization
#[doc(hidden)]
pub(crate) mod internal {
    pub(crate) mod validation;
    pub(crate) mod utils;
}

// Layer 3: Core Types
#[derive(Debug, Clone)]
pub struct Config {
    pub input_zip: std::path::PathBuf,
    pub output_dir: std::path::PathBuf,
    pub workers: usize,
    pub buffer_size: usize,
    pub shutdown_timeout: std::time::Duration,
    pub verbose: bool,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<()> {
        use internal::validation::PathValidator;
        
        PathValidator::new()
            .exists(&self.input_zip)
            .is_file(&self.input_zip)
            .is_zip(&self.input_zip)
            .parent_exists(&self.output_dir)
            .validate()?;

        if self.workers == 0 {
            return Err(Error::InvalidConfig("workers cannot be zero"));
        }
        if self.buffer_size < 1024 {
            return Err(Error::InvalidConfig("buffer_size must be at least 1KB"));
        }
        Ok(())
    }
}

// Layer 4: Builder Implementation
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    input_zip: Option<std::path::PathBuf>,
    output_dir: Option<std::path::PathBuf>,
    workers: Option<usize>,
    buffer_size: Option<usize>,
    shutdown_timeout: Option<std::time::Duration>,
    verbose: bool,
}

impl ConfigBuilder {
    pub fn input_zip<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.input_zip = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn output_dir<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
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

    pub fn shutdown_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.shutdown_timeout = Some(timeout);
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn build(self) -> Result<Config> {
        let config = Config {
            input_zip: self.input_zip.ok_or(Error::MissingConfig("input_zip"))?,
            output_dir: self.output_dir.ok_or(Error::MissingConfig("output_dir"))?,
            workers: self.workers.unwrap_or_else(num_cpus::get),
            buffer_size: self.buffer_size.unwrap_or(8 * 1024),
            shutdown_timeout: self.shutdown_timeout
                .unwrap_or_else(|| std::time::Duration::from_secs(30)),
            verbose: self.verbose,
        };
        config.validate()?;
        Ok(config)
    }
}

// Layer 5: Constants & Feature Gates
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MIN_RUST_VERSION: &str = "1.70.0";

#[cfg(feature = "metrics")]
pub use crate::metrics::MetricsManager;

// Default constants
pub const DEFAULT_BUFFER_SIZE: usize = 8 * 1024; // 8KB
pub const DEFAULT_SHUTDOWN_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_config_builder_validation() {
        let config = Config::builder()
            .input_zip("test.zip")
            .output_dir("output")
            .workers(2)
            .build();
        assert!(config.is_ok());
    }
}

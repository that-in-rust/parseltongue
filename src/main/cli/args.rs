//! CLI Argument Parsing - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Argument Definitions
//! Layer 3: Validation Rules
//! Layer 4: Conversion Logic
//! Layer 5: Helper Functions

use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;
use tracing::debug;

use super::config::Config;

// Layer 1: Core Types
#[derive(Parser, Debug)]
#[clap(version, about = "ZIP file analyzer and storage system")]
pub struct Args {
    /// Input ZIP file path
    #[clap(short, long, value_parser)]
    pub input_zip: PathBuf,

    /// Output directory path
    #[clap(short, long, value_parser)]
    pub output_dir: PathBuf,

    /// Number of worker threads
    #[clap(short, long, value_parser, default_value_t = num_cpus::get())]
    pub workers: usize,

    /// Buffer size in bytes
    #[clap(short, long, value_parser, default_value = "8192")]
    pub buffer_size: usize,

    /// Shutdown timeout in seconds
    #[clap(short, long, value_parser, default_value = "30")]
    pub shutdown_timeout: u64,

    /// Enable verbose logging
    #[clap(short, long, env = "PARSELTONGUE_VERBOSE")]
    pub verbose: bool,
}

// Layer 2: Implementation
impl Args {
    pub fn parse() -> Self {
        let args = <Self as Parser>::parse();
        debug!("Parsed CLI arguments: {:?}", args);
        args
    }

    // Layer 3: Validation
    fn validate(&self) -> Result<()> {
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

    // Layer 4: Conversion
    pub fn into_config(self) -> Result<Config> {
        self.validate()?;
        
        Config::builder()
            .input_path(self.input_zip)
            .output_dir(self.output_dir)
            .workers(self.workers)
            .buffer_size(self.buffer_size)
            .shutdown_timeout(std::time::Duration::from_secs(self.shutdown_timeout))
            .verbose(self.verbose)
            .build()
    }
}

// Layer 5: Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_args_validation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let input_file = temp_dir.path().join("test.zip");
        File::create(&input_file)?;

        let args = Args {
            input_zip: input_file,
            output_dir: temp_dir.path().to_path_buf(),
            workers: 2,
            buffer_size: 8192,
            shutdown_timeout: 30,
            verbose: false,
        };

        assert!(args.validate().is_ok());
        Ok(())
    }

    #[test]
    fn test_invalid_args() {
        let args = Args {
            input_zip: PathBuf::from("nonexistent.zip"),
            output_dir: PathBuf::from("output"),
            workers: 0,
            buffer_size: 512,
            shutdown_timeout: 30,
            verbose: false,
        };

        assert!(args.validate().is_err());
    }
}

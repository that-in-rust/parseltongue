//! CLI Argument Handling
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Argument Processing
//! - Argument validation
//! - Path canonicalization
//! - Default handling
//! 
//! Level 3: Argument Types
//! - Input arguments
//! - Output arguments
//! - Runtime arguments
//! 
//! Level 2: Validation Rules
//! - Path validation
//! - Value validation
//! - Format validation
//! 
//! Level 1 (Base): Core Types
//! - Argument struct
//! - Validation types
//! - Helper types

use std::path::PathBuf;
use clap::Parser;
use crate::core::error::Result;

// Design Choice: Using clap for argument parsing
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// Input ZIP file path
    #[clap(short = 'i', long = "input-zip")]
    pub input_path: PathBuf,

    /// Output directory path
    #[clap(short = 'o', long = "output-dir")]
    pub output_dir: PathBuf,

    /// Number of worker threads
    #[clap(short = 'w', long = "workers", default_value = "4")]
    pub workers: usize,

    /// Buffer size in bytes
    #[clap(short = 'b', long = "buffer-size", default_value = "65536")]
    pub buffer_size: usize,

    /// Shutdown timeout in seconds
    #[clap(short = 's', long = "shutdown-timeout", default_value = "30")]
    pub shutdown_timeout: u64,

    /// Enable verbose output
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,
}

impl Args {
    pub fn validate(&self) -> Result<()> {
        self.validate_input_path()?;
        self.validate_output_dir()?;
        self.validate_workers()?;
        self.validate_buffer_size()?;
        Ok(())
    }

    fn validate_input_path(&self) -> Result<()> {
        if !self.input_path.exists() {
            return Err(crate::core::error::Error::InvalidPath(self.input_path.clone()));
        }
        Ok(())
    }

    fn validate_output_dir(&self) -> Result<()> {
        if !self.output_dir.exists() {
            std::fs::create_dir_all(&self.output_dir)?;
        }
        Ok(())
    }

    fn validate_workers(&self) -> Result<()> {
        if self.workers == 0 {
            return Err(crate::core::error::Error::ResourceLimit(
                "Worker count must be > 0".into()
            ));
        }
        Ok(())
    }

    fn validate_buffer_size(&self) -> Result<()> {
        if self.buffer_size == 0 {
            return Err(crate::core::error::Error::ResourceLimit(
                "Buffer size must be > 0".into()
            ));
        }
        Ok(())
    }
}

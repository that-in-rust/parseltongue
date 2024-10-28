//! CLI Arguments - Pyramidal Structure
//! Layer 1: Argument Definition
//! Layer 2: Argument Parsing
//! Layer 3: Validation & Conversion
//! Layer 4: Error Handling
//! Layer 5: Help Text Generation

use clap::{Parser, ArgAction};
use std::path::PathBuf;
use anyhow::Result;
use super::config::{Config, ConfigBuilder};

// Layer 1: CLI Arguments Structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input ZIP file path
    #[arg(short = 'i', long, value_name = "FILE")]
    input_zip: PathBuf,

    /// Output directory path
    #[arg(short = 'o', long, value_name = "DIR")]
    output_dir: PathBuf,

    /// Number of worker threads
    #[arg(short = 'w', long, default_value_t = num_cpus::get())]
    workers: usize,

    /// Buffer size in KB
    #[arg(short = 'b', long, default_value_t = 8)]
    buffer_size: usize,

    /// Shutdown timeout in seconds
    #[arg(short = 's', long, default_value_t = 10)]
    shutdown_timeout: u64,

    /// Enable verbose logging
    #[arg(short = 'v', long, action = ArgAction::SetTrue)]
    verbose: bool,
}

// Layer 2: Argument Implementation
impl Args {
    // Layer 3: Parse and Convert
    pub fn parse_args() -> Result<Config> {
        let args = Self::parse();
        args.into_config()
    }

    // Layer 4: Config Conversion
    fn into_config(self) -> Result<Config> {
        ConfigBuilder::new()
            .input_zip(self.input_zip)
            .output_dir(self.output_dir)
            .workers(self.workers)
            .build()
    }
}

// Layer 5: Custom Error Types
#[derive(Debug, thiserror::Error)]
pub enum ArgsError {
    #[error("Invalid input path: {0}")]
    InvalidInput(String),
    
    #[error("Invalid output path: {0}")]
    InvalidOutput(String),
    
    #[error("Invalid worker count: {0}")]
    InvalidWorkers(String),
}

// Level 4: CLI Argument Handling
// - Manages command-line interface
// - Validates user input
// - Provides configuration
// - Handles environment vars

use clap::Parser;
use std::path::PathBuf;
use crate::core::{error::Result, types::Config};

// Level 3: CLI Arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, help = "Input ZIP file path")]
    input: PathBuf,

    #[arg(short, long, help = "Output directory")]
    output: PathBuf,

    #[arg(short, long, default_value_t = num_cpus::get(),
          help = "Number of worker threads")]
    workers: usize,

    #[arg(short, long, default_value = "32768",
          help = "Buffer size in bytes")]
    buffer_size: usize,
}

impl Args {
    // Level 2: Argument Processing
    pub fn parse_args() -> Self {
        Self::parse()
    }

    // Level 1: Configuration Creation
    pub fn into_config(self) -> Result<Config> {
        Ok(Config {
            input_path: self.input,
            output_dir: self.output,
            worker_threads: self.workers,
            buffer_size: self.buffer_size,
            shutdown_timeout: std::time::Duration::from_secs(30),
        })
    }
} 
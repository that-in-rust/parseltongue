// Level 4: CLI Argument Processing
// - Handles command line argument parsing
// - Validates input parameters
// - Constructs configuration

use clap::{Parser, ArgAction};
use crate::error::Result;
use crate::config::Config;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, help = "Input ZIP file path")]
    input: PathBuf,

    #[arg(short, long, help = "Output directory path")]
    output: PathBuf,

    #[arg(short, long, action = ArgAction::Count, help = "Verbosity level")]
    verbose: u8,

    #[arg(short, long, default_value = "4", help = "Number of worker threads")]
    workers: usize,

    #[arg(short, long, default_value = "8192", help = "Buffer size in bytes")]
    buffer_size: usize,
}

impl Args {
    pub fn parse_args() -> Result<Config> {
        let args = Self::parse();
        
        Ok(Config {
            input_zip: args.input,
            output_dir: args.output,
            verbose: args.verbose > 0,
            workers: args.workers,
            buffer_size: args.buffer_size,
        })
    }
} 
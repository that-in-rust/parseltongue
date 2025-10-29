use clap::Parser;
use std::path::PathBuf;

/// Ultra-minimalist state reset tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to CozoDB database
    #[arg(long)]
    pub database: PathBuf,

    /// Project root directory for re-indexing
    #[arg(long)]
    pub project_path: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

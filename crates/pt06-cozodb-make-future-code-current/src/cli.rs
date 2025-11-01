use clap::Parser;
use std::path::PathBuf;

/// Ultra-minimalist state reset tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// CozoDB connection string (e.g., "rocksdb:parseltongue.db" or "sqlite:db.sqlite")
    #[arg(long)]
    pub database: String,

    /// Project root directory for re-indexing
    #[arg(long)]
    pub project_path: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Automatically re-index after reset (PRD-compliant)
    #[arg(long, default_value_t = true)]
    pub reindex: bool,
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

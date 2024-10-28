//! CLI Module - Pyramidal Structure
//! Layer 1: Public Interface
//! Layer 2: Command Processing
//! Layer 3: Error Handling
//! Layer 4: Logging Setup
//! Layer 5: Resource Management

use anyhow::Result;
use tracing::{info, warn};

mod args;
mod config;

pub use config::Config;
use args::Args;

// Layer 1: Public Interface
pub async fn initialize() -> Result<Config> {
    // Layer 2: Setup & Configuration
    setup_logging()?;
    let config = process_args()?;
    
    // Layer 3: Validation & Logging
    info!("Initializing with config: {:?}", config);
    validate_paths(&config)?;
    
    Ok(config)
}

// Layer 4: Support Functions
fn setup_logging() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
    
    Ok(())
}

fn process_args() -> Result<Config> {
    let config = Args::parse_args()?;
    
    if config.verbose {
        warn!("Verbose logging enabled");
    }
    
    Ok(config)
}

// Layer 5: Validation Functions
fn validate_paths(config: &Config) -> Result<()> {
    use std::fs;
    
    // Ensure output directory exists or create it
    if !config.output_dir.exists() {
        fs::create_dir_all(&config.output_dir)?;
    }
    
    // Validate input file
    if !config.input_zip.is_file() {
        anyhow::bail!("Input path is not a file: {:?}", config.input_zip);
    }
    
    Ok(())
}

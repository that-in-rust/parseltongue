//! CLI Binary - Pyramidal Structure
//! Layer 1: CLI Interface & Arguments
//!   - Command-line parsing and validation
//! Layer 2: Application Setup
//!   - Logging, configuration, and initialization
//! Layer 3: Core Processing
//!   - Main application logic and workflow
//! Layer 4: Error Handling & Cleanup
//!   - Error management and resource cleanup
//! Layer 5: Support Functions
//!   - Utility functions and helpers

use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::Parser;
use tokio;
use tracing::{error, info, Level};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter, FmtSubscriber};

// Layer 1: CLI Arguments
#[derive(Parser, Debug)]
#[clap(version, about = "ZIP file analyzer and storage system")]
struct Args {
    /// Input ZIP file path
    #[clap(short, long, value_parser)]
    input_zip: PathBuf,

    /// Output directory path
    #[clap(short, long, value_parser)]
    output_dir: PathBuf,

    /// Number of worker threads
    #[clap(short, long, value_parser, default_value_t = num_cpus::get())]
    workers: usize,

    /// Buffer size in bytes
    #[clap(short, long, value_parser, default_value = "8192")]
    buffer_size: usize,

    /// Shutdown timeout in seconds
    #[clap(short, long, value_parser, default_value = "30")]
    shutdown_timeout: u64,

    /// Enable verbose logging
    #[clap(short, long, env = "PARSELTONGUE_VERBOSE")]
    verbose: bool,
}

// Layer 2: Main Entry Point
#[tokio::main]
async fn main() -> Result<()> {
    // Early CLI parsing to catch errors before logging setup
    let args = Args::parse();
    
    // Setup logging with proper context
    setup_logging(args.verbose)
        .context("Failed to initialize logging")?;
    
    info!("Starting ZIP processor v{}", parseltongue::VERSION);

    // Early path validation
    validate_paths(&args.input_zip, &args.output_dir)
        .context("Invalid paths provided")?;

    // Layer 3: Configuration & Processing
    let config = build_config(args)
        .context("Failed to build configuration")?;

    // Main processing with proper error context
    process_zip(config).await
        .context("ZIP processing failed")?;

    info!("Processing completed successfully");
    Ok(())
}

// Layer 4: Core Functions
fn build_config(args: Args) -> Result<parseltongue::Config> {
    parseltongue::Config::builder()
        .input_zip(args.input_zip)
        .output_dir(args.output_dir)
        .workers(args.workers)
        .buffer_size(args.buffer_size)
        .shutdown_timeout(std::time::Duration::from_secs(args.shutdown_timeout))
        .verbose(args.verbose)
        .build()
        .context("Invalid configuration")
}

async fn process_zip(config: parseltongue::Config) -> Result<()> {
    info!("Processing ZIP file: {}", config.input_zip.display());
    
    // TODO: Implement actual processing
    // This is a placeholder that follows error handling patterns
    Ok(())
}

// Layer 5: Support Functions
fn validate_paths(input: &PathBuf, output: &PathBuf) -> Result<()> {
    if !input.exists() {
        anyhow::bail!("Input file does not exist: {}", input.display());
    }
    if !input.is_file() {
        anyhow::bail!("Input path is not a file: {}", input.display());
    }
    if output.exists() && !output.is_dir() {
        anyhow::bail!("Output path exists but is not a directory: {}", output.display());
    }
    Ok(())
}

fn setup_logging(verbose: bool) -> Result<()> {
    let level = if verbose { Level::DEBUG } else { Level::INFO };
    
    FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .try_init()
        .context("Failed to initialize logging subscriber")?;

    Ok(())
}

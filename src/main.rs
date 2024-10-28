//! CLI Binary - Pyramidal Structure
//! Layer 1: Entry Point & CLI
//! Layer 2: Configuration
//! Layer 3: Runtime Setup
//! Layer 4: Processing Pipeline
//! Layer 5: Cleanup & Shutdown

use anyhow::Result;
use clap::Parser;
use tracing::{info, error};

// Layer 1: Local Imports
use parseltongue::{
    main::cli::{Args, Config},
    runtime::RuntimeManager,
    zip::ZipProcessor,
    storage::StorageManager,
    metrics::MetricsManager,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Layer 2: CLI & Config
    let args = Args::parse();
    let config = Config::from_args(args)?;
    setup_tracing(&config)?;

    info!("Initializing ZIP processor...");

    // Layer 3: Component Setup
    let runtime = RuntimeManager::new(&config.runtime)?;
    let storage = StorageManager::new(&config.storage).await?;
    let metrics = MetricsManager::new();

    let processor = ZipProcessor::new(
        config.input_zip,
        config.output_dir,
        storage,
        metrics,
    ).await?;

    // Layer 4: Processing
    let result = tokio::select! {
        r = processor.process() => r,
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
            Ok(())
        }
    };

    // Layer 5: Cleanup
    runtime.shutdown().await?;
    
    match result {
        Ok(_) => info!("Processing complete"),
        Err(e) => error!("Processing failed: {}", e),
    }
    
    result
}

fn setup_tracing(config: &Config) -> Result<()> {
    let level = if config.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    Ok(())
}

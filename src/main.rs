//! Main Binary - Pyramidal Structure
//! Layer 1: Entry Point
//! Layer 2: Configuration
//! Layer 3: Runtime Setup
//! Layer 4: Processing
//! Layer 5: Cleanup

use anyhow::Result;
use clap::Parser;
use tokio;
use tracing::{info, error};

use parseltongue::{
    ZipAnalyzer,
    AnalyzerConfig,
    main::cli::{Args, Config},
};

// Layer 1: Entry Point
#[tokio::main]
async fn main() -> Result<()> {
    // Layer 2: Setup
    let args = Args::parse();
    let config = Config::from_args(args)?;
    setup_logging(config.verbose)?;

    info!("Starting ZIP analysis...");

    // Layer 3: Analyzer Setup
    let analyzer = ZipAnalyzer::new(AnalyzerConfig {
        input_zip: config.input_zip,
        output_dir: config.output_dir,
        workers: config.workers,
        buffer_size: config.buffer_size,
        shutdown_timeout: config.shutdown_timeout,
    }).await?;

    // Layer 4: Processing
    let result = tokio::select! {
        stats = analyzer.analyze() => {
            match stats {
                Ok(stats) => {
                    info!(
                        "Analysis complete: {} files ({} bytes) in {:?}",
                        stats.files_processed,
                        stats.bytes_processed,
                        stats.duration
                    );
                    Ok(())
                },
                Err(e) => {
                    error!("Analysis failed: {}", e);
                    Err(e)
                }
            }
        },
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
            Ok(())
        }
    };

    // Layer 5: Cleanup
    analyzer.shutdown().await?;
    result
}

fn setup_logging(verbose: bool) -> Result<()> {
    let level = if verbose {
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

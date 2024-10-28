//! Main Entry Point
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Application Orchestration
//! - RuntimeManager    (manages async runtime)
//! - ShutdownManager   (manages graceful shutdown)
//! - MetricsManager    (manages metrics collection)
//! 
//! Level 3: Processing Pipeline
//! - ZipProcessor     (processes ZIP files)
//! - StorageManager   (manages storage)
//! - ValidationManager (manages validation)
//! 
//! Level 2: Infrastructure Setup
//! - LoggingSetup     (configures logging)
//! - MetricsSetup     (configures metrics)
//! - StorageSetup     (configures storage)
//! 
//! Level 1 (Base): Core Setup
//! - ArgumentParser   (parses CLI args)
//! - ConfigBuilder    (builds configuration)
//! - ErrorHandler     (handles errors)

use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Context, Result};
use clap::Parser;
use tokio::signal;
use tracing::{info, warn, error};
use tracing_subscriber::{self, EnvFilter};

use crate::{
    cli::{Args, ProgressBar},
    core::{error::Error, types::*},
    storage::{StorageManager, StorageConfig},
    zip::{ZipProcessor, ZipConfig},
    metrics::{MetricsManager, MetricsConfig},
};

// ===== Level 1: Core Setup =====
// Design Choice: Using clap for CLI argument parsing

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct CliArgs {
    /// Input ZIP file path
    #[clap(short = 'i', long = "input-zip")]
    input_path: PathBuf,

    /// Output directory path
    #[clap(short = 'o', long = "output-dir")]
    output_dir: PathBuf,

    /// Number of worker threads
    #[clap(short = 'w', long = "workers", default_value = "4")]
    workers: usize,

    /// Buffer size in bytes
    #[clap(short = 'b', long = "buffer-size", default_value = "8192")]
    buffer_size: usize,

    /// Shutdown timeout in seconds
    #[clap(short = 's', long = "shutdown-timeout", default_value = "30")]
    shutdown_timeout: u64,

    /// Enable verbose output
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

// ===== Level 2: Infrastructure Setup =====
// Design Choice: Using builder pattern for setup

/// Application configuration
struct AppConfig {
    args: CliArgs,
    runtime_config: RuntimeConfig,
    storage_config: StorageConfig,
    metrics_config: MetricsConfig,
}

impl AppConfig {
    /// Creates configuration from CLI arguments
    fn from_args(args: CliArgs) -> Result<Self> {
        // Validate paths
        if !args.input_path.exists() {
            return Err(Error::InvalidPath(args.input_path.clone()).into());
        }

        std::fs::create_dir_all(&args.output_dir)
            .context("Failed to create output directory")?;

        // Create configurations
        let runtime_config = RuntimeConfig {
            worker_threads: args.workers,
            buffer_size: args.buffer_size,
            shutdown_timeout: std::time::Duration::from_secs(args.shutdown_timeout),
        };

        let storage_config = StorageConfig {
            path: args.output_dir.join("storage"),
            pool_size: args.workers,
            batch_size: 1000,
            index_config: Default::default(),
        };

        let metrics_config = MetricsConfig {
            enabled: true,
            interval: std::time::Duration::from_secs(1),
            format: Default::default(),
        };

        Ok(Self {
            args,
            runtime_config,
            storage_config,
            metrics_config,
        })
    }
}

// ===== Level 3: Processing Pipeline =====
// Design Choice: Using async/await for concurrency

/// Application state
struct App {
    config: AppConfig,
    storage: Arc<StorageManager>,
    processor: Arc<ZipProcessor>,
    metrics: Arc<MetricsManager>,
}

impl App {
    /// Creates new application instance
    async fn new(config: AppConfig) -> Result<Self> {
        // Initialize components
        let storage = Arc::new(StorageManager::new(config.storage_config.clone()).await?);
        let processor = Arc::new(ZipProcessor::new(
            ZipConfig::default(),
            storage.clone(),
        ));
        let metrics = Arc::new(MetricsManager::new(config.metrics_config.clone()));

        Ok(Self {
            config,
            storage,
            processor,
            metrics,
        })
    }

    /// Runs the application
    async fn run(&self) -> Result<()> {
        // Start metrics collection
        self.metrics.start().await?;

        // Create progress bar
        let progress = ProgressBar::new();
        progress.set_message("Processing ZIP file...");

        // Process ZIP file
        let file = tokio::fs::File::open(&self.config.args.input_path).await
            .context("Failed to open ZIP file")?;

        self.processor.process(file).await?;

        // Cleanup
        progress.finish_with_message("Processing complete");
        self.metrics.stop().await?;

        Ok(())
    }
}

// ===== Level 4: Application Orchestration =====
// Design Choice: Using tokio for async runtime

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments
    let args = CliArgs::parse();

    // Setup logging
    let filter = if args.verbose {
        EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into())
    } else {
        EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into())
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Create application
    let config = AppConfig::from_args(args)?;
    let app = App::new(config).await?;

    // Setup signal handlers
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
    let shutdown_tx = Arc::new(shutdown_tx);

    tokio::spawn({
        let tx = shutdown_tx.clone();
        async move {
            if let Ok(()) = signal::ctrl_c().await {
                info!("Received Ctrl+C, initiating shutdown");
                let _ = tx.send(());
            }
        }
    });

    // Run application with shutdown handling
    tokio::select! {
        result = app.run() => {
            if let Err(e) = result {
                error!("Application error: {}", e);
                return Err(e);
            }
        }
        _ = shutdown_rx.recv() => {
            info!("Shutting down gracefully");
        }
    }

    Ok(())
}

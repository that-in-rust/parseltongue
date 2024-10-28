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

use std::path::Path;
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

/// Application configuration
#[derive(Debug)]
struct AppConfig {
    /// CLI arguments
    args: Args,
    /// Runtime configuration
    runtime_config: RuntimeConfig,
    /// Storage configuration
    storage_config: StorageConfig,
    /// Metrics configuration
    metrics_config: MetricsConfig,
}

impl AppConfig {
    /// Creates configuration from CLI arguments
    fn from_args(args: Args) -> Result<Self> {
        // Validate paths using AsRef<Path>
        let input_path = args.input_path.as_path();
        if !input_path.exists() {
            return Err(Error::InvalidPath(args.input_path.clone()).into());
        }

        let output_dir = args.output_dir.as_path();
        std::fs::create_dir_all(output_dir)
            .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

        // Create configurations
        let runtime_config = args.into_config();

        let storage_config = StorageConfig {
            path: output_dir.join("storage"),
            pool_size: args.workers,
            batch_size: 1000,
            index_config: Default::default(),
        };

        let metrics_config = MetricsConfig {
            enabled: true,
            interval: std::time::Duration::from_secs(1),
            format: Default::default(),
            output_dir: output_dir.join("metrics"),
        };

        Ok(Self {
            args,
            runtime_config,
            storage_config,
            metrics_config,
        })
    }
}

// ===== Level 2: Infrastructure Setup =====
// Design Choice: Using builder pattern for setup

/// Application state
struct App {
    /// Application configuration
    config: AppConfig,
    /// Storage manager
    storage: Arc<StorageManager>,
    /// ZIP processor
    processor: Arc<ZipProcessor>,
    /// Metrics manager
    metrics: Arc<MetricsManager>,
}

impl App {
    /// Creates new application instance
    async fn new(config: AppConfig) -> Result<Self> {
        // Initialize storage
        let storage = Arc::new(StorageManager::new(config.storage_config.clone()).await?);
        
        // Initialize processor
        let processor = Arc::new(ZipProcessor::new(
            ZipConfig::default(),
            storage.clone(),
        ));
        
        // Initialize metrics
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
            .with_context(|| format!("Failed to open ZIP file: {}", self.config.args.input_path.display()))?;

        self.processor.process(file).await?;

        // Cleanup
        progress.finish_with_message("Processing complete");
        self.metrics.stop().await?;

        Ok(())
    }
}

// ===== Level 3: Processing Pipeline =====
// Design Choice: Using async/await for concurrency

/// Sets up logging
fn setup_logging(verbose: bool) -> Result<()> {
    let filter = if verbose {
        EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into())
    } else {
        EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into())
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_file(true)
        .with_line_number(true)
        .init();

    Ok(())
}

// ===== Level 4: Application Orchestration =====
// Design Choice: Using tokio for async runtime

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments
    let args = Args::parse();

    // Setup logging
    setup_logging(args.verbose)?;

    // Create application
    let config = AppConfig::from_args(args)?;
    let app = App::new(config).await?;

    // Setup signal handlers
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
    let shutdown_tx = Arc::new(shutdown_tx);

    tokio::spawn({
        let tx = Arc::clone(&shutdown_tx);
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
                error!("Application error: {:#}", e);
                return Err(e);
            }
        }
        _ = shutdown_rx.recv() => {
            info!("Shutting down gracefully");
        }
    }

    Ok(())
}

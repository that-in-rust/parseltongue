use std::sync::Arc;
use anyhow::Result;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber::{self, EnvFilter};
use clap::Parser;

use crate::{
    cli::{Args, ProgressBar},
    core::{error::Error, types::*},
    storage::{StorageManager, StorageConfig},
    zip::{ZipProcessor, ZipConfig},
    metrics::{MetricsManager, MetricsConfig},
    runtime::{RuntimeConfig, WorkerConfig, ResourceLimits, ShutdownConfig},
};

// Design Choice: Using builder pattern for application configuration
#[derive(Debug)]
struct AppConfig {
    args: Args,
    runtime: RuntimeConfig,
    storage: StorageConfig,
    metrics: MetricsConfig,
    zip: ZipConfig,
}

impl AppConfig {
    fn from_args(args: Args) -> Result<Self> {
        // Create configurations based on CLI args
        let runtime = RuntimeConfig::new()
            .with_workers(args.workers)
            .with_buffer_size(args.buffer_size)
            .with_shutdown_timeout(args.shutdown_timeout);

        let storage = StorageConfig::new(&args.output_dir.join("db"));
        let metrics = MetricsConfig::new(&args.output_dir.join("metrics"));
        let zip = ZipConfig::new(args.buffer_size);

        Ok(Self {
            args,
            runtime,
            storage,
            metrics,
            zip,
        })
    }
}

// Design Choice: Using tokio for async runtime
#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let args = Args::parse();

    // Setup logging with tracing
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

    // Initialize application
    let app = App::new(args).await?;

    // Handle shutdown signals
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
    let shutdown_tx = Arc::new(shutdown_tx);

    // Design Choice: Using separate task for signal handling
    tokio::spawn({
        let tx = Arc::clone(&shutdown_tx);
        async move {
            if let Ok(()) = signal::ctrl_c().await {
                info!("Received Ctrl+C, initiating shutdown");
                let _ = tx.send(());
            }
        }
    });

    // Design Choice: Using select for shutdown coordination
    tokio::select! {
        result = app.run() => {
            if let Err(e) = result {
                error!("Application error: {:#}", e);
                return Err(e.into());
            }
        }
        _ = shutdown_rx.recv() => {
            info!("Shutting down gracefully");
            app.shutdown().await?;
        }
    }

    Ok(())
}

// Design Choice: Using Arc for shared components
struct App {
    config: AppConfig,
    storage: Arc<StorageManager>,
    processor: Arc<ZipProcessor>,
    metrics: Arc<MetricsManager>,
}

impl App {
    async fn new(args: Args) -> Result<Self> {
        let config = AppConfig::from_args(args)?;
        
        let storage: Arc<StorageManager> = Arc::new(
            StorageManager::new(config.storage.clone()).await?
        );
        
        let processor = Arc::new(ZipProcessor::new(
            config.zip.clone(),
            storage.clone(),
        ));
        
        let metrics = Arc::new(MetricsManager::new(config.metrics.clone()));

        Ok(Self {
            config,
            storage,
            processor,
            metrics,
        })
    }

    async fn run(&self) -> Result<()> {
        self.metrics.start().await?;
        
        let progress = ProgressBar::new();
        progress.set_message("Processing ZIP file...");

        self.processor.process_file(&self.config.args.input_path).await?;

        progress.finish_with_message("Processing complete");
        self.metrics.stop().await?;

        Ok(())
    }

    async fn shutdown(self) -> Result<()> {
        info!("Shutting down application components");
        self.metrics.stop().await?;
        self.processor.shutdown().await?;
        self.storage.shutdown().await?;
        Ok(())
    }
}

// Rest of the code...

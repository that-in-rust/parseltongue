//! ZIP Processing CLI Entry Point
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Application Entry
//! - Main Function     (entry point)
//!   ├── CLI setup
//!   ├── Runtime setup
//!   └── Error handling
//! 
//! Level 3: Core Setup
//! - Logging Setup    (tracing setup)
//! - Runtime Setup   (tokio runtime)
//! - Signal Setup    (signal handling)
//! 
//! Level 2: Infrastructure
//! - Config Building (config setup)
//! - App Building    (app setup)
//! - Error Handling  (error setup)
//! 
//! Level 1 (Base): Core Types
//! - App Config      (configuration)
//! - App Error       (error types)
//! - App State       (state types)

use std::path::Path;
use std::sync::Arc;
use anyhow::{Context, Result};
use clap::Parser;
use tokio::signal;
use tracing::{info, warn, error};
use tracing_subscriber::{self, EnvFilter};
use chrono::Utc;

use parseltongue::{
    cli::{Args, ProgressBar},
    core::{error::Error, types::*},
    storage::{StorageManager, StorageConfig},
    zip::{ZipProcessor, ZipConfig},
    metrics::{MetricsManager, MetricsConfig},
};

// Design Choice: Using builder pattern for configuration
#[derive(Debug)]
struct AppConfig {
    args: Args,
    runtime_config: RuntimeConfig,
    storage_config: StorageConfig,
    metrics_config: MetricsConfig,
}

impl AppConfig {
    fn from_args(args: Args) -> Result<Self> {
        // Create timestamped output directory
        let zip_name = args.input_path.file_stem()
            .ok_or_else(|| Error::InvalidPath("ZIP file has no name".into()))?
            .to_string_lossy();
        
        let timestamp = Utc::now().format("%Y%m%d%H%M%S");
        let analysis_dir = args.output_dir.join(format!("{}-{}", zip_name, timestamp));

        // Create directory structure using tokio::fs
        tokio::fs::create_dir_all(&analysis_dir)
            .await
            .with_context(|| format!("Failed to create analysis directory: {}", analysis_dir.display()))?;

        for subdir in ["db", "logs", "metrics"] {
            tokio::fs::create_dir_all(analysis_dir.join(subdir))
                .await
                .with_context(|| format!("Failed to create {} directory", subdir))?;
        }

        // Create configurations
        let runtime_config = RuntimeConfig {
            worker_config: WorkerConfig {
                thread_count: args.workers,
                queue_capacity: 1000,
                stack_size: 3 * 1024 * 1024,
            },
            resource_limits: ResourceLimits {
                max_tasks: args.workers * 2,
                max_memory: 1024 * 1024 * 1024,
                max_connections: args.workers,
            },
            shutdown_config: ShutdownConfig {
                timeout: std::time::Duration::from_secs(args.shutdown_timeout),
                force_after_timeout: true,
            },
        };

        let storage_config = StorageConfig {
            path: analysis_dir.join("db"),
            pool_size: args.workers,
            batch_size: 1000,
            index_config: Default::default(),
        };

        let metrics_config = MetricsConfig {
            enabled: true,
            interval: std::time::Duration::from_secs(1),
            format: Default::default(),
            output_dir: analysis_dir.join("metrics"),
        };

        Ok(Self {
            args,
            runtime_config,
            storage_config,
            metrics_config,
        })
    }
}

// Design Choice: Using async/await for main application
struct App {
    config: AppConfig,
    storage: Arc<StorageManager>,
    processor: Arc<ZipProcessor>,
    metrics: Arc<MetricsManager>,
}

impl App {
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

// Design Choice: Using tokio for async runtime
#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments
    let args = Args::parse();

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

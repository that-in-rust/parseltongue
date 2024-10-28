//! Main Entry Point
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Application Orchestration
//! - App              (main application)
//!   ├── Lifecycle management
//!   ├── Error handling
//!   └── Shutdown coordination
//! 
//! Level 3: Core Setup
//! - RuntimeSetup    (tokio runtime)
//!   ├── Worker configuration
//!   ├── Resource limits
//!   └── Shutdown parameters
//! 
//! Level 2: Infrastructure Setup
//! - LoggingSetup    (tracing setup)
//! - MetricsSetup    (metrics setup)
//! - StorageSetup    (storage setup)
//! 
//! Level 1 (Base): CLI Setup
//! - ArgParser       (argument parsing)
//! - ConfigBuilder   (config building)
//! - ErrorHandler    (error handling)

pub mod cli;

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

// Design Choice: Using builder pattern for configuration
#[derive(Debug)]
struct AppConfig {
    args: Args,
    runtime_config: RuntimeConfig,
    storage_config: StorageConfig,
    metrics_config: MetricsConfig,
}

// Rest of implementation remains the same as it's well-aligned

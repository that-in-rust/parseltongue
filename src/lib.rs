//! Core Library Interface
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Public API Layer
//! - Core traits and interfaces for ZIP processing
//! - Storage abstractions and implementations
//! - Metrics and monitoring interfaces
//! 
//! Level 3: Feature Coordination Layer
//! - ZIP processing coordination
//! - Storage management coordination
//! - Runtime coordination
//! 
//! Level 2: Component Layer
//! - CLI infrastructure
//! - Async processing components
//! - Resource management
//! 
//! Level 1 (Base): Core Infrastructure Layer
//! - Error handling
//! - Common types
//! - Shared utilities

// Design Choice: Using explicit module hierarchy for clear boundaries
pub mod cli;
pub mod core;
pub mod storage;
pub mod zip;
pub mod metrics;
pub mod runtime;

// Design Choice: Re-exporting key types for ergonomic public API
pub use cli::{Args, ProgressBar};
pub use core::{error::{Error, Result}, types::*};
pub use storage::{StorageManager, StorageConfig};
pub use zip::{ZipProcessor, ZipConfig};
pub use metrics::{MetricsManager, MetricsConfig};
pub use runtime::{RuntimeConfig, WorkerConfig, ResourceLimits, ShutdownConfig};

// Design Choice: Using prelude for commonly used imports
pub mod prelude {
    pub use super::core::error::{Error, Result};
    pub use super::core::types::*;
    pub use tokio;
    pub use async_trait::async_trait;
    pub use futures::{Stream, StreamExt};
    pub use bytes::Bytes;
}

// Design Choice: Using async traits for core interfaces
#[async_trait::async_trait]
pub trait AsyncProcessor: Send + Sync + 'static {
    async fn process(&self) -> Result<()>;
    async fn shutdown(&self) -> Result<()>;
}

// Design Choice: Using builder pattern for configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub runtime: RuntimeConfig,
    pub storage: StorageConfig,
    pub metrics: MetricsConfig,
    pub zip: ZipConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            runtime: RuntimeConfig::default(),
            storage: StorageConfig::default(),
            metrics: MetricsConfig::default(),
            zip: ZipConfig::default(),
        }
    }
}

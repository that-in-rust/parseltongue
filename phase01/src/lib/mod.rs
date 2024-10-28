//! ZIP File Analysis and Storage Library
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Core Traits
//! - AsyncZipProcessor: ZIP processing interface
//! - AsyncStorageBackend: Storage abstraction
//! - StreamProcessor: Stream handling with backpressure
//! 
//! Level 3: Error Types
//! - ProcessingError: ZIP/encoding errors
//! - StorageError: Database errors
//! - RuntimeError: Task/resource errors
//! 
//! Level 2: Configuration Types
//! - ProcessingConfig: ZIP handling settings
//! - StorageConfig: Database settings
//! - RuntimeConfig: Worker/resource settings
//! 
//! Level 1 (Base): Core Types
//! - Buffer: Byte buffer type
//! - Path: Path type
//! - Result: Result type

// Design Choice: Using explicit module hierarchy
pub mod error;
pub mod prelude;

// Re-export main modules
pub mod core;
pub mod cli;
pub mod storage;
pub mod zip;
pub mod utils;
pub mod metrics;

// Re-export main types for public API
pub use crate::{
    zip::{ZipProcessor, ZipConfig, ZipEntry},
    storage::{StorageManager, StorageConfig},
    metrics::{MetricsManager, MetricsConfig},
    core::{error::{Error, Result}, types::*},
};

// Design Choice: Using type aliases for common types
pub type Buffer = bytes::Bytes;
pub type Path = std::path::PathBuf;

// Design Choice: Using async traits for main operations
#[async_trait::async_trait]
pub trait AsyncZipProcessor: Send + Sync + 'static {
    /// Process ZIP file
    async fn process(&self, path: &Path) -> Result<ProcessingStats>;
    /// Get processing metrics
    async fn metrics(&self) -> Result<ProcessingMetrics>;
}

#[async_trait::async_trait]
pub trait AsyncStorageBackend: Send + Sync + 'static {
    /// Store data with key
    async fn store(&self, key: &str, data: Buffer) -> Result<()>;
    /// Batch store multiple entries
    async fn batch_store(&self, entries: Vec<(String, Buffer)>) -> Result<()>;
}

#[async_trait::async_trait]
pub trait StreamProcessor: Send + Sync + 'static {
    /// Process stream with backpressure
    async fn process_stream<S, T>(&self, stream: S) -> Result<()>
    where
        S: Stream<Item = Result<T>> + Send + 'static,
        T: Send + 'static;
}

// Design Choice: Using structs for statistics and metrics
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessingStats {
    pub entries_processed: usize,
    pub bytes_processed: u64,
    pub duration: std::time::Duration,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessingMetrics {
    pub throughput: f64,
    pub memory_usage: u64,
    pub error_count: u64,
}

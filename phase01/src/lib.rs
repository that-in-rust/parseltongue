//! ZIP File Analysis and Storage Library
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Public API
//! - ZipAnalyzer      (main public interface)
//! - StorageManager   (storage interface)
//! - MetricsCollector (metrics interface)
//! 
//! Level 3: Feature Modules
//! - ZIP Processing   (streaming, validation)
//! - Storage Layer    (database, indexing)
//! - Metrics System   (collection, reporting)
//! 
//! Level 2: Core Infrastructure
//! - Async Runtime    (worker management)
//! - Error Handling   (error types)
//! - Resource Control (pooling, limits)
//! 
//! Level 1 (Base): Utility Layer
//! - Buffer Management (streaming buffers)
//! - Resource Cleanup (RAII guards)
//! - Type Definitions (core types)

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

// ===== Level 1: Core Types =====
// Design Choice: Using type aliases for common types

/// Byte buffer type
pub type Buffer = bytes::Bytes;

/// Path type
pub type Path = std::path::PathBuf;

// ===== Level 2: Core Traits =====
// Design Choice: Using async traits for main operations

/// ZIP analysis interface
#[async_trait::async_trait]
pub trait ZipAnalysis: Send + Sync + 'static {
    /// Analyzes ZIP file
    async fn analyze(&self, path: &Path) -> Result<AnalysisResult>;
    /// Gets analysis metrics
    async fn metrics(&self) -> Result<AnalysisMetrics>;
}

/// Analysis result type
#[derive(Debug, Clone, serde::Serialize)]
pub struct AnalysisResult {
    /// Total entries processed
    pub entries: usize,
    /// Total bytes processed
    pub bytes: u64,
    /// Processing duration
    pub duration: std::time::Duration,
}

/// Analysis metrics type
#[derive(Debug, Clone, serde::Serialize)]
pub struct AnalysisMetrics {
    /// Processing throughput
    pub throughput: f64,
    /// Memory usage
    pub memory_usage: u64,
    /// Error count
    pub errors: u64,
}

// ===== Level 3: Main Implementation =====
// Design Choice: Using builder pattern for configuration

/// ZIP analyzer implementation
pub struct ZipAnalyzer {
    /// ZIP processor
    processor: Arc<ZipProcessor>,
    /// Storage manager
    storage: Arc<StorageManager>,
    /// Metrics manager
    metrics: Arc<MetricsManager>,
}

impl ZipAnalyzer {
    /// Creates new ZIP analyzer
    pub async fn new(config: AnalyzerConfig) -> Result<Self> {
        // Initialize storage
        let storage = Arc::new(StorageManager::new(config.storage).await?);
        
        // Initialize processor
        let processor = Arc::new(ZipProcessor::new(
            config.zip,
            storage.clone(),
        ));
        
        // Initialize metrics
        let metrics = Arc::new(MetricsManager::new(config.metrics));

        Ok(Self {
            processor,
            storage,
            metrics,
        })
    }

    /// Processes ZIP file
    pub async fn process(&self, path: impl AsRef<Path>) -> Result<AnalysisResult> {
        // Start metrics collection
        self.metrics.start().await?;

        // Open and process file
        let file = tokio::fs::File::open(path.as_ref()).await?;
        let start = std::time::Instant::now();
        
        self.processor.process(file).await?;

        // Build result
        let result = AnalysisResult {
            entries: self.metrics.get_counter("entries_processed").await?,
            bytes: self.metrics.get_counter("bytes_processed").await?,
            duration: start.elapsed(),
        };

        // Stop metrics collection
        self.metrics.stop().await?;

        Ok(result)
    }
}

// ===== Level 4: Configuration =====
// Design Choice: Using separate configs for components

/// Analyzer configuration
#[derive(Debug, Clone)]
pub struct AnalyzerConfig {
    /// ZIP configuration
    pub zip: ZipConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            zip: ZipConfig::default(),
            storage: StorageConfig {
                path: "storage".into(),
                pool_size: 4,
                batch_size: 1000,
                index_config: Default::default(),
            },
            metrics: MetricsConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_analyzer() {
        let temp_dir = TempDir::new().unwrap();
        
        let config = AnalyzerConfig {
            storage: StorageConfig {
                path: temp_dir.path().to_path_buf(),
                pool_size: 4,
                batch_size: 100,
                index_config: Default::default(),
            },
            ..Default::default()
        };

        let analyzer = ZipAnalyzer::new(config).await.unwrap();
        
        // Create test ZIP
        let zip_path = temp_dir.path().join("test.zip");
        let mut zip = zip::ZipWriter::new(std::fs::File::create(&zip_path).unwrap());
        
        zip.start_file("test.txt", Default::default()).unwrap();
        zip.write_all(b"test data").unwrap();
        zip.finish().unwrap();

        // Process ZIP
        let result = analyzer.process(&zip_path).await.unwrap();
        
        assert_eq!(result.entries, 1);
        assert_eq!(result.bytes, 9); // "test data".len()
    }
}

//! ZIP Processing Layer
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): ZIP Processing
//! - ZipProcessor     (main processor)
//!   ├── Entry processing
//!   ├── Stream handling
//!   └── Resource management
//! 
//! Level 3: Processing Components
//! - AsyncReader      (async reading)
//! - EntryValidator   (validation)
//! - StreamProcessor  (streaming)
//! 
//! Level 2: Implementation
//! - Reader          (ZIP reading)
//! - Stream          (streaming)
//! - Guard           (RAII guards)
//! 
//! Level 1 (Base): Core Types
//! - ZipConfig       (configuration)
//! - ZipEntry        (entry type)
//! - ProcessingError (errors)

pub mod reader;
pub mod stream;
pub mod guard;

use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use metrics::{Counter, Gauge};
use crate::core::error::Result;

// Re-export main types
pub use reader::AsyncZipReader;

// Design Choice: Using async traits for operations
#[async_trait::async_trait]
pub trait AsyncZipProcessor: Send + Sync + 'static {
    async fn process_entry(&self, entry: ZipEntry) -> Result<()>;
    async fn complete(&self) -> Result<()>;
}

// Design Choice: Using builder pattern for configuration
#[derive(Debug, Clone)]
pub struct ZipConfig {
    pub buffer_size: usize,
    pub max_concurrent_entries: usize,
    pub validation_config: ValidationConfig,
    pub encoding_config: EncodingConfig,
}

impl Default for ZipConfig {
    fn default() -> Self {
        Self {
            buffer_size: 64 * 1024, // 64KB
            max_concurrent_entries: 4,
            validation_config: ValidationConfig::default(),
            encoding_config: EncodingConfig::default(),
        }
    }
}

// Design Choice: Using strong types for entries
#[derive(Debug, Clone)]
pub struct ZipEntry {
    pub path: std::path::PathBuf,
    pub data: Bytes,
    pub crc32: u32,
    pub size: u64,
}

// Design Choice: Using separate configs for validation
#[derive(Debug, Clone, Default)]
pub struct ValidationConfig {
    pub max_path_length: usize,
    pub max_file_size: u64,
    pub allowed_extensions: Vec<String>,
}

// Design Choice: Using separate configs for encoding
#[derive(Debug, Clone, Default)]
pub struct EncodingConfig {
    pub default_encoding: String,
    pub detect_encoding: bool,
    pub fallback_encoding: String,
}

// Design Choice: Using metrics for monitoring
#[derive(Debug)]
struct ProcessingMetrics {
    entries_processed: Counter,
    bytes_processed: Counter,
    active_processors: Gauge,
    error_count: Counter,
}

impl ProcessingMetrics {
    fn new() -> Self {
        Self {
            entries_processed: Counter::new(),
            bytes_processed: Counter::new(),
            active_processors: Gauge::new(),
            error_count: Counter::new(),
        }
    }
}

// Design Choice: Using processor for coordination
pub struct ZipProcessor {
    config: ZipConfig,
    backpressure: Arc<Semaphore>,
    metrics: ProcessingMetrics,
    entry_tx: mpsc::Sender<ZipEntry>,
}

impl ZipProcessor {
    pub fn new(config: ZipConfig) -> Self {
        let (entry_tx, _) = mpsc::channel(config.max_concurrent_entries);
        let backpressure = Arc::new(Semaphore::new(config.max_concurrent_entries));
        let metrics = ProcessingMetrics::new();

        Self {
            config,
            backpressure,
            metrics,
            entry_tx,
        }
    }

    pub async fn process<R>(&self, reader: R) -> Result<()>
    where
        R: AsyncRead + AsyncSeek + Unpin + Send + 'static,
    {
        let mut reader = AsyncZipReader::new(reader, self.config.clone());
        
        while let Some(entry) = reader.next_entry().await? {
            let _permit = self.backpressure.acquire().await?;
            self.metrics.active_processors.increment(1.0);
            
            self.process_entry(entry).await?;
            
            self.metrics.active_processors.decrement(1.0);
        }

        Ok(())
    }

    async fn process_entry(&self, entry: ZipEntry) -> Result<()> {
        self.metrics.entries_processed.increment(1);
        self.metrics.bytes_processed.increment(entry.size);
        
        self.entry_tx.send(entry).await
            .map_err(|_| Error::Shutdown)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_zip_processor() {
        let config = ZipConfig::default();
        let processor = ZipProcessor::new(config);

        // Create test ZIP
        let mut data = Vec::new();
        {
            let mut zip = zip::ZipWriter::new(Cursor::new(&mut data));
            zip.start_file("test.txt", Default::default()).unwrap();
            zip.write_all(b"test data").unwrap();
            zip.finish().unwrap();
        }

        let cursor = Cursor::new(data);
        assert!(processor.process(cursor).await.is_ok());
    }
}

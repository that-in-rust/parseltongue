//! ZIP Processing Coordination
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): ZIP Processing Orchestration
//! - ZipProcessor     (coordinates ZIP operations)
//! - ProcessingMetrics (aggregates ZIP metrics)
//! - StreamManager    (manages ZIP streams)
//! 
//! Level 3: Processing Management
//! - EntryProcessor   (processes ZIP entries)
//! - ValidationManager (manages validation)
//! - EncodingManager  (manages encodings)
//! 
//! Level 2: ZIP Traits
//! - AsyncZipReader   (async ZIP reading)
//! - EntryValidator   (entry validation)
//! - EncodingDetector (encoding detection)
//! 
//! Level 1 (Base): Core ZIP Types
//! - ZipConfig       (ZIP configuration)
//! - ZipEntry        (entry representation)
//! - ProcessingError (ZIP-specific errors)

use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use metrics::{Counter, Gauge};
use crate::core::{error::Result, types::*};

pub mod reader;
pub mod validation;
pub mod encoding;

// Re-export main types
pub use reader::AsyncZipReader;
pub use validation::{EntryValidator, ValidationConfig};
pub use encoding::{EncodingDetector, EncodingConfig};

// ===== Level 1: Core ZIP Types =====
// Design Choice: Using async streams for processing

/// Processing metrics collection
#[derive(Debug, Default)]
struct ProcessingMetrics {
    entries_processed: Counter,
    bytes_processed: Counter,
    active_processors: Gauge,
    error_count: Counter,
}

impl ProcessingMetrics {
    fn new() -> Self {
        Self::default()
    }
}

/// ZIP processing configuration
#[derive(Debug, Clone)]
pub struct ZipConfig {
    /// Buffer size for reading
    pub buffer_size: usize,
    /// Concurrent entry limit
    pub max_concurrent_entries: usize,
    /// Validation configuration
    pub validation_config: ValidationConfig,
    /// Encoding configuration
    pub encoding_config: EncodingConfig,
}

impl Default for ZipConfig {
    fn default() -> Self {
        Self {
            buffer_size: 8192,
            max_concurrent_entries: 4,
            validation_config: ValidationConfig::default(),
            encoding_config: EncodingConfig::default(),
        }
    }
}

/// ZIP entry representation
#[derive(Debug, Clone)]
pub struct ZipEntry {
    /// Entry path
    pub path: std::path::PathBuf,
    /// Entry data
    pub data: Bytes,
    /// Entry CRC32
    pub crc32: u32,
    /// Entry size
    pub size: u64,
}

// ===== Level 2: ZIP Traits =====
// Design Choice: Using async traits for operations

/// Async ZIP processing interface
#[async_trait::async_trait]
pub trait AsyncZipProcessor: Send + Sync + 'static {
    /// Process ZIP entry
    async fn process_entry(&self, entry: ZipEntry) -> Result<()>;
    /// Process completion
    async fn complete(&self) -> Result<()>;
}

// ===== Level 3: Processing Management =====
// Design Choice: Using channels for communication

/// ZIP processor implementation
pub struct ZipProcessor {
    /// Processing configuration
    config: ZipConfig,
    /// Entry processor
    processor: Arc<dyn AsyncZipProcessor>,
    /// Validation manager
    validator: Arc<EntryValidator>,
    /// Encoding detector
    encoding: Arc<EncodingDetector>,
    /// Processing metrics
    metrics: ProcessingMetrics,
    /// Entry channel
    entry_tx: mpsc::Sender<ZipEntry>,
}

impl ZipProcessor {
    /// Creates new ZIP processor
    pub fn new(
        config: ZipConfig,
        processor: Arc<dyn AsyncZipProcessor>,
    ) -> (Self, mpsc::Receiver<ZipEntry>) {
        let (entry_tx, entry_rx) = mpsc::channel(config.max_concurrent_entries);
        let validator = Arc::new(EntryValidator::new(config.validation_config.clone()));
        let encoding = Arc::new(EncodingDetector::new(config.encoding_config.clone()));
        let metrics = ProcessingMetrics::new();

        (Self {
            config,
            processor,
            validator,
            encoding,
            metrics,
            entry_tx,
        }, entry_rx)
    }

    // ===== Level 4: Processing Orchestration =====
    // Design Choice: Using worker tasks for processing

    /// Processes ZIP file
    pub async fn process<R>(&self, reader: R) -> Result<()>
    where
        R: AsyncRead + AsyncSeek + Unpin + Send + 'static,
    {
        let mut reader = AsyncZipReader::new(reader, self.config.clone());
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_entries));

        let mut tasks = Vec::new();
        
        while let Some(entry) = reader.next_entry().await? {
            let permit = semaphore.clone().acquire_owned().await?;
            let processor = self.processor.clone();
            let validator = self.validator.clone();
            let metrics = self.metrics.clone();
            
            // Spawn processing task
            let task = tokio::spawn(async move {
                metrics.active_processors.increment(1.0);
                
                let result = async {
                    // Validate entry
                    validator.validate(&entry).await?;
                    
                    // Process entry
                    processor.process_entry(entry).await?;
                    
                    Ok(())
                }.await;
                
                metrics.active_processors.decrement(1.0);
                drop(permit);
                
                result
            });
            
            tasks.push(task);
        }

        // Wait for all tasks
        for task in tasks {
            task.await??;
        }

        // Signal completion
        self.processor.complete().await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct TestProcessor;

    #[async_trait::async_trait]
    impl AsyncZipProcessor for TestProcessor {
        async fn process_entry(&self, entry: ZipEntry) -> Result<()> {
            println!("Processing: {}", entry.path.display());
            Ok(())
        }

        async fn complete(&self) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_zip_processor() {
        let config = ZipConfig::default();
        let processor = Arc::new(TestProcessor);
        
        let (zip_processor, _rx) = ZipProcessor::new(config.clone(), processor);

        // Create test ZIP
        let mut data = Vec::new();
        {
            let mut zip = zip::ZipWriter::new(Cursor::new(&mut data));
            zip.start_file("test.txt", Default::default()).unwrap();
            zip.write_all(b"Hello, World!").unwrap();
            zip.finish().unwrap();
        }

        let cursor = Cursor::new(data);
        assert!(zip_processor.process(cursor).await.is_ok());
    }
}

//! ZIP Processing Module - Pyramidal Structure
//! Layer 1: Public Interface
//! Layer 2: ZIP Management
//! Layer 3: Stream Processing
//! Layer 4: Error Handling
//! Layer 5: Resource Management

use std::path::Path;
use std::sync::Arc;
use anyhow::Result;
use bytes::Bytes;
use tokio::fs::File;
use tokio::sync::Semaphore;

mod reader;
mod stream;
mod guard;

pub use reader::ZipReader;
pub use stream::ZipStream;

// Layer 1: Core Types
#[derive(Clone)]
pub struct ZipProcessor {
    inner: Arc<ZipInner>,
}

struct ZipInner {
    config: ZipConfig,
    pool: Arc<Semaphore>,
}

// Layer 2: Configuration
#[derive(Debug, Clone)]
pub struct ZipConfig {
    pub buffer_size: usize,
    pub max_concurrent_entries: usize,
    pub chunk_size: usize,
}

impl Default for ZipConfig {
    fn default() -> Self {
        Self {
            buffer_size: 64 * 1024,      // 64KB
            max_concurrent_entries: 100,
            chunk_size: 1024 * 1024,     // 1MB
        }
    }
}

// Layer 3: Implementation
impl ZipProcessor {
    pub fn new(config: ZipConfig) -> Self {
        Self {
            inner: Arc::new(ZipInner {
                config,
                pool: Arc::new(Semaphore::new(config.max_concurrent_entries)),
            }),
        }
    }

    // Layer 4: Processing
    pub async fn process_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<ZipEntry>> {
        let file = File::open(path).await?;
        let reader = ZipReader::new(file, self.inner.config.clone());
        let mut entries = Vec::new();

        let mut stream = reader.stream_entries().await?;
        while let Some(entry) = stream.next_entry().await? {
            let _permit = self.inner.pool.acquire().await?;
            entries.push(entry);
        }

        Ok(entries)
    }

    // Layer 5: Entry Processing
    pub async fn process_entry(&self, entry: ZipEntry) -> Result<Bytes> {
        let _permit = self.inner.pool.acquire().await?;
        entry.read_content().await
    }
}

// Public Types
#[derive(Debug)]
pub struct ZipEntry {
    name: String,
    size: u64,
    compressed_size: u64,
    content: Arc<Bytes>,
}

impl ZipEntry {
    pub async fn read_content(&self) -> Result<Bytes> {
        Ok(self.content.as_ref().clone())
    }
}

// Error Types
#[derive(Debug, thiserror::Error)]
pub enum ZipError {
    #[error("Failed to open ZIP file: {0}")]
    OpenError(String),
    
    #[error("Failed to read ZIP entry: {0}")]
    ReadError(String),
    
    #[error("Invalid ZIP format: {0}")]
    FormatError(String),
}

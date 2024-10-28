//! ZIP File Reading Implementation
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Stream Processing
//! - StreamProcessor   (processes ZIP stream)
//! - StreamMetrics     (tracks stream metrics)
//! - BufferManager     (manages read buffers)
//! 
//! Level 3: Entry Processing
//! - EntryReader      (reads ZIP entries)
//! - EntryStream      (streams entry data)
//! - DataProcessor    (processes entry data)
//! 
//! Level 2: Reading Implementation
//! - AsyncReader      (async reading logic)
//! - ReadBuffer       (buffer management)
//! - ChunkProcessor   (chunk processing)
//! 
//! Level 1 (Base): Core Reading Types
//! - ReaderConfig     (reader configuration)
//! - ReadBuffer       (buffer types)
//! - ReadError        (reading errors)

use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
use bytes::{Bytes, BytesMut};
use futures::{Stream, StreamExt};
use zip::{ZipArchive, result::ZipError};
use crate::core::{error::{Error, Result}, types::*};
use super::{ZipEntry, ZipConfig};

// ===== Level 1: Core Reading Types =====
// Design Choice: Using BytesMut for efficient buffering

/// Reader metrics collection
#[derive(Debug, Default)]
struct ReaderMetrics {
    bytes_read: Counter,
    entries_processed: Counter,
    buffer_resizes: Counter,
}

impl ReaderMetrics {
    fn new() -> Self {
        Self::default()
    }
}

/// Async ZIP reader implementation
pub struct AsyncZipReader<R> {
    /// Inner reader
    reader: R,
    /// Reader configuration
    config: ZipConfig,
    /// Read buffer
    buffer: BytesMut,
    /// Current position
    position: u64,
    /// Reader metrics
    metrics: ReaderMetrics,
}

// ===== Level 2: Reading Implementation =====
// Design Choice: Using async/await for non-blocking I/O

impl<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> AsyncZipReader<R> {
    /// Creates new async ZIP reader
    pub fn new(reader: R, config: ZipConfig) -> Self {
        let buffer = BytesMut::with_capacity(config.buffer_size);
        let metrics = ReaderMetrics::new();

        Self {
            reader,
            config,
            buffer,
            position: 0,
            metrics,
        }
    }

    /// Reads next ZIP entry
    pub async fn next_entry(&mut self) -> Result<Option<ZipEntry>> {
        // Read ZIP header
        let header = self.read_entry_header().await?;
        
        if header.is_none() {
            return Ok(None);
        }
        let header = header.unwrap();

        // Read entry data
        let data = self.read_entry_data(&header).await?;
        
        // Create ZIP entry
        let entry = ZipEntry {
            path: header.name().into(),
            data: data.into(),
            crc32: header.crc32(),
            size: header.size(),
        };

        self.metrics.entries_processed.increment(1);
        Ok(Some(entry))
    }

    // ===== Level 3: Entry Processing =====
    // Design Choice: Using async helpers for modularity

    /// Reads entry header
    async fn read_entry_header(&mut self) -> Result<Option<zip::read::ZipFile>> {
        // Ensure buffer has enough space
        if self.buffer.len() < self.config.buffer_size {
            self.buffer.reserve(self.config.buffer_size);
            self.metrics.buffer_resizes.increment(1);
        }

        // Read header bytes
        let bytes_read = self.reader.read_buf(&mut self.buffer).await?;
        if bytes_read == 0 {
            return Ok(None);
        }

        self.metrics.bytes_read.increment(bytes_read as u64);
        self.position += bytes_read as u64;

        // Parse header
        let mut archive = ZipArchive::new(std::io::Cursor::new(&self.buffer))?;
        Ok(archive.by_index(0).ok())
    }

    /// Reads entry data
    async fn read_entry_data(&mut self, entry: &zip::read::ZipFile) -> Result<Bytes> {
        let mut data = Vec::with_capacity(entry.size() as usize);
        
        // Read compressed data
        let mut remaining = entry.compressed_size();
        while remaining > 0 {
            let bytes_read = self.reader.read_buf(&mut self.buffer).await?;
            if bytes_read == 0 {
                break;
            }

            data.extend_from_slice(&self.buffer[..bytes_read]);
            remaining = remaining.saturating_sub(bytes_read as u64);
            
            self.metrics.bytes_read.increment(bytes_read as u64);
            self.position += bytes_read as u64;
        }

        // Decompress if needed
        if entry.compression() != zip::CompressionMethod::Stored {
            // Use zip crate's decompression
            let decompressed = entry.decompress()?;
            Ok(Bytes::from(decompressed))
        } else {
            Ok(Bytes::from(data))
        }
    }
}

// ===== Level 4: Stream Processing =====
// Design Choice: Using Stream trait for composability

impl<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> Stream for AsyncZipReader<R> {
    type Item = Result<ZipEntry>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        use std::task::Poll;
        
        let future = self.next_entry();
        futures::pin_mut!(future);
        
        future.poll(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_reader_basic() {
        // Create test ZIP data
        let mut data = Vec::new();
        {
            let mut zip = zip::ZipWriter::new(Cursor::new(&mut data));
            zip.start_file("test.txt", Default::default()).unwrap();
            zip.write_all(b"Hello, World!").unwrap();
            zip.finish().unwrap();
        }

        let cursor = Cursor::new(data);
        let config = ZipConfig {
            buffer_size: 8192,
            max_concurrent_entries: 4,
            validation_config: ValidationConfig::default(),
            encoding_config: EncodingConfig::default(),
        };

        let mut reader = AsyncZipReader::new(cursor, config);
        
        // Test reading entry
        let entry = reader.next_entry().await.unwrap().unwrap();
        assert_eq!(entry.path.to_str().unwrap(), "test.txt");
        assert_eq!(&entry.data[..], b"Hello, World!");
    }
}

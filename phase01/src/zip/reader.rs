//! ZIP Reader Implementation - Pyramidal Structure
//! Layer 1: Reader Interface
//! Layer 2: Entry Reading
//! Layer 3: Content Processing
//! Layer 4: Buffering
//! Layer 5: Resource Management

use std::io::SeekFrom;
use anyhow::Result;
use bytes::Bytes;
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
use super::{ZipConfig, ZipEntry, ZipError};

// Layer 1: Core Types
pub struct ZipReader<R> 
where
    R: AsyncRead + AsyncSeek + Unpin,
{
    reader: R,
    config: ZipConfig,
    current_pos: u64,
}

// Layer 2: Implementation
impl<R> ZipReader<R> 
where
    R: AsyncRead + AsyncSeek + Unpin,
{
    pub fn new(reader: R, config: ZipConfig) -> Self {
        Self {
            reader,
            config,
            current_pos: 0,
        }
    }

    // Layer 3: Entry Reading
    pub async fn read_entry(&mut self) -> Result<Option<ZipEntry>> {
        let signature = self.read_u32_le().await?;
        if signature != 0x04034b50 {
            return Ok(None); // End of central directory
        }

        let _version = self.read_u16_le().await?;
        let _flags = self.read_u16_le().await?;
        let _method = self.read_u16_le().await?;
        let _mod_time = self.read_u16_le().await?;
        let _mod_date = self.read_u16_le().await?;
        let _crc = self.read_u32_le().await?;
        let compressed_size = self.read_u32_le().await? as u64;
        let size = self.read_u32_le().await? as u64;
        let name_length = self.read_u16_le().await? as usize;
        let extra_length = self.read_u16_le().await? as usize;

        // Layer 4: Name Reading
        let mut name_bytes = vec![0u8; name_length];
        self.reader.read_exact(&mut name_bytes).await?;
        let name = String::from_utf8(name_bytes)
            .map_err(|e| ZipError::FormatError(e.to_string()))?;

        // Skip extra field
        self.reader.seek(SeekFrom::Current(extra_length as i64)).await?;

        // Layer 5: Content Reading
        let mut content = vec![0u8; compressed_size as usize];
        self.reader.read_exact(&mut content).await?;

        Ok(Some(ZipEntry {
            name,
            size,
            compressed_size,
            content: std::sync::Arc::new(Bytes::from(content)),
        }))
    }

    // Helper Methods
    async fn read_u16_le(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).await?;
        Ok(u16::from_le_bytes(buf))
    }

    async fn read_u32_le(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).await?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl ZipReader<File> {
    pub async fn from_file<P: AsRef<std::path::Path>>(path: P, config: ZipConfig) -> Result<Self> {
        let file = File::open(path).await?;
        Ok(Self::new(file, config))
    }
}

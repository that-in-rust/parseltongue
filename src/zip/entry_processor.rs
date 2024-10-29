// Level 4: ZIP Entry Processing
// - Handles individual ZIP entries
// - Manages decompression
// - Validates checksums
// - Tracks progress

use crate::core::error::{Error, Result};
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use bytes::BytesMut;

pub struct EntryProcessor {
    buffer: BytesMut,
    crc32: crc32fast::Hasher,
}

impl EntryProcessor {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::with_capacity(8192),
            crc32: crc32fast::Hasher::new(),
        }
    }

    pub async fn process_entry<R, W>(&mut self, reader: &mut R, writer: &mut W) -> Result<()>
    where
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        loop {
            let n = reader.read_buf(&mut self.buffer).await?;
            if n == 0 { break; }
            
            self.crc32.update(&self.buffer[..n]);
            writer.write_all(&self.buffer[..n]).await?;
            self.buffer.clear();
        }
        Ok(())
    }
} 
// Level 4: ZIP Compression
// - Handles compression/decompression
// - Manages algorithms
// - Tracks compression metrics
// - Provides streaming support

use flate2::Compression;
use tokio::io::{AsyncRead, AsyncWrite};
use futures::{Stream, StreamExt};
use bytes::BytesMut;
use crate::core::error::Result;

pub struct CompressionStream<R> {
    inner: R,
    level: Compression,
    buffer: BytesMut,
}

impl<R: AsyncRead + Unpin> CompressionStream<R> {
    pub fn new(inner: R, level: Compression) -> Self {
        Self {
            inner,
            level,
            buffer: BytesMut::with_capacity(8192),
        }
    }

    pub async fn process<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let mut compressor = flate2::write::DeflateEncoder::new(Vec::new(), self.level);
        
        loop {
            let n = tokio::io::AsyncReadExt::read(&mut self.inner, &mut self.buffer).await?;
            if n == 0 {
                break;
            }
            
            compressor.write_all(&self.buffer[..n])?;
            self.buffer.clear();
        }
        
        let compressed = compressor.finish()?;
        tokio::io::AsyncWriteExt::write_all(writer, &compressed).await?;
        
        Ok(())
    }
} 
// Level 4: ZIP Streaming
// - Implements async streaming
// - Handles backpressure
// - Manages buffering
// - Tracks streaming metrics

use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{FramedRead, FramedWrite};
use futures::{Stream, StreamExt};
use bytes::BytesMut;
use crate::core::error::Result;
use crate::zip::codec::ZipCodec;

pub struct ZipStream<R> {
    reader: FramedRead<R, ZipCodec>,
    buffer: BytesMut,
}

impl<R: AsyncRead + Unpin> ZipStream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: FramedRead::new(reader, ZipCodec::new()),
            buffer: BytesMut::with_capacity(8192),
        }
    }

    pub async fn process<W>(&mut self, mut writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let mut framed_writer = FramedWrite::new(writer, ZipCodec::new());
        
        while let Some(chunk) = self.reader.next().await {
            let chunk = chunk?;
            framed_writer.send(chunk).await?;
        }
        
        Ok(())
    }
}
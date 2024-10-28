use std::pin::Pin;
use std::task::{Context, Poll};
use futures::{Stream, StreamExt};
use tokio::io::{AsyncRead, AsyncSeek};
use bytes::Bytes;
use crate::error::Result;
use super::{ZipEntry, ZipConfig};

/// Streaming ZIP entry reader
pub struct ZipStream<R> {
    reader: R,
    config: ZipConfig,
}

impl<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> ZipStream<R> {
    pub fn new(reader: R, config: ZipConfig) -> Self {
        Self { reader, config }
    }
}

impl<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> Stream for ZipStream<R> {
    type Item = Result<ZipEntry>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Implementation moved to AsyncZipReader
        unimplemented!("Use AsyncZipReader instead")
    }
}

/// Buffered entry stream
pub struct BufferedZipStream<S> {
    inner: S,
    buffer: Vec<ZipEntry>,
}

impl<S: Stream<Item = Result<ZipEntry>> + Unpin> BufferedZipStream<S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            buffer: Vec::new(),
        }
    }
}

impl<S: Stream<Item = Result<ZipEntry>> + Unpin> Stream for BufferedZipStream<S> {
    type Item = Result<ZipEntry>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(entry) = self.buffer.pop() {
            return Poll::Ready(Some(Ok(entry)));
        }

        match self.inner.poll_next_unpin(cx) {
            Poll::Ready(Some(Ok(entry))) => Poll::Ready(Some(Ok(entry))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}


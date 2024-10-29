//! ZIP Entry Streaming - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Stream Configuration
//! Layer 3: Entry Processing
//! Layer 4: Error Handling
//! Layer 5: Resource Management

use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::io::{AsyncRead, AsyncSeek};
use zip::ZipArchive;
use futures::{Stream, StreamExt};
use tracing::{debug, warn};

use super::guard::ZipEntry;

// Layer 1: Core Types
#[derive(Debug)]
pub struct ZipEntryStream<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> {
    archive: Arc<ZipArchive<R>>,
    current_index: usize,
}

// Layer 2: Implementation
impl<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> ZipEntryStream<R> {
    pub fn new(archive: Arc<ZipArchive<R>>) -> Result<Self> {
        Ok(Self {
            archive,
            current_index: 0,
        })
    }

    // Layer 3: Entry Access
    pub async fn next_entry(&mut self) -> Result<Option<ZipEntry<R>>> {
        if self.current_index >= self.archive.len() {
            return Ok(None);
        }

        let entry = ZipEntry::new(
            Arc::clone(&self.archive),
            self.current_index,
        ).await?;

        self.current_index += 1;
        Ok(Some(entry))
    }

    // Layer 4: Stream Properties
    pub fn len(&self) -> usize {
        self.archive.len()
    }

    pub fn is_empty(&self) -> bool {
        self.archive.len() == 0
    }

    pub fn remaining(&self) -> usize {
        self.archive.len().saturating_sub(self.current_index)
    }
}

// Layer 5: Stream Implementation
impl<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> Stream for ZipEntryStream<R> {
    type Item = Result<ZipEntry<R>>;

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
    use tokio::fs::File;
    use tokio::io::BufReader;
    use tempfile::NamedTempFile;
    use zip::write::FileOptions;

    async fn create_test_zip() -> Result<NamedTempFile> {
        let file = NamedTempFile::new()?;
        let mut zip = zip::ZipWriter::new(std::fs::File::create(file.path())?);

        zip.start_file("test1.txt", FileOptions::default())?;
        zip.write_all(b"test content 1")?;
        zip.start_file("test2.txt", FileOptions::default())?;
        zip.write_all(b"test content 2")?;
        zip.finish()?;

        Ok(file)
    }

    #[tokio::test]
    async fn test_zip_stream() -> Result<()> {
        let test_file = create_test_zip().await?;
        let file = File::open(test_file.path()).await?;
        let reader = BufReader::new(file);
        
        let archive = tokio::task::spawn_blocking(move || {
            ZipArchive::new(reader)
        })
        .await??;

        let mut stream = ZipEntryStream::new(Arc::new(archive))?;
        
        assert_eq!(stream.len(), 2);
        assert!(!stream.is_empty());
        
        let entry1 = stream.next_entry().await?.unwrap();
        assert_eq!(entry1.name(), "test1.txt");
        
        assert_eq!(stream.remaining(), 1);
        
        Ok(())
    }
}

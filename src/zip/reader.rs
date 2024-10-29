//! ZIP Reading - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: File Operations
//! Layer 3: ZIP Entry Reading
//! Layer 4: Error Handling
//! Layer 5: Resource Management

use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncSeek, BufReader};
use zip::ZipArchive;
use tracing::{debug, warn};
use bytes::Bytes;

use crate::error::ErrorExt;
use super::stream::ZipEntryStream;

// Layer 1: Core Types
#[derive(Debug)]
pub struct ZipReader {
    archive: Arc<ZipArchive<BufReader<File>>>,
    path: PathBuf,
    total_size: u64,
}

// Layer 2: Implementation
impl ZipReader {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        debug!("Opening ZIP file: {}", path.display());

        let file = File::open(&path)
            .await
            .with_context(|| format!("Failed to open ZIP file: {}", path.display()))?;

        let metadata = file.metadata()
            .await
            .context("Failed to read file metadata")?;

        let reader = BufReader::new(file);
        let archive = tokio::task::spawn_blocking(move || {
            ZipArchive::new(reader)
        })
        .await
        .context("ZIP reading task failed")?
        .context("Failed to parse ZIP archive")?;

        Ok(Self {
            archive: Arc::new(archive),
            path,
            total_size: metadata.len(),
        })
    }

    // Layer 3: Entry Access
    pub fn stream_entries(&self) -> Result<ZipEntryStream> {
        debug!("Creating entry stream for: {}", self.path.display());
        ZipEntryStream::new(Arc::clone(&self.archive))
    }

    pub fn entry_count(&self) -> usize {
        self.archive.len()
    }

    pub fn total_size(&self) -> u64 {
        self.total_size
    }

    // Layer 4: Validation
    pub fn validate(&self) -> Result<()> {
        debug!("Validating ZIP archive: {}", self.path.display());
        
        if self.archive.len() == 0 {
            warn!("ZIP archive is empty: {}", self.path.display());
            anyhow::bail!("ZIP archive is empty");
        }

        // Check for central directory
        if !self.has_central_directory() {
            warn!("ZIP archive has no central directory: {}", self.path.display());
            anyhow::bail!("ZIP archive is corrupted (no central directory)");
        }

        Ok(())
    }

    // Layer 5: Internal Helpers
    fn has_central_directory(&self) -> bool {
        // Basic validation - if we can read the archive, it has a central directory
        true
    }
}

impl Drop for ZipReader {
    fn drop(&mut self) {
        debug!("Closing ZIP reader: {}", self.path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;
    use zip::write::FileOptions;

    async fn create_test_zip() -> Result<NamedTempFile> {
        let file = NamedTempFile::new()?;
        let mut zip = zip::ZipWriter::new(std::fs::File::create(file.path())?);

        zip.start_file("test.txt", FileOptions::default())?;
        zip.write_all(b"test content")?;
        zip.finish()?;

        Ok(file)
    }

    #[tokio::test]
    async fn test_zip_reader() -> Result<()> {
        let test_file = create_test_zip().await?;
        let reader = ZipReader::new(test_file.path()).await?;
        
        assert_eq!(reader.entry_count(), 1);
        assert!(reader.validate().is_ok());
        
        let mut stream = reader.stream_entries()?;
        let entry = stream.next_entry().await?.unwrap();
        assert_eq!(entry.name(), "test.txt");
        
        Ok(())
    }
}

use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::io::{AsyncRead, AsyncSeek};
use tokio::sync::Semaphore;
use zip::ZipArchive;
use bytes::Bytes;
use tracing::{debug, warn};

//! ZIP Resource Guards - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Entry Management
//! Layer 3: Resource Control
//! Layer 4: Processing Logic
//! Layer 5: Cleanup & Safety

// Layer 1: Core Types
#[derive(Debug)]
pub struct ZipEntry<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> {
    archive: Arc<ZipArchive<R>>,
    index: usize,
    path: PathBuf,
    size: u64,
}

#[derive(Debug)]
pub struct ZipGuard {
    _permit: tokio::sync::OwnedSemaphorePermit,
    buffer_size: usize,
    entry: ZipEntry<tokio::io::BufReader<tokio::fs::File>>,
}

// Layer 2: Entry Implementation
impl<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> ZipEntry<R> {
    pub async fn new(archive: Arc<ZipArchive<R>>, index: usize) -> Result<Self> {
        let entry = archive.by_index(index)
            .context("Failed to get ZIP entry")?;

        let path = PathBuf::from(entry.name());
        let size = entry.size();

        Ok(Self {
            archive,
            index,
            path,
            size,
        })
    }

    // Layer 3: Entry Properties
    pub fn name(&self) -> &str {
        self.path.to_str().unwrap_or_default()
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

// Layer 4: Guard Implementation
impl ZipGuard {
    pub async fn new(
        entry: ZipEntry<tokio::io::BufReader<tokio::fs::File>>,
        semaphore: Arc<Semaphore>,
        buffer_size: usize,
    ) -> Result<Self> {
        let permit = semaphore.try_acquire_owned()
            .context("Failed to acquire semaphore permit")?;

        Ok(Self {
            _permit: permit,
            buffer_size,
            entry,
        })
    }

    pub async fn process(&self) -> Result<()> {
        debug!("Processing entry: {}", self.entry.name());
        
        // Validate path
        if !self.is_safe_path() {
            warn!("Unsafe path detected: {}", self.entry.name());
            anyhow::bail!("Unsafe ZIP entry path: {}", self.entry.name());
        }

        // Process entry with proper buffer size
        let content = self.read_content().await?;
        
        // TODO: Store content via storage layer
        debug!("Read {} bytes from {}", content.len(), self.entry.name());
        
        Ok(())
    }

    // Layer 5: Safety & Validation
    fn is_safe_path(&self) -> bool {
        let path = self.entry.path();
        
        // Check for path traversal attempts
        if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
            return false;
        }

        // Check for absolute paths
        if path.is_absolute() {
            return false;
        }

        true
    }

    async fn read_content(&self) -> Result<Bytes> {
        // TODO: Implement streaming read with proper buffer size
        Ok(Bytes::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::fs::File;
    use tokio::io::BufReader;

    async fn create_test_entry() -> Result<ZipEntry<BufReader<File>>> {
        let file = NamedTempFile::new()?;
        let mut zip = zip::ZipWriter::new(std::fs::File::create(file.path())?);
        
        zip.start_file("test.txt", Default::default())?;
        zip.write_all(b"test content")?;
        zip.finish()?;

        let file = File::open(file.path()).await?;
        let reader = BufReader::new(file);
        let archive = tokio::task::spawn_blocking(move || {
            ZipArchive::new(reader)
        })
        .await??;

        ZipEntry::new(Arc::new(archive), 0).await
    }

    #[tokio::test]
    async fn test_zip_guard() -> Result<()> {
        let entry = create_test_entry().await?;
        let semaphore = Arc::new(Semaphore::new(1));
        
        let guard = ZipGuard::new(entry, semaphore, 8192).await?;
        assert!(guard.is_safe_path());
        
        Ok(())
    }
}

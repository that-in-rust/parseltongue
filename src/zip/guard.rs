use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite};
use tokio::sync::OwnedSemaphorePermit;
use bytes::Bytes;
use tracing::{debug, warn};
use tokio::sync::broadcast;
use tokio::sync::RwLock;

use crate::internal::validation::PathValidator;

//! ZIP Resource Guards - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Entry Management
//! Layer 3: Resource Control
//! Layer 4: Processing Logic
//! Layer 5: Cleanup & Safety

// Layer 1: Core Types
#[derive(Debug)]
pub struct ZipEntry<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> {
    archive: Arc<zip::ZipArchive<R>>,
    index: usize,
    path: PathBuf,
    size: u64,
    buffer_size: usize,
}

#[derive(Debug)]
pub struct ZipGuard {
    shutdown_rx: broadcast::Receiver<()>,
    state: Arc<RwLock<GuardState>>,
}

#[derive(Debug, Default)]
struct GuardState {
    is_active: bool,
}

// Layer 2: Entry Implementation
impl<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> ZipEntry<R> {
    pub async fn new(
        archive: Arc<zip::ZipArchive<R>>,
        index: usize,
        buffer_size: usize,
    ) -> Result<Self> {
        let entry = archive.by_index(index)
            .context("Failed to get ZIP entry")?;

        let path = PathBuf::from(entry.name());
        let size = entry.size();

        // Validate path
        PathValidator::new()
            .is_safe_path(&path)
            .validate()?;

        Ok(Self {
            archive,
            index,
            path,
            size,
            buffer_size,
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

    // Layer 4: Content Access
    pub async fn read_content(&self) -> Result<Bytes> {
        let mut entry = self.archive.by_index(self.index)
            .context("Failed to reopen ZIP entry")?;

        let mut buffer = Vec::with_capacity(self.buffer_size);
        tokio::task::spawn_blocking(move || {
            std::io::copy(&mut entry, &mut buffer)
        })
        .await??;

        Ok(Bytes::from(buffer))
    }
}

// Layer 5: Guard Implementation
impl ZipGuard {
    pub fn new(shutdown_rx: broadcast::Receiver<()>) -> Self {
        Self {
            shutdown_rx,
            state: Arc::new(RwLock::new(GuardState::default())),
        }
    }

    // Layer 3: Guard Activation
    pub async fn activate(&self) -> Result<()> {
        let mut state = self.state.write().await;
        state.is_active = true;
        debug!("ZipGuard activated");
        Ok(())
    }

    // Layer 4: Shutdown Listening
    pub async fn listen_shutdown(&self) -> Result<()> {
        if let Ok(_) = self.shutdown_rx.recv().await {
            let mut state = self.state.write().await;
            state.is_active = false;
            warn!("ZipGuard received shutdown signal");
        }
        Ok(())
    }

    // Layer 5: Resource Management
    pub async fn is_active(&self) -> bool {
        let state = self.state.read().await;
        state.is_active
    }

    pub fn name(&self) -> &str {
        self.entry.name()
    }

    pub fn path(&self) -> &Path {
        self.entry.path()
    }

    pub fn size(&self) -> u64 {
        self.entry.size()
    }

    pub async fn process(&self) -> Result<()> {
        debug!("Processing entry: {}", self.name());
        
        // Read content with proper buffer size
        let content = self.entry.read_content().await?;
        
        // TODO: Store content via storage layer
        debug!("Read {} bytes from {}", content.len(), self.name());
        
        Ok(())
    }
}

impl Drop for ZipGuard {
    fn drop(&mut self) {
        debug!("ZipGuard dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::fs::File;
    use tokio::io::BufReader;
    use tokio::sync::Semaphore;

    async fn create_test_entry() -> Result<ZipEntry<BufReader<File>>> {
        let file = NamedTempFile::new()?;
        let mut zip = zip::ZipWriter::new(std::fs::File::create(file.path())?);
        
        zip.start_file("test.txt", Default::default())?;
        zip.write_all(b"test content")?;
        zip.finish()?;

        let file = File::open(file.path()).await?;
        let reader = BufReader::new(file);
        let archive = tokio::task::spawn_blocking(move || {
            zip::ZipArchive::new(reader)
        })
        .await??;

        ZipEntry::new(Arc::new(archive), 0, 8192).await
    }

    #[tokio::test]
    async fn test_zip_guard() -> Result<()> {
        let entry = create_test_entry().await?;
        let semaphore = Arc::new(Semaphore::new(1));
        let permit = semaphore.try_acquire_owned()?;
        
        let guard = ZipGuard::new(entry, permit, 8192).await?;
        assert_eq!(guard.name(), "test.txt");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_zip_guard_activation() -> Result<()> {
        let (tx, rx) = broadcast::channel(1);
        let guard = ZipGuard::new(rx);
        
        assert!(!guard.is_active().await);
        guard.activate().await?;
        assert!(guard.is_active().await);
        Ok(())
    }

    #[tokio::test]
    async fn test_zip_guard_shutdown() -> Result<()> {
        let (tx, rx) = broadcast::channel(1);
        let guard = ZipGuard::new(rx);
        guard.activate().await?;
        assert!(guard.is_active().await);
        
        tx.send(()).unwrap();
        guard.listen_shutdown().await?;
        assert!(!guard.is_active().await);
        Ok(())
    }
}

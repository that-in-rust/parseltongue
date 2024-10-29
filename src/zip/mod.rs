//! ZIP Processing Core - Pyramidal Structure
//! Layer 1: Core Types & Exports
//! Layer 2: ZIP Configuration
//! Layer 3: Processing Pipeline
//! Layer 4: Resource Management
//! Layer 5: Error Handling

pub mod reader;
pub mod stream;
pub mod guard;

use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::sync::Semaphore;
use tracing::{debug, error, info};

use crate::Config;
use reader::ZipReader;
use stream::ZipEntryStream;
use guard::ZipGuard;

// Layer 1: Core Types
#[derive(Debug)]
pub struct ZipProcessor {
    config: Arc<Config>,
    semaphore: Arc<Semaphore>,
    input_path: PathBuf,
    output_dir: PathBuf,
}

// Layer 2: Configuration
#[derive(Debug, Clone)]
pub struct ZipConfig {
    pub max_concurrent_entries: usize,
    pub buffer_size: usize,
}

// Layer 3: Implementation
impl ZipProcessor {
    pub fn new(config: Config) -> Result<Self> {
        let zip_config = ZipConfig {
            max_concurrent_entries: config.workers,
            buffer_size: config.buffer_size,
        };

        let semaphore = Arc::new(Semaphore::new(zip_config.max_concurrent_entries));

        Ok(Self {
            config: Arc::new(config.clone()),
            semaphore,
            input_path: config.input_zip,
            output_dir: config.output_dir,
        })
    }

    // Layer 4: Processing Pipeline
    pub async fn process(&self) -> Result<()> {
        info!("Processing ZIP file: {}", self.input_path.display());

        let reader = ZipReader::new(&self.input_path).await
            .context("Failed to create ZIP reader")?;

        let mut stream = reader.stream_entries()
            .context("Failed to create entry stream")?;

        while let Some(entry) = stream.next_entry().await? {
            let guard = ZipGuard::new(
                entry,
                Arc::clone(&self.semaphore),
                self.config.buffer_size,
            ).await?;

            // Process entry with backpressure
            self.process_entry(guard).await?;
        }

        info!("ZIP processing completed");
        Ok(())
    }

    // Layer 5: Entry Processing
    async fn process_entry(&self, guard: ZipGuard) -> Result<()> {
        let path = guard.path().to_string_lossy().into_owned();
        debug!("Processing entry: {}", path);

        // Actual processing will be implemented by storage layer
        // This is just the coordination logic
        if let Err(e) = guard.process().await {
            error!("Failed to process entry {}: {}", path, e);
            return Err(e);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_zip_processor() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config::builder()
            .input_zip("test.zip")
            .output_dir(temp_dir.path())
            .workers(2)
            .buffer_size(8192)
            .shutdown_timeout(Duration::from_secs(1))
            .build()
            .unwrap();

        let processor = ZipProcessor::new(config).unwrap();
        assert!(processor.process().await.is_err()); // Should fail as test.zip doesn't exist
    }
}

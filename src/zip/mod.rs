//! ZIP Processing Core - Pyramidal Structure
//! Layer 1: Core Types & Exports
//! Layer 2: ZIP Configuration
//! Layer 3: Processing Logic
//! Layer 4: Resource Management
//! Layer 5: Error Handling

pub mod reader;
pub mod stream;
pub mod guard;

use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::sync::Semaphore;
use tracing::{debug, info};

use crate::Config;
use crate::metrics::MetricsManager;
use reader::ZipReader;

// Layer 1: Core Types
#[derive(Debug)]
pub struct ZipProcessor {
    config: Config,
    reader: Arc<ZipReader>,
    semaphore: Arc<Semaphore>,
    #[cfg(feature = "metrics")]
    metrics: Arc<MetricsManager>,
}

// Layer 2: Implementation
impl ZipProcessor {
    pub fn new(config: Config) -> Result<Self> {
        let reader = ZipReader::new(&config.input_zip)
            .context("Failed to create ZIP reader")?;
        
        Ok(Self {
            reader: Arc::new(reader),
            semaphore: Arc::new(Semaphore::new(config.workers)),
            #[cfg(feature = "metrics")]
            metrics: Arc::new(MetricsManager::new()),
            config,
        })
    }

    // Layer 3: Processing
    pub async fn process(&self) -> Result<()> {
        info!("Starting ZIP processing");
        
        let mut stream = self.reader.stream_entries()?;
        
        while let Some(entry) = stream.next_entry().await? {
            let permit = self.semaphore.acquire().await?;
            let reader = Arc::clone(&self.reader);
            
            #[cfg(feature = "metrics")]
            let metrics = Arc::clone(&self.metrics);
            
            tokio::spawn(async move {
                let start = std::time::Instant::now();
                if let Err(e) = entry.process().await {
                    #[cfg(feature = "metrics")]
                    metrics.record_error(&e.to_string()).await?;
                }
                #[cfg(feature = "metrics")]
                metrics.record_file_processed(entry.size(), start.elapsed()).await?;
                drop(permit);
                Ok::<_, anyhow::Error>(())
            });
        }

        Ok(())
    }

    // Layer 4: Status
    pub fn total_size(&self) -> u64 {
        self.reader.total_size()
    }

    pub fn entry_count(&self) -> usize {
        self.reader.entry_count()
    }

    // Layer 5: Cleanup
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down ZIP processor");
        
        #[cfg(feature = "metrics")]
        self.metrics.shutdown().await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_zip_processor() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config = Config::builder()
            .input_zip("test.zip")
            .output_dir(temp_dir.path())
            .workers(2)
            .build()?;

        let processor = ZipProcessor::new(config)?;
        processor.process().await?;
        processor.shutdown().await?;
        
        Ok(())
    }
}

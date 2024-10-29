//! Main Application - Pyramidal Structure
//! Layer 1: Core Types & Exports
//! Layer 2: Application Configuration
//! Layer 3: Runtime Management
//! Layer 4: Error Handling
//! Layer 5: Resource Management

pub mod cli;

use anyhow::{Context, Result};
use tracing::{debug, info};

use crate::cli::{CliManager, Config};
use parseltongue::{MetricsManager, RuntimeManager, StorageManager, ZipProcessor};

// Layer 1: Core Types
#[derive(Debug)]
pub struct Application {
    config: Config,
    runtime: RuntimeManager,
    storage: StorageManager,
    metrics: MetricsManager,
}

// Layer 2: Implementation
impl Application {
    pub async fn new() -> Result<Self> {
        let cli = CliManager::new()
            .context("Failed to initialize CLI")?;
        let config = cli.config().clone();

        let runtime = RuntimeManager::new(&config)
            .context("Failed to initialize runtime")?;
        let storage = StorageManager::new(&config).await
            .context("Failed to initialize storage")?;
        let metrics = MetricsManager::new();

        Ok(Self {
            config,
            runtime,
            storage,
            metrics,
        })
    }

    // Layer 3: Application Logic
    pub async fn run(&self) -> Result<()> {
        info!("Starting application");
        
        let processor = ZipProcessor::new(self.config.clone())
            .context("Failed to create ZIP processor")?;

        // Process ZIP file
        processor.process().await
            .context("Failed to process ZIP file")?;

        Ok(())
    }

    // Layer 4: Error Handling
    pub async fn handle_error(&self, error: anyhow::Error) {
        self.metrics.record_error(&error.to_string()).await
            .unwrap_or_else(|e| debug!("Failed to record error: {}", e));
    }

    // Layer 5: Cleanup
    pub async fn shutdown(self) -> Result<()> {
        info!("Shutting down application");

        self.runtime.shutdown().await
            .context("Failed to shutdown runtime")?;
        self.storage.shutdown().await
            .context("Failed to shutdown storage")?;
        self.metrics.shutdown().await
            .context("Failed to shutdown metrics")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_application_lifecycle() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_zip = temp_dir.path().join("test.zip");
        std::fs::write(&test_zip, b"test data")?;

        let app = Application::new().await?;
        app.shutdown().await?;
        
        Ok(())
    }
}

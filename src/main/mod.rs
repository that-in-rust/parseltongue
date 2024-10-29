//! Main Application - Pyramidal Structure
//! Layer 1: Core Types & Exports
//! Layer 2: Application Configuration
//! Layer 3: Runtime Management
//! Layer 4: Error Handling
//! Layer 5: Resource Management

pub mod cli;

use anyhow::{Context, Result};
use tracing::{debug, info};

use crate::cli::{Cli, Config};
use crate::runtime::RuntimeManager;

// Layer 1: Core Types
#[derive(Debug)]
pub struct Application {
    config: Config,
    runtime: RuntimeManager,
}

// Layer 2: Implementation
impl Application {
    /// Initializes the application
    pub async fn new() -> Result<Self> {
        let cli = Cli::new()?;
        let config = cli.build_config()?;
        let runtime = RuntimeManager::new(&config).await?;
        Ok(Self { config, runtime })
    }

    /// Runs the main application logic
    pub async fn run(&self) -> Result<()> {
        info!("Running application with config: {:?}", self.config);
        // ... implement core processing logic ...
        Ok(())
    }

    /// Handles errors gracefully
    pub async fn handle_error(&self, error: crate::error::AppError) {
        tracing::error!("Error encountered: {:?}", error);
        // ... implement additional error handling ...
    }

    /// Shuts down the application gracefully
    pub async fn shutdown(&self) -> Result<()> {
        self.runtime.shutdown().await?;
        info!("Application shutdown complete");
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

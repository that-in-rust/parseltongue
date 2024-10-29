//! Main Module - Pyramidal Structure
//! Layer 1: Module Organization & Exports
//!   - Public interface and module structure
//! Layer 2: Core Types & Traits
//!   - Essential types and trait definitions
//! Layer 3: Runtime Management
//!   - Tokio runtime configuration
//! Layer 4: Error Handling
//!   - Error propagation and context
//! Layer 5: Resource Management
//!   - Cleanup and shutdown coordination

// Layer 1: Module Organization
pub mod cli;
use cli::{Args, Config};

use anyhow::{Context, Result};
use tokio::runtime::Runtime;
use tracing::{error, info};

// Layer 2: Core Runtime Management
pub struct MainRunner {
    config: Config,
    runtime: Runtime,
}

// Layer 3: Implementation
impl MainRunner {
    pub fn new(config: Config) -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(config.workers)
            .enable_all()
            .build()
            .context("Failed to create Tokio runtime")?;

        Ok(Self { config, runtime })
    }

    pub fn run(self) -> Result<()> {
        self.runtime.block_on(async {
            self.run_async().await
        })
    }

    // Layer 4: Async Processing
    async fn run_async(&self) -> Result<()> {
        info!("Starting processing with {} workers", self.config.workers);

        let result = tokio::select! {
            r = self.process() => r,
            _ = tokio::signal::ctrl_c() => {
                info!("Received shutdown signal");
                Ok(())
            }
        };

        // Layer 5: Cleanup
        self.cleanup().await?;
        result
    }

    async fn process(&self) -> Result<()> {
        // TODO: Implement core processing logic
        Ok(())
    }

    async fn cleanup(&self) -> Result<()> {
        info!("Cleaning up resources...");
        // TODO: Implement resource cleanup
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runner_creation() {
        let config = Config::builder()
            .workers(2)
            .build()
            .unwrap();
        
        let runner = MainRunner::new(config);
        assert!(runner.is_ok());
    }
}

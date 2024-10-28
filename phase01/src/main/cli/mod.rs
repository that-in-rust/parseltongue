//! CLI Layer Coordination
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): CLI Orchestration
//! - CliManager       (coordinates CLI)
//!   ├── Argument handling
//!   ├── Progress tracking
//!   └── Metrics display
//! 
//! Level 3: Feature Management
//! - ArgumentManager  (manages args)
//! - ProgressManager  (manages progress)
//! - MetricsManager   (manages metrics)
//! 
//! Level 2: CLI Implementation
//! - ArgParser        (parses args)
//! - ProgressBar     (shows progress)
//! - MetricsDisplay  (shows metrics)
//! 
//! Level 1 (Base): Core CLI Types
//! - Args            (argument types)
//! - Config          (config types)
//! - Display         (display types)

pub mod args;
pub mod config;

use std::sync::Arc;
use tokio::sync::RwLock;
use indicatif::{ProgressBar, ProgressStyle};
use crate::core::error::Result;

// Re-export main types
pub use args::Args;
pub use config::CliConfig;

// Design Choice: Using builder pattern for CLI setup
pub struct CliManager {
    config: CliConfig,
    args: Arc<ArgumentManager>,
    progress: Arc<ProgressManager>,
    metrics: Arc<MetricsManager>,
}

impl CliManager {
    pub fn new(config: CliConfig) -> Self {
        let args = Arc::new(ArgumentManager::new());
        let progress = Arc::new(ProgressManager::new(config.progress_style.clone()));
        let metrics = Arc::new(MetricsManager::new(config.metrics_format.clone()));

        Self {
            config,
            args,
            progress,
            metrics,
        }
    }

    pub async fn execute<C: CliCommand>(&self, command: C) -> Result<()> {
        // Start metrics collection
        self.metrics.start().await?;

        // Execute command with progress
        let progress = self.progress.create_progress_bar();
        command.execute().await?;
        progress.finish().await?;

        // Stop metrics collection
        self.metrics.stop().await?;

        Ok(())
    }
}

// Design Choice: Using async traits for operations
#[async_trait::async_trait]
pub trait CliCommand: Send + Sync {
    async fn execute(&self) -> Result<()>;
    async fn display_progress(&self) -> Result<()>;
    async fn show_metrics(&self) -> Result<()>;
}

// Design Choice: Using separate managers for features
struct ArgumentManager {
    args: RwLock<Option<Args>>,
}

impl ArgumentManager {
    fn new() -> Self {
        Self {
            args: RwLock::new(None),
        }
    }

    async fn parse_args(&self) -> Result<Args> {
        use clap::Parser;
        Ok(Args::parse())
    }
}

// Design Choice: Using indicatif for progress
struct ProgressManager {
    style: ProgressStyle,
    bars: RwLock<Vec<Arc<ProgressBar>>>,
}

impl ProgressManager {
    fn new(style: ProgressStyle) -> Self {
        Self {
            style,
            bars: RwLock::new(Vec::new()),
        }
    }

    async fn create_progress_bar(&self) -> Arc<ProgressBar> {
        let bar = Arc::new(ProgressBar::new_with_style(self.style.clone()));
        self.bars.write().await.push(bar.clone());
        bar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_manager() {
        let config = CliConfig {
            verbose: true,
            progress_style: ProgressStyle::default(),
            metrics_format: MetricsFormat::default(),
        };

        let manager = CliManager::new(config);
        
        struct TestCommand;
        
        #[async_trait::async_trait]
        impl CliCommand for TestCommand {
            async fn execute(&self) -> Result<()> {
                Ok(())
            }
            
            async fn display_progress(&self) -> Result<()> {
                Ok(())
            }
            
            async fn show_metrics(&self) -> Result<()> {
                Ok(())
            }
        }

        assert!(manager.execute(TestCommand).await.is_ok());
    }
}

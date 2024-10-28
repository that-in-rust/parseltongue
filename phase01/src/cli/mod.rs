//! CLI Layer Coordination
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): CLI Orchestration
//! - CliManager       (coordinates all CLI components)
//! - CliMetrics       (aggregates CLI performance)
//! - UserInterface    (manages user interaction)
//! 
//! Level 3: Feature Management
//! - ArgumentManager  (manages CLI arguments)
//! - ProgressManager  (manages progress display)
//! - MetricsManager   (manages metrics display)
//! 
//! Level 2: CLI Traits
//! - CliCommand       (command interface)
//! - ProgressDisplay  (progress interface)
//! - MetricsDisplay   (metrics interface)
//! 
//! Level 1 (Base): Core CLI Types
//! - CliConfig       (CLI configuration)
//! - CliError        (CLI-specific errors)
//! - DisplayConfig   (display settings)

pub mod args;
pub mod progress;
pub mod metrics;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::core::error::Result;

// Re-export main types
pub use args::Args;
pub use progress::ProgressBar;
pub use metrics::MetricsDisplay;

// ===== Level 1: Core CLI Types =====
// Design Choice: Using builder pattern for configuration

/// CLI configuration
#[derive(Debug, Clone)]
pub struct CliConfig {
    /// Enable verbose output
    pub verbose: bool,
    /// Progress bar style
    pub progress_style: ProgressStyle,
    /// Metrics display format
    pub metrics_format: MetricsFormat,
}

/// CLI error types
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Display error: {0}")]
    DisplayError(String),
    #[error("Progress error: {0}")]
    ProgressError(String),
}

// ===== Level 2: CLI Traits =====
// Design Choice: Using async traits for operations

/// CLI command interface
#[async_trait::async_trait]
pub trait CliCommand {
    /// Execute command
    async fn execute(&self) -> Result<()>;
    /// Display progress
    async fn display_progress(&self) -> Result<()>;
    /// Show metrics
    async fn show_metrics(&self) -> Result<()>;
}

/// Progress display interface
#[async_trait::async_trait]
pub trait ProgressDisplay: Send + Sync {
    /// Update progress
    async fn update(&self, current: u64, total: u64) -> Result<()>;
    /// Set message
    async fn set_message(&self, msg: &str) -> Result<()>;
    /// Finish progress
    async fn finish(&self) -> Result<()>;
}

// ===== Level 3: Feature Management =====
// Design Choice: Using separate managers for features

/// CLI manager implementation
pub struct CliManager {
    /// CLI configuration
    config: CliConfig,
    /// Argument manager
    args: Arc<ArgumentManager>,
    /// Progress manager
    progress: Arc<ProgressManager>,
    /// Metrics manager
    metrics: Arc<MetricsManager>,
}

impl CliManager {
    /// Creates new CLI manager
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

    /// Executes CLI command
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

// ===== Level 4: CLI Orchestration =====
// Design Choice: Using separate components for modularity

/// Argument manager implementation
struct ArgumentManager {
    /// Parsed arguments
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

/// Progress manager implementation
struct ProgressManager {
    /// Progress style
    style: ProgressStyle,
    /// Active progress bars
    progress_bars: RwLock<Vec<Arc<dyn ProgressDisplay>>>,
}

impl ProgressManager {
    fn new(style: ProgressStyle) -> Self {
        Self {
            style,
            progress_bars: RwLock::new(Vec::new()),
        }
    }

    async fn create_progress_bar(&self) -> Arc<dyn ProgressDisplay> {
        let bar = Arc::new(ProgressBar::new_with_style(self.style.clone()));
        self.progress_bars.write().await.push(bar.clone());
        bar
    }
}

/// Metrics manager implementation
struct MetricsManager {
    /// Metrics format
    format: MetricsFormat,
    /// Metrics display
    display: Arc<MetricsDisplay>,
}

impl MetricsManager {
    fn new(format: MetricsFormat) -> Self {
        Self {
            format,
            display: Arc::new(MetricsDisplay::new(format)),
        }
    }

    async fn start(&self) -> Result<()> {
        self.display.start().await
    }

    async fn stop(&self) -> Result<()> {
        self.display.stop().await
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

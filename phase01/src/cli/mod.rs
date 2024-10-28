//! CLI Layer Coordination
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): CLI Orchestration
//! - CliManager       (coordinates CLI components)
//! - CliMetrics       (aggregates CLI metrics)
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

// Re-export main types
pub use args::Args;
pub use progress::ProgressBar;
pub use metrics::MetricsDisplay;

use crate::core::error::Result;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_config() {
        let config = CliConfig {
            verbose: true,
            progress_style: ProgressStyle::default(),
            metrics_format: MetricsFormat::default(),
        };

        assert!(config.verbose);
    }
}

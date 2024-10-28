//! CLI Layer Coordination
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): CLI Orchestration
//! - CLI manager coordination
//! - Progress tracking
//! - User interaction
//! 
//! Level 3: Feature Management
//! - Argument handling
//! - Progress display
//! - Configuration
//! 
//! Level 2: Component Integration
//! - Progress bars
//! - Error display
//! - Help messages
//! 
//! Level 1 (Base): Core Types
//! - Argument types
//! - Display types
//! - Config types

pub mod args;
pub mod config;
pub mod progress;

use std::sync::Arc;
use tokio::sync::RwLock;
use indicatif::{ProgressBar as InternalProgressBar, ProgressStyle};
use crate::core::error::Result;

// Design Choice: Re-export main types
pub use args::Args;
pub use config::CliConfig;
pub use progress::ProgressBar;

// Design Choice: Using builder pattern for CLI setup
pub struct CliManager {
    config: CliConfig,
    progress: Arc<ProgressManager>,
}

impl CliManager {
    pub fn new(config: CliConfig) -> Self {
        Self {
            config,
            progress: Arc::new(ProgressManager::new()),
        }
    }

    pub async fn execute<F, Fut>(&self, f: F) -> Result<()>
    where
        F: FnOnce(ProgressBar) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let progress = self.progress.create_progress_bar();
        f(progress).await
    }
}

// Design Choice: Using separate manager for progress
struct ProgressManager {
    bars: RwLock<Vec<Arc<InternalProgressBar>>>,
}

impl ProgressManager {
    fn new() -> Self {
        Self {
            bars: RwLock::new(Vec::new()),
        }
    }

    fn create_progress_bar(&self) -> ProgressBar {
        ProgressBar::new()
    }
}

// Level 4: Progress Tracking
// - Manages progress bar updates
// - Handles multi-threaded progress
// - Implements custom styles
// - Provides user feedback

use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::sync::RwLock;

// Level 3: Progress Manager
pub struct ProgressManager {
    bar: Arc<RwLock<ProgressBar>>,
    total_files: u64,
}

impl ProgressManager {
    // Level 2: Display Configuration
    pub fn new(total: u64) -> Self {
        let bar = ProgressBar::new(total)
            .with_style(ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap());
                
        Self {
            bar: Arc::new(RwLock::new(bar)),
            total_files: total,
        }
    }

    // Level 1: Progress Updates
    pub async fn increment(&self) {
        self.bar.write().await.inc(1);
    }

    pub async fn finish(&self) {
        self.bar.write().await.finish_with_message("Complete");
    }
} 
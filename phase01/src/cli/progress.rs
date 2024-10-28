//! Progress Display Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Progress Coordination
//! - ProgressManager   (manages progress display)
//! - ProgressMetrics   (tracks progress stats)
//! - DisplayManager    (manages output)
//! 
//! Level 3: Progress Types
//! - ProgressBar      (progress bar display)
//! - SpinnerDisplay   (activity spinner)
//! - StatusLine       (status messages)
//! 
//! Level 2: Display Implementation
//! - ProgressRenderer  (renders progress)
//! - StyleManager     (manages styles)
//! - UpdateManager    (manages updates)
//! 
//! Level 1 (Base): Core Progress Types
//! - ProgressConfig   (progress configuration)
//! - ProgressStyle    (style configuration)
//! - DisplayError     (display errors)

use std::sync::Arc;
use tokio::sync::Mutex;
use indicatif::{ProgressBar, ProgressStyle};
use crate::core::error::Result;

// ===== Level 1: Core Progress Types =====
// Design Choice: Using indicatif for progress display

/// Progress display configuration
#[derive(Debug, Clone)]
pub struct ProgressConfig {
    /// Enable progress bar
    pub show_progress: bool,
    /// Update interval
    pub update_interval: std::time::Duration,
    /// Progress style
    pub style: ProgressStyle,
}

// ===== Level 2: Display Implementation =====
// Design Choice: Using async updates

/// Progress display implementation
pub struct ProgressDisplay {
    /// Progress bar
    progress_bar: Arc<ProgressBar>,
    /// Display configuration
    config: ProgressConfig,
    /// Current position
    position: Arc<Mutex<u64>>,
}

impl ProgressDisplay {
    /// Creates new progress display
    pub fn new(total: u64, config: ProgressConfig) -> Self {
        let progress_bar = ProgressBar::new(total)
            .with_style(config.style.clone());

        Self {
            progress_bar: Arc::new(progress_bar),
            config,
            position: Arc::new(Mutex::new(0)),
        }
    }

    /// Updates progress
    pub async fn update(&self, delta: u64) -> Result<()> {
        if !self.config.show_progress {
            return Ok(());
        }

        let mut position = self.position.lock().await;
        *position += delta;
        self.progress_bar.set_position(*position);
        
        Ok(())
    }

    /// Sets progress message
    pub fn set_message(&self, msg: &str) {
        if self.config.show_progress {
            self.progress_bar.set_message(msg.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_progress_display() {
        let config = ProgressConfig {
            show_progress: true,
            update_interval: std::time::Duration::from_millis(100),
            style: ProgressStyle::default_bar(),
        };

        let progress = ProgressDisplay::new(100, config);
        
        progress.update(50).await.unwrap();
        
        let position = *progress.position.lock().await;
        assert_eq!(position, 50);
    }
}


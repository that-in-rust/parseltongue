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
use tokio::sync::RwLock;
use indicatif::{ProgressBar as IndicatifBar, ProgressStyle};
use metrics::{Counter, Gauge, Histogram};
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

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            show_progress: true,
            update_interval: std::time::Duration::from_millis(100),
            style: ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        }
    }
}

// ===== Level 2: Display Implementation =====
// Design Choice: Using async updates

/// Progress bar implementation
pub struct ProgressBar {
    /// Inner progress bar
    bar: Arc<IndicatifBar>,
    /// Progress configuration
    config: ProgressConfig,
    /// Progress metrics
    metrics: ProgressMetrics,
    /// Current state
    state: Arc<RwLock<ProgressState>>,
}

impl ProgressBar {
    /// Creates new progress bar
    pub fn new() -> Self {
        Self::new_with_style(ProgressStyle::default_bar())
    }

    /// Creates new progress bar with style
    pub fn new_with_style(style: ProgressStyle) -> Self {
        let config = ProgressConfig {
            style,
            ..Default::default()
        };

        let bar = IndicatifBar::new(100);
        bar.set_style(config.style.clone());

        Self {
            bar: Arc::new(bar),
            config,
            metrics: ProgressMetrics::new(),
            state: Arc::new(RwLock::new(ProgressState::new())),
        }
    }

    /// Sets progress length
    pub fn set_length(&self, len: u64) {
        self.bar.set_length(len);
    }

    /// Sets progress position
    pub async fn set_position(&self, pos: u64) -> Result<()> {
        let mut state = self.state.write().await;
        state.position = pos;
        
        if self.config.show_progress {
            self.bar.set_position(pos);
            self.metrics.updates.increment(1);
        }
        
        Ok(())
    }

    /// Sets progress message
    pub fn set_message<S: Into<String>>(&self, msg: S) {
        if self.config.show_progress {
            self.bar.set_message(msg);
        }
    }

    /// Finishes progress bar
    pub fn finish_with_message<S: Into<String>>(&self, msg: S) {
        if self.config.show_progress {
            self.bar.finish_with_message(msg);
        }
    }
}

// ===== Level 3: Progress Types =====
// Design Choice: Using separate types for different displays

/// Progress state tracking
#[derive(Debug)]
struct ProgressState {
    position: u64,
    total: u64,
    message: String,
}

impl ProgressState {
    fn new() -> Self {
        Self {
            position: 0,
            total: 100,
            message: String::new(),
        }
    }
}

/// Progress metrics collection
#[derive(Debug)]
struct ProgressMetrics {
    updates: Counter,
    refresh_rate: Histogram,
    active_bars: Gauge,
}

impl ProgressMetrics {
    fn new() -> Self {
        Self {
            updates: Counter::new(),
            refresh_rate: Histogram::new(),
            active_bars: Gauge::new(),
        }
    }
}

// ===== Level 4: Progress Coordination =====
// Design Choice: Using multi-progress for multiple bars

/// Progress manager implementation
pub struct ProgressManager {
    /// Active progress bars
    bars: Arc<RwLock<Vec<Arc<ProgressBar>>>>,
    /// Manager configuration
    config: ProgressConfig,
    /// Manager metrics
    metrics: ProgressMetrics,
}

impl ProgressManager {
    /// Creates new progress manager
    pub fn new(config: ProgressConfig) -> Self {
        Self {
            bars: Arc::new(RwLock::new(Vec::new())),
            config,
            metrics: ProgressMetrics::new(),
        }
    }

    /// Creates new progress bar
    pub async fn create_bar(&self) -> Arc<ProgressBar> {
        let bar = Arc::new(ProgressBar::new_with_style(self.config.style.clone()));
        self.bars.write().await.push(bar.clone());
        self.metrics.active_bars.increment(1.0);
        bar
    }

    /// Updates all progress bars
    pub async fn update_all(&self) -> Result<()> {
        let bars = self.bars.read().await;
        for bar in bars.iter() {
            let state = bar.state.read().await;
            bar.set_position(state.position).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_progress_bar() {
        let bar = ProgressBar::new();
        
        bar.set_length(100);
        assert!(bar.set_position(50).await.is_ok());
        
        bar.set_message("Testing...");
        bar.finish_with_message("Done!");
    }

    #[tokio::test]
    async fn test_progress_manager() {
        let config = ProgressConfig::default();
        let manager = ProgressManager::new(config);
        
        let bar = manager.create_bar().await;
        assert!(bar.set_position(50).await.is_ok());
        
        assert!(manager.update_all().await.is_ok());
    }
}

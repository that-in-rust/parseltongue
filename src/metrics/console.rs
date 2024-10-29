//! Console Metrics - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Metrics Configuration
//! Layer 3: Metrics Collection
//! Layer 4: Console Output
//! Layer 5: Resource Management

use std::sync::Arc;
use anyhow::Result;
use tokio::sync::RwLock;
use tracing::{debug, info};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

// Layer 1: Core Types
#[derive(Debug)]
pub struct ConsoleMetrics {
    state: Arc<RwLock<ConsoleState>>,
    progress: MultiProgress,
    file_bar: ProgressBar,
    bytes_bar: ProgressBar,
}

#[derive(Debug, Default)]
struct ConsoleState {
    total_files: usize,
    total_bytes: u64,
    errors: usize,
}

// Layer 2: Implementation
impl ConsoleMetrics {
    pub fn new() -> Self {
        let progress = MultiProgress::new();
        let file_bar = progress.add(ProgressBar::new(0));
        let bytes_bar = progress.add(ProgressBar::new(0));

        file_bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        bytes_bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        Self {
            state: Arc::new(RwLock::new(ConsoleState::default())),
            progress,
            file_bar,
            bytes_bar,
        }
    }

    // Layer 3: Metrics Recording
    pub async fn record_file_processed(&self, size: u64) -> Result<()> {
        let mut state = self.state.write().await;
        state.total_files += 1;
        state.total_bytes += size;

        self.file_bar.inc(1);
        self.bytes_bar.inc(size);

        Ok(())
    }

    pub async fn record_error(&self) -> Result<()> {
        let mut state = self.state.write().await;
        state.errors += 1;
        Ok(())
    }

    // Layer 4: Progress Updates
    pub fn set_file_target(&self, count: usize) {
        self.file_bar.set_length(count as u64);
    }

    pub fn set_byte_target(&self, size: u64) {
        self.bytes_bar.set_length(size);
    }

    // Layer 5: Resource Management
    pub async fn shutdown(&self) -> Result<()> {
        debug!("Shutting down console metrics");
        
        let state = self.state.read().await;
        info!(
            "Processed {} files ({} bytes) with {} errors",
            state.total_files, state.total_bytes, state.errors
        );

        self.file_bar.finish_with_message("Done!");
        self.bytes_bar.finish_with_message("Done!");
        
        Ok(())
    }
}

impl Drop for ConsoleMetrics {
    fn drop(&mut self) {
        self.file_bar.finish_and_clear();
        self.bytes_bar.finish_and_clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    use std::time::Duration;

    #[tokio::test]
    async fn test_console_metrics() -> Result<()> {
        let metrics = ConsoleMetrics::new();
        
        metrics.set_file_target(2);
        metrics.set_byte_target(200);
        
        metrics.record_file_processed(100).await?;
        sleep(Duration::from_millis(100)).await;
        metrics.record_file_processed(100).await?;
        
        metrics.shutdown().await?;
        Ok(())
    }
}

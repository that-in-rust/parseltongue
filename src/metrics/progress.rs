// Level 4: Progress Tracking
// - Manages progress updates
// - Handles rate limiting
// - Provides metrics collection
// - Coordinates UI updates

use tokio::sync::mpsc;
use std::sync::Arc;
use metrics::{counter, gauge};
use crate::core::error::Result;

pub struct ProgressTracker {
    sender: mpsc::Sender<ProgressUpdate>,
    total_bytes: u64,
    processed_bytes: u64,
}

#[derive(Debug)]
pub enum ProgressUpdate {
    BytesProcessed(u64),
    EntryCompleted(String),
    Error(String),
}

impl ProgressTracker {
    pub fn new(total_bytes: u64) -> (Self, mpsc::Receiver<ProgressUpdate>) {
        let (sender, receiver) = mpsc::channel(100);
        gauge!("progress.total_bytes").set(total_bytes as f64);
        
        (Self {
            sender,
            total_bytes,
            processed_bytes: 0,
        }, receiver)
    }

    pub async fn update(&mut self, bytes: u64) -> Result<()> {
        self.processed_bytes += bytes;
        gauge!("progress.processed_bytes").set(self.processed_bytes as f64);
        self.sender.send(ProgressUpdate::BytesProcessed(bytes)).await?;
        Ok(())
    }
} 
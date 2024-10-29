// Level 4: Cleanup Management
// - Handles resource cleanup
// - Manages temporary files
// - Coordinates shutdown
// - Tracks cleanup metrics

use tokio::sync::oneshot;
use std::path::PathBuf;
use metrics::{counter, gauge};
use crate::core::error::Result;

pub struct CleanupManager {
    temp_files: Vec<PathBuf>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl CleanupManager {
    pub fn new() -> Self {
        Self {
            temp_files: Vec::new(),
            shutdown_tx: None,
        }
    }

    pub fn register_temp_file(&mut self, path: PathBuf) {
        self.temp_files.push(path);
        gauge!("cleanup.temp_files").set(self.temp_files.len() as f64);
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        for path in self.temp_files.drain(..) {
            if let Err(e) = tokio::fs::remove_file(&path).await {
                counter!("cleanup.errors").increment(1);
                log::warn!("Failed to remove temp file {}: {}", path.display(), e);
            } else {
                counter!("cleanup.files_removed").increment(1);
            }
        }
        
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        
        Ok(())
    }
}

impl Drop for CleanupManager {
    fn drop(&mut self) {
        for path in &self.temp_files {
            if let Err(e) = std::fs::remove_file(path) {
                log::error!("Failed to remove temp file in cleanup: {}", e);
            }
        }
    }
} 
} 
// Level 4: Resource Cleanup
// - Manages resource lifecycle
// - Implements RAII patterns
// - Handles cleanup errors
// - Provides cleanup metrics

use std::future::Future;
use std::pin::Pin;
use tokio::sync::oneshot;
use crate::core::error::Result;

// Level 3: Cleanup Manager
pub struct CleanupManager {
    handlers: Vec<Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send>>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl CleanupManager {
    // Level 2: Handler Management
    pub fn new() -> Self {
        let (shutdown_tx, _) = oneshot::channel();
        Self {
            handlers: Vec::new(),
            shutdown_tx: Some(shutdown_tx),
        }
    }

    // Level 1: Cleanup Operations
    pub fn add_handler<F, Fut>(&mut self, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.handlers.push(Box::new(move || Box::pin(f())));
    }

    pub async fn cleanup(mut self) -> Result<()> {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        for handler in self.handlers {
            handler().await?;
        }
        Ok(())
    }
} 
//! Shutdown Management - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Resource Management
//! Layer 3: Shutdown Coordination
//! Layer 4: Cleanup Handling
//! Layer 5: Metrics Collection

use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tokio::time::{Duration, timeout};
use anyhow::{Context, Result};
use tracing::{error, info, warn};

use super::RuntimeConfig;

// Layer 1: Core Types
#[derive(Debug)]
pub struct ShutdownManager {
    config: RuntimeConfig,
    state: Arc<RwLock<ShutdownState>>,
    shutdown_tx: broadcast::Sender<()>,
}

#[derive(Debug, Default)]
struct ShutdownState {
    is_shutting_down: bool,
    active_tasks: usize,
}

// Layer 2: Implementation
impl ShutdownManager {
    pub fn new(config: RuntimeConfig) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            config,
            state: Arc::new(RwLock::new(ShutdownState::default())),
            shutdown_tx,
        }
    }

    // Layer 3: Task Management
    pub async fn register_task(&self) -> Result<ShutdownGuard> {
        let mut state = self.state.write().await;
        if state.is_shutting_down {
            anyhow::bail!("System is shutting down, cannot register new tasks");
        }
        state.active_tasks += 1;
        
        Ok(ShutdownGuard {
            state: Arc::clone(&self.state),
            rx: self.shutdown_tx.subscribe(),
        })
    }

    // Layer 4: Shutdown Coordination
    pub async fn initiate(&self) -> Result<()> {
        info!("Initiating shutdown sequence");
        {
            let mut state = self.state.write().await;
            state.is_shutting_down = true;
        }

        // Notify all tasks
        let _ = self.shutdown_tx.send(());

        // Wait for tasks with timeout
        let timeout_duration = self.config.shutdown_timeout;
        match timeout(timeout_duration, self.wait_for_tasks()).await {
            Ok(_) => Ok(()),
            Err(_) => {
                warn!("Shutdown timed out after {:?}", timeout_duration);
                Ok(())
            }
        }
    }

    // Layer 5: State Management
    pub async fn wait_for_tasks(&self) -> Result<()> {
        loop {
            let state = self.state.read().await;
            if state.active_tasks == 0 {
                break;
            }
            drop(state);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state.try_read(), Ok(state) if state.active_tasks == 0)
    }
}

// Resource Guard
#[derive(Debug)]
pub struct ShutdownGuard {
    state: Arc<RwLock<ShutdownState>>,
    rx: broadcast::Receiver<()>,
}

impl ShutdownGuard {
    pub async fn wait_for_shutdown(&mut self) -> Result<()> {
        self.rx.recv().await.context("Shutdown signal failed")
    }
}

impl Drop for ShutdownGuard {
    fn drop(&mut self) {
        if let Ok(mut state) = self.state.try_write() {
            state.active_tasks = state.active_tasks.saturating_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shutdown_sequence() {
        let config = RuntimeConfig {
            worker_threads: 1,
            shutdown_timeout: Duration::from_secs(1),
        };

        let manager = ShutdownManager::new(config);
        
        // Register a task
        let guard = manager.register_task().await.unwrap();
        
        // Initiate shutdown
        let shutdown_handle = tokio::spawn(async move {
            manager.initiate().await.unwrap();
        });

        // Drop the guard to simulate task completion
        drop(guard);
        
        shutdown_handle.await.unwrap();
    }
}

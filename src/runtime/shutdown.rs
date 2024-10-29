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
// Split async and sync traits per step03_avoidObviousBugs.txt
pub trait ResourceSync: Send + Sync {
    fn name(&self) -> &str;
    fn priority(&self) -> CleanupPriority;
}

#[async_trait::async_trait]
pub trait ResourceCleanup: ResourceSync {
    async fn cleanup(&self) -> Result<()>;
    async fn force_cleanup(&self) -> Result<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CleanupPriority {
    High,
    Normal,
    Low,
}

// Layer 2: State Management
#[derive(Debug)]
pub struct ShutdownManager {
    config: RuntimeConfig,
    state: Arc<RwLock<ShutdownState>>,
    shutdown_tx: broadcast::Sender<()>,
    resources: Vec<Box<dyn ResourceSync>>,
    cleaners: Vec<Box<dyn ResourceCleanup>>,
}

#[derive(Debug, Default)]
struct ShutdownState {
    is_shutting_down: bool,
    active_tasks: usize,
}

// Layer 3: Implementation
impl ShutdownManager {
    pub fn new(config: RuntimeConfig) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            config,
            state: Arc::new(RwLock::new(ShutdownState::default())),
            shutdown_tx,
            resources: Vec::new(),
            cleaners: Vec::new(),
        }
    }

    // Layer 4: Resource Registration
    pub fn register_resource<R>(&mut self, resource: R)
    where
        R: ResourceSync + 'static,
    {
        self.resources.push(Box::new(resource));
    }

    pub fn register_cleanup<R>(&mut self, resource: R)
    where
        R: ResourceCleanup + 'static,
    {
        self.cleaners.push(Box::new(resource));
    }

    // Layer 5: Shutdown Implementation
    pub async fn initiate(&self) -> Result<()> {
        info!("Initiating shutdown sequence");
        {
            let mut state = self.state.write().await;
            state.is_shutting_down = true;
        }

        // Notify all tasks
        let _ = self.shutdown_tx.send(());

        // Cleanup in priority order
        for cleaner in self.cleaners.iter() {
            match timeout(
                self.config.shutdown_timeout,
                cleaner.cleanup()
            ).await {
                Ok(result) => {
                    if let Err(e) = result {
                        warn!("Cleanup failed for {}: {}", cleaner.name(), e);
                    }
                }
                Err(_) => {
                    warn!("Cleanup timed out for {}", cleaner.name());
                    cleaner.force_cleanup().await?;
                }
            }
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
    pub async fn new(
        state: Arc<RwLock<ShutdownState>>,
        rx: broadcast::Receiver<()>,
    ) -> Result<Self> {
        let mut guard_state = state.write().await;
        if guard_state.is_shutting_down {
            anyhow::bail!("System is shutting down");
        }
        guard_state.active_tasks += 1;
        
        Ok(Self { state, rx })
    }

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
    use std::time::Duration;

    struct TestResource {
        name: String,
    }

    impl ResourceSync for TestResource {
        fn name(&self) -> &str {
            &self.name
        }

        fn priority(&self) -> CleanupPriority {
            CleanupPriority::Normal
        }
    }

    #[async_trait::async_trait]
    impl ResourceCleanup for TestResource {
        async fn cleanup(&self) -> Result<()> {
            Ok(())
        }

        async fn force_cleanup(&self) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_shutdown_sequence() -> Result<()> {
        let config = RuntimeConfig {
            worker_threads: 1,
            shutdown_timeout: Duration::from_secs(1),
        };

        let mut manager = ShutdownManager::new(config);
        
        let resource = TestResource {
            name: "test".to_string(),
        };
        
        manager.register_cleanup(resource);
        manager.initiate().await?;
        
        assert!(manager.is_complete());
        Ok(())
    }
}

//! Graceful Shutdown Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Shutdown Orchestration
//! - ShutdownManager    (coordinates complete shutdown)
//! - ShutdownMetrics    (tracks shutdown progress)
//! - ResourceReclaimer  (ensures cleanup)
//! 
//! Level 3: Component Shutdown
//! - WorkerShutdown     (worker graceful stop)
//! - StorageShutdown    (storage cleanup)
//! - TaskShutdown       (task cancellation)
//! 
//! Level 2: Resource Management
//! - ConnectionDraining (DB connection cleanup)
//! - BufferReclaiming  (memory cleanup)
//! - TaskDraining      (task completion)
//! 
//! Level 1 (Base): Core Shutdown Types
//! - ShutdownConfig    (shutdown parameters)
//! - ShutdownState     (shutdown progress)
//! - ShutdownError     (shutdown failures)

use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio::time::{Duration, timeout, Instant};
use metrics::{Counter, Gauge, Histogram};
use futures::future::join_all;
use crate::core::{error::{Error, Result}, types::*};
use tracing::{info, warn, error};

// ===== Level 1: Core Shutdown Types =====
// Design Choice: Using enums for explicit state transitions

/// Shutdown progress tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownPhase {
    /// Initial shutdown signal sent
    Initiated,
    /// Tasks are being drained
    DrainingSoftly,
    /// Force cancelling remaining tasks
    ForcingShutdown,
    /// All resources cleaned up
    Completed,
}

/// Shutdown progress metrics
#[derive(Debug)]
struct ShutdownMetrics {
    /// Active tasks remaining
    active_tasks: Gauge,
    /// Resources pending cleanup
    pending_resources: Gauge,
    /// Shutdown duration
    shutdown_duration: Histogram,
    /// Forced shutdowns counter
    forced_shutdowns: Counter,
}

// ===== Level 2: Resource Management =====
// Design Choice: Using async traits for cleanup

/// Resource cleanup tracking
#[async_trait::async_trait]
pub trait ResourceCleanup: Send + Sync {
    /// Cleanup resource
    async fn cleanup(&self) -> Result<()>;
    /// Force cleanup after timeout
    async fn force_cleanup(&self) -> Result<()>;
}

/// Connection draining implementation
pub struct ConnectionDraining {
    /// Active connections
    connections: Arc<Semaphore>,
    /// Drain timeout
    timeout: Duration,
    /// Metrics
    metrics: ShutdownMetrics,
}

// ===== Level 3: Component Shutdown =====
// Design Choice: Using builder pattern for configuration

/// Main shutdown manager
pub struct ShutdownManager {
    /// Shutdown configuration
    config: ShutdownConfig,
    /// Shutdown signal
    shutdown_tx: broadcast::Sender<()>,
    /// Resource cleanup handlers
    resources: Vec<Arc<dyn ResourceCleanup>>,
    /// Metrics
    metrics: Arc<ShutdownMetrics>,
    /// Current phase
    phase: Arc<Mutex<ShutdownPhase>>,
    /// Shutdown deadline
    deadline: Arc<Mutex<Option<Instant>>>,
}

impl ShutdownManager {
    /// Creates new shutdown manager
    pub fn new(config: ShutdownConfig) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        
        Self {
            config,
            shutdown_tx,
            resources: Vec::new(),
            metrics: Arc::new(ShutdownMetrics::new()),
            phase: Arc::new(Mutex::new(ShutdownPhase::Initiated)),
            deadline: Arc::new(Mutex::new(None)),
        }
    }

    /// Register resource for cleanup
    pub fn register_resource(&mut self, resource: Arc<dyn ResourceCleanup>) {
        self.resources.push(resource);
        self.metrics.pending_resources.increment(1.0);
    }

    /// Initiate graceful shutdown
    pub async fn shutdown(self) -> Result<()> {
        let start = Instant::now();
        info!("Initiating graceful shutdown");

        // Set shutdown deadline
        *self.deadline.lock().await = Some(start + self.config.graceful_timeout);

        // Broadcast shutdown signal
        let _ = self.shutdown_tx.send(());
        
        // Update phase
        *self.phase.lock().await = ShutdownPhase::DrainingSoftly;

        // Try graceful cleanup with deadline
        let deadline = *self.deadline.lock().await.as_ref().unwrap();
        let remaining_time = deadline.saturating_duration_since(Instant::now());

        match timeout(remaining_time, self.cleanup_resources()).await {
            Ok(Ok(_)) => {
                info!("Graceful shutdown completed successfully");
                *self.phase.lock().await = ShutdownPhase::Completed;
            }
            _ => {
                warn!("Graceful shutdown timed out, forcing cleanup");
                *self.phase.lock().await = ShutdownPhase::ForcingShutdown;
                self.metrics.forced_shutdowns.increment(1);
                
                // Force cleanup with short timeout
                let force_timeout = Duration::from_secs(5);
                match timeout(force_timeout, self.force_cleanup_resources()).await {
                    Ok(Ok(_)) => info!("Forced cleanup completed"),
                    _ => error!("Forced cleanup failed"),
                }
            }
        }

        self.metrics.shutdown_duration.record(start.elapsed());
        Ok(())
    }

    /// Cleanup resources gracefully
    async fn cleanup_resources(&self) -> Result<()> {
        let futures: Vec<_> = self.resources.iter()
            .map(|r| r.cleanup())
            .collect();

        for result in join_all(futures).await {
            if let Err(e) = result {
                error!("Resource cleanup failed: {}", e);
            }
            self.metrics.pending_resources.decrement(1.0);
        }

        Ok(())
    }

    /// Force cleanup of resources
    async fn force_cleanup_resources(&self) -> Result<()> {
        let futures: Vec<_> = self.resources.iter()
            .map(|r| r.force_cleanup())
            .collect();

        join_all(futures).await
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    struct TestResource {
        cleanup_duration: Duration,
    }

    #[async_trait::async_trait]
    impl ResourceCleanup for TestResource {
        async fn cleanup(&self) -> Result<()> {
            sleep(self.cleanup_duration).await;
            Ok(())
        }

        async fn force_cleanup(&self) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_graceful_shutdown() {
        let config = ShutdownConfig {
            graceful_timeout: Duration::from_secs(1),
            force_after_timeout: true,
        };

        let mut manager = ShutdownManager::new(config);
        
        // Add quick resource
        manager.register_resource(Arc::new(TestResource {
            cleanup_duration: Duration::from_millis(100),
        }));

        assert!(manager.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_forced_shutdown() {
        let config = ShutdownConfig {
            graceful_timeout: Duration::from_millis(100),
            force_after_timeout: true,
        };

        let mut manager = ShutdownManager::new(config);
        
        // Add slow resource
        manager.register_resource(Arc::new(TestResource {
            cleanup_duration: Duration::from_secs(1),
        }));

        assert!(manager.shutdown().await.is_ok());
    }
}

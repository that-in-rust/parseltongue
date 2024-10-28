//! Graceful Shutdown Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Shutdown Coordination
//! - ShutdownManager   (coordinates shutdown)
//!   ├── Phase tracking
//!   ├── Resource cleanup
//!   └── Timeout handling
//! 
//! Level 3: Resource Management
//! - ResourceManager   (manages cleanup)
//!   ├── Cleanup ordering
//!   ├── Error handling
//!   └── Timeout handling
//! 
//! Level 2: Cleanup Implementation
//! - CleanupTask      (cleanup task)
//!   ├── Task execution
//!   ├── Error handling
//!   └── Timeout handling
//! 
//! Level 1 (Base): Core Types
//! - ShutdownPhase    (shutdown states)
//! - ShutdownConfig   (configuration)
//! - ShutdownError    (error types)

use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio::time::{Duration, timeout, Instant};
use metrics::{Counter, Gauge, Histogram};
use futures::future::join_all;
use crate::core::{error::{Error, Result}, types::*};
use tracing::{info, warn, error};

// Design Choice: Using enums for explicit state transitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownPhase {
    Initiated,
    DrainingSoftly,
    ForcingShutdown,
    Completed,
}

// Design Choice: Using async traits for cleanup
#[async_trait::async_trait]
pub trait ResourceCleanup: Send + Sync {
    async fn cleanup(&self) -> Result<()>;
    async fn force_cleanup(&self) -> Result<()>;
}

// Design Choice: Using builder pattern for configuration
#[derive(Debug, Clone)]
pub struct ShutdownConfig {
    pub graceful_timeout: Duration,
    pub force_timeout: Duration,
    pub cleanup_order: Vec<CleanupPriority>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CleanupPriority {
    High,
    Normal,
    Low,
}

// Design Choice: Using metrics for monitoring
#[derive(Debug)]
struct ShutdownMetrics {
    active_tasks: Gauge,
    pending_resources: Gauge,
    shutdown_duration: Histogram,
    forced_shutdowns: Counter,
}

impl ShutdownMetrics {
    fn new() -> Self {
        Self {
            active_tasks: Gauge::new(),
            pending_resources: Gauge::new(),
            shutdown_duration: Histogram::new(),
            forced_shutdowns: Counter::new(),
        }
    }
}

// Design Choice: Using Arc for shared state
pub struct ShutdownManager {
    config: ShutdownConfig,
    shutdown_tx: broadcast::Sender<()>,
    resources: Vec<(CleanupPriority, Arc<dyn ResourceCleanup>)>,
    metrics: Arc<ShutdownMetrics>,
    phase: Arc<Mutex<ShutdownPhase>>,
    deadline: Arc<Mutex<Option<Instant>>>,
}

impl ShutdownManager {
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

    pub fn register_resource(&mut self, priority: CleanupPriority, resource: Arc<dyn ResourceCleanup>) {
        self.resources.push((priority, resource));
        self.metrics.pending_resources.increment(1.0);
    }

    pub async fn shutdown(mut self) -> Result<()> {
        let start = Instant::now();
        info!("Initiating graceful shutdown");

        // Sort resources by priority
        self.resources.sort_by_key(|(p, _)| *p);

        // Set deadline
        *self.deadline.lock().await = Some(start + self.config.graceful_timeout);

        // Broadcast shutdown signal
        let _ = self.shutdown_tx.send(());
        
        // Update phase
        *self.phase.lock().await = ShutdownPhase::DrainingSoftly;

        // Try graceful cleanup
        let deadline = *self.deadline.lock().await.as_ref().unwrap();
        let remaining = deadline.saturating_duration_since(Instant::now());

        match timeout(remaining, self.cleanup_resources()).await {
            Ok(Ok(_)) => {
                info!("Graceful shutdown completed successfully");
                *self.phase.lock().await = ShutdownPhase::Completed;
            }
            _ => {
                warn!("Graceful shutdown timed out, forcing cleanup");
                *self.phase.lock().await = ShutdownPhase::ForcingShutdown;
                self.metrics.forced_shutdowns.increment(1);
                
                // Force cleanup with short timeout
                match timeout(self.config.force_timeout, self.force_cleanup_resources()).await {
                    Ok(Ok(_)) => info!("Forced cleanup completed"),
                    _ => error!("Forced cleanup failed"),
                }
            }
        }

        self.metrics.shutdown_duration.record(start.elapsed());
        Ok(())
    }

    async fn cleanup_resources(&self) -> Result<()> {
        for (_, resource) in &self.resources {
            if let Err(e) = resource.cleanup().await {
                error!("Resource cleanup failed: {}", e);
            }
            self.metrics.pending_resources.decrement(1.0);
        }
        Ok(())
    }

    async fn force_cleanup_resources(&self) -> Result<()> {
        let futures: Vec<_> = self.resources.iter()
            .map(|(_, r)| r.force_cleanup())
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
            force_timeout: Duration::from_secs(1),
            cleanup_order: vec![CleanupPriority::Normal],
        };

        let mut manager = ShutdownManager::new(config);
        
        manager.register_resource(
            CleanupPriority::Normal,
            Arc::new(TestResource {
                cleanup_duration: Duration::from_millis(100),
            })
        );

        assert!(manager.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_forced_shutdown() {
        let config = ShutdownConfig {
            graceful_timeout: Duration::from_millis(100),
            force_timeout: Duration::from_secs(1),
            cleanup_order: vec![CleanupPriority::Normal],
        };

        let mut manager = ShutdownManager::new(config);
        
        manager.register_resource(
            CleanupPriority::Normal,
            Arc::new(TestResource {
                cleanup_duration: Duration::from_secs(1),
            })
        );

        assert!(manager.shutdown().await.is_ok());
    }
}

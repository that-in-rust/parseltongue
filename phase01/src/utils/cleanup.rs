//! Resource Cleanup Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Cleanup Coordination
//! - CleanupManager    (manages cleanup)
//! - CleanupMetrics    (tracks cleanup)
//! - CleanupScheduler  (schedules cleanup)
//! 
//! Level 3: Cleanup Management
//! - CleanupHandler    (handles cleanup)
//! - CleanupMonitor    (monitors cleanup)
//! - CleanupLogger     (logs cleanup)
//! 
//! Level 2: Cleanup Implementation
//! - Cleanup           (cleanup implementation)
//! - CleanupState      (cleanup lifecycle)
//! - CleanupMetrics    (cleanup stats)
//! 
//! Level 1 (Base): Core Cleanup Types
//! - CleanupConfig     (cleanup configuration)
//! - CleanupMetrics    (cleanup metrics)
//! - CleanupError      (cleanup errors)

use std::sync::Arc;
use tokio::sync::Mutex;
use futures::Future;
use metrics::{Counter, Gauge};
use crate::core::{error::{Error, Result}, types::*};

// ===== Level 1: Core Cleanup Types =====
// Design Choice: Using futures for async cleanup

/// Cleanup configuration
#[derive(Debug, Clone)]
pub struct CleanupConfig {
    /// Cleanup timeout
    pub timeout: std::time::Duration,
    /// Enable metrics
    pub metrics_enabled: bool,
}

// ===== Level 2: Cleanup Implementation =====
// Design Choice: Using RAII for cleanup

/// Cleanup guard implementation
pub struct CleanupGuard<T> {
    /// Managed resource
    resource: Option<T>,
    /// Cleanup function
    cleanup: Box<dyn FnOnce(T) -> Box<dyn Future<Output = ()> + Send + Unpin> + Send>,
    /// Guard metrics
    metrics: Arc<CleanupMetrics>,
}

impl<T> CleanupGuard<T> {
    /// Creates new cleanup guard
    pub fn new<F, Fut>(resource: T, cleanup: F) -> Self
    where
        F: FnOnce(T) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + Unpin + 'static,
    {
        Self {
            resource: Some(resource),
            cleanup: Box::new(move |r| Box::new(cleanup(r))),
            metrics: Arc::new(CleanupMetrics::new()),
        }
    }
}

impl<T> Drop for CleanupGuard<T> {
    fn drop(&mut self) {
        if let Some(resource) = self.resource.take() {
            let cleanup = (self.cleanup)(resource);
            let metrics = self.metrics.clone();
            
            tokio::spawn(async move {
                cleanup.await;
                metrics.cleanups_completed.increment(1);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_cleanup_guard() {
        let cleaned = Arc::new(AtomicBool::new(false));
        let cleaned_clone = cleaned.clone();
        
        {
            let guard = CleanupGuard::new(42, move |_| {
                let cleaned = cleaned_clone.clone();
                async move {
                    cleaned.store(true, Ordering::SeqCst);
                }
            });
            
            assert!(!cleaned.load(Ordering::SeqCst));
        }
        
        // Allow time for async cleanup
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        assert!(cleaned.load(Ordering::SeqCst));
    }
}


use std::sync::Arc;
use tokio::sync::Semaphore;
use anyhow::Result;
use tracing::{info, warn};

//! Storage Guard - Pyramidal Structure
//! Layer 1: Guard Types
//! Layer 2: Resource Management
//! Layer 3: Error Handling
//! Layer 4: Cleanup Logic
//! Layer 5: Metrics Collection

// Layer 1: Core Types
pub struct StorageGuard {
    _permit: Option<Arc<tokio::sync::SemaphorePermit>>,
    metrics: GuardMetrics,
}

#[derive(Debug, Default)]
struct GuardMetrics {
    operation_count: usize,
    error_count: usize,
}

// Layer 2: Implementation
impl StorageGuard {
    pub async fn new(pool: Arc<Semaphore>) -> Result<Self> {
        let permit = pool.acquire_owned().await?;
        
        info!("Acquired storage guard");
        Ok(Self {
            _permit: Some(Arc::new(permit)),
            metrics: GuardMetrics::default(),
        })
    }

    // Layer 3: Operation Tracking
    pub fn track_operation(&mut self, success: bool) {
        self.metrics.operation_count += 1;
        if !success {
            self.metrics.error_count += 1;
        }
    }

    // Layer 4: Status Methods
    pub fn is_healthy(&self) -> bool {
        let error_rate = if self.metrics.operation_count > 0 {
            self.metrics.error_count as f64 / self.metrics.operation_count as f64
        } else {
            0.0
        };
        error_rate < 0.1 // Less than 10% error rate
    }
}

// Layer 5: Cleanup
impl Drop for StorageGuard {
    fn drop(&mut self) {
        if !self.is_healthy() {
            warn!(
                "Storage guard dropped with high error rate: {}/{}", 
                self.metrics.error_count, 
                self.metrics.operation_count
            );
        }
        info!("Released storage guard");
    }
}

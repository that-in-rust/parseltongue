use std::sync::Arc;
use tokio::sync::Semaphore;
use bytes::BytesMut;
use anyhow::Result;
use tracing::{info, warn};

/// RAII guard for ZIP entry buffers
pub struct BufferGuard {
    buffer: BytesMut,
    pool: Arc<Semaphore>,
}

impl BufferGuard {
    pub(crate) fn new(buffer: BytesMut, pool: Arc<Semaphore>) -> Self {
        Self { buffer, pool }
    }

    pub fn get_mut(&mut self) -> &mut BytesMut {
        &mut self.buffer
    }
}

impl Drop for BufferGuard {
    fn drop(&mut self) {
        self.pool.add_permits(1);
    }
}

/// RAII guard for ZIP entry processing
pub struct ProcessingGuard {
    semaphore: Arc<Semaphore>,
}

impl ProcessingGuard {
    pub(crate) fn new(semaphore: Arc<Semaphore>) -> Self {
        Self { semaphore }
    }
}

impl Drop for ProcessingGuard {
    fn drop(&mut self) {
        self.semaphore.add_permits(1);
    }
}

/// ZIP Guard Implementation - Pyramidal Structure
/// Layer 1: Guard Interface
/// Layer 2: Resource Management
/// Layer 3: Error Handling
/// Layer 4: Metrics Collection
/// Layer 5: Cleanup

pub struct ZipGuard {
    _permit: Option<Arc<tokio::sync::SemaphorePermit>>,
    metrics: GuardMetrics,
}

#[derive(Debug, Default)]
struct GuardMetrics {
    entries_processed: usize,
    bytes_processed: u64,
    errors: usize,
}

impl ZipGuard {
    pub async fn new(pool: Arc<Semaphore>) -> Result<Self> {
        let permit = pool.acquire_owned().await?;
        
        info!("Acquired ZIP processing guard");
        Ok(Self {
            _permit: Some(Arc::new(permit)),
            metrics: GuardMetrics::default(),
        })
    }

    // Layer 3: Metrics Collection
    pub fn record_entry(&mut self, size: u64, success: bool) {
        self.metrics.entries_processed += 1;
        self.metrics.bytes_processed += size;
        
        if !success {
            self.metrics.errors += 1;
        }
    }

    // Layer 4: Status Methods
    pub fn is_healthy(&self) -> bool {
        let error_rate = if self.metrics.entries_processed > 0 {
            self.metrics.errors as f64 / self.metrics.entries_processed as f64
        } else {
            0.0
        };
        error_rate < 0.1 // Less than 10% error rate
    }

    pub fn get_metrics(&self) -> &GuardMetrics {
        &self.metrics
    }
}

// Layer 5: Cleanup
impl Drop for ZipGuard {
    fn drop(&mut self) {
        if !self.is_healthy() {
            warn!(
                "ZIP guard dropped with high error rate: {}/{}", 
                self.metrics.errors,
                self.metrics.entries_processed
            );
        }
        info!(
            "Released ZIP guard. Processed {} entries ({} bytes)", 
            self.metrics.entries_processed,
            self.metrics.bytes_processed
        );
    }
}

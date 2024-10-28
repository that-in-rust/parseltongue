//! Buffer Management Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Buffer Coordination
//! - BufferManager    (manages buffer pools)
//! - BufferMetrics    (tracks buffer usage)
//! - PoolBalancer     (balances pool load)
//! 
//! Level 3: Pool Management
//! - BufferPool       (manages buffer allocation)
//! - PoolMetrics      (tracks pool stats)
//! - ResizePolicy     (handles pool sizing)
//! 
//! Level 2: Buffer Implementation
//! - Buffer           (buffer implementation)
//! - BufferState      (buffer lifecycle)
//! - BufferMetrics    (buffer stats)
//! 
//! Level 1 (Base): Core Buffer Types
//! - BufferConfig     (buffer configuration)
//! - BufferMetrics    (buffer metrics)
//! - BufferError      (buffer errors)

use std::sync::Arc;
use tokio::sync::{Semaphore, Mutex};
use bytes::{Bytes, BytesMut};
use metrics::{Counter, Gauge};
use crate::core::{error::{Error, Result}, types::*};

// ===== Level 1: Core Buffer Types =====
// Design Choice: Using BytesMut for zero-copy

/// Buffer pool configuration
#[derive(Debug, Clone)]
pub struct BufferConfig {
    /// Initial buffer size
    pub initial_size: usize,
    /// Maximum buffer size
    pub max_size: usize,
    /// Pool capacity
    pub pool_capacity: usize,
}

// ===== Level 2: Buffer Implementation =====
// Design Choice: Using Arc for shared buffers

/// Buffer pool implementation
pub struct BufferPool {
    /// Available buffers
    buffers: Arc<Mutex<Vec<BytesMut>>>,
    /// Buffer semaphore
    semaphore: Arc<Semaphore>,
    /// Pool configuration
    config: BufferConfig,
    /// Pool metrics
    metrics: BufferMetrics,
}

impl BufferPool {
    /// Creates new buffer pool
    pub fn new(config: BufferConfig) -> Self {
        let buffers = Arc::new(Mutex::new(Vec::with_capacity(config.pool_capacity)));
        let semaphore = Arc::new(Semaphore::new(config.pool_capacity));
        let metrics = BufferMetrics::new();

        Self {
            buffers,
            semaphore,
            config,
            metrics,
        }
    }

    /// Acquires buffer from pool
    pub async fn acquire(&self) -> Result<BufferGuard> {
        let _permit = self.semaphore.acquire().await?;
        
        let mut buffers = self.buffers.lock().await;
        let buffer = buffers.pop()
            .unwrap_or_else(|| BytesMut::with_capacity(self.config.initial_size));

        self.metrics.active_buffers.increment(1.0);
        
        Ok(BufferGuard {
            buffer,
            pool: self.clone(),
        })
    }

    /// Returns buffer to pool
    async fn release(&self, mut buffer: BytesMut) {
        buffer.clear();
        
        if buffer.capacity() <= self.config.max_size {
            self.buffers.lock().await.push(buffer);
        }

        self.metrics.active_buffers.decrement(1.0);
    }
}

// ===== Level 3: Pool Management =====
// Design Choice: Using RAII for buffer management

/// Buffer guard for automatic return
pub struct BufferGuard {
    /// Managed buffer
    buffer: BytesMut,
    /// Owner pool
    pool: BufferPool,
}

impl Drop for BufferGuard {
    fn drop(&mut self) {
        let buffer = std::mem::replace(&mut self.buffer, BytesMut::new());
        let pool = self.pool.clone();
        
        tokio::spawn(async move {
            pool.release(buffer).await;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_buffer_pool() {
        let config = BufferConfig {
            initial_size: 1024,
            max_size: 8192,
            pool_capacity: 10,
        };

        let pool = BufferPool::new(config);
        
        // Test buffer acquisition
        let guard = pool.acquire().await.unwrap();
        assert_eq!(guard.buffer.capacity(), 1024);
        
        // Buffer should be returned to pool on drop
        drop(guard);
        
        // Allow time for async release
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        assert_eq!(pool.metrics.active_buffers.get(), 0.0);
    }
}


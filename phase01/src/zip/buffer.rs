//! ZIP Buffer Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Buffer Orchestration
//! - BufferManager    (manages buffer lifecycle)
//!   ├── Pool management
//!   ├── Size adaptation
//!   └── Memory pressure handling
//! 
//! Level 3: Memory Management
//! - MemoryTracker    (tracks memory usage)
//!   ├── Usage monitoring
//!   ├── Pressure detection
//!   └── Size adjustment
//! 
//! Level 2: Buffer Implementation
//! - AdaptiveBuffer   (smart buffering)
//!   ├── Size scaling
//!   ├── Chunk management
//!   └── Memory limits
//! 
//! Level 1 (Base): Buffer Types
//! - BufferConfig     (configuration)
//!   ├── Size limits
//!   ├── Growth policy
//!   └── Pressure thresholds

use std::sync::Arc;
use tokio::sync::Semaphore;
use bytes::{Bytes, BytesMut};
use metrics::{Counter, Gauge};
use crate::core::{error::{Error, Result}, types::*};

// ===== Level 1: Buffer Types =====
// Design Choice: Using BytesMut for zero-copy operations

/// Buffer configuration
#[derive(Debug, Clone)]
pub struct BufferConfig {
    /// Initial buffer size
    pub initial_size: usize,
    /// Maximum buffer size
    pub max_size: usize,
    /// Growth factor
    pub growth_factor: f32,
    /// Memory pressure threshold
    pub pressure_threshold: f32,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            initial_size: 8 * 1024,     // 8KB initial
            max_size: 1024 * 1024,      // 1MB max
            growth_factor: 1.5,         // 50% growth
            pressure_threshold: 0.75,    // 75% memory pressure
        }
    }
}

// ===== Level 2: Buffer Implementation =====
// Design Choice: Using adaptive sizing based on pressure

/// Adaptive buffer implementation
pub struct AdaptiveBuffer {
    /// Current buffer
    buffer: BytesMut,
    /// Buffer configuration
    config: BufferConfig,
    /// Memory tracker
    tracker: Arc<MemoryTracker>,
}

impl AdaptiveBuffer {
    /// Creates new adaptive buffer
    pub fn new(config: BufferConfig) -> Self {
        let buffer = BytesMut::with_capacity(config.initial_size);
        let tracker = Arc::new(MemoryTracker::new());

        Self {
            buffer,
            config,
            tracker,
        }
    }

    /// Gets current buffer size
    pub fn size(&self) -> usize {
        self.buffer.capacity()
    }

    /// Resizes buffer based on pressure
    pub fn resize(&mut self) {
        let pressure = self.tracker.pressure();
        
        if pressure > self.config.pressure_threshold {
            // Under pressure - shrink
            let new_size = (self.size() as f32 / self.config.growth_factor) as usize;
            self.buffer.reserve(new_size.max(self.config.initial_size));
        } else {
            // Room to grow
            let new_size = (self.size() as f32 * self.config.growth_factor) as usize;
            self.buffer.reserve(new_size.min(self.config.max_size));
        }
    }
}

// ===== Level 3: Memory Management =====
// Design Choice: Using atomic metrics for thread safety

/// Memory usage tracker
pub struct MemoryTracker {
    /// Total allocated
    allocated: Gauge,
    /// Memory pressure
    pressure: Gauge,
    /// Buffer resizes
    resizes: Counter,
}

impl MemoryTracker {
    /// Creates new memory tracker
    fn new() -> Self {
        Self {
            allocated: Gauge::new(),
            pressure: Gauge::new(),
            resizes: Counter::new(),
        }
    }

    /// Gets current memory pressure
    pub fn pressure(&self) -> f32 {
        self.pressure.get()
    }

    /// Updates memory allocation
    pub fn update_allocation(&self, size: usize) {
        self.allocated.set(size as f64);
        
        // Update pressure based on system memory
        if let Ok(mem_info) = sys_info::mem_info() {
            let pressure = size as f64 / mem_info.total as f64;
            self.pressure.set(pressure);
        }
    }
}

// ===== Level 4: Buffer Orchestration =====
// Design Choice: Using pool for buffer reuse

/// Buffer pool manager
pub struct BufferManager {
    /// Buffer pool
    pool: Arc<Semaphore>,
    /// Buffer configuration
    config: BufferConfig,
    /// Memory tracker
    tracker: Arc<MemoryTracker>,
}

impl BufferManager {
    /// Creates new buffer manager
    pub fn new(config: BufferConfig, pool_size: usize) -> Self {
        Self {
            pool: Arc::new(Semaphore::new(pool_size)),
            config,
            tracker: Arc::new(MemoryTracker::new()),
        }
    }

    /// Acquires buffer from pool
    pub async fn acquire_buffer(&self) -> Result<BufferGuard> {
        let _permit = self.pool.acquire().await?;
        
        Ok(BufferGuard {
            buffer: AdaptiveBuffer::new(self.config.clone()),
            pool: self.pool.clone(),
            tracker: self.tracker.clone(),
        })
    }
}

/// RAII guard for buffer
pub struct BufferGuard {
    /// Managed buffer
    buffer: AdaptiveBuffer,
    /// Buffer pool
    pool: Arc<Semaphore>,
    /// Memory tracker
    tracker: Arc<MemoryTracker>,
}

impl BufferGuard {
    /// Gets mutable buffer reference
    pub fn get_mut(&mut self) -> &mut BytesMut {
        &mut self.buffer.buffer
    }

    /// Resizes buffer if needed
    pub fn resize(&mut self) {
        self.buffer.resize();
        self.tracker.resizes.increment(1);
        self.tracker.update_allocation(self.buffer.size());
    }
}

impl Drop for BufferGuard {
    fn drop(&mut self) {
        self.tracker.update_allocation(0);
        self.pool.add_permits(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    use std::time::Duration;

    #[tokio::test]
    async fn test_buffer_lifecycle() {
        let config = BufferConfig::default();
        let manager = BufferManager::new(config, 4);

        let mut guard = manager.acquire_buffer().await.unwrap();
        
        // Test initial size
        assert_eq!(guard.buffer.size(), 8 * 1024);

        // Test growth
        guard.resize();
        assert!(guard.buffer.size() > 8 * 1024);

        // Test cleanup
        drop(guard);
        
        // Pool should have permit available
        assert!(manager.acquire_buffer().await.is_ok());
    }

    #[tokio::test]
    async fn test_pressure_handling() {
        let config = BufferConfig {
            pressure_threshold: 0.5,
            ..Default::default()
        };
        
        let manager = BufferManager::new(config, 1);
        let mut guard = manager.acquire_buffer().await.unwrap();

        // Simulate memory pressure
        guard.tracker.pressure.set(0.8);
        guard.resize();

        // Buffer should shrink under pressure
        assert!(guard.buffer.size() < 8 * 1024);
    }
}

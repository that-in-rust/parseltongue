//! Core Domain Types
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): High-Level Compositions
//! - RuntimeManager     (combines workers, resources, metrics)
//! - ProcessingPipeline (combines ZIP, storage, validation)
//! 
//! Level 3: Coordinating Types
//! - WorkerPool        (manages async tasks)
//! - StorageManager    (manages DB connections)
//! - ValidationManager (manages CRC checks)
//! 
//! Level 2: Operational Types
//! - ZipEntry         (represents ZIP contents)
//! - StorageEntry     (represents DB records)
//! - MetricsData      (represents performance data)
//! 
//! Level 1 (Base): Fundamental Types
//! - ResourceLimits   (control system boundaries)
//! - BufferConfig     (manage memory usage)
//! - ValidationConfig (control validation behavior)

use std::{path::PathBuf, time::Duration, sync::Arc};
use bytes::Bytes;
use serde::{Serialize, Deserialize};
use tokio::sync::Semaphore;

// ===== Level 1: Fundamental Types =====
// Design Choice: Using newtype pattern for type safety and semantic meaning

/// Resource limits for the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum number of concurrent tasks
    pub max_tasks: usize,
    /// Maximum memory usage in bytes
    pub max_memory: usize,
    /// Maximum DB connections
    pub max_connections: usize,
}

/// Buffer configuration for streaming operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferConfig {
    /// Initial buffer size
    pub initial_size: usize,
    /// Maximum buffer size
    pub max_size: usize,
    /// Growth factor for adaptive sizing
    pub growth_factor: f32,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Whether to validate CRC
    pub validate_crc: bool,
    /// Whether to validate file names
    pub validate_names: bool,
    /// Maximum parallel validations
    pub max_parallel: usize,
}

// ===== Level 2: Operational Types =====
// Design Choice: Using Arc for shared ownership in async context

/// Represents a ZIP file entry
#[derive(Debug, Clone)]
pub struct ZipEntry {
    /// Relative path within ZIP
    pub path: PathBuf,
    /// Entry contents
    pub content: Bytes,
    /// Content CRC32
    pub crc32: u32,
    /// Original size
    pub size: u64,
}

/// Represents a storage entry
#[derive(Debug, Clone)]
pub struct StorageEntry {
    /// Storage key
    pub key: String,
    /// Entry data
    pub data: Bytes,
    /// Metadata
    pub metadata: EntryMetadata,
}

/// Entry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadata {
    /// Original path
    pub original_path: PathBuf,
    /// Content hash
    pub content_hash: String,
    /// Storage timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ===== Level 3: Coordinating Types =====
// Design Choice: Using builder pattern for complex configurations

/// Worker pool configuration
#[derive(Debug, Clone)]
pub struct WorkerPool {
    /// Number of worker threads
    pub worker_threads: usize,
    /// Task priorities
    pub priorities: Vec<TaskPriority>,
    /// Resource semaphore
    pub semaphore: Arc<Semaphore>,
}

/// Storage manager configuration
#[derive(Debug, Clone)]
pub struct StorageManager {
    /// Connection pool size
    pub pool_size: usize,
    /// Batch size for writes
    pub batch_size: usize,
    /// Write timeout
    pub write_timeout: Duration,
}

// ===== Level 4: High-Level Compositions =====
// Design Choice: Using composition over inheritance

/// Runtime manager configuration
#[derive(Debug)]
pub struct RuntimeManager {
    /// Worker pool
    pub workers: WorkerPool,
    /// Resource limits
    pub limits: ResourceLimits,
    /// Buffer config
    pub buffers: BufferConfig,
    /// Validation config
    pub validation: ValidationConfig,
}

/// Processing pipeline configuration
#[derive(Debug)]
pub struct ProcessingPipeline {
    /// Storage manager
    pub storage: StorageManager,
    /// Worker pool
    pub workers: WorkerPool,
    /// Validation config
    pub validation: ValidationConfig,
}

// ===== Supporting Types =====

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Builder for RuntimeManager
#[derive(Default)]
pub struct RuntimeManagerBuilder {
    workers: Option<WorkerPool>,
    limits: Option<ResourceLimits>,
    buffers: Option<BufferConfig>,
    validation: Option<ValidationConfig>,
}

impl RuntimeManagerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn workers(mut self, workers: WorkerPool) -> Self {
        self.workers = Some(workers);
        self
    }

    pub fn limits(mut self, limits: ResourceLimits) -> Self {
        self.limits = Some(limits);
        self
    }

    pub fn build(self) -> Result<RuntimeManager, crate::core::error::Error> {
        Ok(RuntimeManager {
            workers: self.workers.ok_or_else(|| 
                crate::core::error::Error::ResourceLimit("Workers not configured".into()))?,
            limits: self.limits.ok_or_else(|| 
                crate::core::error::Error::ResourceLimit("Limits not configured".into()))?,
            buffers: self.buffers.unwrap_or_default(),
            validation: self.validation.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_manager_builder() {
        let workers = WorkerPool {
            worker_threads: 4,
            priorities: vec![TaskPriority::Normal],
            semaphore: Arc::new(Semaphore::new(4)),
        };

        let limits = ResourceLimits {
            max_tasks: 10,
            max_memory: 1024 * 1024,
            max_connections: 5,
        };

        let manager = RuntimeManagerBuilder::new()
            .workers(workers)
            .limits(limits)
            .build();

        assert!(manager.is_ok());
    }
}


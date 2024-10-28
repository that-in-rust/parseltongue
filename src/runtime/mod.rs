//! Runtime Management and Coordination
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Runtime Coordination
//! - Runtime configuration
//! - Worker pool management
//! - Resource coordination
//! 
//! Level 3: Resource Management
//! - Thread pool control
//! - Memory management
//! - Connection limits
//! 
//! Level 2: Configuration Types
//! - Runtime settings
//! - Worker settings
//! - Resource limits
//! 
//! Level 1 (Base): Core Types
//! - Basic configurations
//! - Resource types
//! - Limit types

pub mod worker;
pub mod shutdown;

use std::time::Duration;
use serde::{Serialize, Deserialize};
use crate::core::{error::Result, types::*};

// Design Choice: Using builder pattern for configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub worker_config: WorkerConfig,
    pub resource_limits: ResourceLimits,
    pub shutdown_config: ShutdownConfig,
}

impl RuntimeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_workers(mut self, count: usize) -> Self {
        self.worker_config.thread_count = count;
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.worker_config.queue_capacity = size;
        self
    }

    pub fn with_shutdown_timeout(mut self, timeout: u64) -> Self {
        self.shutdown_config.timeout = Duration::from_secs(timeout);
        self
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            worker_config: WorkerConfig::default(),
            resource_limits: ResourceLimits::default(),
            shutdown_config: ShutdownConfig::default(),
        }
    }
}

// Design Choice: Using strong typing for worker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    pub thread_count: usize,
    pub queue_capacity: usize,
    pub stack_size: usize,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            thread_count: num_cpus::get(),
            queue_capacity: 1000,
            stack_size: 3 * 1024 * 1024, // 3MB
        }
    }
}

// Design Choice: Using explicit resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_tasks: usize,
    pub max_memory: usize,
    pub max_connections: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_tasks: num_cpus::get() * 2,
            max_memory: 1024 * 1024 * 1024, // 1GB
            max_connections: num_cpus::get(),
        }
    }
}

// Design Choice: Using explicit shutdown configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownConfig {
    pub timeout: Duration,
    pub force_after_timeout: bool,
}

impl Default for ShutdownConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            force_after_timeout: true,
        }
    }
}

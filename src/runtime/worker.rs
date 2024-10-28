//! Worker Thread Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Worker Management
//! - Worker lifecycle
//! - Task scheduling
//! - Resource tracking
//! 
//! Level 3: Task Management
//! - Task execution
//! - Priority handling
//! - Error recovery
//! 
//! Level 2: Resource Control
//! - Memory limits
//! - CPU usage
//! - Queue depth
//! 
//! Level 1 (Base): Worker Types
//! - Worker state
//! - Task types
//! - Metrics

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use futures::Future;
use metrics::{Counter, Gauge};
use crate::core::{error::Result, types::*};

// Design Choice: Using strong typing for worker state
#[derive(Debug)]
pub struct Worker {
    id: String,
    task_rx: mpsc::Receiver<Task>,
    limits: Arc<ResourceLimits>,
    metrics: WorkerMetrics,
}

impl Worker {
    pub fn new(
        id: String,
        task_rx: mpsc::Receiver<Task>,
        limits: Arc<ResourceLimits>,
    ) -> Self {
        Self {
            id,
            task_rx,
            limits,
            metrics: WorkerMetrics::new(&id),
        }
    }

    pub async fn run(mut self) -> Result<()> {
        while let Some(task) = self.task_rx.recv().await {
            self.metrics.tasks_started.increment(1);
            
            match self.execute_task(task).await {
                Ok(_) => self.metrics.tasks_completed.increment(1),
                Err(_) => self.metrics.tasks_failed.increment(1),
            }
        }
        Ok(())
    }

    async fn execute_task(&self, task: Task) -> Result<()> {
        let _guard = ResourceGuard::new(self.limits.clone());
        task.execute().await
    }
}

// Design Choice: Using RAII for resource management
struct ResourceGuard {
    limits: Arc<ResourceLimits>,
}

impl ResourceGuard {
    fn new(limits: Arc<ResourceLimits>) -> Self {
        Self { limits }
    }
}

// Design Choice: Using metrics for monitoring
#[derive(Debug)]
struct WorkerMetrics {
    tasks_started: Counter,
    tasks_completed: Counter,
    tasks_failed: Counter,
    active_tasks: Gauge,
}

impl WorkerMetrics {
    fn new(worker_id: &str) -> Self {
        Self {
            tasks_started: Counter::new(),
            tasks_completed: Counter::new(),
            tasks_failed: Counter::new(),
            active_tasks: Gauge::new(),
        }
    }
}

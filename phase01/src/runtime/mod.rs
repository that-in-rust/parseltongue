//! Runtime Management Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Runtime Orchestration
//! - RuntimeManager     (coordinates runtime components)
//!   ├── Lifecycle management
//!   ├── Resource coordination
//!   └── Global state handling
//! 
//! Level 3: Resource Management
//! - WorkerPoolManager (manages worker threads)
//!   ├── Thread pool administration
//!   ├── Load distribution
//!   └── Resource monitoring
//! 
//! Level 2: Task Management
//! - TaskScheduler     (schedules async tasks)
//!   ├── Priority handling
//!   ├── Backpressure control
//!   └── Performance tracking
//! 
//! Level 1 (Base): Core Runtime Types
//! - Configuration    (runtime settings)
//!   ├── Worker settings
//!   ├── Resource limits
//!   └── Shutdown parameters

// Design Choice: Using explicit module hierarchy
pub mod worker;
pub mod shutdown;
pub mod metrics;

use std::sync::Arc;
use tokio::sync::{Semaphore, broadcast, mpsc};
use tokio::task::JoinSet;
use metrics::{Counter, Gauge, Histogram};
use std::time::Duration;
use crate::core::{error::{Error, Result}, types::*};

// Design Choice: Using builder pattern for configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub worker_config: WorkerConfig,
    pub resource_limits: ResourceLimits,
    pub shutdown_config: ShutdownConfig,
}

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub thread_count: usize,
    pub queue_capacity: usize,
    pub stack_size: usize,
}

#[derive(Debug, Clone)]
pub struct ShutdownConfig {
    pub timeout: Duration,
    pub force_after_timeout: bool,
}

// Design Choice: Using channels for task communication
pub struct RuntimeManager {
    workers: Arc<WorkerPoolManager>,
    scheduler: Arc<TaskScheduler>,
    shutdown: broadcast::Sender<()>,
    metrics: RuntimeMetrics,
}

impl RuntimeManager {
    pub async fn new(config: RuntimeConfig) -> Result<Self> {
        let (shutdown_tx, _) = broadcast::channel(1);
        
        let workers = Arc::new(WorkerPoolManager::new(
            config.worker_config,
            config.resource_limits.clone(),
        )?);

        let scheduler = Arc::new(TaskScheduler::new(
            config.worker_config.queue_capacity,
        )?);

        Ok(Self {
            workers,
            scheduler,
            shutdown: shutdown_tx,
            metrics: RuntimeMetrics::new(),
        })
    }

    pub async fn spawn<F>(&self, priority: TaskPriority, future: F) -> Result<()>
    where
        F: Future<Output = Result<()>> + Send + 'static,
    {
        let task = Task {
            priority,
            future: Box::pin(future),
            metadata: TaskMetadata::new(),
        };

        self.scheduler.schedule(task).await
    }

    pub async fn shutdown(self) -> Result<()> {
        tracing::info!("Initiating graceful shutdown");
        let _ = self.shutdown.send(());
        
        self.workers.shutdown().await?;
        
        tracing::info!("Runtime shutdown complete");
        Ok(())
    }
}

// Design Choice: Using worker pool for task execution
struct WorkerPoolManager {
    workers: JoinSet<Result<()>>,
    capacity: Arc<Semaphore>,
    resources: Arc<ResourceMonitor>,
}

impl WorkerPoolManager {
    fn new(config: WorkerConfig, limits: ResourceLimits) -> Result<Self> {
        Ok(Self {
            workers: JoinSet::new(),
            capacity: Arc::new(Semaphore::new(config.thread_count)),
            resources: Arc::new(ResourceMonitor::new(limits)),
        })
    }

    async fn shutdown(&self) -> Result<()> {
        self.workers.shutdown().await;
        Ok(())
    }
}

// Design Choice: Using priority queue for tasks
struct TaskScheduler {
    queue: mpsc::Sender<Task>,
    metrics: TaskMetrics,
}

impl TaskScheduler {
    fn new(capacity: usize) -> Result<Self> {
        let (tx, _) = mpsc::channel(capacity);
        Ok(Self {
            queue: tx,
            metrics: TaskMetrics::new(),
        })
    }

    async fn schedule(&self, task: Task) -> Result<()> {
        self.queue.send(task).await.map_err(|_| Error::Shutdown)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_runtime_basic_operation() {
        let config = RuntimeConfig {
            worker_config: WorkerConfig {
                thread_count: 2,
                queue_capacity: 100,
                stack_size: 3 * 1024 * 1024,
            },
            resource_limits: ResourceLimits {
                max_tasks: 10,
                max_memory: 1024 * 1024 * 1024,
                max_connections: 5,
            },
            shutdown_config: ShutdownConfig {
                timeout: Duration::from_secs(5),
                force_after_timeout: true,
            },
        };

        let runtime = RuntimeManager::new(config).await.unwrap();

        runtime.spawn(TaskPriority::Normal, async {
            sleep(Duration::from_millis(100)).await;
            Ok(())
        }).await.unwrap();

        runtime.shutdown().await.unwrap();
    }
}

//! Worker Thread Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Worker Pool
//! - WorkerPool       (manages workers)
//!   ├── Load balancing
//!   ├── Worker lifecycle
//!   └── Pool metrics
//! 
//! Level 3: Worker Management
//! - Worker          (individual worker)
//!   ├── Task execution
//!   ├── State management
//!   └── Resource tracking
//! 
//! Level 2: Task Handling
//! - TaskExecutor    (executes tasks)
//!   ├── Priority handling
//!   ├── Error handling
//!   └── Metrics collection
//! 
//! Level 1 (Base): Core Types
//! - WorkerConfig    (configuration)
//! - WorkerState     (worker state)
//! - WorkerMetrics   (metrics)

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use futures::Future;
use metrics::{Counter, Gauge, Histogram};
use std::time::{Duration, Instant};
use crate::core::{error::{Error, Result}, types::*};
use tracing::{info, warn, error, Instrument};

// Design Choice: Using strong typing for worker configuration
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub id: String,
    pub queue_capacity: usize,
    pub task_timeout: Duration,
    pub metrics_enabled: bool,
}

// Design Choice: Using async traits for worker behavior
pub struct Worker {
    config: WorkerConfig,
    task_rx: mpsc::Receiver<TaskDefinition>,
    shutdown: broadcast::Receiver<()>,
    metrics: WorkerMetrics,
    state: Arc<WorkerState>,
}

impl Worker {
    pub fn new(config: WorkerConfig, shutdown: broadcast::Receiver<()>) -> Result<(Self, mpsc::Sender<TaskDefinition>)> {
        let (task_tx, task_rx) = mpsc::channel(config.queue_capacity);
        let metrics = WorkerMetrics::new(&config.id);
        let state = Arc::new(WorkerState::new());

        Ok((Self {
            config,
            task_rx,
            shutdown,
            metrics,
            state,
        }, task_tx))
    }

    pub async fn run(mut self) -> Result<()> {
        info!(worker.id = %self.config.id, "Worker starting");
        
        let mut shutdown = self.shutdown.resubscribe();
        
        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!(worker.id = %self.config.id, "Worker received shutdown signal");
                    break;
                }
                
                Some(task) = self.task_rx.recv() => {
                    self.process_task(task).await?;
                }
            }
        }

        self.shutdown_gracefully().await
    }

    async fn process_task(&mut self, task: TaskDefinition) -> Result<()> {
        let start = Instant::now();
        let task_id = task.id.clone();
        
        self.metrics.tasks_started.increment(1);
        self.state.active_tasks.fetch_add(1, Ordering::SeqCst);

        let result = tokio::time::timeout(
            self.config.task_timeout,
            task.future
        ).await;

        self.state.active_tasks.fetch_sub(1, Ordering::SeqCst);
        
        match result {
            Ok(Ok(_)) => {
                self.metrics.tasks_completed.increment(1);
                self.metrics.task_duration.record(start.elapsed());
                info!(worker.id = %self.config.id, task.id = %task_id, "Task completed successfully");
            }
            Ok(Err(e)) => {
                self.metrics.tasks_failed.increment(1);
                error!(worker.id = %self.config.id, task.id = %task_id, error = ?e, "Task failed");
            }
            Err(_) => {
                self.metrics.tasks_timeout.increment(1);
                warn!(worker.id = %self.config.id, task.id = %task_id, "Task timed out");
            }
        }

        Ok(())
    }

    async fn shutdown_gracefully(self) -> Result<()> {
        info!(worker.id = %self.config.id, "Worker shutting down gracefully");
        
        while self.state.active_tasks.load(Ordering::SeqCst) > 0 {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        info!(worker.id = %self.config.id, "Worker shutdown complete");
        Ok(())
    }
}

// Design Choice: Using atomic types for thread-safe state
#[derive(Debug)]
struct WorkerState {
    active_tasks: AtomicUsize,
    status: AtomicEnum<WorkerStatus>,
    last_heartbeat: AtomicTime,
}

#[derive(Debug)]
struct WorkerMetrics {
    tasks_started: Counter,
    tasks_completed: Counter,
    tasks_failed: Counter,
    tasks_timeout: Counter,
    task_duration: Histogram,
}

impl WorkerMetrics {
    fn new(worker_id: &str) -> Self {
        Self {
            tasks_started: Counter::new(),
            tasks_completed: Counter::new(),
            tasks_failed: Counter::new(),
            tasks_timeout: Counter::new(),
            task_duration: Histogram::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_worker_lifecycle() {
        let config = WorkerConfig {
            id: "test-worker".into(),
            queue_capacity: 10,
            task_timeout: Duration::from_secs(1),
            metrics_enabled: true,
        };

        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        let (worker, task_tx) = Worker::new(config, shutdown_rx).unwrap();
        
        let worker_handle = tokio::spawn(async move {
            worker.run().await
        });

        // Send test task
        task_tx.send(TaskDefinition {
            id: "test-task".into(),
            priority: TaskPriority::Normal,
            future: Box::pin(async { 
                sleep(Duration::from_millis(100)).await;
                Ok(())
            }),
            metadata: TaskMetadata::default(),
        }).await.unwrap();
        
        // Initiate shutdown
        sleep(Duration::from_millis(200)).await;
        shutdown_tx.send(()).unwrap();
        
        assert!(worker_handle.await.unwrap().is_ok());
    }
}

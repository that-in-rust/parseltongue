//! Worker Thread Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Worker Coordination
//! - WorkerGroup       (manages multiple workers)
//! - WorkerMetrics     (aggregates worker performance)
//! - LoadBalancer      (distributes tasks across workers)
//! 
//! Level 3: Task Management
//! - TaskQueue         (priority-based task queue)
//! - TaskDispatcher    (handles task distribution)
//! - BackpressureControl (manages worker load)
//! 
//! Level 2: Worker Implementation
//! - Worker           (individual worker implementation)
//! - WorkerState      (worker lifecycle management)
//! - TaskExecution    (task execution logic)
//! 
//! Level 1 (Base): Core Worker Types
//! - WorkerConfig     (worker configuration)
//! - TaskDefinition   (task representation)
//! - WorkerMetrics    (performance metrics)

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use futures::Future;
use metrics::{Counter, Gauge, Histogram};
use std::time::{Duration, Instant};
use crate::core::{error::{Error, Result}, types::*};
use tracing::{info, warn, error, Instrument};

// ===== Level 1: Core Worker Types =====
// Design Choice: Using strong typing for worker configuration

/// Individual worker configuration
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    /// Worker identifier
    pub id: String,
    /// Queue capacity
    pub queue_capacity: usize,
    /// Task execution timeout
    pub task_timeout: Duration,
    /// Metrics enabled
    pub metrics_enabled: bool,
}

/// Task definition with metadata
#[derive(Debug)]
pub struct TaskDefinition {
    /// Task identifier
    pub id: String,
    /// Task priority
    pub priority: TaskPriority,
    /// Task future
    pub future: Pin<Box<dyn Future<Output = Result<()>> + Send>>,
    /// Task metadata
    pub metadata: TaskMetadata,
}

// ===== Level 2: Worker Implementation =====
// Design Choice: Using async traits for worker behavior

/// Individual worker implementation
pub struct Worker {
    /// Worker configuration
    config: WorkerConfig,
    /// Task receiver
    task_rx: mpsc::Receiver<TaskDefinition>,
    /// Shutdown signal
    shutdown: broadcast::Receiver<()>,
    /// Worker metrics
    metrics: WorkerMetrics,
    /// Worker state
    state: Arc<WorkerState>,
}

/// Worker state management
#[derive(Debug)]
struct WorkerState {
    /// Active tasks counter
    active_tasks: AtomicUsize,
    /// Worker status
    status: AtomicEnum<WorkerStatus>,
    /// Last heartbeat
    last_heartbeat: AtomicTime,
}

// ===== Level 3: Task Management =====
// Design Choice: Using priority queue for task scheduling

/// Priority-based task queue
pub struct TaskQueue {
    /// High priority queue
    high: mpsc::Sender<TaskDefinition>,
    /// Normal priority queue
    normal: mpsc::Sender<TaskDefinition>,
    /// Low priority queue
    low: mpsc::Sender<TaskDefinition>,
    /// Queue metrics
    metrics: QueueMetrics,
}

/// Task dispatcher implementation
pub struct TaskDispatcher {
    /// Task queues
    queues: Arc<TaskQueue>,
    /// Worker pool
    workers: Arc<WorkerGroup>,
    /// Backpressure control
    backpressure: Arc<BackpressureControl>,
}

// ===== Level 4: Worker Coordination =====
// Design Choice: Using Arc for shared state

impl Worker {
    /// Creates a new worker with given configuration
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

    /// Starts the worker processing loop
    pub async fn run(mut self) -> Result<()> {
        info!(worker.id = %self.config.id, "Worker starting");
        
        let mut shutdown = self.shutdown.resubscribe();
        
        loop {
            tokio::select! {
                // Handle shutdown signal
                _ = shutdown.recv() => {
                    info!(worker.id = %self.config.id, "Worker received shutdown signal");
                    break;
                }
                
                // Process tasks
                Some(task) = self.task_rx.recv() => {
                    self.process_task(task).await?;
                }
            }
        }

        self.shutdown_gracefully().await
    }

    /// Processes a single task
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

    /// Performs graceful shutdown
    async fn shutdown_gracefully(self) -> Result<()> {
        info!(worker.id = %self.config.id, "Worker shutting down gracefully");
        
        // Wait for active tasks to complete
        while self.state.active_tasks.load(Ordering::SeqCst) > 0 {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        info!(worker.id = %self.config.id, "Worker shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_worker_basic_operation() {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        
        let config = WorkerConfig {
            id: "test-worker".into(),
            queue_capacity: 10,
            task_timeout: Duration::from_secs(1),
            metrics_enabled: true,
        };

        let (worker, task_tx) = Worker::new(config, shutdown_rx).unwrap();
        
        // Spawn worker
        let worker_handle = tokio::spawn(async move {
            worker.run().await
        });

        // Send a test task
        let task = TaskDefinition {
            id: "test-task".into(),
            priority: TaskPriority::Normal,
            future: Box::pin(async { 
                sleep(Duration::from_millis(100)).await;
                Ok(())
            }),
            metadata: TaskMetadata::default(),
        };

        task_tx.send(task).await.unwrap();
        
        // Initiate shutdown
        sleep(Duration::from_millis(200)).await;
        shutdown_tx.send(()).unwrap();
        
        // Worker should complete gracefully
        let result = worker_handle.await.unwrap();
        assert!(result.is_ok());
    }
}

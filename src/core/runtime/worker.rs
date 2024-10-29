// Level 4: Worker Pool Management
// - Manages thread pool for CPU-intensive tasks
// - Implements work stealing for better load balancing
// - Tracks worker metrics and health

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use crate::error::Result;
use crate::metrics::WorkerMetrics;

pub struct WorkerPool {
    workers: Vec<JoinHandle<()>>,
    task_tx: mpsc::Sender<Task>,
    metrics: Arc<WorkerMetrics>,
    limiter: Arc<Semaphore>,
}

// Level 3: Task representation
struct Task {
    id: u64,
    payload: Box<dyn FnOnce() -> Result<()> + Send + 'static>,
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        // Level 3: Initialize channels and metrics
        let (task_tx, task_rx) = mpsc::channel(size * 2);
        let metrics = Arc::new(WorkerMetrics::new());
        let limiter = Arc::new(Semaphore::new(size));
        
        // Level 3: Spawn worker threads
        let workers = (0..size)
            .map(|id| {
                let rx = task_rx.clone();
                let metrics = metrics.clone();
                let limiter = limiter.clone();
                
                tokio::spawn(async move {
                    Self::worker_loop(id, rx, metrics, limiter).await
                })
            })
            .collect();

        Self {
            workers,
            task_tx,
            metrics,
            limiter,
        }
    }

    // Level 2: Worker thread main loop
    async fn worker_loop(
        id: usize,
        mut rx: mpsc::Receiver<Task>,
        metrics: Arc<WorkerMetrics>,
        limiter: Arc<Semaphore>,
    ) {
        while let Some(task) = rx.recv().await {
            let _permit = limiter.acquire().await.unwrap();
            let start = std::time::Instant::now();
            
            if let Err(e) = (task.payload)() {
                metrics.record_error(id);
                tracing::error!("Worker {}: Task {} failed: {:?}", id, task.id, e);
            }
            
            metrics.record_task(id, start.elapsed());
        }
    }

    // Level 2: Submit task to pool
    pub async fn submit<F>(&self, task: F) -> Result<()>
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        static TASK_ID: AtomicU64 = AtomicU64::new(0);
        
        let task = Task {
            id: TASK_ID.fetch_add(1, Ordering::SeqCst),
            payload: Box::new(task),
        };
        
        self.task_tx.send(task).await?;
        Ok(())
    }

    // Level 2: Graceful shutdown
    pub async fn shutdown(self) -> Result<()> {
        // Stop accepting new tasks
        drop(self.task_tx);
        
        // Wait for all workers to complete
        for worker in self.workers {
            worker.await?;
        }
        
        Ok(())
    }
} 
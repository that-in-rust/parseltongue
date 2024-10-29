// Level 4: Worker Pool Management
// - Manages worker threads
// - Handles task distribution
// - Collects metrics

use crate::error::Result;
use crate::metrics::WorkerMetrics;
use tokio::sync::{mpsc, Semaphore};
use std::sync::Arc;
use std::time::Instant;
use rand::random;
use tokio::sync::mpsc::{self, Receiver};

pub struct Task {
    id: usize,
    payload: Box<dyn FnOnce() -> Result<()> + Send + 'static>,
}

pub struct WorkerPool {
    workers: Vec<tokio::task::JoinHandle<()>>,
    task_tx: mpsc::Sender<Task>,
    metrics: Arc<WorkerMetrics>,
    limiter: Arc<Semaphore>,
}

impl WorkerPool {
    pub fn new(num_workers: usize) -> Result<Self> {
        let (task_tx, task_rx) = mpsc::channel(32);
        let metrics = Arc::new(WorkerMetrics::new());
        let limiter = Arc::new(Semaphore::new(num_workers));
        let mut workers = Vec::new();

        for id in 0..num_workers {
            let rx = task_rx.clone();
            let metrics = metrics.clone();
            let limiter = limiter.clone();

            let handle = tokio::spawn(async move {
                while let Some(task) = rx.recv().await {
                    let _permit = limiter.acquire().await.unwrap();
                    let start = Instant::now();
                    
                    if let Err(e) = (task.payload)() {
                        tracing::error!("Task {} failed: {:?}", task.id, e);
                        metrics.record_error(id);
                    }
                    
                    metrics.record_task(id, start.elapsed());
                }
            });

            workers.push(handle);
        }

        Ok(WorkerPool {
            workers,
            task_tx,
            metrics,
            limiter,
        })
    }

    pub async fn spawn<F>(&self, task: F) -> Result<()>
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        let task = Task {
            id: rand::random(),
            payload: Box::new(task),
        };
        
        self.task_tx.send(task).await?;
        Ok(())
    }
} 
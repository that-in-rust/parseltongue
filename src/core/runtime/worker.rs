// Level 4: Worker Pool Management
// - Manages a pool of worker tasks for CPU-intensive operations
// - Implements task distribution and backpressure control
// - Tracks worker metrics

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use crate::error::Result;
use crate::metrics::WorkerMetrics;
use tracing::error;

pub struct WorkerPool {
    workers: Vec<JoinHandle<()>>,
    task_tx: mpsc::Sender<Task>,
    metrics: Arc<WorkerMetrics>,
    limiter: Arc<Semaphore>,
}

struct Task {
    id: u64,
    payload: Box<dyn FnOnce() + Send + 'static>,
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        let (task_tx, mut task_rx) = mpsc::channel::<Task>(size * 2);
        let metrics = Arc::new(WorkerMetrics::new());
        let limiter = Arc::new(Semaphore::new(size));

        let workers = (0..size)
            .map(|id| {
                let metrics = metrics.clone();
                let limiter = limiter.clone();
                let mut rx = task_rx.clone();

                tokio::spawn(async move {
                    while let Some(task) = rx.recv().await {
                        let _permit = limiter.acquire().await.unwrap();
                        let start = std::time::Instant::now();

                        (task.payload)();

                        metrics.record_task(id, start.elapsed());
                    }
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

    pub async fn submit<F>(&self, task: F) -> Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        static TASK_ID: AtomicU64 = AtomicU64::new(0);

        let task = Task {
            id: TASK_ID.fetch_add(1, Ordering::SeqCst),
            payload: Box::new(task),
        };

        self.task_tx.send(task).await?;
        Ok(())
    }

    pub async fn shutdown(self) -> Result<()> {
        drop(self.task_tx);

        for worker in self.workers {
            worker.await?;
        }

        Ok(())
    }
} 
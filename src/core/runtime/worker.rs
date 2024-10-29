// Level 4: Worker Pool Management
// - Manages a pool of worker tasks
// - Distributes tasks and collects metrics

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore, Mutex};
use tokio::task::JoinHandle;
use crate::error::Result;
use crate::metrics::WorkerMetrics;

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
    // Level 3: Create a new worker pool
    pub fn new(size: usize) -> Self {
        let (task_tx, task_rx) = mpsc::channel::<Task>(size * 2);
        let metrics = Arc::new(WorkerMetrics { /* ... */ });
        let limiter = Arc::new(Semaphore::new(size));

        let task_rx = Arc::new(Mutex::new(task_rx));

        let workers = (0..size)
            .map(|id| {
                let metrics = metrics.clone();
                let limiter = limiter.clone();
                let task_rx = task_rx.clone();

                tokio::spawn(async move {
                    loop {
                        let task = {
                            let mut rx = task_rx.lock().await;
                            rx.recv().await
                        };

                        if let Some(task) = task {
                            let _permit = limiter.acquire().await.unwrap();
                            let start = std::time::Instant::now();

                            (task.payload)();

                            metrics.record_task(id, start.elapsed());
                        } else {
                            break;
                        }
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

    // Level 2: Submit a task to the pool
    pub async fn submit<F>(&self, task: F) -> Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        static TASK_ID: AtomicU64 = AtomicU64::new(0);

        let task = Task {
            id: TASK_ID.fetch_add(1, Ordering::SeqCst),
            payload: Box::new(task),
        };

        self.task_tx.send(task).await.map_err(|e| e.into())
    }

    // Level 1: Shutdown the worker pool
    pub async fn shutdown(self) -> Result<()> {
        drop(self.task_tx);
        for worker in self.workers {
            worker.await?;
        }
        Ok(())
    }
} 
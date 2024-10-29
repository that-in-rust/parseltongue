//! Worker Pool Management - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Worker Configuration
//! Layer 3: Task Management
//! Layer 4: Resource Control
//! Layer 5: Metrics Integration

use std::sync::Arc;
use anyhow::{Context, Result};
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

use super::metrics::RuntimeMetrics;
use super::shutdown::ShutdownManager;

// Layer 1: Core Types
#[derive(Debug)]
pub struct WorkerPool {
    workers: Vec<JoinHandle<Result<()>>>,
    task_tx: mpsc::Sender<Task>,
    semaphore: Arc<Semaphore>,
    metrics: Arc<RuntimeMetrics>,
    shutdown: Arc<ShutdownManager>,
}

type Task = Box<dyn FnOnce() -> Result<()> + Send + 'static>;

// Layer 2: Implementation
impl WorkerPool {
    pub fn new(
        worker_count: usize,
        metrics: Arc<RuntimeMetrics>,
        shutdown: Arc<ShutdownManager>,
    ) -> Self {
        let (task_tx, task_rx) = mpsc::channel(32); // Bounded channel
        let semaphore = Arc::new(Semaphore::new(worker_count));
        let workers = Self::spawn_workers(
            worker_count,
            task_rx,
            Arc::clone(&metrics),
            Arc::clone(&shutdown),
        );

        Self {
            workers,
            task_tx,
            semaphore,
            metrics,
            shutdown,
        }
    }

    // Layer 3: Task Scheduling
    pub async fn spawn<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<T>> + Send + 'static,
        T: Send + 'static,
    {
        let _permit = self.semaphore.acquire().await
            .context("Failed to acquire worker permit")?;

        let (result_tx, mut result_rx) = mpsc::channel(1);
        let metrics = Arc::clone(&self.metrics);

        let task = Box::new(move || {
            let start = std::time::Instant::now();
            let result = tokio::runtime::Handle::current().block_on(f());
            metrics.record_task_completion(start.elapsed());
            let _ = result_tx.blocking_send(result);
            Ok(())
        });

        self.task_tx.send(task).await
            .context("Failed to send task to worker")?;

        result_rx.recv().await
            .context("Failed to receive task result")?
    }

    // Layer 4: Worker Management
    fn spawn_workers(
        count: usize,
        mut task_rx: mpsc::Receiver<Task>,
        metrics: Arc<RuntimeMetrics>,
        shutdown: Arc<ShutdownManager>,
    ) -> Vec<JoinHandle<Result<()>>> {
        (0..count).map(|id| {
            let metrics = Arc::clone(&metrics);
            let shutdown = Arc::clone(&shutdown);

            tokio::spawn(async move {
                debug!("Starting worker {}", id);
                metrics.record_worker_start().await?;

                while let Some(task) = task_rx.recv().await {
                    if let Err(e) = task() {
                        error!("Task error in worker {}: {}", id, e);
                        metrics.record_task_failure(&e.to_string()).await?;
                    }

                    if shutdown.is_shutting_down() {
                        break;
                    }
                }

                metrics.record_worker_stop().await?;
                debug!("Worker {} stopped", id);
                Ok(())
            })
        }).collect()
    }

    // Layer 5: Cleanup
    pub async fn shutdown(self) -> Result<()> {
        info!("Shutting down worker pool");
        
        // Drop sender to stop accepting new tasks
        drop(self.task_tx);

        // Wait for all workers to complete
        for handle in self.workers {
            handle.await??;
        }

        Ok(())
    }

    pub async fn active_count(&self) -> usize {
        self.semaphore.available_permits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_worker_pool() -> Result<()> {
        let metrics = Arc::new(RuntimeMetrics::new(2));
        let shutdown = Arc::new(ShutdownManager::new(super::super::RuntimeConfig {
            worker_threads: 2,
            shutdown_timeout: Duration::from_secs(1),
        }));

        let pool = WorkerPool::new(2, metrics, shutdown);

        let result = pool.spawn(|| async {
            sleep(Duration::from_millis(100)).await;
            Ok::<_, anyhow::Error>(42)
        }).await?;

        assert_eq!(result, 42);
        pool.shutdown().await?;

        Ok(())
    }
}

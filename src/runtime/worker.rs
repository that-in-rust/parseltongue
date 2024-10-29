//! Worker Management - Pyramidal Structure
//! Layer 1: Core Types & Traits
//! Layer 2: Worker Configuration
//! Layer 3: Worker Pool Management
//! Layer 4: Task Scheduling
//! Layer 5: Resource Management

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use anyhow::{Context, Result};
use tracing::{debug, error, info, warn};

use crate::error::ErrorExt;
use super::RuntimeConfig;

// Layer 1: Core Types
#[derive(Debug)]
pub struct WorkerPool {
    config: RuntimeConfig,
    semaphore: Arc<Semaphore>,
    tx: mpsc::Sender<Task>,
    rx: mpsc::Receiver<Task>,
    workers: Vec<JoinHandle<()>>,
}

type Task = Box<dyn FnOnce() -> Result<()> + Send + 'static>;

// Layer 2: Worker Implementation
impl WorkerPool {
    pub fn new(config: RuntimeConfig) -> Result<Self> {
        let (tx, rx) = mpsc::channel(config.worker_threads * 2);
        let semaphore = Arc::new(Semaphore::new(config.worker_threads));

        Ok(Self {
            config,
            semaphore,
            tx,
            rx,
            workers: Vec::new(),
        })
    }

    // Layer 3: Pool Management
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting worker pool with {} threads", self.config.worker_threads);

        for id in 0..self.config.worker_threads {
            let worker = self.spawn_worker(id).await?;
            self.workers.push(worker);
        }

        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down worker pool");
        
        // Drop sender to signal shutdown
        drop(self.tx.clone());

        for worker in self.workers.drain(..) {
            worker.await.context("Worker failed to shutdown")?;
        }

        Ok(())
    }

    // Layer 4: Task Management
    pub async fn spawn<F>(&self, task: F) -> Result<()>
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        let permit = self.semaphore.acquire()
            .await
            .context("Failed to acquire worker permit")?;

        self.tx.send(Box::new(move || {
            let result = task();
            drop(permit);
            result
        }))
        .await
        .context("Failed to schedule task")?;

        Ok(())
    }

    pub fn count(&self) -> usize {
        self.config.worker_threads
    }

    // Layer 5: Worker Creation
    async fn spawn_worker(&self, id: usize) -> Result<JoinHandle<()>> {
        let mut rx = self.rx.clone();
        let worker_id = id;

        let handle = tokio::spawn(async move {
            debug!("Worker {} started", worker_id);

            while let Some(task) = rx.recv().await {
                if let Err(e) = task() {
                    error!("Worker {} task failed: {}", worker_id, e);
                }
            }

            debug!("Worker {} shutting down", worker_id);
        });

        Ok(handle)
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        if !self.workers.is_empty() {
            warn!("WorkerPool dropped with active workers");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_worker_lifecycle() {
        let config = RuntimeConfig {
            worker_threads: 2,
            shutdown_timeout: Duration::from_secs(1),
        };

        let mut pool = WorkerPool::new(config).unwrap();
        pool.start().await.unwrap();

        // Test task execution
        pool.spawn(|| {
            sleep(Duration::from_millis(100));
            Ok(())
        })
        .await
        .unwrap();

        pool.shutdown().await.unwrap();
    }
}

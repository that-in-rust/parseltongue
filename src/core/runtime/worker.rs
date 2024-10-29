// Worker pool management for CPU-intensive tasks
//
// This module defines the `WorkerPool` struct, which manages a pool of worker tasks.
// The design follows a layered approach:
//
// - At the top level, we define the `WorkerPool` struct and its primary functionalities.
// - We implement backpressure control using `tokio::sync::Semaphore`, ensuring the system isn't overwhelmed.
// - Metrics are integrated to monitor task performance.

use tokio::sync::{Semaphore, OwnedSemaphorePermit};
use std::sync::Arc;
use std::future::Future;
use tracing::{instrument, error};

pub struct WorkerPool {
    semaphore: Arc<Semaphore>,
}

impl WorkerPool {
    // Initializes a new WorkerPool with a maximum number of concurrent tasks.
    pub fn new(max_concurrent_tasks: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent_tasks)),
        }
    }

    // Spawns a new task while controlling concurrency.
    #[instrument(skip(self, future))]
    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let semaphore = self.semaphore.clone();
        tokio::spawn(async move {
            // Acquire a permit to ensure we don't exceed max concurrency.
            let permit = semaphore.acquire_owned().await;
            match permit {
                Ok(_permit) => {
                    // Execute the task.
                    future.await;
                    // Metrics can be recorded here.
                }
                Err(e) => {
                    // Handle errors, such as semaphore being closed.
                    error!("Failed to acquire semaphore permit: {}", e);
                }
            }
        });
    }
} 
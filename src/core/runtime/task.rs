// Level 4: Task Management
// - Implements task scheduling
// - Handles task priorities
// - Manages cancellation
// - Provides metrics

use tokio::task::{JoinHandle, spawn};
use std::future::Future;
use crate::core::error::Result;

pub struct TaskManager {
    active_tasks: std::sync::atomic::AtomicUsize,
    max_concurrent: usize,
}

impl TaskManager {
    // Level 2: Task Operations
    pub async fn spawn<F, T>(&self, future: F) -> Result<T> 
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.active_tasks.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let result = spawn(future).await?;
        self.active_tasks.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        Ok(result)
    }
} 
// Level 4: Channel Management
// - Implements backpressure-aware channels
// - Manages async communication
// - Handles graceful shutdown
// - Provides metrics collection

use tokio::sync::{mpsc, oneshot};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::core::error::{Error, Result};

// Level 3: Channel Types
pub struct Channel<T> {
    sender: mpsc::Sender<T>,
    receiver: Arc<Mutex<mpsc::Receiver<T>>>,
    capacity: usize,
}

impl<T> Channel<T> {
    // Level 2: Channel Creation
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            capacity,
        }
    }

    // Level 1: Channel Operations
    pub async fn send(&self, value: T) -> Result<()> {
        self.sender.send(value).await
            .map_err(|_| Error::Channel("Send failed".into()))
    }

    pub async fn recv(&self) -> Result<Option<T>> {
        self.receiver.lock()
            .recv().await
            .transpose()
            .map_err(|_| Error::Channel("Receive failed".into()))
    }
} 
// Level 4: Channel Management
use tokio::sync::mpsc;
use std::sync::Arc;
use metrics::{counter, gauge};
use crate::core::error::Result;

pub struct Channel<T> {
    sender: mpsc::Sender<T>,
    receiver: mpsc::Receiver<T>,
    capacity: usize,
}

impl<T> Channel<T> {
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);
        gauge!("channel.capacity").set(capacity as f64);
        
        Self {
            sender,
            receiver,
            capacity,
        }
    }

    pub async fn send(&self, item: T) -> Result<()> {
        self.sender.send(item).await?;
        counter!("channel.messages").increment(1);
        Ok(())
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
} 
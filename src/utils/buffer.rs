// Level 4: Buffer Management
// - Manages memory buffers
// - Handles pooling
// - Provides metrics
// - Controls allocation

use bytes::BytesMut;
use std::sync::Arc;
use tokio::sync::Semaphore;
use metrics::{counter, gauge};
use crate::core::error::Result;

pub struct BufferPool {
    buffers: Vec<BytesMut>,
    semaphore: Arc<Semaphore>,
    buffer_size: usize,
}

impl BufferPool {
    pub fn new(capacity: usize, buffer_size: usize) -> Self {
        let buffers = (0..capacity)
            .map(|_| BytesMut::with_capacity(buffer_size))
            .collect();
            
        gauge!("buffer.pool.capacity").set(capacity as f64);
        gauge!("buffer.size").set(buffer_size as f64);
        
        Self {
            buffers,
            semaphore: Arc::new(Semaphore::new(capacity)),
            buffer_size,
        }
    }

    pub async fn acquire(&self) -> Result<BytesMut> {
        let _permit = self.semaphore.acquire().await?;
        let buffer = self.buffers.pop()
            .unwrap_or_else(|| BytesMut::with_capacity(self.buffer_size));
            
        counter!("buffer.acquired").increment(1);
        Ok(buffer)
    }

    pub fn release(&self, mut buffer: BytesMut) {
        buffer.clear();
        self.buffers.push(buffer);
        counter!("buffer.released").increment(1);
    }
} 
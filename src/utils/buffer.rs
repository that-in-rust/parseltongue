// Level 4: Buffer Management
// - Implements buffer pooling
// - Manages memory limits
// - Handles buffer recycling
// - Provides metrics collection

use bytes::{BytesMut, BufMut};
use std::sync::Arc;
use parking_lot::Mutex;

// Level 3: Buffer Pool
pub struct BufferPool {
    buffers: Arc<Mutex<Vec<BytesMut>>>,
    buffer_size: usize,
    max_buffers: usize,
}

impl BufferPool {
    // Level 2: Pool Operations
    pub fn new(buffer_size: usize, max_buffers: usize) -> Self {
        Self {
            buffers: Arc::new(Mutex::new(Vec::with_capacity(max_buffers))),
            buffer_size,
            max_buffers,
        }
    }

    // Level 1: Buffer Management
    pub fn acquire(&self) -> BytesMut {
        let mut buffers = self.buffers.lock();
        buffers.pop()
            .unwrap_or_else(|| BytesMut::with_capacity(self.buffer_size))
    }

    pub fn release(&self, mut buffer: BytesMut) {
        buffer.clear();
        let mut buffers = self.buffers.lock();
        if buffers.len() < self.max_buffers {
            buffers.push(buffer);
        }
    }
} 
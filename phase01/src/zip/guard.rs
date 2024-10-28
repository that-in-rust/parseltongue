use std::sync::Arc;
use tokio::sync::Semaphore;
use bytes::BytesMut;

/// RAII guard for ZIP entry buffers
pub struct BufferGuard {
    buffer: BytesMut,
    pool: Arc<Semaphore>,
}

impl BufferGuard {
    pub(crate) fn new(buffer: BytesMut, pool: Arc<Semaphore>) -> Self {
        Self { buffer, pool }
    }

    pub fn get_mut(&mut self) -> &mut BytesMut {
        &mut self.buffer
    }
}

impl Drop for BufferGuard {
    fn drop(&mut self) {
        self.pool.add_permits(1);
    }
}

/// RAII guard for ZIP entry processing
pub struct ProcessingGuard {
    semaphore: Arc<Semaphore>,
}

impl ProcessingGuard {
    pub(crate) fn new(semaphore: Arc<Semaphore>) -> Self {
        Self { semaphore }
    }
}

impl Drop for ProcessingGuard {
    fn drop(&mut self) {
        self.semaphore.add_permits(1);
    }
}


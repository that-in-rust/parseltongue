//! Prelude Module - Pyramidal Structure
//! Layer 1: Core Re-exports
//! Layer 2: Type Aliases
//! Layer 3: Common Traits
//! Layer 4: Error Types
//! Layer 5: Helper Functions

// Layer 1: Core Re-exports
pub use crate::{
    Processor,
    ProcessorConfig,
    ProcessingStats,
};

// Layer 2: Common Types
pub use crate::{
    zip::{ZipProcessor, ZipConfig, ZipEntry},
    storage::{StorageManager, StorageConfig},
    metrics::{MetricsManager, MetricsConfig},
    runtime::{RuntimeManager, RuntimeConfig},
};

// Layer 3: Error Types
pub use crate::error::{
    ProcessorError,
    ErrorContext,
    ErrorExt,
    Result,
};

// Layer 4: Common Traits
pub use tokio::{
    io::{AsyncRead, AsyncWrite, AsyncSeek},
    sync::Semaphore,
};

pub use futures::{Stream, StreamExt};
pub use bytes::{Bytes, BytesMut};

// Layer 5: Helper Types
pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;
pub type BoxedStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;

use std::pin::Pin;

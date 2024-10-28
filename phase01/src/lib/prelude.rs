// Common types and traits for public API
pub use crate::error::{Error, Result};
pub use crate::zip::{ZipProcessor, ZipConfig, ZipEntry};
pub use crate::storage::{StorageManager, StorageConfig};
pub use crate::metrics::{MetricsManager, MetricsConfig};

// Type aliases
pub type Buffer = bytes::Bytes;
pub type Path = std::path::PathBuf;

// Common traits
pub use async_trait::async_trait;
pub use futures::Stream;
pub use tokio::io::{AsyncRead, AsyncWrite, AsyncSeek};


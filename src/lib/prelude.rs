//! Common Types - Pyramidal Structure
//! Layer 1: Core Re-exports
//! Layer 2: Type Aliases
//! Layer 3: Common Traits
//! Layer 4: Helper Types
//! Layer 5: Constants

// Layer 1: Core Re-exports
pub use crate::error::{Error, Result, ErrorExt};
pub use crate::Config;

// Layer 2: Common Types
pub use std::path::{Path, PathBuf};
pub use std::time::Duration;
pub use tokio::sync::oneshot;

// Layer 3: Async Types
pub use tokio::io::{AsyncRead, AsyncWrite};
pub use tokio::sync::{mpsc, Mutex, RwLock};
pub use futures::Stream;

// Layer 4: Feature-gated Exports
#[cfg(feature = "metrics")]
pub use crate::metrics::{MetricsManager, TaskMetrics};

// Layer 5: Common Constants
pub use crate::{
    VERSION,
    MIN_RUST_VERSION,
    DEFAULT_BUFFER_SIZE,
    DEFAULT_SHUTDOWN_TIMEOUT,
};

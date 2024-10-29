//! Common Types Prelude - Pyramidal Structure
//! Layer 1: Core Re-exports
//! Layer 2: Type Aliases
//! Layer 3: Common Traits
//! Layer 4: Helper Types
//! Layer 5: Constants

// Layer 1: Core Re-exports
pub use crate::error::{Error, Result, ErrorExt};
pub use crate::Config;
pub use crate::storage::StorageManager;
pub use crate::zip::ZipProcessor;
pub use crate::runtime::RuntimeManager;

// Layer 2: Common Types
pub use std::path::{Path, PathBuf};
pub use std::time::Duration;
pub use tokio::sync::oneshot;

// Layer 3: Async Types
pub use tokio::io::{AsyncRead, AsyncWrite, AsyncSeek};
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

// Common Result/Option Extensions
pub trait ResultExt<T, E> {
    fn on_err<F>(self, f: F) -> Self
    where
        F: FnOnce(&E);
}

impl<T, E: std::fmt::Debug> ResultExt<T, E> for Result<T, E> {
    fn on_err<F>(self, f: F) -> Self
    where
        F: FnOnce(&E),
    {
        if let Err(ref e) = self {
            f(e);
        }
        self
    }
}

// Common Path Extensions
pub trait PathExt {
    fn ensure_dir_exists(&self) -> std::io::Result<()>;
    fn is_descendant_of(&self, ancestor: &Path) -> bool;
}

impl PathExt for Path {
    fn ensure_dir_exists(&self) -> std::io::Result<()> {
        if !self.exists() {
            std::fs::create_dir_all(self)?;
        }
        Ok(())
    }

    fn is_descendant_of(&self, ancestor: &Path) -> bool {
        self.ancestors().any(|p| p == ancestor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_path_extensions() -> std::io::Result<()> {
        let temp = TempDir::new()?;
        let path = temp.path().join("test/nested/dir");
        
        path.ensure_dir_exists()?;
        assert!(path.exists());
        assert!(path.is_descendant_of(temp.path()));
        
        Ok(())
    }

    #[test]
    fn test_result_extensions() {
        let mut called = false;
        let result: Result<(), &str> = Err("test error");
        
        result.on_err(|_| called = true);
        assert!(called);
    }
}

//! Parseltongue Library - Pyramidal Structure
//! Layer 1: Public Interface
//! Layer 2: Module Exports
//! Layer 3: Feature Configuration
//! Layer 4: Internal Organization
//! Layer 5: Documentation

// Layer 1: Core Re-exports
pub use crate::lib::{Config, ConfigBuilder};
pub use crate::error::{Error, Result};
pub use crate::storage::StorageManager;
pub use crate::zip::ZipProcessor;
pub use crate::runtime::RuntimeManager;

#[cfg(feature = "metrics")]
pub use crate::metrics::MetricsManager;

// Layer 2: Module Structure
pub mod error;
pub mod lib;
pub mod storage;
pub mod zip;
pub mod runtime;
pub mod metrics;

// Layer 3: Internal Modules
mod internal {
    pub(crate) mod validation;
    pub(crate) mod utils;
}

// Layer 4: Feature Gates
#[cfg(feature = "metrics")]
pub use metrics::TaskMetrics;

// Layer 5: Version Information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MIN_RUST_VERSION: &str = "1.70.0";

// Re-export common types in prelude
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::Config;
    pub use crate::storage::StorageManager;
    pub use crate::zip::ZipProcessor;
    pub use crate::runtime::RuntimeManager;
    
    #[cfg(feature = "metrics")]
    pub use crate::metrics::MetricsManager;
}

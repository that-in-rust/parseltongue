//! Public API Layer - Pyramidal Structure
//! Layer 1: Public Exports
//! Layer 2: Module Organization
//! Layer 3: Feature Gates
//! Layer 4: Error Handling
//! Layer 5: Resource Management

// Layer 1: Core Public Modules
pub mod lib;      // Public API Layer
pub mod main;     // CLI Layer
pub mod zip;      // ZIP Processing Layer
pub mod storage;  // Database Layer
pub mod runtime;  // Tokio Runtime Layer
pub mod metrics;  // Metrics Layer

// Layer 2: Public Re-exports
pub use lib::prelude::*;
pub use lib::error::Error;

// Layer 3: Type Definitions
pub type Result<T> = std::result::Result<T, Error>;

// Layer 4: Feature Gates
#[cfg(feature = "metrics")]
pub use metrics::{MetricsManager, ConsoleMetrics, TaskMetrics};

// Layer 5: Version Info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

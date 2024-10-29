// Level 4: Library Root
// - Defines public API
// - Re-exports key components
// - Organizes module structure

pub mod app;
pub mod cli;
pub mod config;
pub mod core;
pub mod error;
pub mod logging;
pub mod metrics;
pub mod output;
pub mod storage;
pub mod zip;

// Re-export commonly used types
pub use error::{Error, Result};
pub use config::Config;
pub use storage::Database;
pub use metrics::MetricsExporter;

// Feature-gated exports
#[cfg(feature = "metrics")]
pub use metrics::init as init_metrics;

#[cfg(feature = "console")]
pub use tokio::console_subscriber;
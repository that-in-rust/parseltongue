// Level 4: Library Entry Point
// - Manages module organization
// - Provides public API
// - Coordinates feature modules
// - Handles re-exports

pub mod core;
pub mod cli;
pub mod storage;  // Now only points to storage/mod.rs
pub mod metrics;  // Now only points to metrics/mod.rs
pub mod zip;
pub mod utils;
pub mod logging;

// Re-exports for public API
pub use crate::core::{error::Error, types::Config};
pub use crate::storage::db::Database;
pub use crate::zip::stream::ZipStream;
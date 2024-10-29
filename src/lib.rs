// Level 4: Library Interface
// - Public API definitions
// - Feature configuration
// - Module organization
// - Documentation

pub mod core;
pub mod cli;
pub mod storage;
pub mod zip;
pub mod utils;
pub mod metrics;

// Re-export commonly used types
pub use core::{error::Error, types::Config};
pub use storage::db::Database;
pub use zip::stream::ZipStream;
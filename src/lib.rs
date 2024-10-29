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

// Key Type Re-exports
pub use crate::error::{Error, Result};
pub use crate::config::Config;
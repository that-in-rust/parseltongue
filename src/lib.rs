// Level 4: Library Root
// - Defines public API
// - Re-exports key components
// - Organizes module structure

// Level 3: Module declarations
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

// Level 3: Key type re-exports
pub use error::{Error, Result};
pub use config::Config;

// Level 2: Public API
pub async fn process_zip_file(input: &str, output: &str) -> Result<()> {
    let config = Config::from_paths(input, output)?;
    app::run(config).await
} 
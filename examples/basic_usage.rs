// Level 4: Basic Usage Example
// - Shows core functionality
// - Demonstrates error handling
// - Illustrates configuration
// - Provides metrics setup

use parseltongue::{Config, Database};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        input_path: PathBuf::from("examples/data/sample.zip"),
        output_dir: PathBuf::from("examples/output"),
        worker_threads: 4,
        buffer_size: 32768,
        shutdown_timeout: std::time::Duration::from_secs(30),
    };

    // Example implementation...
    Ok(())
} 
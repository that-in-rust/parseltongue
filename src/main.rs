// Level 4: Application Entry Point
// - Parses command-line arguments
// - Initializes configuration and logging
// - Runs the application asynchronously

use parseltongue::{cli, config::Config, app, logging, error::Result};
use std::process;

#[tokio::main]
async fn main() {
    // Level 3: Parse command-line arguments
    let config = match cli::parse_args() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            process::exit(1);
        }
    };

    // Level 3: Initialize logging
    if let Err(e) = logging::init(&config) {
        eprintln!("Error initializing logging: {}", e);
        process::exit(1);
    }

    // Level 3: Run the application
    if let Err(e) = app::run(config).await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
} 
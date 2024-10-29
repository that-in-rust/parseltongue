// Level 4: Application Entry Point
// - Initializes logging and configuration
// - Starts Tokio runtime
// - Runs application logic asynchronously

use crate::cli;
use crate::config::Config;
use crate::app;
use crate::error::Result;
use crate::logging;

#[tokio::main]
async fn main() -> Result<()> {
    // Level 3: Parse command-line arguments
    let config = cli::parse_args()?;

    // Level 3: Initialize logging
    logging::init(&config)?;

    // Level 2: Run the main application logic
    app::run(config).await
} 
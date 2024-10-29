// Level 4: Main Application Logic Coordination
// - Sets up components and orchestrates processing
// - Handles graceful shutdown procedures
// - Coordinates ZIP processing and storage

use crate::config::Config;
use crate::zip_processor;
use crate::storage::Database;
use crate::output::OutputDirs;
use crate::metrics;
use crate::error::Result;
use crate::logging;

pub async fn run(config: Config) -> Result<()> {
    // Level 3: Create output directories
    let output_dirs = OutputDirs::create(&config.output_dir, &config.input_zip).await?;

    // Level 3: Initialize logging
    logging::init(&config, output_dirs.logs_path())?;

    // Level 3: Initialize metrics collection
    metrics::start_collection(&output_dirs).await;

    // Level 3: Initialize database
    let db = Database::open(&output_dirs.db_path()).await?;

    // Level 3: Process the ZIP file
    zip_processor::process_zip(&config, &db).await?;

    // Level 3: Perform graceful shutdown
    // Level 2: Flush and close the database
    db.close().await?;

    // Level 2: Shutdown metrics collection
    metrics::shutdown().await?;

    Ok(())
} 
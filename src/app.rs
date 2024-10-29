// Level 4: Main Application Logic Coordination
// - Initializes components
// - Coordinates processing
// - Handles shutdown and resource cleanup

use crate::config::Config;
use crate::logging;
use crate::metrics;
use crate::output::OutputDirs;
use crate::storage::Database;
use crate::zip::zip_processor;
use crate::error::Result;

pub async fn run(config: Config) -> Result<()> {
    // Level 3: Initialize output directories
    let output_dirs = OutputDirs::create(&config.output_dir, &config.input_zip).await?;

    // Level 3: Initialize logging
    logging::init(&config, output_dirs.logs_path())?;

    // Level 3: Start metrics collection
    metrics::start_collection(&output_dirs).await;

    // Level 3: Open database
    let db = Database::open(output_dirs.db_path()).await?;

    // Level 3: Process ZIP file
    zip_processor::process_zip(&config, &db).await?;

    // Level 3: Shutdown resources
    db.close().await?;
    metrics::shutdown().await;

    Ok(())
} 
// Asynchronous ZIP processing
//
// This module processes ZIP files, ensuring efficient resource utilization.
// Design highlights:
//
// - Uses `tokio::spawn` and `spawn_blocking` to handle async and blocking operations.
// - Implements backpressure using a semaphore to limit concurrent tasks.

use crate::config::Config;
use crate::storage::Database;
use crate::zip::entry_processor::process_entry;
use crate::error::Result;
use tokio::sync::Semaphore;
use tokio::task;
use std::sync::Arc;
use std::fs::File;
use zip::ZipArchive;

pub async fn process_zip(config: &Config, db: &Database) -> Result<()> {
    // Open the ZIP file in a blocking task.
    let zip_path = config.input_zip.clone();
    let file = task::spawn_blocking(move || File::open(zip_path)).await??;

    // Initialize the ZIP archive.
    let archive = task::spawn_blocking(|| ZipArchive::new(file)).await??;

    let semaphore = Arc::new(Semaphore::new(config.workers));

    // Process each entry with controlled concurrency.
    for i in 0..archive.len() {
        let entry = archive.by_index(i)?;
        let db = db.clone();
        let permit = semaphore.clone().acquire_owned().await?;
        tokio::spawn(async move {
            if let Err(e) = process_entry(entry, &db).await {
                // Log or handle the error.
            }
            drop(permit); // Release the permit.
        });
    }
    Ok(())
}
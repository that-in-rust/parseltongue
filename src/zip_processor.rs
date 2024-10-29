// Level 4: ZIP File Processing
// - Validates and opens the ZIP file
// - Streams entries asynchronously
// - Manages concurrency and backpressure

use crate::config::Config;
use crate::storage::Database;
use crate::zip::entry_processor::process_entry;
use crate::error::Result;
use tokio::sync::Semaphore;
use std::sync::Arc;
use tokio::task;
use std::fs::File;
use zip::ZipArchive;

pub async fn process_zip(config: &Config, db: &Database) -> Result<()> {
    let semaphore = Arc::new(Semaphore::new(config.workers));
    let db = Arc::new(db.clone());
    let input_zip = config.input_zip.clone();

    // Open ZIP archive in blocking task
    let archive = task::spawn_blocking(move || {
        let file = File::open(&input_zip)?;
        let archive = ZipArchive::new(file)?;
        Ok::<_, Error>(archive)
    })
    .await??;

    let num_files = archive.len();

    // Process entries
    for i in 0..num_files {
        let permit = semaphore.clone().acquire_owned().await?;
        let mut zip_file = archive.by_index(i)?;
        let db_clone = db.clone();

        task::spawn(async move {
            let _permit = permit;
            if let Err(e) = process_entry(zip_file, &db_clone).await {
                tracing::error!("Error processing entry: {:?}", e);
            }
        });
    }

    Ok(())
} 
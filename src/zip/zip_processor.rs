use crate::config::Config;
use crate::storage::Database;
use crate::zip::entry_processor::process_entry;
use crate::error::{Result, Error};
use tokio::sync::{Semaphore, Mutex};
use std::sync::Arc;
use tokio::task::spawn_blocking;
use zip::ZipArchive;
use std::fs::File;
use std::io::BufReader;

pub async fn process_zip(config: &Config, db: &Database) -> Result<()> {
    let semaphore = Arc::new(Semaphore::new(config.workers));
    let db = Arc::new(db.clone());
    let input_zip = config.input_zip.clone();

    // Level 3: Open ZIP archive in blocking task
    let archive = spawn_blocking(move || {
        let file = File::open(&input_zip)?;
        let reader = BufReader::new(file);
        let archive = ZipArchive::new(reader)?;
        Ok::<ZipArchive<BufReader<File>>, Error>(archive)
    })
    .await??;

    let num_files = archive.len();
    let archive = Arc::new(Mutex::new(archive));

    // Level 2: Process entries concurrently
    let mut handles = Vec::new();

    for i in 0..num_files {
        let permit = semaphore.clone().acquire_owned().await?;
        let db_clone = db.clone();
        let archive = archive.clone();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            let mut archive = archive.lock().await;
            let mut zip_file = archive.by_index(i).map_err(Error::from)?;

            if let Err(e) = process_entry(&mut zip_file, &db_clone).await {
                tracing::error!("Error processing entry: {:?}", e);
            }
            Ok::<(), Error>(())
        });

        handles.push(handle);
    }

    // Level 1: Await all tasks
    for handle in handles {
        handle.await??;
    }

    Ok(())
}
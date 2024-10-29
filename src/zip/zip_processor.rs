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

    let archive = spawn_blocking(move || {
        let file = File::open(&input_zip)?;
        let reader = BufReader::new(file);
        ZipArchive::new(reader)
    }).await??;

    let num_files = archive.len();
    let archive = Arc::new(tokio::sync::Mutex::new(archive));

    let mut handles = Vec::new();

    for i in 0..num_files {
        let permit = semaphore.clone().acquire_owned().await?;
        let db_clone = db.clone();
        let archive = archive.clone();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            let mut archive = archive.lock().await;
            let zip_file = archive.by_index(i)?;
            process_entry(zip_file, &db_clone).await
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    Ok(())
}
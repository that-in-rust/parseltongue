use crate::config::Config;
use crate::storage::Database;
use crate::zip::entry_processor::process_entry;
use crate::error::{Result, Error};
use tokio::sync::Semaphore;
use tokio::task;
use std::fs::File;
use std::io::SeekFrom;
use tokio_stream::wrappers::ReceiverStream;
use zip::ZipArchive;

pub async fn process_zip(config: &Config, db: &Database) -> Result<()> {
    // Level 3: Validate and open ZIP file
    let zip_path = config.input_zip.clone();
    let file = task::spawn_blocking(move || File::open(zip_path)).await??;

    // Level 3: Create ZipArchive
    let mut archive = task::spawn_blocking(|| ZipArchive::new(file)).await??;

    // Level 3: Set up concurrency controls
    let semaphore = Arc::new(Semaphore::new(config.workers));
    let (tx, rx) = tokio::sync::mpsc::channel(100);

    // Level 3: Spawn tasks for each entry
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i)?;
        let db_clone = db.clone();
        let permit = semaphore.clone().acquire_owned().await?;
        let tx = tx.clone();

        task::spawn(async move {
            let _permit = permit;
            if let Err(e) = process_entry(&mut zip_file, &db_clone).await {
                tracing::error!("Error processing entry: {:?}", e);
            }
            let _ = tx.send(()).await;
        });
    }

    // Level 3: Wait for all tasks to complete
    drop(tx);
    ReceiverStream::new(rx).collect::<Vec<_>>().await;

    Ok(())
}
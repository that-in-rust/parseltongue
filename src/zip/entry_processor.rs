// Level 4: ZIP Entry Processing
// - Processes individual ZIP entries
// - Handles data extraction and storage

use crate::storage::Database;
use crate::error::Result;
use zip::read::ZipFile;
use std::io::Read;
use tokio::task::spawn_blocking;

pub async fn process_entry<'a>(mut zip_file: ZipFile<'a>, db: &Database) -> Result<()> {
    let name = zip_file.name().to_string();
    
    // Read entry data in blocking task
    let data = spawn_blocking(move || {
        let mut data = Vec::new();
        zip_file.read_to_end(&mut data)?;
        Ok::<_, std::io::Error>(data)
    })
    .await??;

    // Store in database
    db.store(&name, &data).await?;

    Ok(())
} 
// Level 4: Individual ZIP Entry Processing
// - Reads entry data asynchronously
// - Performs CRC validation
// - Stores data in the database

use crate::storage::Database;
use crate::error::Result;
use tokio::task::spawn_blocking;
use zip::read::ZipFile;

pub async fn process_entry(zip_file: &mut ZipFile<'_>, db: &Database) -> Result<()> {
    // Read entry data in blocking task
    let data = spawn_blocking(move || {
        let mut data = Vec::new();
        zip_file.read_to_end(&mut data)?;
        Ok::<Vec<u8>, Error>(data)
    })
    .await??;

    // Store data in the database asynchronously
    let key = zip_file.name().to_string();
    db.store(&key, &data).await?;

    Ok(())
} 
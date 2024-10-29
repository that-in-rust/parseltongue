// Level 4: Individual ZIP Entry Processing
// - Reads and processes each entry
// - Stores the processed data in the database

use crate::storage::Database;
use crate::error::{Result, Error};
use zip::read::ZipFile;
use tokio::task::spawn_blocking;

pub async fn process_entry(zip_file: &mut ZipFile<'_>, db: &Database) -> Result<()> {
    // Level 3: Read entry data in blocking task
    let data = spawn_blocking(move || {
        let mut data = Vec::new();
        zip_file.read_to_end(&mut data)?;
        Ok::<Vec<u8>, Error>(data)
    })
    .await??;

    // Level 2: Store data in the database asynchronously
    let key = zip_file.name().to_string();
    db.store(&key, &data).await?;

    Ok(())
} 
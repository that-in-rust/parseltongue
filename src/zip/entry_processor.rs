// Individual ZIP entry processing
//
// This module handles the processing of a single ZIP entry.
// Key aspects:
//
// - Reads entry data in a blocking context to avoid blocking the async runtime.
// - Performs CRC32 validation before storing data.

use crate::storage::Database;
use crate::error::{Result, Error};
use tokio::task;
use crc32fast::Hasher;
use zip::read::ZipFile;

pub async fn process_entry(mut entry: ZipFile<'_>, db: &Database) -> Result<()> {
    // Read data in a blocking task.
    let data = task::spawn_blocking(move || {
        let mut buf = Vec::with_capacity(entry.size() as usize);
        std::io::copy(&mut entry, &mut buf)?;
        Ok(buf)
    }).await??;

    // Validate CRC32 checksum.
    let mut hasher = Hasher::new();
    hasher.update(&data);
    let checksum = hasher.finalize();
    if checksum != entry.crc32() {
        return Err(Error::CrcMismatch);
    }

    // Store the data asynchronously.
    db.put(entry.name().as_bytes().to_vec(), data).await?;

    Ok(())
} 
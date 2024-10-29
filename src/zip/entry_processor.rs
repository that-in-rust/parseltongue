// Level 4: ZIP Entry Processing
// - Handles individual ZIP file entries
// - Manages data extraction
// - Coordinates storage operations

use crate::storage::Database;
use crate::error::Result;
use std::io::Read;
use zip::read::ZipFile;

pub async fn process_entry(mut zip_file: ZipFile<'_>, db: &Database) -> Result<()> {
    let name = zip_file.name().to_string();
    let mut data = Vec::new();
    
    // Read data synchronously since ZipFile implements Read
    zip_file.read_to_end(&mut data)?;
    
    // Store data asynchronously
    db.store(&name, &data).await?;
    
    Ok(())
} 
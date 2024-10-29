// Level 4: ZIP Entry Processing
// - Handles individual ZIP entry extraction
// - Manages async I/O operations
// - Implements backpressure
// - Collects processing metrics

use tokio::task::spawn_blocking;
use std::io::Read;
use crate::core::error::{Error, Result};

pub async fn process_entry(mut zip_file: zip::read::ZipFile<'_>) -> Result<Vec<u8>> {
    // Level 3: Memory Management
    let mut data = Vec::with_capacity(zip_file.size() as usize);
    
    // Level 2: Async Processing
    spawn_blocking(move || {
        // Level 1: I/O Operations
        zip_file.read_to_end(&mut data)?;
        Ok::<Vec<u8>, Error>(data)
    })
    .await??
} 
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use tokio::sync::mpsc;
use zip::ZipArchive;

use crate::logging::ErrorLogger;

pub struct ZipEntry {
    pub name: String,
    pub content: Vec<u8>,
}

pub fn process_zip(
    mut zip: ZipArchive<File>,
    tx: mpsc::Sender<ZipEntry>,
    pb: Arc<ProgressBar>,
    error_logger: Arc<ErrorLogger>,
) -> Result<()> {
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).context("Failed to get ZIP entry")?;
        
        if file.is_dir() {
            warn!("Skipping directory entry: {}", file.name());
            continue;
        }

        let name = file.name().to_string();
        let mut content = Vec::new();
        file.read_to_end(&mut content).context("Failed to read ZIP entry content")?;

        tx.blocking_send(ZipEntry { name, content }).context("Failed to send ZIP entry")?;
        pb.inc(1);
    }
    Ok(())
}

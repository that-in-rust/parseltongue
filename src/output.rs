// Level 4: Output Directory Management
// - Creates and manages output directories
// - Provides paths to other components

use std::path::{Path, PathBuf};
use chrono::Utc;
use tokio::fs;
use crate::error::Result;

pub struct OutputDirs {
    analysis_dir: PathBuf,
    db_dir: PathBuf,
    logs_dir: PathBuf,
    metrics_dir: PathBuf,
}

impl OutputDirs {
    // Level 3: Create required directories asynchronously
    pub async fn create(base_dir: &Path, input_zip: &Path) -> Result<Self> {
        // Level 2: Generate timestamped directory
        let zip_name = input_zip.file_stem().unwrap().to_string_lossy();
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let analysis_dir = base_dir.join(format!("{}-{}", zip_name, timestamp));

        // Level 2: Create base analysis directory
        fs::create_dir_all(&analysis_dir).await?;

        // Level 2: Create subdirectories
        let db_dir = analysis_dir.join("db");
        let logs_dir = analysis_dir.join("logs");
        let metrics_dir = analysis_dir.join("metrics");

        fs::create_dir_all(&db_dir).await?;
        fs::create_dir_all(&logs_dir).await?;
        fs::create_dir_all(&metrics_dir).await?;

        Ok(OutputDirs {
            analysis_dir,
            db_dir,
            logs_dir,
            metrics_dir,
        })
    }

    // Level 3: Provide paths to other components
    pub fn db_path(&self) -> &Path {
        &self.db_dir
    }

    pub fn logs_path(&self) -> &Path {
        &self.logs_dir
    }

    pub fn metrics_path(&self) -> &Path {
        &self.metrics_dir
    }
} 
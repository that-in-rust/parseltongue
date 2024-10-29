// Level 4: Output Directory Management
// - Handles creation and organization of output directories
// - Provides paths for logs, metrics, and analysis results

use std::path::{Path, PathBuf};
use crate::config::Config;
use crate::error::Result;
use tokio::fs;

pub struct OutputDirs {
    pub analysis_dir: PathBuf,
    pub db_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub metrics_dir: PathBuf,
}

impl OutputDirs {
    // Level 3: Create necessary directories
    pub async fn create(base_dir: &Path, _input_zip: &Path) -> Result<Self> {
        let timestamp = chrono::Local::now().format("%Y%m%dT%H%M%S").to_string();
        let base = base_dir.join(format!("output_{}", timestamp));

        let analysis_dir = base.join("analysis");
        let db_dir = base.join("db");
        let logs_dir = base.join("logs");
        let metrics_dir = base.join("metrics");

        // Level 2: Create directories asynchronously
        fs::create_dir_all(&analysis_dir).await?;
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

    // Level 1: Provide utility methods for paths
    pub fn metrics_path(&self) -> &Path {
        &self.metrics_dir
    }

    pub fn logs_path(&self) -> &Path {
        &self.logs_dir
    }

    pub fn db_path(&self) -> &Path {
        &self.db_dir
    }
} 
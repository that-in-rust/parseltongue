//! Tool 5: CozoDB State Reset and Metadata Backup
//!
//! This tool provides simplified state reset functionality that re-triggers Tool 1
//! to re-ingest current file state and manages metadata backups via Git integration.
//!
//! Following TDD-first principle - tests first, implementation second

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tokio::fs;
use anyhow::Result;
use miette::{IntoDiagnostic, WrapErr};
use parseltongue_02::{
    storage::{CozoDBConnection, DatabaseStats},
    streamer::{FolderToCozoDBStreamer, StreamConfig},
};

pub mod error;
pub mod metadata_backup;
pub mod state_reset;
pub mod cli;

// Re-export key types for convenience
pub use error::{Tool5Error, Tool5Result};
pub use metadata_backup::{MetadataBackupManager, BackupManifest, BackupEntry};
pub use state_reset::{StateResetManager, ResetStats};
pub use cli::{Tool5Cli, Tool5Config};

/// Tool 5: CozoDB Make Future Code Current
///
/// Simplified state reset that re-triggers Tool 1 to re-ingest current file state
/// and manages metadata backups through Git-integrated backup system.
#[derive(Debug, Clone)]
pub struct CozoDBMakeFutureCodeCurrent {
    db_connection: CozoDBConnection,
    project_path: PathBuf,
    backup_manager: MetadataBackupManager,
    state_reset_manager: StateResetManager,
}

impl CozoDBMakeFutureCodeCurrent {
    /// Create a new Tool 5 instance
    pub fn new(project_path: PathBuf) -> Tool5Result<Self> {
        let db_connection = CozoDBConnection::new(Some(project_path.join("parseltongue.db")))
            .wrap_err("Failed to create CozoDB connection")?;

        let backup_manager = MetadataBackupManager::new(project_path.clone());
        let state_reset_manager = StateResetManager::new(db_connection.clone());

        Ok(Self {
            db_connection,
            project_path,
            backup_manager,
            state_reset_manager,
        })
    }

    /// Reset the database state after successful code changes
    ///
    /// This is the main operation that:
    /// 1. Backs up current metadata to timestamped MD files
    /// 2. Re-triggers Tool 1 to re-ingest current file state
    /// 3. Resets all current/future flags appropriately
    pub async fn reset_state(&self) -> Tool5Result<ResetStats> {
        println!("🔄 Starting state reset for project: {}", self.project_path.display());

        // Step 1: Backup current metadata
        println!("📋 Backing up current metadata...");
        let backup_path = self.backup_manager.backup_metadata().await
            .wrap_err("Failed to backup metadata")?;

        // Step 2: Re-trigger Tool 1 to re-ingest current state
        println!("🔄 Re-ingesting current file state...");
        let stream_config = StreamConfig::default();
        let streamer = FolderToCozoDBStreamer::new(stream_config);
        let ingest_result = streamer.process_folder().await
            .wrap_err("Failed to re-ingest files")?;

        // Step 3: Reset all flags to current state
        println!("🔄 Resetting database flags...");
        let reset_stats = self.state_reset_manager.reset_all_to_current().await
            .wrap_err("Failed to reset database flags")?;

        println!("✅ State reset completed successfully!");
        println!("📁 Backup stored at: {}", backup_path.display());
        println!("📊 Files processed: {}", ingest_result.files_processed);
        println!("📊 Chunks ingested: {}", ingest_result.chunks_processed);

        Ok(ResetStats {
            backup_path,
            files_processed: ingest_result.files_processed,
            chunks_processed: ingest_result.chunks_processed,
            relationships_processed: 0, // TODO: Get from actual result
            timestamp: Utc::now(),
        })
    }

    /// Get current database statistics
    pub async fn get_stats(&self) -> Tool5Result<DatabaseStats> {
        self.db_connection.get_stats().await
            .wrap_err("Failed to get database stats")
    }

    /// Get the project path
    pub fn project_path(&self) -> &Path {
        &self.project_path
    }
}

impl Default for CozoDBMakeFutureCodeCurrent {
    fn default() -> Self {
        Self::new(PathBuf::from(".")).expect("Failed to create Tool 5 with default path")
    }
}

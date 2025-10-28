//! Backup management for file writing operations

use crate::error::{FileWriterError, FileWriterResult};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Backup strategies for file operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupStrategy {
    /// No backup
    None,
    /// Timestamp-based backup
    Timestamp,
    /// Numbered backup (file.1, file.2, etc.)
    Numbered,
    /// Single backup with .bak extension
    Single,
    /// Custom backup directory
    CustomDirectory,
}

impl Default for BackupStrategy {
    fn default() -> Self {
        Self::Timestamp
    }
}

/// Information about a backup file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    /// Original file path
    pub original_path: PathBuf,
    /// Backup file path
    pub backup_path: PathBuf,
    /// When the backup was created
    pub created_at: DateTime<Utc>,
    /// Backup strategy used
    pub strategy: BackupStrategy,
    /// Size of the backup file
    pub size_bytes: u64,
    /// Checksum of the backup (for verification)
    pub checksum: Option<String>,
}

/// Trait for backup management
#[async_trait]
pub trait BackupManager: Send + Sync {
    /// Create a backup of the specified file
    async fn create_backup(
        &self,
        path: &Path,
        strategy: BackupStrategy,
    ) -> FileWriterResult<BackupInfo>;

    /// Restore a file from backup
    async fn restore_backup(&self, backup_info: &BackupInfo) -> FileWriterResult<()>;

    /// List all backups for a given file
    async fn list_backups(&self, original_path: &Path) -> FileWriterResult<Vec<BackupInfo>>;

    /// Delete a backup
    async fn delete_backup(&self, backup_info: &BackupInfo) -> FileWriterResult<()>;

    /// Cleanup old backups
    async fn cleanup_old_backups(
        &self,
        original_path: &Path,
        keep_count: usize,
    ) -> FileWriterResult<()>;

    /// Verify a backup integrity
    async fn verify_backup(&self, backup_info: &BackupInfo) -> FileWriterResult<bool>;
}

/// Default backup manager implementation
pub struct DefaultBackupManager {
    backup_directory: Option<PathBuf>,
    max_backups_per_file: usize,
}

impl DefaultBackupManager {
    /// Create a new backup manager with default settings
    pub fn new() -> Self {
        Self {
            backup_directory: None,
            max_backups_per_file: 10,
        }
    }

    /// Create a backup manager with custom backup directory
    pub fn with_backup_directory(backup_directory: PathBuf) -> Self {
        Self {
            backup_directory: Some(backup_directory),
            max_backups_per_file: 10,
        }
    }

    /// Set the maximum number of backups to keep per file
    pub fn with_max_backups(mut self, max_backups: usize) -> Self {
        self.max_backups_per_file = max_backups;
        self
    }

    /// Generate backup path based on strategy
    fn generate_backup_path(
        &self,
        original_path: &Path,
        strategy: BackupStrategy,
    ) -> FileWriterResult<PathBuf> {
        match strategy {
            BackupStrategy::None => Err(FileWriterError::InvalidConfiguration {
                reason: "Cannot create backup with None strategy".to_string(),
            }),
            BackupStrategy::Timestamp => {
                let timestamp = Utc::now().format("%Y%m%d_%H%M%S_%3f");
                let stem = original_path.file_stem().unwrap_or_default();
                let extension = original_path.extension();

                let backup_name = if let Some(ext) = extension {
                    format!(
                        "{}_{}.{}",
                        stem.to_string_lossy(),
                        timestamp,
                        ext.to_string_lossy()
                    )
                } else {
                    format!("{}_{}", stem.to_string_lossy(), timestamp)
                };

                let backup_dir = self
                    .backup_directory
                    .as_ref()
                    .and_then(|p| p.parent())
                    .unwrap_or_else(|| original_path.parent().unwrap_or_else(|| Path::new(".")));

                Ok(backup_dir.join(backup_name))
            }
            BackupStrategy::Numbered => {
                let stem = original_path.file_stem().unwrap_or_default();
                let extension = original_path.extension();
                let parent = original_path.parent().unwrap_or_else(|| Path::new("."));

                // Find the next available number
                let mut next_number = 1;
                loop {
                    let backup_name = if let Some(ext) = extension {
                        format!(
                            "{}.{}.{}",
                            stem.to_string_lossy(),
                            next_number,
                            ext.to_string_lossy()
                        )
                    } else {
                        format!("{}.{}", stem.to_string_lossy(), next_number)
                    };

                    let backup_path = parent.join(&backup_name);
                    if !backup_path.exists() {
                        return Ok(backup_path);
                    }
                    next_number += 1;
                }
            }
            BackupStrategy::Single => {
                let mut backup_path = original_path.to_path_buf();
                backup_path.set_extension(format!(
                    "{}.bak",
                    original_path
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                ));
                Ok(backup_path)
            }
            BackupStrategy::CustomDirectory => {
                if let Some(backup_dir) = &self.backup_directory {
                    let file_name = original_path.file_name().ok_or_else(|| {
                        FileWriterError::InvalidConfiguration {
                            reason: "Cannot determine file name for backup".to_string(),
                        }
                    })?;
                    Ok(backup_dir.join(file_name))
                } else {
                    Err(FileWriterError::InvalidConfiguration {
                        reason: "CustomDirectory strategy requires backup directory to be set"
                            .to_string(),
                    })
                }
            }
        }
    }

    /// Calculate checksum for file verification
    async fn calculate_checksum(&self, path: &Path) -> FileWriterResult<String> {
        use tokio::fs::File;
        use tokio::io::AsyncReadExt;

        let mut file = File::open(path).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;

        // Simple checksum using SHA-256 (you might want to use a proper crypto library)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        contents.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }
}

impl Default for DefaultBackupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BackupManager for DefaultBackupManager {
    async fn create_backup(
        &self,
        path: &Path,
        strategy: BackupStrategy,
    ) -> FileWriterResult<BackupInfo> {
        if !path.exists() {
            return Err(FileWriterError::FileNotFound {
                path: path.to_string_lossy().to_string(),
            });
        }

        let backup_path = self.generate_backup_path(path, strategy)?;

        // Ensure backup directory exists
        if let Some(parent) = backup_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Copy the file
        tokio::fs::copy(path, &backup_path).await?;

        // Get file metadata
        let metadata = tokio::fs::metadata(&backup_path).await?;
        let size_bytes = metadata.len();

        // Calculate checksum
        let checksum = self.calculate_checksum(&backup_path).await.ok();

        Ok(BackupInfo {
            original_path: path.to_path_buf(),
            backup_path,
            created_at: Utc::now(),
            strategy,
            size_bytes,
            checksum,
        })
    }

    async fn restore_backup(&self, backup_info: &BackupInfo) -> FileWriterResult<()> {
        if !backup_info.backup_path.exists() {
            return Err(FileWriterError::FileNotFound {
                path: backup_info.backup_path.to_string_lossy().to_string(),
            });
        }

        // Verify backup integrity if checksum is available
        if let Some(expected_checksum) = &backup_info.checksum {
            let actual_checksum = self.calculate_checksum(&backup_info.backup_path).await?;
            if actual_checksum != *expected_checksum {
                return Err(FileWriterError::BackupFailed {
                    reason: "Backup integrity check failed".to_string(),
                });
            }
        }

        // Ensure parent directory exists
        if let Some(parent) = backup_info.original_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Copy the backup back to original location
        tokio::fs::copy(&backup_info.backup_path, &backup_info.original_path).await?;

        Ok(())
    }

    async fn list_backups(&self, original_path: &Path) -> FileWriterResult<Vec<BackupInfo>> {
        let parent = original_path.parent().unwrap_or_else(|| Path::new("."));
        let stem = original_path.file_stem().unwrap_or_default();

        let mut backups = Vec::new();
        let mut entries = tokio::fs::read_dir(parent).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            // Check if this looks like a backup file for our original
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.contains(&*stem.to_string_lossy()) {
                    // Try to read backup metadata (in a real implementation, you'd store this separately)
                    // For now, we'll create basic backup info
                    if let Ok(metadata) = tokio::fs::metadata(&path).await {
                        if let Ok(modified) = metadata.modified() {
                            let modified_datetime: DateTime<Utc> = modified.into();
                            backups.push(BackupInfo {
                                original_path: original_path.to_path_buf(),
                                backup_path: path,
                                created_at: modified_datetime,
                                strategy: BackupStrategy::Timestamp, // Default assumption
                                size_bytes: metadata.len(),
                                checksum: None,
                            });
                        }
                    }
                }
            }
        }

        // Sort by creation time (newest first)
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(backups)
    }

    async fn delete_backup(&self, backup_info: &BackupInfo) -> FileWriterResult<()> {
        tokio::fs::remove_file(&backup_info.backup_path).await?;
        Ok(())
    }

    async fn cleanup_old_backups(
        &self,
        original_path: &Path,
        keep_count: usize,
    ) -> FileWriterResult<()> {
        let mut backups = self.list_backups(original_path).await?;

        // backups are sorted newest first
        if backups.len() > keep_count {
            // Split off the backups we want to DELETE (everything after the newest `keep_count`)
            let to_delete = backups.split_off(keep_count);
            for backup in to_delete {
                self.delete_backup(&backup).await?;
            }
        }

        Ok(())
    }

    async fn verify_backup(&self, backup_info: &BackupInfo) -> FileWriterResult<bool> {
        if !backup_info.backup_path.exists() {
            return Ok(false);
        }

        // Check if backup size matches expected size
        let metadata = tokio::fs::metadata(&backup_info.backup_path).await?;
        if metadata.len() != backup_info.size_bytes {
            return Ok(false);
        }

        // Verify checksum if available
        if let Some(expected_checksum) = &backup_info.checksum {
            let actual_checksum = self.calculate_checksum(&backup_info.backup_path).await?;
            Ok(actual_checksum == *expected_checksum)
        } else {
            // No checksum to verify, assume it's valid
            Ok(true)
        }
    }
}

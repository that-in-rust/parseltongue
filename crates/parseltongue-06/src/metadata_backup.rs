//! Metadata backup management for Tool 5
//!
//! Provides Git-integrated backup functionality that stores metadata in timestamped
//! markdown files before state reset operations.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tokio::fs;
use anyhow::Result;

use super::error::{Tool5Error, Tool5Result};

/// Metadata backup entry for a chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEntry {
    pub id: String,
    pub file_path: String,
    pub chunk_type: String,
    pub start_line: usize,
    pub end_line: usize,
    pub interface_name: String,
    pub metadata: serde_json::Value,
    pub tdd_classification: Option<String>,
    pub interface_signature: Option<String>,
    pub lsp_metadata: Option<serde_json::Value>,
}

/// Backup manifest containing summary information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    pub timestamp: DateTime<Utc>,
    pub backup_id: String,
    pub project_path: String,
    pub total_chunks: usize,
    pub total_relationships: usize,
    pub backup_reason: String,
}

/// Metadata backup manager
#[derive(Debug, Clone)]
pub struct MetadataBackupManager {
    project_path: PathBuf,
    backup_dir: PathBuf,
}

impl MetadataBackupManager {
    /// Create a new metadata backup manager
    pub fn new(project_path: PathBuf) -> Self {
        let backup_dir = project_path.join(".parseltongue").join("metadata-backups");

        Self {
            project_path,
            backup_dir,
        }
    }

    /// Backup current metadata to timestamped MD files
    pub async fn backup_metadata(&self) -> Tool5Result<PathBuf> {
        // Create backup directory with timestamp
        let timestamp = Utc::now();
        let backup_id = timestamp.format("%Y-%m-%d-%H-%M-%S").to_string();
        let backup_path = self.backup_dir.join(&backup_id);

        // Create backup directory
        fs::create_dir_all(&backup_path).await
            .map_err(|e| Tool5Error::file_system(format!("Failed to create backup directory: {}", e)))?;

        // TODO: Extract actual metadata from CozoDB
        // For now, create placeholder backup
        self.create_placeholder_backup(&backup_path, &backup_id, &timestamp).await?;

        // Create manifest file
        self.create_backup_manifest(&backup_path, &backup_id, &timestamp).await?;

        // Add .gitkeep to ensure directory is tracked
        let gitkeep_path = backup_path.join(".gitkeep");
        fs::write(&gitkeep_path, "").await
            .map_err(|e| Tool5Error::file_system(format!("Failed to create .gitkeep: {}", e)))?;

        println!("📁 Metadata backup created at: {}", backup_path.display());

        Ok(backup_path)
    }

    /// Create placeholder backup for development
    async fn create_placeholder_backup(
        &self,
        backup_path: &Path,
        backup_id: &str,
        timestamp: &DateTime<Utc>,
    ) -> Tool5Result<()> {
        // Create a sample backup entry
        let sample_entry = BackupEntry {
            id: "sample-chunk-123".to_string(),
            file_path: "src/main.rs".to_string(),
            chunk_type: "Function".to_string(),
            start_line: 1,
            end_line: 10,
            interface_name: "main".to_string(),
            metadata: serde_json::json!({
                "description": "Sample backup entry - TODO: implement real metadata extraction"
            }),
            tdd_classification: Some("CODE_IMPLEMENTATION".to_string()),
            interface_signature: Some("sample-signature".to_string()),
            lsp_metadata: Some(serde_json::json!({
                "sample": "lsp_data"
            })),
        };

        // Write backup entry to MD file
        let md_content = format!(
            r#"# Metadata Backup: {}

## Chunk: {}
- **File**: {}
- **Type**: {}
- **Lines**: {}-{}
- **Interface Name**: {}
- **TDD Classification**: {}
- **Interface Signature**: {}

## Metadata
```json
{}
```

## LSP Metadata
```json
{}
```

---
*Backup created on: {}*
*Backup ID: {}*
"#,
            backup_id,
            sample_entry.id,
            sample_entry.file_path,
            sample_entry.start_line,
            sample_entry.end_line,
            sample_entry.interface_name,
            sample_entry.tdd_classification.unwrap_or_default(),
            sample_entry.interface_signature.unwrap_or_default(),
            serde_json::to_string_pretty(&sample_entry).unwrap_or_default(),
            serde_json::to_string_pretty(&sample_entry.lsp_metadata.unwrap_or_default()).unwrap_or_default(),
            timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            backup_id
        );

        let backup_file = backup_path.join("metadata-chunks.md");
        fs::write(&backup_file, md_content).await
            .map_err(|e| Tool5Error::file_system(format!("Failed to write backup file: {}", e)))?;

        Ok(())
    }

    /// Create backup manifest
    async fn create_backup_manifest(
        &self,
        backup_path: &Path,
        backup_id: &str,
        timestamp: &DateTime<Utc>,
    ) -> Tool5Result<()> {
        let manifest = BackupManifest {
            timestamp: timestamp.clone(),
            backup_id: backup_id.to_string(),
            project_path: self.project_path.to_string_lossy().to_string(),
            total_chunks: 1, // TODO: Get actual count
            total_relationships: 0, // TODO: Get actual count
            backup_reason: "State reset after successful code changes".to_string(),
        };

        let manifest_content = format!(
            r#"# Backup Manifest

## Information
- **Backup ID**: {}
- **Timestamp**: {}
- **Project Path**: {}
- **Total Chunks**: {}
- **Total Relationships**: {}
- **Backup Reason**: {}

## Files
- `metadata-chunks.md` - Chunk metadata backup
- `metadata-relationships.md` - Relationship metadata backup
- `.gitkeep` - Ensures directory is tracked in Git

## Notes
This backup was created automatically by Tool 5 before resetting the CozoDB state.
The metadata can be restored if needed for debugging or analysis purposes.
"#,
            manifest.backup_id,
            manifest.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            manifest.project_path,
            manifest.total_chunks,
            manifest.total_relationships,
            manifest.backup_reason
        );

        let manifest_file = backup_path.join("backup-manifest.md");
        fs::write(&manifest_file, manifest_content).await
            .map_err(|e| Tool5Error::file_system(format!("Failed to write manifest file: {}", e)))?;

        Ok(())
    }

    /// Get backup directory path
    pub fn backup_dir(&self) -> &Path {
        &self.backup_dir
    }

    /// List all available backups
    pub async fn list_backups(&self) -> Tool5Result<Vec<PathBuf>> {
        if !self.backup_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();
        let mut entries = fs::read_dir(&self.backup_dir).await
            .map_err(|e| Tool5Error::file_system(format!("Failed to read backup directory: {}", e)))?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| Tool5Error::file_system(format!("Failed to read directory entry: {}", e)))? {
            let path = entry.path();
            if path.is_dir() {
                // Check if it's a valid backup (has manifest)
                let manifest_path = path.join("backup-manifest.md");
                if manifest_path.exists() {
                    backups.push(path);
                }
            }
        }

        // Sort by timestamp (newest first)
        backups.sort_by(|a, b| b.cmp(a));

        Ok(backups)
    }

    /// Get project path
    pub fn project_path(&self) -> &Path {
        &self.project_path
    }
}
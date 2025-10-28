//! Core file writer trait and related types

use crate::backup::BackupStrategy;
use crate::error::FileWriterResult;
use crate::report::{FileOperation, FileWriteReport, OperationResult};
use crate::safety::{WriteSafetyCheck, WriteSafetyLevel};
use async_trait::async_trait;
use std::path::Path;

/// Configuration for file writing operations
#[derive(Debug, Clone)]
pub struct FileWriterConfig {
    /// Safety level for operations
    pub safety_level: WriteSafetyLevel,
    /// Whether to create backups before modifications
    pub create_backups: bool,
    /// Backup strategy to use
    pub backup_strategy: BackupStrategy,
    /// Maximum file size to process (in bytes)
    pub max_file_size: Option<u64>,
    /// Timeout for operations (in milliseconds)
    pub timeout_ms: Option<u64>,
    /// Whether to verify writes after completion
    pub verify_writes: bool,
    /// Maximum number of concurrent operations
    pub max_concurrent_ops: Option<usize>,
}

impl Default for FileWriterConfig {
    fn default() -> Self {
        Self {
            safety_level: WriteSafetyLevel::Standard,
            create_backups: true,
            backup_strategy: BackupStrategy::Timestamp,
            max_file_size: Some(100 * 1024 * 1024), // 100MB
            timeout_ms: Some(30_000),               // 30 seconds
            verify_writes: true,
            max_concurrent_ops: Some(10),
        }
    }
}

/// Input for file writing operations
#[derive(Debug, Clone)]
pub struct FileWriteInput {
    /// Path to the file to write
    pub path: String,
    /// Content to write
    pub content: Vec<u8>,
    /// Operation type
    pub operation_type: FileOperation,
    /// Optional validation results from Tool 3
    pub validation_results: Option<crate::tool3_integration::ValidationResult>,
}

/// Core file writer trait
#[async_trait]
pub trait FileWriter: Send + Sync {
    /// Write a single file with safety checks and backup
    async fn write_file(&self, input: FileWriteInput) -> FileWriterResult<OperationResult>;

    /// Write multiple files in a batch
    async fn write_files(&self, inputs: Vec<FileWriteInput>) -> FileWriterResult<FileWriteReport>;

    /// Perform safety checks before writing
    async fn check_safety(&self, input: &FileWriteInput) -> FileWriterResult<WriteSafetyCheck>;

    /// Create a backup of a file
    async fn create_backup(
        &self,
        path: &Path,
    ) -> FileWriterResult<Option<crate::backup::BackupInfo>>;

    /// Restore a file from backup
    async fn restore_from_backup(
        &self,
        original_path: &Path,
        backup_path: &Path,
    ) -> FileWriterResult<()>;

    /// Rollback a failed operation
    async fn rollback_operation(&self, report: &FileWriteReport) -> FileWriterResult<()>;

    /// Get the current configuration
    fn config(&self) -> &FileWriterConfig;

    /// Update the configuration
    fn update_config(&mut self, config: FileWriterConfig);
}

/// Batch file writing options
#[derive(Debug, Clone)]
pub struct BatchWriteOptions {
    /// Whether to continue on individual file failures
    pub continue_on_error: bool,
    /// Maximum number of concurrent operations
    pub max_concurrent: Option<usize>,
    /// Delay between operations (in milliseconds)
    pub delay_between_ops: Option<u64>,
    /// Whether to generate a detailed report
    pub generate_detailed_report: bool,
}

impl Default for BatchWriteOptions {
    fn default() -> Self {
        Self {
            continue_on_error: true,
            max_concurrent: Some(5),
            delay_between_ops: None,
            generate_detailed_report: true,
        }
    }
}

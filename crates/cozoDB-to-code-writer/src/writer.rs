//! Default file writer implementation

use crate::backup::{BackupManager, DefaultBackupManager};
use crate::error::{FileWriterError, FileWriterResult};
use crate::file_writer::{BatchWriteOptions, FileWriteInput, FileWriter, FileWriterConfig};
use crate::report::{FileOperation, FileWriteReport, OperationResult};
use crate::safety::{DefaultSafetyChecker, SafetyChecker, SafetyIssueType, WriteSafetyCheck};
use async_trait::async_trait;
use std::path::Path;
use std::time::Instant;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Default file writer implementation
pub struct DefaultFileWriter {
    config: FileWriterConfig,
    backup_manager: Box<dyn BackupManager + Send + Sync>,
    safety_checker: Box<dyn SafetyChecker + Send + Sync>,
}

impl DefaultFileWriter {
    /// Create a new file writer with default configuration
    pub fn new() -> Self {
        Self {
            config: FileWriterConfig::default(),
            backup_manager: Box::new(DefaultBackupManager::new()),
            safety_checker: Box::new(DefaultSafetyChecker::new()),
        }
    }

    /// Create a file writer with custom configuration
    pub fn with_config(config: FileWriterConfig) -> Self {
        Self {
            config,
            backup_manager: Box::new(DefaultBackupManager::new()),
            safety_checker: Box::new(DefaultSafetyChecker::new()),
        }
    }

    /// Create a file writer with custom components
    pub fn with_components(
        config: FileWriterConfig,
        backup_manager: Box<dyn BackupManager + Send + Sync>,
        safety_checker: Box<dyn SafetyChecker + Send + Sync>,
    ) -> Self {
        Self {
            config,
            backup_manager,
            safety_checker,
        }
    }

    /// Write content to a file with atomic operations
    async fn write_file_atomic(&self, path: &Path, content: &[u8]) -> FileWriterResult<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write to temporary file first
        let temp_path = path.with_extension(format!("tmp.{}", uuid::Uuid::new_v4()));

        {
            let mut file = fs::File::create(&temp_path).await?;
            file.write_all(content).await?;
            file.flush().await?;
            file.sync_all().await?;
        }

        // Verify write if configured
        if self.config.verify_writes {
            let written_content = fs::read(&temp_path).await?;
            if written_content != content {
                fs::remove_file(&temp_path).await?;
                return Err(FileWriterError::BackupFailed {
                    reason: "Write verification failed".to_string(),
                });
            }
        }

        // Atomic rename
        fs::rename(&temp_path, path).await?;

        Ok(())
    }
}

impl Default for DefaultFileWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultFileWriter {
    /// Safety check that considers the operation type
    async fn check_safety_with_operation(
        &self,
        input: &FileWriteInput,
        operation_type: FileOperation,
    ) -> FileWriterResult<WriteSafetyCheck> {
        let path = Path::new(&input.path);

        // Use configured safety level, but adjust based on validation if present
        let safety_level = if let Some(validation) = &input.validation_results {
            validation
                .recommended_safety_level
                .max(self.config.safety_level)
        } else {
            self.config.safety_level
        };

        // Update safety checker with the determined level
        let checker = DefaultSafetyChecker::with_config(
            safety_level,
            self.config.max_file_size.unwrap_or(100 * 1024 * 1024),
            1024 * 1024 * 1024, // 1GB minimum disk space
        );

        // For Create operations, we expect the file not to exist
        let mut safety_check = checker.check_file_safety(path)?;

        if operation_type == FileOperation::Create {
            // Remove FileNotFound issues for Create operations since we expect the file not to exist
            safety_check
                .issues
                .retain(|issue| issue.issue_type != SafetyIssueType::FileNotFound);

            // Re-calculate safety after removing FileNotFound issues
            if safety_check.issues.is_empty() {
                safety_check.is_safe = true;
                safety_check.risk_score = safety_check.risk_score.min(0.3); // Low risk for creating new files
            }
        }

        Ok(safety_check)
    }
}

#[async_trait]
impl FileWriter for DefaultFileWriter {
    async fn write_file(&self, input: FileWriteInput) -> FileWriterResult<OperationResult> {
        let start_time = Instant::now();
        let path = Path::new(&input.path);

        // Perform safety checks
        let safety_check = self
            .check_safety_with_operation(&input, input.operation_type)
            .await?;
        if !safety_check.is_safe_to_proceed() {
            return Ok(OperationResult::Skipped {
                original_path: path.to_path_buf(),
                reason: format!("Safety check failed: {:?}", safety_check.issues),
            });
        }

        // Check validation results if present
        if let Some(validation) = &input.validation_results {
            if !validation.should_write {
                return Ok(OperationResult::Skipped {
                    original_path: path.to_path_buf(),
                    reason: "Validation indicates file should not be written".to_string(),
                });
            }
        }

        // Create backup if configured and file exists
        let backup_path = if self.config.create_backups && path.exists() {
            let backup_info =
                self.create_backup(path)
                    .await
                    .map_err(|e| FileWriterError::BackupFailed {
                        reason: format!("Failed to create backup: {}", e),
                    })?;
            backup_info.map(|info| info.backup_path.clone())
        } else {
            None
        };

        // Perform the write operation
        let write_result = match input.operation_type {
            FileOperation::Create | FileOperation::Modify => {
                self.write_file_atomic(path, &input.content).await
            }
            FileOperation::Delete => {
                if path.exists() {
                    fs::remove_file(path).await.map_err(Into::into)
                } else {
                    Ok(())
                }
            }
            FileOperation::Rename | FileOperation::Copy => {
                // These would need additional parameters in a real implementation
                Err(FileWriterError::InvalidConfiguration {
                    reason: "Rename/Copy operations not yet implemented".to_string(),
                })
            }
        };

        let duration_ms = start_time.elapsed().as_millis() as u64;

        match write_result {
            Ok(()) => Ok(OperationResult::Success {
                original_path: path.to_path_buf(),
                backup_path,
                bytes_written: input.content.len() as u64,
                duration_ms,
            }),
            Err(e) => {
                // Attempt rollback if backup was created
                if let Some(ref backup_path) = backup_path {
                    if let Err(rollback_err) = fs::copy(backup_path, path).await {
                        tracing::error!("Failed to rollback after write failure: {}", rollback_err);
                    }
                }

                Ok(OperationResult::Failed {
                    original_path: path.to_path_buf(),
                    error: e.to_string(),
                    duration_ms,
                })
            }
        }
    }

    async fn write_files(&self, inputs: Vec<FileWriteInput>) -> FileWriterResult<FileWriteReport> {
        let mut report = FileWriteReport::new(FileOperation::Modify); // Default operation type
        let batch_options = BatchWriteOptions::default();

        // Process each file
        for input in inputs {
            let result = self.write_file(input).await;
            match result {
                Ok(operation_result) => {
                    report.summary_mut().add_file_result(operation_result);
                }
                Err(e) => {
                    if !batch_options.continue_on_error {
                        report.summary_mut().complete(false);
                        return Err(e);
                    }

                    report
                        .summary_mut()
                        .add_message(format!("File write failed: {}", e));
                }
            }

            // Add delay between operations if configured
            if let Some(delay_ms) = batch_options.delay_between_ops {
                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            }
        }

        report.summary_mut().complete(true);
        Ok(report)
    }

    async fn check_safety(&self, input: &FileWriteInput) -> FileWriterResult<WriteSafetyCheck> {
        let path = Path::new(&input.path);

        // Use configured safety level, but adjust based on validation if present
        let safety_level = if let Some(validation) = &input.validation_results {
            validation
                .recommended_safety_level
                .max(self.config.safety_level)
        } else {
            self.config.safety_level
        };

        // Update safety checker with the determined level
        let checker = DefaultSafetyChecker::with_config(
            safety_level,
            self.config.max_file_size.unwrap_or(100 * 1024 * 1024),
            1024 * 1024 * 1024, // 1GB minimum disk space
        );

        checker.check_file_safety(path)
    }

    async fn create_backup(
        &self,
        path: &Path,
    ) -> FileWriterResult<Option<crate::backup::BackupInfo>> {
        if !path.exists() {
            return Ok(None);
        }

        let backup_info = self
            .backup_manager
            .create_backup(path, self.config.backup_strategy)
            .await?;
        Ok(Some(backup_info))
    }

    async fn restore_from_backup(
        &self,
        original_path: &Path,
        backup_path: &Path,
    ) -> FileWriterResult<()> {
        // Find backup info
        let backup_info = crate::backup::BackupInfo {
            original_path: original_path.to_path_buf(),
            backup_path: backup_path.to_path_buf(),
            created_at: chrono::Utc::now(),
            strategy: self.config.backup_strategy,
            size_bytes: fs::metadata(backup_path).await?.len(),
            checksum: None,
        };

        self.backup_manager.restore_backup(&backup_info).await
    }

    async fn rollback_operation(&self, report: &FileWriteReport) -> FileWriterResult<()> {
        let mut rollback_errors = Vec::new();

        for result in &report.summary().file_results {
            if let OperationResult::Success {
                backup_path: Some(backup),
                original_path,
                ..
            } = result
            {
                if let Err(e) = self.restore_from_backup(original_path, backup).await {
                    rollback_errors.push(format!(
                        "Failed to restore {}: {}",
                        original_path.display(),
                        e
                    ));
                }
            }
        }

        if !rollback_errors.is_empty() {
            return Err(FileWriterError::RollbackFailed {
                reason: rollback_errors.join("; "),
            });
        }

        Ok(())
    }

    fn config(&self) -> &FileWriterConfig {
        &self.config
    }

    fn update_config(&mut self, config: FileWriterConfig) {
        self.config = config;
    }
}

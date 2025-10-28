//! Reporting structures for file writing operations

use crate::error::FileWriterResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Result of an individual file operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationResult {
    /// Operation completed successfully
    Success {
        /// Original file path
        original_path: PathBuf,
        /// Backup file path (if created)
        backup_path: Option<PathBuf>,
        /// Number of bytes written
        bytes_written: u64,
        /// Time taken for the operation
        duration_ms: u64,
    },
    /// Operation failed
    Failed {
        /// Original file path
        original_path: PathBuf,
        /// Error message
        error: String,
        /// Time taken before failure
        duration_ms: u64,
    },
    /// Operation was skipped (e.g., due to safety check)
    Skipped {
        /// Original file path
        original_path: PathBuf,
        /// Reason for skipping
        reason: String,
    },
}

impl std::fmt::Display for OperationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success { original_path, backup_path, bytes_written, duration_ms } => {
                write!(f, "Successfully wrote {} bytes to {} in {}ms",
                       bytes_written,
                       original_path.display(),
                       duration_ms)?;
                if let Some(backup) = backup_path {
                    write!(f, " (backup: {})", backup.display())?;
                }
                Ok(())
            }
            Self::Failed { original_path, error, duration_ms } => {
                write!(f, "Failed to write to {} after {}ms: {}",
                       original_path.display(),
                       duration_ms,
                       error)
            }
            Self::Skipped { original_path, reason } => {
                write!(f, "Skipped writing to {}: {}",
                       original_path.display(),
                       reason)
            }
        }
    }
}

impl OperationResult {
    /// Check if the operation was successful
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Check if the operation failed
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Check if the operation was skipped
    pub fn is_skipped(&self) -> bool {
        matches!(self, Self::Skipped { .. })
    }

    /// Get the file path for this operation
    pub fn path(&self) -> &PathBuf {
        match self {
            Self::Success { original_path, .. }
            | Self::Failed { original_path, .. }
            | Self::Skipped { original_path, .. } => original_path,
        }
    }

    /// Get the duration of the operation in milliseconds
    pub fn duration_ms(&self) -> u64 {
        match self {
            Self::Success { duration_ms, .. } | Self::Failed { duration_ms, .. } => *duration_ms,
            Self::Skipped { .. } => 0,
        }
    }

    /// Get the backup path for successful operations (if any)
    pub fn backup_path(&self) -> Option<&PathBuf> {
        match self {
            Self::Success { backup_path, .. } => backup_path.as_ref(),
            _ => None,
        }
    }

    /// Get the number of bytes written for successful operations
    pub fn bytes_written(&self) -> u64 {
        match self {
            Self::Success { bytes_written, .. } => *bytes_written,
            _ => 0,
        }
    }
}

/// Type of file operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileOperation {
    /// Create a new file
    Create,
    /// Modify an existing file
    Modify,
    /// Delete a file
    Delete,
    /// Rename/move a file
    Rename,
    /// Copy a file
    Copy,
}

impl FileOperation {
    /// Get a string representation of the operation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Modify => "modify",
            Self::Delete => "delete",
            Self::Rename => "rename",
            Self::Copy => "copy",
        }
    }
}

/// Performance metrics for write operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritePerformanceMetrics {
    /// Total time taken in milliseconds
    pub total_duration_ms: u64,
    /// Total number of files processed
    pub files_processed: usize,
    /// Number of files successfully written
    pub successful_files: usize,
    /// Number of files that failed
    pub failed_files: usize,
    /// Number of files skipped
    pub skipped_files: usize,
    /// Total bytes written
    pub total_bytes_written: u64,
    /// Average write speed in bytes per second
    pub avg_write_speed_bps: f64,
    /// Peak memory usage in bytes (if available)
    pub peak_memory_bytes: Option<u64>,
    /// Number of backup files created
    pub backup_files_created: usize,
}

impl WritePerformanceMetrics {
    /// Calculate success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.files_processed == 0 {
            return 0.0;
        }
        (self.successful_files as f64 / self.files_processed as f64) * 100.0
    }

    /// Calculate failure rate as a percentage
    pub fn failure_rate(&self) -> f64 {
        if self.files_processed == 0 {
            return 0.0;
        }
        (self.failed_files as f64 / self.files_processed as f64) * 100.0
    }
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOperationSummary {
    /// Unique identifier for this operation
    pub operation_id: Uuid,
    /// Type of operation
    pub operation_type: FileOperation,
    /// When the operation started
    pub start_time: DateTime<Utc>,
    /// When the operation completed
    pub end_time: Option<DateTime<Utc>>,
    /// Overall success status
    pub success: bool,
    /// Performance metrics
    pub performance_metrics: WritePerformanceMetrics,
    /// Individual file results
    pub file_results: Vec<OperationResult>,
    /// Safety check results
    pub safety_check_passed: bool,
    /// Any warnings or messages
    pub messages: Vec<String>,
}

impl WriteOperationSummary {
    /// Create a new operation summary
    pub fn new(operation_type: FileOperation) -> Self {
        let operation_id = Uuid::new_v4();
        let start_time = Utc::now();

        Self {
            operation_id,
            operation_type,
            start_time,
            end_time: None,
            success: false,
            performance_metrics: WritePerformanceMetrics {
                total_duration_ms: 0,
                files_processed: 0,
                successful_files: 0,
                failed_files: 0,
                skipped_files: 0,
                total_bytes_written: 0,
                avg_write_speed_bps: 0.0,
                peak_memory_bytes: None,
                backup_files_created: 0,
            },
            file_results: Vec::new(),
            safety_check_passed: true,
            messages: Vec::new(),
        }
    }

    /// Mark the operation as completed
    pub fn complete(&mut self, success: bool) {
        self.end_time = Some(Utc::now());
        self.success = success;

        if let Some(end_time) = self.end_time {
            let duration = end_time.signed_duration_since(self.start_time);
            self.performance_metrics.total_duration_ms = duration.num_milliseconds() as u64;

            // Calculate average write speed
            if self.performance_metrics.total_duration_ms > 0 {
                let duration_seconds = self.performance_metrics.total_duration_ms as f64 / 1000.0;
                self.performance_metrics.avg_write_speed_bps =
                    self.performance_metrics.total_bytes_written as f64 / duration_seconds;
            }
        }
    }

    /// Add a file result to the summary
    pub fn add_file_result(&mut self, result: OperationResult) {
        self.performance_metrics.files_processed += 1;

        match &result {
            OperationResult::Success {
                bytes_written,
                backup_path,
                ..
            } => {
                self.performance_metrics.successful_files += 1;
                self.performance_metrics.total_bytes_written += bytes_written;
                if backup_path.is_some() {
                    self.performance_metrics.backup_files_created += 1;
                }
            }
            OperationResult::Failed { .. } => {
                self.performance_metrics.failed_files += 1;
            }
            OperationResult::Skipped { .. } => {
                self.performance_metrics.skipped_files += 1;
            }
        }

        self.file_results.push(result);
    }

    /// Add a message to the summary
    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    /// Get the duration of the operation in milliseconds
    pub fn duration_ms(&self) -> u64 {
        self.performance_metrics.total_duration_ms
    }

    /// Check if the operation is complete
    pub fn is_complete(&self) -> bool {
        self.end_time.is_some()
    }
}

/// Comprehensive report for file writing operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWriteReport {
    /// Operation summary
    pub summary: WriteOperationSummary,
    /// Detailed information about the operation
    pub details: ReportDetails,
}

impl FileWriteReport {
    /// Create a new file write report
    pub fn new(operation_type: FileOperation) -> Self {
        Self {
            summary: WriteOperationSummary::new(operation_type),
            details: ReportDetails::default(),
        }
    }

    /// Get a reference to the operation summary
    pub fn summary(&self) -> &WriteOperationSummary {
        &self.summary
    }

    /// Get a mutable reference to the operation summary
    pub fn summary_mut(&mut self) -> &mut WriteOperationSummary {
        &mut self.summary
    }

    /// Serialize the report to JSON
    pub fn to_json(&self) -> FileWriterResult<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }

    /// Deserialize the report from JSON
    pub fn from_json(json: &str) -> FileWriterResult<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }

    /// Generate a human-readable summary
    pub fn generate_summary_text(&self) -> String {
        let summary = &self.summary;
        let metrics = &summary.performance_metrics;

        format!(
            "File Write Report - Operation {} ({})\n\
            Status: {}\n\
            Duration: {}ms\n\
            Files Processed: {}\n\
            Successful: {} ({:.1}%)\n\
            Failed: {} ({:.1}%)\n\
            Skipped: {} ({:.1}%)\n\
            Bytes Written: {}\n\
            Backup Files Created: {}\n\
            Average Write Speed: {:.2} bytes/sec",
            summary.operation_id,
            summary.operation_type.as_str(),
            if summary.success { "SUCCESS" } else { "FAILED" },
            metrics.total_duration_ms,
            metrics.files_processed,
            metrics.successful_files,
            metrics.success_rate(),
            metrics.failed_files,
            metrics.failure_rate(),
            metrics.skipped_files,
            (metrics.skipped_files as f64 / metrics.files_processed as f64) * 100.0,
            metrics.total_bytes_written,
            metrics.backup_files_created,
            metrics.avg_write_speed_bps
        )
    }
}

/// Additional details for the report
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportDetails {
    /// Safety check details
    pub safety_check_details: Option<String>,
    /// Backup strategy used
    pub backup_strategy: Option<String>,
    /// Validation results (if any)
    pub validation_results: Option<String>,
    /// Error details (if any)
    pub error_details: Option<String>,
    /// Configuration used
    pub configuration: Option<String>,
    /// Environment information
    pub environment: Option<EnvironmentInfo>,
}

/// Environment information for the report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// Operating system
    pub os: String,
    /// Rust version
    pub rust_version: String,
    /// Working directory
    pub working_directory: String,
    /// User information (if available)
    pub user: Option<String>,
}

impl Default for EnvironmentInfo {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            rust_version: "1.0.0".to_string(), // Placeholder for GREEN phase
            working_directory: std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("unknown"))
                .to_string_lossy()
                .to_string(),
            user: std::env::var("USER").ok(),
        }
    }
}

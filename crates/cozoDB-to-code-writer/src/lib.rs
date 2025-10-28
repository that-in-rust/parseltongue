//! Tool 4: Rust File Writer CLI
//!
//! This crate provides comprehensive file writing capabilities with safety mechanisms,
//! backup creation, and integration with Tool 3 validation results.
//!
//! Following TDD-first principle - tests first, implementation second

pub mod backup;
pub mod error;
pub mod file_writer;
pub mod report;
pub mod safety;
pub mod tool3_integration;
pub mod writer;

// Re-export key types for convenience
pub use backup::{BackupManager, BackupStrategy, DefaultBackupManager};
pub use error::{FileWriterError, FileWriterResult};
pub use file_writer::{FileWriteInput, FileWriter, FileWriterConfig};
pub use report::{
    FileOperation, FileWriteReport, OperationResult, WriteOperationSummary, WritePerformanceMetrics,
};
pub use safety::{
    DefaultSafetyChecker, SafetyChecker, SafetyIssueType, WriteSafetyCheck, WriteSafetyLevel,
};
pub use tool3_integration::{
    DefaultValidationConverter, ValidationDetails, ValidationResult,
    ValidationToFileWriterConverter, ValidationWritePipeline,
};
pub use writer::DefaultFileWriter;

/// Tool 4 re-export for convenience
pub struct Tool4 {
    writer: DefaultFileWriter,
}

impl Tool4 {
    pub fn new() -> Self {
        Self {
            writer: DefaultFileWriter::new(),
        }
    }

    pub fn writer(&self) -> &DefaultFileWriter {
        &self.writer
    }
}

impl Default for Tool4 {
    fn default() -> Self {
        Self::new()
    }
}

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Result of a file write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteResult {
    /// Whether the operation succeeded
    pub success: bool,
    /// Path to the file that was written/deleted
    pub file_path: PathBuf,
    /// Type of operation performed
    pub operation: WriteOperation,
    /// Optional message describing the result
    pub message: Option<String>,
}

impl WriteResult {
    /// Create a successful write result
    pub fn success(file_path: PathBuf, operation: WriteOperation) -> Self {
        Self {
            success: true,
            file_path,
            operation,
            message: None,
        }
    }

    /// Create a no-op result (nothing to do)
    pub fn no_op() -> Self {
        Self {
            success: true,
            file_path: PathBuf::new(),
            operation: WriteOperation::NoOp,
            message: Some("No operation required".to_string()),
        }
    }

    /// Add a message to this result
    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}

/// Type of write operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriteOperation {
    /// Created a new file
    Create,
    /// Modified an existing file
    Edit,
    /// Deleted a file
    Delete,
    /// No operation performed
    NoOp,
}

/// Summary of all write operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteSummary {
    /// Number of files created
    pub created: usize,
    /// Number of files edited
    pub edited: usize,
    /// Number of files deleted
    pub deleted: usize,
    /// Total number of operations
    pub total: usize,
    /// Number of errors encountered
    pub errors: usize,
}

impl WriteSummary {
    /// Create a new empty summary
    pub fn new() -> Self {
        Self {
            created: 0,
            edited: 0,
            deleted: 0,
            total: 0,
            errors: 0,
        }
    }

    /// Add a result to this summary
    pub fn add_result(&mut self, result: &WriteResult) {
        if !result.success {
            self.errors += 1;
            return;
        }

        match result.operation {
            WriteOperation::Create => self.created += 1,
            WriteOperation::Edit => self.edited += 1,
            WriteOperation::Delete => self.deleted += 1,
            WriteOperation::NoOp => {}
        }
        self.total += 1;
    }
}

impl Default for WriteSummary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_result_success() {
        let result = WriteResult::success(
            PathBuf::from("test.rs"),
            WriteOperation::Create,
        );
        assert!(result.success);
        assert_eq!(result.operation, WriteOperation::Create);
    }

    #[test]
    fn test_write_summary() {
        let mut summary = WriteSummary::new();

        summary.add_result(&WriteResult::success(
            PathBuf::from("file1.rs"),
            WriteOperation::Create,
        ));
        summary.add_result(&WriteResult::success(
            PathBuf::from("file2.rs"),
            WriteOperation::Edit,
        ));

        assert_eq!(summary.created, 1);
        assert_eq!(summary.edited, 1);
        assert_eq!(summary.total, 2);
    }
}

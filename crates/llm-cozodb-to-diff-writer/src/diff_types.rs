//! # CodeDiff Types
//!
//! Data structures for CodeDiff.json generation.
//!
//! ## Purpose
//! Define the schema for CodeDiff.json that the LLM reads to apply changes.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// CodeDiff.json root structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeDiff {
    /// List of changes to apply
    pub changes: Vec<Change>,

    /// Metadata about the diff generation
    pub metadata: DiffMetadata,
}

/// A single change to apply
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Change {
    /// ISGL1 key of the entity
    pub isgl1_key: String,

    /// File path relative to project root
    pub file_path: PathBuf,

    /// Operation to perform
    pub operation: Operation,

    /// Future code content (Some for Create/Edit, None for Delete)
    pub future_code: Option<String>,

    /// Interface signature for reference
    pub interface_signature: String,
}

/// Operation type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    /// Create a new file/entity
    Create,
    /// Edit an existing file/entity
    Edit,
    /// Delete a file/entity
    Delete,
}

/// Metadata about diff generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiffMetadata {
    /// Number of changes
    pub total_changes: usize,

    /// Breakdown by operation
    pub create_count: usize,
    pub edit_count: usize,
    pub delete_count: usize,

    /// Generation timestamp (ISO 8601)
    pub generated_at: String,
}

impl CodeDiff {
    /// Create a new empty CodeDiff
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            metadata: DiffMetadata {
                total_changes: 0,
                create_count: 0,
                edit_count: 0,
                delete_count: 0,
                generated_at: chrono::Utc::now().to_rfc3339(),
            },
        }
    }

    /// Add a change to the diff
    pub fn add_change(&mut self, change: Change) {
        match change.operation {
            Operation::Create => self.metadata.create_count += 1,
            Operation::Edit => self.metadata.edit_count += 1,
            Operation::Delete => self.metadata.delete_count += 1,
        }
        self.metadata.total_changes += 1;
        self.changes.push(change);
    }

    /// Convert to pretty-printed JSON
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for CodeDiff {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_code_diff() {
        let diff = CodeDiff::new();
        assert_eq!(diff.changes.len(), 0);
        assert_eq!(diff.metadata.total_changes, 0);
    }

    #[test]
    fn test_add_change_updates_metadata() {
        let mut diff = CodeDiff::new();

        let change = Change {
            isgl1_key: "test-key".to_string(),
            file_path: PathBuf::from("src/test.rs"),
            operation: Operation::Create,
            future_code: Some("fn test() {}".to_string()),
            interface_signature: "fn test()".to_string(),
        };

        diff.add_change(change);

        assert_eq!(diff.changes.len(), 1);
        assert_eq!(diff.metadata.total_changes, 1);
        assert_eq!(diff.metadata.create_count, 1);
        assert_eq!(diff.metadata.edit_count, 0);
        assert_eq!(diff.metadata.delete_count, 0);
    }

    #[test]
    fn test_operation_counts() {
        let mut diff = CodeDiff::new();

        // Add create
        diff.add_change(Change {
            isgl1_key: "create-key".to_string(),
            file_path: PathBuf::from("src/new.rs"),
            operation: Operation::Create,
            future_code: Some("fn new() {}".to_string()),
            interface_signature: "fn new()".to_string(),
        });

        // Add edit
        diff.add_change(Change {
            isgl1_key: "edit-key".to_string(),
            file_path: PathBuf::from("src/old.rs"),
            operation: Operation::Edit,
            future_code: Some("fn updated() {}".to_string()),
            interface_signature: "fn updated()".to_string(),
        });

        // Add delete
        diff.add_change(Change {
            isgl1_key: "delete-key".to_string(),
            file_path: PathBuf::from("src/gone.rs"),
            operation: Operation::Delete,
            future_code: None,
            interface_signature: "fn gone()".to_string(),
        });

        assert_eq!(diff.metadata.total_changes, 3);
        assert_eq!(diff.metadata.create_count, 1);
        assert_eq!(diff.metadata.edit_count, 1);
        assert_eq!(diff.metadata.delete_count, 1);
    }

    #[test]
    fn test_json_serialization() {
        let mut diff = CodeDiff::new();

        diff.add_change(Change {
            isgl1_key: "test-key".to_string(),
            file_path: PathBuf::from("src/test.rs"),
            operation: Operation::Create,
            future_code: Some("fn test() {}".to_string()),
            interface_signature: "fn test()".to_string(),
        });

        let json = diff.to_json_pretty().expect("JSON serialization failed");
        assert!(json.contains("\"changes\""));
        assert!(json.contains("\"metadata\""));
        assert!(json.contains("\"CREATE\""));
    }
}

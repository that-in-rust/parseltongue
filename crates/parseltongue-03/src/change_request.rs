//! Change request module for Tool 2
//!
//! Defines the structure for code change requests that will be simulated

use parseltongue_01::types::{CoreResult, ISGL1Key};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Represents a request to change code that needs simulation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeRequest {
    /// Unique identifier for this change request
    pub id: Uuid,

    /// Target code element to change
    pub target: ChangeTarget,

    /// Type of change being requested
    pub change_type: ChangeType,

    /// Description of the change
    pub description: String,

    /// Current code (for reference)
    pub current_code: String,

    /// Proposed new code
    pub proposed_code: String,

    /// Additional metadata
    pub metadata: ChangeMetadata,
}

/// Target of the change request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeTarget {
    /// ISG L1 key identifying the target (custom serialized)
    #[serde(with = "isgl1key_serde")]
    pub key: ISGL1Key,

    /// Optional line range for the change
    pub line_range: Option<(usize, usize)>,
}

/// Custom serialization module for ISGL1Key
mod isgl1key_serde {
    use parseltongue_01::types::ISGL1Key;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize)]
    struct SerializedKey {
        filepath: String,
        filename: String,
        interface_name: String,
    }

    pub fn serialize<S>(key: &ISGL1Key, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized = SerializedKey {
            filepath: key.filepath.to_string_lossy().to_string(),
            filename: key.filename.clone(),
            interface_name: key.interface_name.clone(),
        };
        serialized.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ISGL1Key, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serialized = SerializedKey::deserialize(deserializer)?;
        Ok(ISGL1Key::new(
            PathBuf::from(serialized.filepath),
            serialized.filename,
            serialized.interface_name,
        ))
    }
}

/// Type of change being requested
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// Adding new functionality
    Add,
    /// Modifying existing functionality
    Modify,
    /// Removing functionality
    Remove,
    /// Refactoring (structure change without behavior change)
    Refactor,
    /// Bug fix
    Fix,
}

/// Additional metadata for the change request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeMetadata {
    /// Priority of this change
    pub priority: ChangePriority,

    /// Estimated complexity
    pub complexity: Complexity,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Reasoning for the change
    pub reasoning: Option<String>,
}

/// Priority levels for change requests
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangePriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Complexity levels for change requests
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Complexity {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

/// Validation result for a change request
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the request is valid
    pub is_valid: bool,

    /// List of validation errors
    pub errors: Vec<String>,

    /// List of warnings
    pub warnings: Vec<String>,
}

impl ChangeRequest {
    /// Create a new change request
    pub fn new(
        key: ISGL1Key,
        change_type: ChangeType,
        description: String,
        current_code: String,
        proposed_code: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            target: ChangeTarget {
                key,
                line_range: None,
            },
            change_type,
            description,
            current_code,
            proposed_code,
            metadata: ChangeMetadata {
                priority: ChangePriority::Medium,
                complexity: Complexity::Moderate,
                tags: Vec::new(),
                reasoning: None,
            },
        }
    }

    /// Create a change request with line range
    pub fn with_line_range(
        key: ISGL1Key,
        line_range: (usize, usize),
        change_type: ChangeType,
        description: String,
        current_code: String,
        proposed_code: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            target: ChangeTarget {
                key,
                line_range: Some(line_range),
            },
            change_type,
            description,
            current_code,
            proposed_code,
            metadata: ChangeMetadata {
                priority: ChangePriority::Medium,
                complexity: Complexity::Moderate,
                tags: Vec::new(),
                reasoning: None,
            },
        }
    }

    /// Validate the change request
    pub fn validate(&self) -> CoreResult<ValidationResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate target
        if self.target.key.interface_name.is_empty() {
            errors.push("Interface name cannot be empty".to_string());
        }

        // Validate description
        if self.description.trim().is_empty() {
            errors.push("Description cannot be empty".to_string());
        }

        // Validate code content
        if self.current_code.trim().is_empty() {
            warnings.push("Current code is empty".to_string());
        }

        if self.proposed_code.trim().is_empty() {
            errors.push("Proposed code cannot be empty".to_string());
        }

        // Validate line range if present
        if let Some((start, end)) = self.target.line_range {
            if start > end {
                errors.push("Invalid line range: start > end".to_string());
            }
            if start == 0 {
                warnings.push("Line numbers are 1-based, but start is 0".to_string());
            }
        }

        // Check for suspicious patterns in proposed code
        if self.proposed_code.contains("TODO") || self.proposed_code.contains("FIXME") {
            warnings.push("Proposed code contains TODO/FIXME comments".to_string());
        }

        let is_valid = errors.is_empty();
        Ok(ValidationResult {
            is_valid,
            errors,
            warnings,
        })
    }

    /// Set priority
    pub fn with_priority(mut self, priority: ChangePriority) -> Self {
        self.metadata.priority = priority;
        self
    }

    /// Set complexity
    pub fn with_complexity(mut self, complexity: Complexity) -> Self {
        self.metadata.complexity = complexity;
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: String) -> Self {
        self.metadata.tags.push(tag);
        self
    }

    /// Set reasoning
    pub fn with_reasoning(mut self, reasoning: String) -> Self {
        self.metadata.reasoning = Some(reasoning);
        self
    }

    /// Get estimated impact size
    pub fn estimate_impact_size(&self) -> usize {
        // Simple heuristic: size difference plus complexity factor
        let current_lines = self.current_code.lines().count();
        let proposed_lines = self.proposed_code.lines().count();
        let line_diff = if proposed_lines > current_lines {
            proposed_lines - current_lines
        } else {
            current_lines - proposed_lines
        };

        let complexity_factor = match self.metadata.complexity {
            Complexity::Simple => 1,
            Complexity::Moderate => 2,
            Complexity::Complex => 4,
            Complexity::VeryComplex => 8,
        };

        line_diff * complexity_factor
    }

    /// Create an invalid change request for testing
    pub fn invalid() -> Self {
        let key = ISGL1Key::new(PathBuf::from(""), String::new(), String::new());

        Self {
            id: Uuid::new_v4(),
            target: ChangeTarget {
                key,
                line_range: Some((10, 5)), // Invalid range
            },
            change_type: ChangeType::Modify,
            description: String::new(), // Empty description
            current_code: String::new(),
            proposed_code: String::new(), // Empty proposed code
            metadata: ChangeMetadata {
                priority: ChangePriority::Medium,
                complexity: Complexity::Moderate,
                tags: Vec::new(),
                reasoning: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_key() -> ISGL1Key {
        ISGL1Key::new(
            PathBuf::from("/test/src/lib.rs"),
            "lib.rs".to_string(),
            "test_function".to_string(),
        )
    }

    #[test]
    fn test_valid_change_request() {
        let key = create_test_key();
        let request = ChangeRequest::new(
            key,
            ChangeType::Modify,
            "Add error handling".to_string(),
            "fn test_function() {}".to_string(),
            "fn test_function() -> Result<(), Error> { Ok(()) }".to_string(),
        );

        let result = request.validate().unwrap();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_invalid_change_request() {
        let request = ChangeRequest::invalid();
        let result = request.validate().unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_change_request_with_line_range() {
        let key = create_test_key();
        let request = ChangeRequest::with_line_range(
            key,
            (1, 5),
            ChangeType::Modify,
            "Add error handling".to_string(),
            "fn test_function() {}".to_string(),
            "fn test_function() -> Result<(), Error> { Ok(()) }".to_string(),
        );

        assert_eq!(request.target.line_range, Some((1, 5)));

        let result = request.validate().unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_invalid_line_range() {
        let key = create_test_key();
        let request = ChangeRequest::with_line_range(
            key,
            (10, 5), // Invalid: start > end
            ChangeType::Modify,
            "Test".to_string(),
            "old code".to_string(),
            "new code".to_string(),
        );

        let result = request.validate().unwrap();
        assert!(!result.is_valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("Invalid line range")));
    }

    #[test]
    fn test_impact_size_estimation() {
        let key = create_test_key();
        let request = ChangeRequest::new(
            key,
            ChangeType::Modify,
            "Add many lines".to_string(),
            "fn test() {}".to_string(),
            "fn test() -> Result<(), Error> {\n    // Complex implementation\n    let result = compute();\n    Ok(result)\n}".to_string(),
        )
        .with_complexity(Complexity::Complex);

        let impact = request.estimate_impact_size();
        assert!(impact > 0);
    }
}

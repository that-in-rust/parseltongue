//! # Diff Generator
//!
//! Generates CodeDiff.json from CozoDB entities with future_code.

use anyhow::{Context, Result};
use parseltongue_core::entities::{CodeEntity, FutureAction, TemporalState};
use parseltongue_core::storage::CozoDbStorage;
use std::path::PathBuf;

use crate::diff_types::{Change, CodeDiff, Operation};

/// Diff generator that reads from CozoDB
pub struct DiffGenerator {
    storage: CozoDbStorage,
}

impl DiffGenerator {
    /// Create a new diff generator
    pub fn new(storage: CozoDbStorage) -> Self {
        Self { storage }
    }

    /// Generate CodeDiff from all entities with future_action
    pub async fn generate_diff(&self) -> Result<CodeDiff> {
        // Get all changed entities from CozoDB
        let changed_entities = self
            .storage
            .get_changed_entities()
            .await
            .context("Failed to get changed entities from CozoDB")?;

        let mut diff = CodeDiff::new();

        for entity in changed_entities {
            if let Some(change) = self.entity_to_change(&entity)? {
                diff.add_change(change);
            }
        }

        Ok(diff)
    }

    /// Convert CodeEntity to Change
    fn entity_to_change(&self, entity: &CodeEntity) -> Result<Option<Change>> {
        // Determine operation from temporal state
        let operation = match entity.temporal_state {
            TemporalState::WillBeCreated => Operation::Create,
            TemporalState::WillBeModified => Operation::Edit,
            TemporalState::WillBeDeleted => Operation::Delete,
            TemporalState::Unchanged => return Ok(None), // Skip unchanged
            _ => return Ok(None), // Skip other states
        };

        // Extract file path from ISGL1 key
        let file_path = self.extract_file_path(&entity.isgl1_key)?;

        // Create change
        let change = Change {
            isgl1_key: entity.isgl1_key.clone(),
            file_path,
            operation,
            future_code: entity.future_code.clone(),
            interface_signature: entity.interface_signature.raw.clone(),
        };

        Ok(Some(change))
    }

    /// Extract file path from ISGL1 key
    fn extract_file_path(&self, isgl1_key: &str) -> Result<PathBuf> {
        // ISGL1 key formats:
        // - Line-based: "rust:fn:name:src_lib_rs:42-56"
        // - Hash-based: "src_lib_rs-new_feature-fn-abc12345"

        if isgl1_key.contains(':') {
            // Line-based format
            let parts: Vec<&str> = isgl1_key.split(':').collect();
            if parts.len() >= 4 {
                let path_part = parts[3]; // "src_lib_rs"
                let file_path = path_part.replace('_', "/") + ".rs";
                return Ok(PathBuf::from(file_path));
            }
        } else {
            // Hash-based format
            let parts: Vec<&str> = isgl1_key.split('-').collect();
            if !parts.is_empty() {
                // First part is sanitized file path
                let path_part = parts[0];
                let file_path = path_part.replace('_', "/") + ".rs";
                return Ok(PathBuf::from(file_path));
            }
        }

        anyhow::bail!("Invalid ISGL1 key format: {}", isgl1_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_file_path_line_based() {
        let generator = DiffGenerator {
            storage: CozoDbStorage::new_mock(),
        };

        let path = generator
            .extract_file_path("rust:fn:calculate_sum:src_lib_rs:42-56")
            .unwrap();
        assert_eq!(path, PathBuf::from("src/lib.rs"));
    }

    #[test]
    fn test_extract_file_path_hash_based() {
        let generator = DiffGenerator {
            storage: CozoDbStorage::new_mock(),
        };

        let path = generator
            .extract_file_path("src_lib_rs-new_feature-fn-abc12345")
            .unwrap();
        assert_eq!(path, PathBuf::from("src/lib.rs"));
    }

    #[test]
    fn test_extract_nested_file_path() {
        let generator = DiffGenerator {
            storage: CozoDbStorage::new_mock(),
        };

        let path = generator
            .extract_file_path("rust:fn:helper:src_models_user_rs:10-20")
            .unwrap();
        assert_eq!(path, PathBuf::from("src/models/user.rs"));
    }
}

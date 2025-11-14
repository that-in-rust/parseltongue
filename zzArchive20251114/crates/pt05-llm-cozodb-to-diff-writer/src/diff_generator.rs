//! # Diff Generator
//!
//! Generates CodeDiff.json from CozoDB entities with future_code.
//!
//! ## Enhanced Features (Post-TDD Refactor):
//! - ✅ Dependency injection with Arc<CozoDbStorage>
//! - ✅ Extracts current_code and future_code from entities
//! - ✅ Parses line_range from ISGL1 keys
//! - ✅ Supports entity-level operations
//!
//! ## Key Implementation Insights
//!
//! ### Operation-Specific Code Field Logic
//!
//! Different temporal operations require different code fields:
//!
//! | Operation | current_code | future_code | line_range | Rationale |
//! |-----------|--------------|-------------|------------|-----------|
//! | CREATE    | None         | Some        | None       | Entity doesn't exist yet, use hash-based key |
//! | EDIT      | Some         | Some        | Some       | Need both before/after, precise line location |
//! | DELETE    | Some         | None        | Some       | Show what's being removed, location to delete |
//!
//! This table drives the pattern matching in `entity_to_change()`.
//!
//! ### File Path Desanitization Strategy
//!
//! ISGL1 keys encode paths as `src_lib_rs` instead of `src/lib.rs`. The challenge:
//! file extensions also use underscores ("_rs" for ".rs").
//!
//! **Naive approach** (WRONG): `sanitized.replace('_', '/')` → "src/lib/rs"
//!
//! **Correct approach**:
//! 1. Search for known extension suffixes at end of string
//! 2. Split path at extension boundary
//! 3. Replace underscores in path portion only
//! 4. Reconstruct with proper extension
//!
//! Example: `src_lib_rs` → "src/lib" + ".rs" → "src/lib.rs"

use anyhow::{Context, Result};
use parseltongue_core::entities::{CodeEntity, TemporalAction};
use parseltongue_core::storage::CozoDbStorage;
use std::path::PathBuf;
use std::sync::Arc;

use crate::diff_types::{Change, CodeDiff, LineRange, Operation};

/// Diff generator that reads from CozoDB (with dependency injection)
pub struct DiffGenerator {
    storage: Arc<CozoDbStorage>,
}

impl DiffGenerator {
    /// Create a new diff generator (dependency injection pattern)
    pub fn new(storage: Arc<CozoDbStorage>) -> Self {
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

    /// Convert CodeEntity to Change (with enhanced fields)
    fn entity_to_change(&self, entity: &CodeEntity) -> Result<Option<Change>> {
        // Determine operation from temporal state's future_action
        let operation = match entity.temporal_state.future_action {
            Some(TemporalAction::Create) => Operation::Create,
            Some(TemporalAction::Edit) => Operation::Edit,
            Some(TemporalAction::Delete) => Operation::Delete,
            None => return Ok(None), // Skip entities with no future action (unchanged)
        };

        // Extract file path from ISGL1 key
        let file_path = self.extract_file_path(&entity.isgl1_key)?;

        // Extract line range from ISGL1 key (if line-based format)
        let line_range = self.extract_line_range(&entity.isgl1_key);

        // Extract current_code based on operation:
        // - CREATE: None (entity doesn't exist yet)
        // - EDIT/DELETE: Some (need to know what to replace/remove)
        let current_code = match operation {
            Operation::Create => None,
            Operation::Edit | Operation::Delete => entity.current_code.clone(),
        };

        // Extract future_code based on operation:
        // - CREATE/EDIT: Some (what to write)
        // - DELETE: None (removing code)
        let future_code = match operation {
            Operation::Create | Operation::Edit => entity.future_code.clone(),
            Operation::Delete => None,
        };

        // Format interface signature from components
        let interface_signature = format!(
            "{:?} {}",
            entity.interface_signature.entity_type,
            entity.interface_signature.name
        );

        // Create change with enhanced fields
        let change = Change {
            isgl1_key: entity.isgl1_key.clone(),
            file_path,
            operation,
            current_code,
            future_code,
            line_range,
            interface_signature,
        };

        Ok(Some(change))
    }

    /// Extract file path from ISGL1 key
    fn extract_file_path(&self, isgl1_key: &str) -> Result<PathBuf> {
        // ISGL1 key formats:
        // - Line-based: "rust:fn:name:src_lib_rs:42-56"
        // - Hash-based: "src_lib_rs-new_feature-fn-abc12345"
        //
        // Sanitized paths encode "src/lib.rs" as "src_lib_rs"
        // The "_rs" suffix represents the ".rs" extension
        // Other underscores represent directory separators

        if isgl1_key.contains(':') {
            // Line-based format
            let parts: Vec<&str> = isgl1_key.split(':').collect();
            if parts.len() >= 4 {
                let sanitized_path = parts[3]; // "src_lib_rs"
                return Ok(self.desanitize_path(sanitized_path));
            }
        } else {
            // Hash-based format
            let parts: Vec<&str> = isgl1_key.split('-').collect();
            if !parts.is_empty() {
                // First part is sanitized file path
                let sanitized_path = parts[0];
                return Ok(self.desanitize_path(sanitized_path));
            }
        }

        anyhow::bail!("Invalid ISGL1 key format: {}", isgl1_key)
    }

    /// Desanitize file path from ISGL1 key format
    /// Converts "src_lib_rs" → "src/lib.rs"
    fn desanitize_path(&self, sanitized: &str) -> PathBuf {
        // Common file extensions
        let extensions = ["_rs", "_js", "_ts", "_py", "_go", "_java", "_cpp", "_c", "_h"];

        // Find and replace extension suffix
        for ext in extensions {
            if let Some(idx) = sanitized.rfind(ext) {
                if idx + ext.len() == sanitized.len() {
                    // Found extension at end
                    let path_part = &sanitized[..idx]; // "src_lib"
                    let ext_part = &ext[1..]; // "rs"
                    let file_path = path_part.replace('_', "/") + "." + ext_part;
                    return PathBuf::from(file_path);
                }
            }
        }

        // No known extension found, treat as-is
        PathBuf::from(sanitized.replace('_', "/"))
    }

    /// Extract line range from ISGL1 key (returns None for hash-based keys)
    fn extract_line_range(&self, isgl1_key: &str) -> Option<LineRange> {
        // Only line-based keys have line ranges: "rust:fn:name:src_lib_rs:42-56"
        // Hash-based keys do not: "src_lib_rs-new_feature-fn-abc12345"

        if !isgl1_key.contains(':') {
            return None; // Hash-based key
        }

        // Line-based format: last part is "start-end"
        let parts: Vec<&str> = isgl1_key.split(':').collect();
        if parts.len() < 5 {
            return None;
        }

        let line_part = parts[4]; // "42-56"
        let line_nums: Vec<&str> = line_part.split('-').collect();
        if line_nums.len() != 2 {
            return None;
        }

        let start = line_nums[0].parse::<u32>().ok()?;
        let end = line_nums[1].parse::<u32>().ok()?;

        Some(LineRange { start, end })
    }
}

// Unit tests for extract_file_path and extract_line_range are covered by integration tests

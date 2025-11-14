//! Data models for PT02 export operations.
//!
//! # Architecture: Progressive Disclosure (Levels 0-2)
//!
//! **Level 0**: Pure edge list (DependencyEdge)
//! **Level 1**: Node-centric + ISG + Temporal (EntityExportLevel1)
//! **Level 2**: + Type system essentials (EntityExportLevel2)
//!
//! ## Design Principles (S01)
//!
//! 1. **Executable Specifications**: Each struct IS the specification
//! 2. **Serde Optimization**: Skip nulls/empty arrays (40% token savings)
//! 3. **Semantic ISGL1 Keys**: NOT integer indices (6.7× better effective context)
//! 4. **Flat Hierarchy**: Level2 flattens Level1 (no nesting for LLM readability)

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ============================================================================
// Export Configuration
// ============================================================================

/// Configuration for export operations
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Export level (0, 1, 2)
    pub level: u8,

    /// Include current_code field (N/A for Level 0, required for Level 1-2)
    pub include_code: bool,

    /// Datalog WHERE clause or "ALL"
    pub where_filter: String,

    /// Output JSON file path
    pub output_path: PathBuf,

    /// Database connection string
    pub db_path: String,

    // v0.9.0: Dual outputs for code/test separation
    /// Code entities output path (when entity_class filtering is enabled)
    pub code_output_path: Option<PathBuf>,

    /// Test entities output path (when entity_class filtering is enabled)
    pub tests_output_path: Option<PathBuf>,

    // v0.9.7: Timestamped folder creation
    /// Session start timestamp for timestamped folder creation
    /// If None, uses current timestamp. If Some, all outputs go to same timestamped folder.
    pub session_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

// ============================================================================
// Export Output (Unified for all levels)
// ============================================================================

/// Top-level export output structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportOutput {
    pub export_metadata: ExportMetadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<DependencyEdge>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<serde_json::Value>,
}

impl ExportOutput {
    /// Write export output to JSON file with structured error handling
    pub fn write_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> anyhow::Result<()> {
        let json_content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json_content)?;
        Ok(())
    }
}

/// Export metadata (common across all levels)
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub level: u8,
    pub timestamp: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_edges: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_entities: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_code: Option<bool>,

    pub where_filter: String,
}

// ============================================================================
// Level 0: Pure Edge List (MINIMAL)
// ============================================================================

/// Level 0: Dependency edge (pure graph topology)
///
/// # Why 3 Fields?
/// - from_key: Source entity (semantic ISGL1 key)
/// - to_key: Target entity (semantic ISGL1 key)
/// - edge_type: Relationship type ("depends_on", "implements", etc.)
///
/// # Token Cost
/// ~2-5K tokens for 1,500-2,000 edges (absolute minimum)
///
/// # Example
/// ```json
/// {
///   "from_key": "rust:fn:calculate_total:src_billing_rs:42",
///   "to_key": "rust:fn:get_tax_rate:src_billing_rs:102",
///   "edge_type": "depends_on"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DependencyEdge {
    pub from_key: String,
    pub to_key: String,
    pub edge_type: String,
}

// ============================================================================
// Level 1: Node-Centric + ISG + Temporal
// ============================================================================

/// Level 1: Entity export with ISG and temporal state
///
/// # Variables (13 total)
/// - Graph: isgl1_key, forward_deps, reverse_deps
/// - Temporal: current_ind, future_ind, future_action, future_code, current_code
/// - Identity: entity_name, entity_type, file_path, line_number
/// - ISG: interface_signature, doc_comment
///
/// # Token Cost
/// ~30K tokens (no code) | ~500K tokens (with code)
///
/// # Why These Fields?
/// - **Node-centric view**: Pre-computed adjacency lists (vs Level 0 edge list)
/// - **Temporal state**: Track multi-step operations (PT03 → PT04 → PT05 → PT06)
/// - **Identity**: What entity am I looking at?
/// - **ISG**: Interface signature separates contract from implementation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityExportLevel1 {
    pub isgl1_key: String,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub forward_deps: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub reverse_deps: Vec<String>,

    pub current_ind: u8,
    pub future_ind: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_action: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_code: Option<String>,

    pub entity_name: String,
    pub entity_type: String,
    pub file_path: String,
    pub line_number: u32,
    pub interface_signature: String,
    
    // v0.9.0: EntityClass for code/test separation
    pub entity_class: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_comment: Option<String>,
}

// ============================================================================
// Level 2: + Type System Essentials
// ============================================================================

/// Level 2: Entity export with type system information
///
/// # Variables Added (8 additional)
/// - return_type, param_types, param_names, generic_constraints
/// - trait_impls, is_public, is_async, is_unsafe
///
/// # Token Cost
/// ~60K tokens (no code) | ~600K tokens (with code)
///
/// # Use Case
/// Type-aware refactoring, API compatibility analysis, safety-critical operations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityExportLevel2 {
    // Level 1 fields (flattened for LLM readability, NOT nested)
    pub isgl1_key: String,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub forward_deps: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub reverse_deps: Vec<String>,

    pub current_ind: u8,
    pub future_ind: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_action: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_code: Option<String>,

    pub entity_name: String,
    pub entity_type: String,
    pub file_path: String,
    pub line_number: u32,
    pub interface_signature: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_comment: Option<String>,

    // Level 2 additions: Type system essentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_type: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub param_types: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub param_names: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub generic_constraints: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub trait_impls: Vec<String>,

    pub is_public: bool,
    pub is_async: bool,
    pub is_unsafe: bool,
}

// ============================================================================
// Helper Functions
// ============================================================================

impl ExportMetadata {
    /// Create metadata for Level 0 (edges)
    pub fn for_level0(total_edges: usize, where_filter: String) -> Self {
        Self {
            level: 0,
            timestamp: chrono::Utc::now().to_rfc3339(),
            total_edges: Some(total_edges),
            total_entities: None,
            include_code: None,
            where_filter,
        }
    }

    /// Create metadata for Level 1-2 (entities)
    pub fn for_entities(level: u8, total_entities: usize, include_code: bool, where_filter: String) -> Self {
        Self {
            level,
            timestamp: chrono::Utc::now().to_rfc3339(),
            total_edges: None,
            total_entities: Some(total_entities),
            include_code: Some(include_code),
            where_filter,
        }
    }
}

impl ExportOutput {
    /// Create output for Level 0 (edges)
    pub fn with_edges(edges: Vec<DependencyEdge>, where_filter: String) -> Self {
        let metadata = ExportMetadata::for_level0(edges.len(), where_filter);
        Self {
            export_metadata: metadata,
            edges: Some(edges),
            entities: None,
        }
    }

    /// Create output for Level 1-2 (entities)
    pub fn with_entities(level: u8, entities: serde_json::Value, include_code: bool, where_filter: String) -> Self {
        let total = if let Some(arr) = entities.as_array() {
            arr.len()
        } else {
            0
        };

        let metadata = ExportMetadata::for_entities(level, total, include_code, where_filter);
        Self {
            export_metadata: metadata,
            edges: None,
            entities: Some(entities),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_edge_serialization() {
        let edge = DependencyEdge {
            from_key: "rust:fn:foo:src_lib_rs:10".to_string(),
            to_key: "rust:fn:bar:src_lib_rs:20".to_string(),
            edge_type: "depends_on".to_string(),
        };

        let json = serde_json::to_string(&edge).unwrap();
        assert!(json.contains("rust:fn:foo"));
        assert!(json.contains("depends_on"));
    }

    #[test]
    fn test_entity_level1_null_skipping() {
        let entity = EntityExportLevel1 {
            isgl1_key: "rust:fn:test:src_lib_rs:10".to_string(),
            forward_deps: vec![],  // Should be skipped
            reverse_deps: vec![],  // Should be skipped
            current_ind: 1,
            future_ind: 0,
            future_action: None,  // Should be skipped
            future_code: None,    // Should be skipped
            current_code: None,   // Should be skipped
            entity_name: "test".to_string(),
            entity_type: "fn".to_string(),
            file_path: "src/lib.rs".to_string(),
            line_number: 10,
            interface_signature: "pub fn test()".to_string(),
            // v0.9.0: EntityClass for code/test separation
            entity_class: "CODE".to_string(),
            doc_comment: None,  // Should be skipped
        };

        let json = serde_json::to_string(&entity).unwrap();

        // Verify null-skipping works (field names should NOT appear)
        assert!(!json.contains("future_action"));
        assert!(!json.contains("future_code"));
        assert!(!json.contains("doc_comment"));
    }

    #[test]
    fn test_export_metadata_for_level0() {
        let metadata = ExportMetadata::for_level0(100, "ALL".to_string());

        assert_eq!(metadata.level, 0);
        assert_eq!(metadata.total_edges, Some(100));
        assert_eq!(metadata.total_entities, None);
        assert_eq!(metadata.include_code, None);
    }

    #[test]
    fn test_export_metadata_for_level1() {
        let metadata = ExportMetadata::for_entities(1, 590, false, "ALL".to_string());

        assert_eq!(metadata.level, 1);
        assert_eq!(metadata.total_edges, None);
        assert_eq!(metadata.total_entities, Some(590));
        assert_eq!(metadata.include_code, Some(false));
    }
}

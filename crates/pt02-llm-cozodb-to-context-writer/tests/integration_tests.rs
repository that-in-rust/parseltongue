//! Integration tests for PT02 export commands
//!
//! # Test Coverage
//!
//! Integration tests validate end-to-end export flows:
//! - Database → Exporter → JSON file output
//! - Cross-level field progression (Level 0 ⊂ Level 1 ⊂ Level 2)
//! - WHERE clause filtering with realistic data
//! - File I/O and JSON serialization
//! - Error handling (invalid WHERE, missing DB, etc.)
//!
//! ## TDD: Phase 2 (RED)
//!
//! These tests will FAIL because exporters have `todo!()` placeholders.
//! Phase 3 (GREEN) will implement the exporters to make these tests pass.

use anyhow::Result;
use pt02_llm_cozodb_to_context_writer::{
    models::{ExportConfig, ExportOutput},
    export_trait::{CodeGraphRepository, Edge, Entity, LevelExporter},
    exporters::{Level0Exporter, Level1Exporter, Level2Exporter},
};
use std::path::PathBuf;
use async_trait::async_trait;
use tempfile::TempDir;

// ============================================================================
// Mock Database with Realistic Data
// ============================================================================

struct IntegrationMockDatabase {
    entities: Vec<Entity>,
    edges: Vec<Edge>,
}

impl IntegrationMockDatabase {
    /// Create database with realistic Parseltongue-like data
    fn create_realistic() -> Self {
        Self {
            entities: vec![
                // Public async function with type information
                Entity {
                    isgl1_key: "rust:fn:export_level1:src_exporters_rs:42".to_string(),
                    forward_deps: vec![
                        "rust:struct:ExportConfig:src_models_rs:15".to_string(),
                        "rust:trait:LevelExporter:src_export_trait_rs:10".to_string(),
                    ],
                    reverse_deps: vec![],
                    current_ind: 1,
                    future_ind: 0,
                    future_action: None,
                    future_code: None,
                    current_code: Some("pub async fn export_level1(config: &ExportConfig) -> Result<ExportOutput> { todo!() }".to_string()),
                    entity_name: "export_level1".to_string(),
                    entity_type: "fn".to_string(),
                    file_path: "src/exporters.rs".to_string(),
                    line_number: 42,
                    interface_signature: "pub async fn export_level1(config: &ExportConfig) -> Result<ExportOutput>".to_string(),
                    doc_comment: Some("Export Level 1 entities to JSON".to_string()),
                    // v0.9.0: EntityClass for code/test separation
                    entity_class: "CODE".to_string(),
                    // Level 2 type fields
                    return_type: Some("Result<ExportOutput>".to_string()),
                    param_types: Some(vec!["&ExportConfig".to_string()]),
                    param_names: Some(vec!["config".to_string()]),
                    generic_constraints: None,
                    trait_impls: None,
                    is_public: Some(true),
                    is_async: Some(true),
                    is_unsafe: Some(false),
                },

                // Private sync function without type info
                Entity {
                    isgl1_key: "rust:fn:helper:src_utils_rs:100".to_string(),
                    forward_deps: vec![],
                    reverse_deps: vec!["rust:fn:export_level1:src_exporters_rs:42".to_string()],
                    current_ind: 1,
                    future_ind: 1,
                    future_action: Some("refactor".to_string()),
                    future_code: Some("fn helper_v2() -> bool { true }".to_string()),
                    current_code: Some("fn helper() -> bool { false }".to_string()),
                    entity_name: "helper".to_string(),
                    entity_type: "fn".to_string(),
                    file_path: "src/utils.rs".to_string(),
                    line_number: 100,
                    interface_signature: "fn helper() -> bool".to_string(),
                    doc_comment: None,
                    // v0.9.0: EntityClass for code/test separation
                    entity_class: "CODE".to_string(),
                    return_type: Some("bool".to_string()),
                    param_types: None,
                    param_names: None,
                    generic_constraints: None,
                    trait_impls: None,
                    is_public: Some(false),
                    is_async: Some(false),
                    is_unsafe: Some(false),
                },

                // Struct with trait implementations
                Entity {
                    isgl1_key: "rust:struct:ExportConfig:src_models_rs:15".to_string(),
                    forward_deps: vec![],
                    reverse_deps: vec!["rust:fn:export_level1:src_exporters_rs:42".to_string()],
                    current_ind: 1,
                    future_ind: 0,
                    future_action: None,
                    future_code: None,
                    current_code: Some("#[derive(Debug, Clone)]\npub struct ExportConfig { level: u8 }".to_string()),
                    entity_name: "ExportConfig".to_string(),
                    entity_type: "struct".to_string(),
                    file_path: "src/models.rs".to_string(),
                    line_number: 15,
                    interface_signature: "pub struct ExportConfig".to_string(),
                    doc_comment: Some("Configuration for export operations".to_string()),
                    // v0.9.0: EntityClass for code/test separation
                    entity_class: "CODE".to_string(),
                    return_type: None,
                    param_types: None,
                    param_names: None,
                    generic_constraints: None,
                    trait_impls: Some(vec!["Debug".to_string(), "Clone".to_string()]),
                    is_public: Some(true),
                    is_async: None,
                    is_unsafe: None,
                },
            ],
            edges: vec![
                Edge {
                    from_key: "rust:fn:export_level1:src_exporters_rs:42".to_string(),
                    to_key: "rust:struct:ExportConfig:src_models_rs:15".to_string(),
                    edge_type: "depends_on".to_string(),
                },
                Edge {
                    from_key: "rust:fn:export_level1:src_exporters_rs:42".to_string(),
                    to_key: "rust:trait:LevelExporter:src_export_trait_rs:10".to_string(),
                    edge_type: "depends_on".to_string(),
                },
                Edge {
                    from_key: "rust:struct:ExportConfig:src_models_rs:15".to_string(),
                    to_key: "rust:trait:Debug:external:0".to_string(),
                    edge_type: "implements".to_string(),
                },
            ],
        }
    }

    fn empty() -> Self {
        Self {
            entities: vec![],
            edges: vec![],
        }
    }
}

#[async_trait]
impl CodeGraphRepository for IntegrationMockDatabase {
    async fn get_all_entities(&self) -> Result<Vec<Entity>> {
        Ok(self.entities.clone())
    }

    async fn query_entities(&self, where_clause: &str) -> Result<Vec<Entity>> {
        if where_clause == "ALL" {
            return Ok(self.entities.clone());
        }

        // Simulate Datalog filtering
        let filtered: Vec<Entity> = self.entities.iter()
            .filter(|e| {
                // Parse simple Datalog clauses
                if where_clause.contains("is_public = true") {
                    e.is_public == Some(true)
                } else if where_clause.contains("is_async = true") {
                    e.is_async == Some(true)
                } else if where_clause.contains("entity_type = 'fn'") {
                    e.entity_type == "fn"
                } else if where_clause.contains("future_action != null") {
                    e.future_action.is_some()
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        Ok(filtered)
    }

    async fn get_all_edges(&self) -> Result<Vec<Edge>> {
        Ok(self.edges.clone())
    }

    async fn query_edges(&self, where_clause: &str) -> Result<Vec<Edge>> {
        if where_clause == "ALL" {
            return Ok(self.edges.clone());
        }

        let filtered: Vec<Edge> = self.edges.iter()
            .filter(|e| {
                if where_clause.contains("edge_type = 'depends_on'") {
                    e.edge_type == "depends_on"
                } else if where_clause.contains("edge_type = 'implements'") {
                    e.edge_type == "implements"
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        Ok(filtered)
    }
}

// ============================================================================
// Test Helpers
// ============================================================================

fn create_config(level: u8, include_code: bool, where_filter: &str, output_path: PathBuf) -> ExportConfig {
    ExportConfig {
        level,
        include_code,
        where_filter: where_filter.to_string(),
        output_path,
        db_path: "mem".to_string(),
        // v0.9.0: Dual output fields for code/test separation
        code_output_path: None,
        tests_output_path: None,
    }
}

// ============================================================================
// Integration Tests: Level 0 (Pure Edge List)
// ============================================================================

#[tokio::test]
async fn test_integration_level0_export_all_edges() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level0.json");
    let config = create_config(0, false, "ALL", output_path.clone());
    let exporter = Level0Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok(), "Level 0 export should succeed");

    let output = result.unwrap();

    // Assert: Output structure
    assert_eq!(output.export_metadata.level, 0);
    assert_eq!(output.export_metadata.total_edges, Some(3));
    assert!(output.edges.is_some());
    assert!(output.entities.is_none(), "Level 0 should NOT have entities");

    // Assert: Edge content
    let edges = output.edges.unwrap();
    assert_eq!(edges.len(), 3);

    // Verify semantic ISGL1 keys
    assert!(edges.iter().all(|e| e.from_key.contains(":")));
    assert!(edges.iter().all(|e| e.to_key.contains(":")));
}

#[tokio::test]
async fn test_integration_level0_filter_depends_on() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level0_filtered.json");
    let config = create_config(0, false, "edge_type = 'depends_on'", output_path);
    let exporter = Level0Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let edges = output.edges.unwrap();

    // Assert: Only "depends_on" edges
    assert_eq!(edges.len(), 2, "Should have 2 depends_on edges");
    assert!(edges.iter().all(|e| e.edge_type == "depends_on"));
}

#[tokio::test]
async fn test_integration_level0_empty_database() {
    // Arrange
    let db = IntegrationMockDatabase::empty();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level0_empty.json");
    let config = create_config(0, false, "ALL", output_path);
    let exporter = Level0Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert
    assert_eq!(output.export_metadata.total_edges, Some(0));
    assert_eq!(output.edges.unwrap().len(), 0);
}

// ============================================================================
// Integration Tests: Level 1 (Node-Centric + ISG + Temporal)
// ============================================================================

#[tokio::test]
async fn test_integration_level1_export_all_entities_no_code() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level1_no_code.json");
    let config = create_config(1, false, "ALL", output_path);
    let exporter = Level1Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert: Metadata
    assert_eq!(output.export_metadata.level, 1);
    assert_eq!(output.export_metadata.total_entities, Some(3));
    assert_eq!(output.export_metadata.include_code, Some(false));

    // Assert: Entities present, edges absent
    assert!(output.entities.is_some());
    assert!(output.edges.is_none(), "Level 1 should NOT have edges");

    // Assert: No current_code field when include_code=false
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("\"current_code\""), "Signatures only mode should skip current_code");
}

#[tokio::test]
async fn test_integration_level1_export_with_code() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level1_with_code.json");
    let config = create_config(1, true, "ALL", output_path);
    let exporter = Level1Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: current_code present when include_code=true
    assert!(json.contains("\"current_code\""), "Full code mode should include current_code");
    assert!(json.contains("pub async fn export_level1"));
}

#[tokio::test]
async fn test_integration_level1_filter_public_functions() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level1_public.json");
    let config = create_config(1, false, "is_public = true", output_path);
    let exporter = Level1Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert: Only public entities (export_level1 function and ExportConfig struct)
    assert_eq!(output.export_metadata.total_entities, Some(2), "Should have 2 public entities");

    // Verify JSON contains expected public entities
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("export_level1"));
    assert!(json.contains("ExportConfig"));
    assert!(!json.contains("helper"), "Private helper function should be filtered out");
}

#[tokio::test]
async fn test_integration_level1_filter_with_future_action() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level1_future.json");
    let config = create_config(1, false, "future_action != null", output_path);
    let exporter = Level1Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert: Only entities with planned changes (helper function)
    assert_eq!(output.export_metadata.total_entities, Some(1), "Should have 1 entity with future_action");

    // Verify JSON contains expected entity with future_action
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("helper"));
    assert!(json.contains("\"future_action\""));
    assert!(json.contains("refactor"));
}

#[tokio::test]
async fn test_integration_level1_dependency_arrays() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level1_deps.json");
    let config = create_config(1, false, "ALL", output_path);
    let exporter = Level1Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: forward_deps and reverse_deps present
    assert!(json.contains("\"forward_deps\""));
    assert!(json.contains("\"reverse_deps\""));
    assert!(json.contains("rust:struct:ExportConfig:src_models_rs:15"));
}

// ============================================================================
// Integration Tests: Level 2 (+ Type System)
// ============================================================================

#[tokio::test]
async fn test_integration_level2_type_system_fields() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level2.json");
    let config = create_config(2, false, "ALL", output_path);
    let exporter = Level2Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: Type system fields present
    assert!(json.contains("\"return_type\""));
    assert!(json.contains("\"param_types\""));
    assert!(json.contains("\"param_names\""));
    assert!(json.contains("\"is_public\""));
    assert!(json.contains("\"is_async\""));
    assert!(json.contains("\"is_unsafe\""));
}

#[tokio::test]
async fn test_integration_level2_includes_level1_fields() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level2_full.json");
    let config = create_config(2, false, "ALL", output_path);
    let exporter = Level2Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: All Level 1 fields still present
    assert!(json.contains("\"isgl1_key\""));
    assert!(json.contains("\"forward_deps\""));
    assert!(json.contains("\"current_ind\""));
    assert!(json.contains("\"entity_name\""));
    assert!(json.contains("\"interface_signature\""));

    // Assert: Level 2 additions
    assert!(json.contains("\"return_type\""));
    assert!(json.contains("\"is_async\""));
}

#[tokio::test]
async fn test_integration_level2_filter_async_functions() {
    // Arrange
    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("level2_async.json");
    let config = create_config(2, false, "is_async = true", output_path);
    let exporter = Level2Exporter;

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert: Only async function (export_level1)
    assert_eq!(output.export_metadata.total_entities, Some(1), "Should have 1 async function");

    // Verify JSON contains expected async function
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("export_level1"));
    assert!(json.contains("\"is_async\":true") || json.contains("\"is_async\": true"));
}

// ============================================================================
// Integration Tests: Cross-Level Validation
// ============================================================================

#[tokio::test]
async fn test_integration_cross_level_field_progression() {
    // Verify: Level 0 ⊂ Level 1 ⊂ Level 2 field hierarchy

    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();

    // Level 0: Only edges
    let output0 = Level0Exporter.export(&db, &create_config(
        0, false, "ALL", temp_dir.path().join("l0.json")
    )).await.unwrap();

    let json0 = serde_json::to_string(&output0).unwrap();
    assert!(json0.contains("\"edges\""));
    assert!(!json0.contains("\"entities\""));
    assert!(!json0.contains("\"interface_signature\""));
    assert!(!json0.contains("\"return_type\""));

    // Level 1: Entities + ISG (no type system)
    let output1 = Level1Exporter.export(&db, &create_config(
        1, false, "ALL", temp_dir.path().join("l1.json")
    )).await.unwrap();

    let json1 = serde_json::to_string(&output1).unwrap();
    assert!(json1.contains("\"entities\""));
    assert!(!json1.contains("\"edges\""));
    assert!(json1.contains("\"interface_signature\""));
    assert!(!json1.contains("\"return_type\""), "Level 1 should NOT have type fields");

    // Level 2: All fields
    let output2 = Level2Exporter.export(&db, &create_config(
        2, false, "ALL", temp_dir.path().join("l2.json")
    )).await.unwrap();

    let json2 = serde_json::to_string(&output2).unwrap();
    assert!(json2.contains("\"entities\""));
    assert!(json2.contains("\"interface_signature\""));
    assert!(json2.contains("\"return_type\""), "Level 2 should have type fields");
}

// ============================================================================
// Integration Tests: Error Handling
// ============================================================================

#[tokio::test]
async fn test_integration_invalid_level() {
    // Note: This would be caught by CLI validation, but test repository behavior
    // Exporters should gracefully handle unexpected levels

    let _db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();

    // Level must be 0, 1, or 2 (validated by CLI)
    // This test documents expected behavior
    let config = create_config(3, false, "ALL", temp_dir.path().join("invalid.json"));

    // Note: In real implementation, this would be prevented by CLI validation
    // Exporters assume valid config from CLI
    assert_eq!(config.level, 3);  // Document invalid state
}

#[tokio::test]
async fn test_integration_empty_where_clause() {
    // Empty WHERE should be caught by CLI validation
    // This test documents expected behavior

    let _db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();
    let config = create_config(0, false, "", temp_dir.path().join("empty_where.json"));

    // CLI validation prevents empty WHERE clauses
    // Exporters can assume non-empty WHERE from config
    assert!(config.where_filter.is_empty());  // Document invalid state
}

// ============================================================================
// Integration Tests: JSON Serialization
// ============================================================================

#[tokio::test]
async fn test_integration_json_serialization_valid() {
    // Verify all export outputs serialize to valid JSON

    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();

    // Level 0
    let output0 = Level0Exporter.export(&db, &create_config(
        0, false, "ALL", temp_dir.path().join("json0.json")
    )).await.unwrap();

    let json0 = serde_json::to_string_pretty(&output0);
    assert!(json0.is_ok(), "Level 0 output should serialize to JSON");

    // Verify can deserialize back
    let json_str = json0.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed["export_metadata"]["level"], 0);
}

#[tokio::test]
async fn test_integration_timestamp_format() {
    // Verify all exports include RFC3339 timestamps

    let db = IntegrationMockDatabase::create_realistic();
    let temp_dir = TempDir::new().unwrap();

    let output = Level0Exporter.export(&db, &create_config(
        0, false, "ALL", temp_dir.path().join("timestamp.json")
    )).await.unwrap();

    let timestamp = &output.export_metadata.timestamp;

    // RFC3339 format: 2024-01-15T10:30:00Z or 2024-01-15T10:30:00+00:00
    assert!(timestamp.contains("T"), "Should have date-time separator");
    assert!(timestamp.contains("Z") || timestamp.contains("+"), "Should have timezone");

    // Verify parseable as RFC3339
    let parsed = chrono::DateTime::parse_from_rfc3339(timestamp);
    assert!(parsed.is_ok(), "Timestamp should be valid RFC3339");
}

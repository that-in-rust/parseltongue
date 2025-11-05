//! Level 0 unit tests
//!
//! # Test Coverage
//!
//! Level 0 exports pure edge lists (from_key, to_key, edge_type).
//! These tests validate:
//! - Edge export with "ALL" filter
//! - Edge export with Datalog WHERE clause
//! - Edge structure (3 fields only)
//! - Metadata generation
//! - Empty database handling
//! - Error handling (invalid WHERE clause)
//!
//! ## TDD: Phase 2 (RED)
//!
//! All tests will FAIL initially because Level0Exporter has `todo!()` placeholders.
//! Phase 3 (GREEN) will implement the minimum code to pass these tests.

use anyhow::Result;
use pt02_llm_cozodb_to_context_writer::{
    models::{DependencyEdge, ExportConfig},
    export_trait::{CodeGraphRepository, Edge, Entity, LevelExporter},
    exporters::Level0Exporter,
};
use std::path::PathBuf;
use async_trait::async_trait;

// ============================================================================
// Mock Database (for testing without real CozoDB)
// ============================================================================

struct MockDatabase {
    edges: Vec<Edge>,
    entities: Vec<Entity>,
}

impl MockDatabase {
    fn new() -> Self {
        Self {
            edges: vec![],
            entities: vec![],
        }
    }

    fn with_edges(edges: Vec<Edge>) -> Self {
        Self {
            edges,
            entities: vec![],
        }
    }
}

#[async_trait]
impl CodeGraphRepository for MockDatabase {
    async fn get_all_entities(&self) -> Result<Vec<Entity>> {
        Ok(self.entities.clone())
    }

    async fn query_entities(&self, _where_clause: &str) -> Result<Vec<Entity>> {
        // Simple filter implementation for testing
        Ok(self.entities.clone())
    }

    async fn get_all_edges(&self) -> Result<Vec<Edge>> {
        Ok(self.edges.clone())
    }

    async fn query_edges(&self, where_clause: &str) -> Result<Vec<Edge>> {
        // Simple filter implementation for testing
        if where_clause == "ALL" {
            Ok(self.edges.clone())
        } else if where_clause.contains("edge_type = 'depends_on'") {
            Ok(self.edges.iter()
                .filter(|e| e.edge_type == "depends_on")
                .cloned()
                .collect())
        } else {
            Ok(self.edges.clone())
        }
    }
}

// ============================================================================
// Test Helpers
// ============================================================================

fn create_test_edge(from: &str, to: &str, edge_type: &str) -> Edge {
    Edge {
        from_key: from.to_string(),
        to_key: to.to_string(),
        edge_type: edge_type.to_string(),
    }
}

fn create_test_config(level: u8, where_filter: &str) -> ExportConfig {
    ExportConfig {
        level,
        include_code: false,  // N/A for Level 0
        where_filter: where_filter.to_string(),
        output_path: PathBuf::from("test.json"),
        db_path: "mem".to_string(),
        // v0.9.0: Dual output fields for code/test separation
        code_output_path: None,
        tests_output_path: None,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[tokio::test]
async fn test_level0_export_all_edges() {
    // Arrange: Create mock database with 3 edges
    let edges = vec![
        create_test_edge(
            "rust:fn:calculate_total:src_billing_rs:42",
            "rust:fn:get_tax_rate:src_billing_rs:102",
            "depends_on"
        ),
        create_test_edge(
            "rust:fn:calculate_total:src_billing_rs:42",
            "rust:struct:Invoice:src_models_rs:15",
            "depends_on"
        ),
        create_test_edge(
            "rust:struct:Invoice:src_models_rs:15",
            "rust:trait:Serialize:external:0",
            "implements"
        ),
    ];

    let db = MockDatabase::with_edges(edges);
    let config = create_test_config(0, "ALL");
    let exporter = Level0Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;

    // Assert
    assert!(result.is_ok(), "Export should succeed");
    let output = result.unwrap();

    // Verify metadata
    assert_eq!(output.export_metadata.level, 0);
    assert_eq!(output.export_metadata.total_edges, Some(3));
    assert_eq!(output.export_metadata.total_entities, None);
    assert_eq!(output.export_metadata.include_code, None);
    assert_eq!(output.export_metadata.where_filter, "ALL");

    // Verify edges present, entities absent
    assert!(output.edges.is_some());
    assert!(output.entities.is_none());

    let exported_edges = output.edges.unwrap();
    assert_eq!(exported_edges.len(), 3);

    // Verify first edge structure
    let edge0 = &exported_edges[0];
    assert_eq!(edge0.from_key, "rust:fn:calculate_total:src_billing_rs:42");
    assert_eq!(edge0.to_key, "rust:fn:get_tax_rate:src_billing_rs:102");
    assert_eq!(edge0.edge_type, "depends_on");
}

#[tokio::test]
async fn test_level0_export_filtered_edges() {
    // Arrange: Database with multiple edge types
    let edges = vec![
        create_test_edge("rust:fn:foo:src_lib_rs:10", "rust:fn:bar:src_lib_rs:20", "depends_on"),
        create_test_edge("rust:struct:Foo:src_lib_rs:30", "rust:trait:Debug:external:0", "implements"),
        create_test_edge("rust:fn:baz:src_lib_rs:40", "rust:fn:qux:src_lib_rs:50", "depends_on"),
    ];

    let db = MockDatabase::with_edges(edges);
    let config = create_test_config(0, "edge_type = 'depends_on'");
    let exporter = Level0Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;

    // Assert
    assert!(result.is_ok());
    let output = result.unwrap();
    let exported_edges = output.edges.unwrap();

    // Should only have 2 "depends_on" edges
    assert_eq!(exported_edges.len(), 2);
    assert!(exported_edges.iter().all(|e| e.edge_type == "depends_on"));
}

#[tokio::test]
async fn test_level0_empty_database() {
    // Arrange: Empty database
    let db = MockDatabase::new();
    let config = create_test_config(0, "ALL");
    let exporter = Level0Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;

    // Assert
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.export_metadata.total_edges, Some(0));
    assert_eq!(output.edges.unwrap().len(), 0);
}

#[tokio::test]
async fn test_level0_semantic_isgl1_keys() {
    // Arrange: Verify semantic ISGL1 keys (NOT integer indices)
    let edges = vec![
        create_test_edge(
            "rust:fn:calculate_total:src_billing_rs:42",
            "rust:fn:get_tax_rate:src_billing_rs:102",
            "depends_on"
        ),
    ];

    let db = MockDatabase::with_edges(edges);
    let config = create_test_config(0, "ALL");
    let exporter = Level0Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;

    // Assert
    assert!(result.is_ok());
    let output = result.unwrap();
    let exported_edges = output.edges.unwrap();

    // Verify keys are semantic, not numeric indices
    let edge = &exported_edges[0];
    assert!(edge.from_key.contains(":"), "ISGL1 key should be semantic (colon-separated)");
    assert!(edge.from_key.contains("rust"), "ISGL1 key should contain language");
    assert!(edge.from_key.contains("fn"), "ISGL1 key should contain entity type");
    assert!(edge.from_key.contains("calculate_total"), "ISGL1 key should contain entity name");

    // Verify NOT integer indices
    assert!(!edge.from_key.chars().all(|c| c.is_numeric()), "Keys should NOT be pure integers");
}

#[tokio::test]
async fn test_level0_exporter_metadata() {
    // Arrange
    let exporter = Level0Exporter::new();

    // Act & Assert
    assert_eq!(exporter.level(), 0);
    assert_eq!(exporter.estimated_tokens(), 5000);
}

#[tokio::test]
async fn test_level0_json_serialization() {
    // Arrange
    let edges = vec![
        create_test_edge("rust:fn:foo:src_lib_rs:10", "rust:fn:bar:src_lib_rs:20", "depends_on"),
    ];

    let db = MockDatabase::with_edges(edges);
    let config = create_test_config(0, "ALL");
    let exporter = Level0Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string_pretty(&output);

    // Assert: JSON should serialize successfully
    assert!(json.is_ok(), "Output should serialize to JSON");

    let json_str = json.unwrap();
    assert!(json_str.contains("\"level\": 0"));
    assert!(json_str.contains("\"from_key\""));
    assert!(json_str.contains("\"to_key\""));
    assert!(json_str.contains("\"edge_type\""));
}

#[tokio::test]
async fn test_level0_no_code_fields() {
    // Arrange: Verify Level 0 has NO code fields (current_code, future_code)
    let edges = vec![
        create_test_edge("rust:fn:foo:src_lib_rs:10", "rust:fn:bar:src_lib_rs:20", "depends_on"),
    ];

    let db = MockDatabase::with_edges(edges);
    let config = create_test_config(0, "ALL");
    let exporter = Level0Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: No code-related fields in JSON
    assert!(!json.contains("current_code"), "Level 0 should NOT have current_code");
    assert!(!json.contains("future_code"), "Level 0 should NOT have future_code");
    assert!(!json.contains("interface_signature"), "Level 0 should NOT have interface_signature");

    // Only edge fields should be present
    assert!(json.contains("from_key"));
    assert!(json.contains("to_key"));
    assert!(json.contains("edge_type"));
}

#[tokio::test]
async fn test_level0_timestamp_format() {
    // Arrange
    let db = MockDatabase::new();
    let config = create_test_config(0, "ALL");
    let exporter = Level0Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert: Timestamp should be RFC3339 format
    let timestamp = &output.export_metadata.timestamp;
    assert!(timestamp.contains("T"), "Timestamp should be RFC3339 (contains T separator)");
    assert!(timestamp.contains("Z") || timestamp.contains("+"), "Timestamp should have timezone");
}

#[tokio::test]
async fn test_level0_edge_count_accuracy() {
    // Arrange: Test that metadata.total_edges matches actual edge count
    let edges = vec![
        create_test_edge("a", "b", "depends_on"),
        create_test_edge("b", "c", "depends_on"),
        create_test_edge("c", "d", "depends_on"),
        create_test_edge("d", "e", "depends_on"),
        create_test_edge("e", "f", "depends_on"),
    ];

    let db = MockDatabase::with_edges(edges);
    let config = create_test_config(0, "ALL");
    let exporter = Level0Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert
    assert_eq!(output.export_metadata.total_edges, Some(5));
    assert_eq!(output.edges.as_ref().unwrap().len(), 5);
}

#[test]
fn test_level0_dependency_edge_clone() {
    // Test that DependencyEdge implements Clone (needed for conversions)
    let edge = DependencyEdge {
        from_key: "test_from".to_string(),
        to_key: "test_to".to_string(),
        edge_type: "test_type".to_string(),
    };

    let cloned = edge.clone();
    assert_eq!(cloned.from_key, "test_from");
    assert_eq!(cloned.to_key, "test_to");
    assert_eq!(cloned.edge_type, "test_type");
}

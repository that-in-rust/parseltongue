//! Level 1 unit tests
//!
//! # Test Coverage
//!
//! Level 1 exports node-centric entities with ISG + temporal state.
//! These tests validate:
//! - Entity export with forward_deps/reverse_deps arrays
//! - Interface signature (ISG core innovation)
//! - Temporal state (current_ind, future_ind, future_action, future_code)
//! - current_code inclusion based on --include-code flag
//! - Datalog WHERE clause filtering
//! - Null-skipping serde optimization
//! - Metadata generation
//!
//! ## Variables (13 total)
//! - Graph: isgl1_key, forward_deps, reverse_deps
//! - Temporal: current_ind, future_ind, future_action, future_code, current_code
//! - Identity: entity_name, entity_type, file_path, line_number
//! - ISG: interface_signature, doc_comment
//!
//! ## TDD: Phase 2 (RED)
//!
//! All tests will FAIL initially because Level1Exporter has `todo!()` placeholders.

use anyhow::Result;
use pt02_llm_cozodb_to_context_writer::{
    models::{EntityExportLevel1, ExportConfig},
    export_trait::{CodeGraphRepository, Edge, Entity, LevelExporter},
};
use std::path::PathBuf;
use async_trait::async_trait;

// ============================================================================
// Mock Database
// ============================================================================

struct MockDatabase {
    entities: Vec<Entity>,
}

impl MockDatabase {
    fn with_entities(entities: Vec<Entity>) -> Self {
        Self { entities }
    }
}

#[async_trait]
impl CodeGraphRepository for MockDatabase {
    async fn get_all_entities(&self) -> Result<Vec<Entity>> {
        Ok(self.entities.clone())
    }

    async fn query_entities(&self, where_clause: &str) -> Result<Vec<Entity>> {
        if where_clause == "ALL" {
            Ok(self.entities.clone())
        } else if where_clause.contains("is_public = true") {
            Ok(self.entities.iter()
                .filter(|e| e.is_public == Some(true))
                .cloned()
                .collect())
        } else if where_clause.contains("future_action != null") {
            Ok(self.entities.iter()
                .filter(|e| e.future_action.is_some())
                .cloned()
                .collect())
        } else {
            Ok(self.entities.clone())
        }
    }

    async fn get_all_edges(&self) -> Result<Vec<Edge>> {
        Ok(vec![])
    }

    async fn query_edges(&self, _where_clause: &str) -> Result<Vec<Edge>> {
        Ok(vec![])
    }
}

// ============================================================================
// Level 1 Exporter (stub)
// ============================================================================

struct Level1Exporter;

impl Level1Exporter {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LevelExporter for Level1Exporter {
    async fn export(
        &self,
        _db: &dyn CodeGraphRepository,
        _config: &ExportConfig,
    ) -> Result<pt02_llm_cozodb_to_context_writer::models::ExportOutput> {
        todo!("Level 1 export implementation (Phase 3)")
    }

    fn level(&self) -> u8 {
        1
    }

    fn estimated_tokens(&self) -> usize {
        // Estimate: ~30K tokens for 590 entities (no code)
        30_000
    }
}

// ============================================================================
// Test Helpers
// ============================================================================

fn create_test_entity(isgl1_key: &str, has_future_action: bool) -> Entity {
    Entity {
        isgl1_key: isgl1_key.to_string(),
        forward_deps: vec!["rust:fn:dependency1:src_lib_rs:100".to_string()],
        reverse_deps: vec!["rust:fn:caller1:src_lib_rs:200".to_string()],
        current_ind: 1,
        future_ind: if has_future_action { 1 } else { 0 },
        future_action: if has_future_action { Some("Edit".to_string()) } else { None },
        future_code: if has_future_action {
            Some("pub fn updated() { /* new code */ }".to_string())
        } else {
            None
        },
        current_code: Some("pub fn original() { /* old code */ }".to_string()),
        entity_name: "test_function".to_string(),
        entity_type: "fn".to_string(),
        file_path: "src/lib.rs".to_string(),
        line_number: 42,
        interface_signature: "pub fn test_function() -> Result<()>".to_string(),
        doc_comment: Some("Test function documentation".to_string()),
        // Level 2 fields (None for Level 1)
        return_type: None,
        param_types: None,
        param_names: None,
        generic_constraints: None,
        trait_impls: None,
        is_public: Some(true),
        is_async: Some(false),
        is_unsafe: Some(false),
    }
}

fn create_test_config(include_code: bool, where_filter: &str) -> ExportConfig {
    ExportConfig {
        level: 1,
        include_code,
        where_filter: where_filter.to_string(),
        output_path: PathBuf::from("test.json"),
        db_path: "mem".to_string(),
    }
}

// ============================================================================
// Tests: Basic Export
// ============================================================================

#[tokio::test]
async fn test_level1_export_all_entities() {
    // Arrange: 3 entities with different states
    let entities = vec![
        create_test_entity("rust:fn:foo:src_lib_rs:10", false),
        create_test_entity("rust:fn:bar:src_lib_rs:20", true),
        create_test_entity("rust:fn:baz:src_lib_rs:30", false),
    ];

    let db = MockDatabase::with_entities(entities);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;

    // Assert
    assert!(result.is_ok(), "Export should succeed");
    let output = result.unwrap();

    // Verify metadata
    assert_eq!(output.export_metadata.level, 1);
    assert_eq!(output.export_metadata.total_entities, Some(3));
    assert_eq!(output.export_metadata.total_edges, None);
    assert_eq!(output.export_metadata.include_code, Some(false));

    // Verify entities present, edges absent
    assert!(output.entities.is_some());
    assert!(output.edges.is_none());
}

#[tokio::test]
async fn test_level1_include_code_false() {
    // Arrange
    let entities = vec![create_test_entity("rust:fn:test:src_lib_rs:10", false)];
    let db = MockDatabase::with_entities(entities);
    let config = create_test_config(false, "ALL");  // include_code = false
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: current_code should NOT be present
    assert!(!json.contains("\"current_code\""),
            "current_code should be excluded when include_code=false");
}

#[tokio::test]
async fn test_level1_include_code_true() {
    // Arrange
    let entities = vec![create_test_entity("rust:fn:test:src_lib_rs:10", false)];
    let db = MockDatabase::with_entities(entities);
    let config = create_test_config(true, "ALL");  // include_code = true
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: current_code should be present
    assert!(json.contains("\"current_code\""),
            "current_code should be included when include_code=true");
    assert!(json.contains("pub fn original"),
            "current_code content should be present");
}

// ============================================================================
// Tests: Temporal State
// ============================================================================

#[tokio::test]
async fn test_level1_future_code_only_when_future_action_present() {
    // Arrange: 2 entities - one with future_action, one without
    let entities = vec![
        create_test_entity("rust:fn:unchanged:src_lib_rs:10", false),  // No future_action
        create_test_entity("rust:fn:changed:src_lib_rs:20", true),     // Has future_action
    ];

    let db = MockDatabase::with_entities(entities);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: future_code appears exactly once (only for changed entity)
    let future_code_count = json.matches("\"future_code\"").count();
    assert_eq!(future_code_count, 1,
               "future_code should only appear for entity with future_action");
}

#[tokio::test]
async fn test_level1_temporal_state_fields() {
    // Arrange
    let mut entity = create_test_entity("rust:fn:test:src_lib_rs:10", true);
    entity.current_ind = 1;
    entity.future_ind = 1;
    entity.future_action = Some("Edit".to_string());

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: All temporal fields present
    assert!(json.contains("\"current_ind\": 1"));
    assert!(json.contains("\"future_ind\": 1"));
    assert!(json.contains("\"future_action\": \"Edit\""));
    assert!(json.contains("\"future_code\""));
}

// ============================================================================
// Tests: ISG (Interface Signature Graph)
// ============================================================================

#[tokio::test]
async fn test_level1_interface_signature_present() {
    // Arrange
    let entities = vec![create_test_entity("rust:fn:test:src_lib_rs:10", false)];
    let db = MockDatabase::with_entities(entities);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: interface_signature is core Level 1 field
    assert!(json.contains("\"interface_signature\""));
    assert!(json.contains("pub fn test_function() -> Result<()>"));
}

#[tokio::test]
async fn test_level1_doc_comment_optional() {
    // Arrange: Entity without doc_comment
    let mut entity = create_test_entity("rust:fn:test:src_lib_rs:10", false);
    entity.doc_comment = None;

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: doc_comment field should be skipped (null-skipping)
    assert!(!json.contains("\"doc_comment\""),
            "doc_comment should be skipped when null (serde optimization)");
}

// ============================================================================
// Tests: Dependency Graph (Node-Centric View)
// ============================================================================

#[tokio::test]
async fn test_level1_forward_deps_array() {
    // Arrange
    let mut entity = create_test_entity("rust:fn:test:src_lib_rs:10", false);
    entity.forward_deps = vec![
        "rust:fn:dep1:src_lib_rs:100".to_string(),
        "rust:fn:dep2:src_lib_rs:200".to_string(),
        "rust:struct:Foo:src_models_rs:50".to_string(),
    ];

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: All dependencies present as semantic ISGL1 keys
    assert!(json.contains("rust:fn:dep1:src_lib_rs:100"));
    assert!(json.contains("rust:fn:dep2:src_lib_rs:200"));
    assert!(json.contains("rust:struct:Foo:src_models_rs:50"));
}

#[tokio::test]
async fn test_level1_reverse_deps_array() {
    // Arrange
    let mut entity = create_test_entity("rust:fn:test:src_lib_rs:10", false);
    entity.reverse_deps = vec![
        "rust:fn:caller1:src_main_rs:10".to_string(),
        "rust:fn:caller2:src_api_rs:20".to_string(),
    ];

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert
    assert!(json.contains("rust:fn:caller1:src_main_rs:10"));
    assert!(json.contains("rust:fn:caller2:src_api_rs:20"));
}

#[tokio::test]
async fn test_level1_empty_deps_arrays_skipped() {
    // Arrange: Entity with no dependencies
    let mut entity = create_test_entity("rust:fn:test:src_lib_rs:10", false);
    entity.forward_deps = vec![];
    entity.reverse_deps = vec![];

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: Empty arrays should be skipped (serde optimization)
    // Note: serde skip_serializing_if = "Vec::is_empty" should hide these
    let forward_deps_count = json.matches("\"forward_deps\"").count();
    let reverse_deps_count = json.matches("\"reverse_deps\"").count();

    assert_eq!(forward_deps_count, 0, "Empty forward_deps should be skipped");
    assert_eq!(reverse_deps_count, 0, "Empty reverse_deps should be skipped");
}

// ============================================================================
// Tests: Core Identity Fields
// ============================================================================

#[tokio::test]
async fn test_level1_core_identity_fields() {
    // Arrange
    let entity = create_test_entity("rust:fn:calculate_total:src_billing_rs:42", false);
    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: All core identity fields present
    assert!(json.contains("\"entity_name\": \"test_function\""));
    assert!(json.contains("\"entity_type\": \"fn\""));
    assert!(json.contains("\"file_path\": \"src/lib.rs\""));
    assert!(json.contains("\"line_number\": 42"));
}

// ============================================================================
// Tests: WHERE Clause Filtering
// ============================================================================

#[tokio::test]
async fn test_level1_where_clause_filter_public() {
    // Arrange: 3 entities - 2 public, 1 private
    let mut entity1 = create_test_entity("rust:fn:public1:src_lib_rs:10", false);
    entity1.is_public = Some(true);

    let mut entity2 = create_test_entity("rust:fn:private:src_lib_rs:20", false);
    entity2.is_public = Some(false);

    let mut entity3 = create_test_entity("rust:fn:public2:src_lib_rs:30", false);
    entity3.is_public = Some(true);

    let db = MockDatabase::with_entities(vec![entity1, entity2, entity3]);
    let config = create_test_config(false, "is_public = true");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert: Only 2 public entities
    assert_eq!(output.export_metadata.total_entities, Some(2));
}

#[tokio::test]
async fn test_level1_where_clause_future_action() {
    // Arrange: 3 entities - 1 with future_action, 2 without
    let entities = vec![
        create_test_entity("rust:fn:unchanged1:src_lib_rs:10", false),
        create_test_entity("rust:fn:changed:src_lib_rs:20", true),
        create_test_entity("rust:fn:unchanged2:src_lib_rs:30", false),
    ];

    let db = MockDatabase::with_entities(entities);
    let config = create_test_config(false, "future_action != null");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert: Only 1 entity with future_action
    assert_eq!(output.export_metadata.total_entities, Some(1));
}

// ============================================================================
// Tests: Serde Null-Skipping Optimization
// ============================================================================

#[tokio::test]
async fn test_level1_null_skipping_token_savings() {
    // Arrange: Entity with many null fields
    let mut entity = create_test_entity("rust:fn:test:src_lib_rs:10", false);
    entity.future_action = None;
    entity.future_code = None;
    entity.doc_comment = None;
    entity.forward_deps = vec![];
    entity.reverse_deps = vec![];

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: Null/empty fields should be skipped
    assert!(!json.contains("\"future_action\""), "Null future_action should be skipped");
    assert!(!json.contains("\"future_code\""), "Null future_code should be skipped");
    assert!(!json.contains("\"doc_comment\""), "Null doc_comment should be skipped");
    assert!(!json.contains("\"forward_deps\""), "Empty forward_deps should be skipped");
    assert!(!json.contains("\"reverse_deps\""), "Empty reverse_deps should be skipped");

    // Verify this reduces token count
    assert!(json.len() < 500, "Null-skipping should keep JSON compact");
}

// ============================================================================
// Tests: Metadata & Token Estimation
// ============================================================================

#[tokio::test]
async fn test_level1_exporter_metadata() {
    let exporter = Level1Exporter::new();

    // Assert
    assert_eq!(exporter.level(), 1);
    assert_eq!(exporter.estimated_tokens(), 30_000);
}

#[tokio::test]
async fn test_level1_metadata_accuracy() {
    // Arrange
    let entities = vec![
        create_test_entity("rust:fn:test1:src_lib_rs:10", false),
        create_test_entity("rust:fn:test2:src_lib_rs:20", false),
        create_test_entity("rust:fn:test3:src_lib_rs:30", false),
    ];

    let db = MockDatabase::with_entities(entities);
    let config = create_test_config(true, "ALL");
    let exporter = Level1Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert metadata fields
    assert_eq!(output.export_metadata.level, 1);
    assert_eq!(output.export_metadata.total_entities, Some(3));
    assert_eq!(output.export_metadata.include_code, Some(true));
    assert_eq!(output.export_metadata.where_filter, "ALL");
    assert!(output.export_metadata.timestamp.contains("T"));
}

// ============================================================================
// Tests: EntityExportLevel1 Structure
// ============================================================================

#[test]
fn test_level1_entity_struct_clone() {
    // Test that EntityExportLevel1 implements Clone
    let entity = EntityExportLevel1 {
        isgl1_key: "test".to_string(),
        forward_deps: vec![],
        reverse_deps: vec![],
        current_ind: 1,
        future_ind: 0,
        future_action: None,
        future_code: None,
        current_code: None,
        entity_name: "test".to_string(),
        entity_type: "fn".to_string(),
        file_path: "test.rs".to_string(),
        line_number: 10,
        interface_signature: "pub fn test()".to_string(),
        doc_comment: None,
    };

    let cloned = entity.clone();
    assert_eq!(cloned.isgl1_key, "test");
    assert_eq!(cloned.entity_name, "test");
}

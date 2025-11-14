//! Level 2 unit tests
//!
//! # Test Coverage
//!
//! Level 2 exports everything from Level 1 PLUS type system essentials.
//! These tests validate:
//! - All Level 1 fields (inherited)
//! - Type system fields: return_type, param_types, param_names
//! - Generic constraints, trait implementations
//! - Safety flags: is_public, is_async, is_unsafe
//! - Null-skipping for optional type fields
//!
//! ## Variables Added (8 additional beyond Level 1)
//! - return_type, param_types, param_names, generic_constraints
//! - trait_impls, is_public, is_async, is_unsafe
//!
//! ## TDD: Phase 2 (RED)
//!
//! All tests will FAIL initially because Level2Exporter has `todo!()` placeholders.

use anyhow::Result;
use pt02_llm_cozodb_to_context_writer::{
    models::{EntityExportLevel2, ExportConfig},
    export_trait::{CodeGraphRepository, Edge, Entity, LevelExporter},
    exporters::Level2Exporter,
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
// Test Helpers
// ============================================================================

fn create_typed_entity(isgl1_key: &str) -> Entity {
    Entity {
        isgl1_key: isgl1_key.to_string(),
        forward_deps: vec![],
        reverse_deps: vec![],
        current_ind: 1,
        future_ind: 0,
        future_action: None,
        future_code: None,
        current_code: Some("pub fn test() -> Result<()> { Ok(()) }".to_string()),
        entity_name: "test_function".to_string(),
        entity_type: "fn".to_string(),
        file_path: "src/lib.rs".to_string(),
        line_number: 42,
        interface_signature: "pub fn test_function() -> Result<()>".to_string(),
        doc_comment: Some("Test function".to_string()),
        // v0.9.0: EntityClass for code/test separation
        entity_class: "CODE".to_string(),
        // Level 2 fields (type system)
        return_type: Some("Result<()>".to_string()),
        param_types: Some(vec!["&str".to_string(), "i32".to_string()]),
        param_names: Some(vec!["name".to_string(), "count".to_string()]),
        generic_constraints: Some(vec!["T: Clone".to_string()]),
        trait_impls: Some(vec!["Debug".to_string(), "Clone".to_string()]),
        is_public: Some(true),
        is_async: Some(false),
        is_unsafe: Some(false),
    }
}

fn create_test_config(include_code: bool, where_filter: &str) -> ExportConfig {
    ExportConfig {
        level: 2,
        include_code,
        where_filter: where_filter.to_string(),
        output_path: PathBuf::from("test.json"),
        db_path: "mem".to_string(),
        // v0.9.0: Dual output fields for code/test separation
        code_output_path: None,
        tests_output_path: None,
        // v0.9.7: Timestamped folder creation (None for tests)
        session_timestamp: None,
    }
}

// ============================================================================
// Tests: Type System Fields
// ============================================================================

#[tokio::test]
async fn test_level2_return_type() {
    // Arrange
    let entity = create_typed_entity("rust:fn:test:src_lib_rs:10");
    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert
    assert!(json.contains("\"return_type\":\"Result<()>\"") || json.contains("\"return_type\": \"Result<()>\""));
}

#[tokio::test]
async fn test_level2_param_types_and_names() {
    // Arrange
    let entity = create_typed_entity("rust:fn:test:src_lib_rs:10");
    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: param_types and param_names are parallel arrays
    assert!(json.contains("\"param_types\""));
    assert!(json.contains("\"&str\""));
    assert!(json.contains("\"i32\""));

    assert!(json.contains("\"param_names\""));
    assert!(json.contains("\"name\""));
    assert!(json.contains("\"count\""));
}

#[tokio::test]
async fn test_level2_generic_constraints() {
    // Arrange
    let entity = create_typed_entity("rust:fn:test:src_lib_rs:10");
    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert
    assert!(json.contains("\"generic_constraints\""));
    assert!(json.contains("\"T: Clone\""));
}

#[tokio::test]
async fn test_level2_trait_impls() {
    // Arrange
    let entity = create_typed_entity("rust:fn:test:src_lib_rs:10");
    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert
    assert!(json.contains("\"trait_impls\""));
    assert!(json.contains("\"Debug\""));
    assert!(json.contains("\"Clone\""));
}

// ============================================================================
// Tests: Safety Flags
// ============================================================================

#[tokio::test]
async fn test_level2_safety_flags() {
    // Arrange
    let entity = create_typed_entity("rust:fn:test:src_lib_rs:10");
    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: All safety flags present
    assert!(json.contains("\"is_public\":true") || json.contains("\"is_public\": true"));
    assert!(json.contains("\"is_async\":false") || json.contains("\"is_async\": false"));
    assert!(json.contains("\"is_unsafe\":false") || json.contains("\"is_unsafe\": false"));
}

#[tokio::test]
async fn test_level2_async_function() {
    // Arrange
    let mut entity = create_typed_entity("rust:fn:async_test:src_lib_rs:10");
    entity.is_async = Some(true);
    entity.interface_signature = "pub async fn async_test() -> Result<()>".to_string();

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert
    assert!(json.contains("\"is_async\":true") || json.contains("\"is_async\": true"));
    assert!(json.contains("async fn async_test"));
}

#[tokio::test]
async fn test_level2_unsafe_function() {
    // Arrange
    let mut entity = create_typed_entity("rust:fn:unsafe_test:src_lib_rs:10");
    entity.is_unsafe = Some(true);
    entity.interface_signature = "pub unsafe fn unsafe_test()".to_string();

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert
    assert!(json.contains("\"is_unsafe\":true") || json.contains("\"is_unsafe\": true"));
}

// ============================================================================
// Tests: Null-Skipping for Optional Type Fields
// ============================================================================

#[tokio::test]
async fn test_level2_null_type_fields_skipped() {
    // Arrange: Entity without type information
    let mut entity = create_typed_entity("rust:fn:test:src_lib_rs:10");
    entity.return_type = None;
    entity.param_types = None;
    entity.param_names = None;
    entity.generic_constraints = None;
    entity.trait_impls = None;

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: Null type fields should be skipped
    assert!(!json.contains("\"return_type\""),
            "Null return_type should be skipped");
    assert!(!json.contains("\"param_types\""),
            "Null param_types should be skipped");
    assert!(!json.contains("\"param_names\""),
            "Null param_names should be skipped");
    assert!(!json.contains("\"generic_constraints\""),
            "Null generic_constraints should be skipped");
    assert!(!json.contains("\"trait_impls\""),
            "Null trait_impls should be skipped");
}

#[tokio::test]
async fn test_level2_empty_type_arrays_skipped() {
    // Arrange: Entity with empty type arrays
    let mut entity = create_typed_entity("rust:fn:test:src_lib_rs:10");
    entity.param_types = Some(vec![]);
    entity.param_names = Some(vec![]);
    entity.generic_constraints = Some(vec![]);
    entity.trait_impls = Some(vec![]);

    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: Empty arrays should be skipped
    // Note: We have Some(vec![]) which may or may not be skipped depending on serde config
    // This test verifies the behavior
    let json_compact = json.replace(" ", "").replace("\n", "");

    // Empty arrays should either be skipped OR be []
    if json_compact.contains("param_types") {
        assert!(json_compact.contains("\"param_types\":[]"));
    }
}

// ============================================================================
// Tests: Level 2 Includes All Level 1 Fields
// ============================================================================

#[tokio::test]
async fn test_level2_includes_level1_fields() {
    // Arrange
    let entity = create_typed_entity("rust:fn:test:src_lib_rs:10");
    let db = MockDatabase::with_entities(vec![entity]);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    let json = serde_json::to_string(&output).unwrap();

    // Assert: All Level 1 fields still present
    assert!(json.contains("\"isgl1_key\""));
    assert!(json.contains("\"current_ind\""));
    assert!(json.contains("\"future_ind\""));
    assert!(json.contains("\"entity_name\""));
    assert!(json.contains("\"entity_type\""));
    assert!(json.contains("\"file_path\""));
    assert!(json.contains("\"line_number\""));
    assert!(json.contains("\"interface_signature\""));
}

// ============================================================================
// Tests: Metadata & Token Estimation
// ============================================================================

#[tokio::test]
async fn test_level2_exporter_metadata() {
    let exporter = Level2Exporter::new();

    // Assert
    assert_eq!(exporter.level(), 2);
    assert_eq!(exporter.estimated_tokens(), 60_000);
}

#[tokio::test]
async fn test_level2_export_metadata() {
    // Arrange
    let entities = vec![
        create_typed_entity("rust:fn:test1:src_lib_rs:10"),
        create_typed_entity("rust:fn:test2:src_lib_rs:20"),
    ];

    let db = MockDatabase::with_entities(entities);
    let config = create_test_config(false, "ALL");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    let output = result.unwrap();

    // Assert
    assert_eq!(output.export_metadata.level, 2);
    assert_eq!(output.export_metadata.total_entities, Some(2));
    assert_eq!(output.export_metadata.include_code, Some(false));
}

// ============================================================================
// Tests: Entity Structure
// ============================================================================

#[test]
fn test_level2_entity_struct_clone() {
    // Test that EntityExportLevel2 implements Clone
    let entity = EntityExportLevel2 {
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
        // Level 2 fields
        return_type: Some("()".to_string()),
        param_types: vec![],
        param_names: vec![],
        generic_constraints: vec![],
        trait_impls: vec![],
        is_public: true,
        is_async: false,
        is_unsafe: false,
    };

    let cloned = entity.clone();
    assert_eq!(cloned.isgl1_key, "test");
    assert_eq!(cloned.return_type, Some("()".to_string()));
    assert_eq!(cloned.is_public, true);
}

// ============================================================================
// Tests: Type-Aware Filtering
// ============================================================================

#[tokio::test]
async fn test_level2_filter_async_functions() {
    // Arrange: Mix of async and sync functions
    let mut entity1 = create_typed_entity("rust:fn:async_fn:src_lib_rs:10");
    entity1.is_async = Some(true);

    let mut entity2 = create_typed_entity("rust:fn:sync_fn:src_lib_rs:20");
    entity2.is_async = Some(false);

    let db = MockDatabase::with_entities(vec![entity1, entity2]);
    let config = create_test_config(false, "is_async = true");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());

    // Note: This test validates the WHERE clause format, actual filtering
    // happens in Phase 3 implementation
}

#[tokio::test]
async fn test_level2_filter_public_functions() {
    // Arrange
    let mut entity1 = create_typed_entity("rust:fn:public_fn:src_lib_rs:10");
    entity1.is_public = Some(true);

    let mut entity2 = create_typed_entity("rust:fn:private_fn:src_lib_rs:20");
    entity2.is_public = Some(false);

    let db = MockDatabase::with_entities(vec![entity1, entity2]);
    let config = create_test_config(false, "is_public = true");
    let exporter = Level2Exporter::new();

    // Act
    let result = exporter.export(&db, &config).await;
    assert!(result.is_ok());
}

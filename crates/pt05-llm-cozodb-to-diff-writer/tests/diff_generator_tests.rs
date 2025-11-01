//! # Diff Generator Integration Tests (RED → GREEN → REFACTOR)
//!
//! Tests for CodeDiff.json generation from CozoDB

use pt05_llm_cozodb_to_diff_writer::{DiffGenerator, Operation};
use parseltongue_core::entities::{CodeEntity, TemporalAction, TemporalState};
use parseltongue_core::storage::CozoDbStorage;
use std::sync::Arc;

/// Test: Generate CodeDiff for entities with Create action
#[tokio::test]
async fn test_generate_diff_for_create_operations() {
    let storage = CozoDbStorage::new("mem").await.expect("Failed to create storage");
    storage.create_schema().await.expect("Failed to create schema");

    // Insert entity with Create action
    let entity = create_test_entity(
        "src_lib_rs-new_feature-fn-abc123",
        Some("fn new_feature() { println!(\"New!\"); }"),
        TemporalAction::Create,
    );

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Generate diff
    let generator = DiffGenerator::new(Arc::new(storage));
    let diff = generator
        .generate_diff()
        .await
        .expect("Failed to generate diff");

    assert_eq!(diff.changes.len(), 1);
    assert_eq!(diff.metadata.create_count, 1);
    assert_eq!(diff.metadata.edit_count, 0);
    assert_eq!(diff.metadata.delete_count, 0);

    let change = &diff.changes[0];
    assert_eq!(change.operation, Operation::Create);
    assert!(change.future_code.is_some());
}

/// Test: Generate CodeDiff for entities with Edit action
#[tokio::test]
async fn test_generate_diff_for_edit_operations() {
    let storage = CozoDbStorage::new("mem").await.expect("Failed to create storage");
    storage.create_schema().await.expect("Failed to create schema");

    // Insert entity with Edit action
    let entity = create_test_entity(
        "rust:fn:calculate_sum:src_lib_rs:42-56",
        Some("fn calculate_sum(a: i32, b: i32) -> i32 { a + b /* fixed */ }"),
        TemporalAction::Edit,
    );

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Generate diff
    let generator = DiffGenerator::new(Arc::new(storage));
    let diff = generator
        .generate_diff()
        .await
        .expect("Failed to generate diff");

    assert_eq!(diff.changes.len(), 1);
    assert_eq!(diff.metadata.edit_count, 1);

    let change = &diff.changes[0];
    assert_eq!(change.operation, Operation::Edit);
    assert!(change.future_code.is_some());
}

/// Test: Generate CodeDiff for entities with Delete action
#[tokio::test]
async fn test_generate_diff_for_delete_operations() {
    let storage = CozoDbStorage::new("mem").await.expect("Failed to create storage");
    storage.create_schema().await.expect("Failed to create schema");

    // Insert entity with Delete action
    let mut entity = create_test_entity(
        "rust:fn:obsolete:src_lib_rs:100-110",
        None, // No future_code for delete
        TemporalAction::Delete,
    );
    entity.future_code = None; // Delete doesn't need future code

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Generate diff
    let generator = DiffGenerator::new(Arc::new(storage));
    let diff = generator
        .generate_diff()
        .await
        .expect("Failed to generate diff");

    assert_eq!(diff.changes.len(), 1);
    assert_eq!(diff.metadata.delete_count, 1);

    let change = &diff.changes[0];
    assert_eq!(change.operation, Operation::Delete);
    assert!(change.future_code.is_none());
}

/// Test: Skip entities without FutureAction
#[tokio::test]
async fn test_skip_unchanged_entities() {
    let storage = CozoDbStorage::new("mem").await.expect("Failed to create storage");
    storage.create_schema().await.expect("Failed to create schema");

    // Insert unchanged entity (no FutureAction)
    let mut entity = create_test_entity(
        "rust:fn:unchanged:src_lib_rs:10-20",
        Some("fn unchanged() {}"),
        TemporalAction::Create, // Will be overridden
    );
    entity.temporal_state = TemporalState::unchanged();

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Generate diff
    let generator = DiffGenerator::new(Arc::new(storage));
    let diff = generator
        .generate_diff()
        .await
        .expect("Failed to generate diff");

    // Should be empty - unchanged entities should be skipped
    assert_eq!(diff.changes.len(), 0);
    assert_eq!(diff.metadata.total_changes, 0);
}

/// Test: Multiple operations in single diff
#[tokio::test]
async fn test_mixed_operations_diff() {
    let storage = CozoDbStorage::new("mem").await.expect("Failed to create storage");
    storage.create_schema().await.expect("Failed to create schema");

    // Create
    let create = create_test_entity(
        "src_lib_rs-new_func-fn-xyz789",
        Some("fn new_func() {}"),
        TemporalAction::Create,
    );

    // Edit
    let edit = create_test_entity(
        "rust:fn:existing:src_lib_rs:50-60",
        Some("fn existing() { /* updated */ }"),
        TemporalAction::Edit,
    );

    // Delete
    let mut delete = create_test_entity(
        "rust:fn:old:src_lib_rs:70-80",
        None,
        TemporalAction::Delete,
    );
    delete.future_code = None;

    storage.insert_entity(&create).await.unwrap();
    storage.insert_entity(&edit).await.unwrap();
    storage.insert_entity(&delete).await.unwrap();

    // Generate diff
    let generator = DiffGenerator::new(Arc::new(storage));
    let diff = generator
        .generate_diff()
        .await
        .expect("Failed to generate diff");

    assert_eq!(diff.changes.len(), 3);
    assert_eq!(diff.metadata.create_count, 1);
    assert_eq!(diff.metadata.edit_count, 1);
    assert_eq!(diff.metadata.delete_count, 1);
}

/// Test: CodeDiff.json serialization
#[tokio::test]
async fn test_code_diff_json_output() {
    let storage = CozoDbStorage::new("mem").await.expect("Failed to create storage");
    storage.create_schema().await.expect("Failed to create schema");

    let entity = create_test_entity(
        "src_lib_rs-test-fn-abc",
        Some("fn test() {}"),
        TemporalAction::Create,
    );

    storage.insert_entity(&entity).await.unwrap();

    let generator = DiffGenerator::new(Arc::new(storage));
    let diff = generator.generate_diff().await.unwrap();

    let json = diff.to_json_pretty().expect("JSON serialization failed");

    // Verify JSON structure
    assert!(json.contains("\"changes\""));
    assert!(json.contains("\"metadata\""));
    assert!(json.contains("\"CREATE\""));
    assert!(json.contains("\"isgl1_key\""));
    assert!(json.contains("\"file_path\""));
    assert!(json.contains("\"future_code\""));
}

// Helper function to create test entities
fn create_test_entity(isgl1_key: &str, future_code: Option<&str>, action: TemporalAction) -> CodeEntity {
    use parseltongue_core::entities::{
        ComplexityLevel, EntityClass, EntityMetadata, EntityType, InterfaceSignature,
        LanguageSpecificSignature, LineRange, RiskLevel, RustSignature, TddClassification, TestabilityLevel,
        Visibility,
    };
    use std::path::PathBuf;

    CodeEntity {
        isgl1_key: isgl1_key.to_string(),
        current_code: Some("old code".to_string()),
        future_code: future_code.map(|s| s.to_string()),
        interface_signature: InterfaceSignature {
            entity_type: EntityType::Function,
            name: isgl1_key.to_string(),
            visibility: Visibility::Public,
            file_path: PathBuf::from("src/test.rs"),
            line_range: LineRange {
                start: 1,
                end: 10,
            },
            module_path: vec!["test".to_string()],
            documentation: None,
            language_specific: LanguageSpecificSignature::Rust(RustSignature {
                generics: vec![],
                lifetimes: vec![],
                where_clauses: vec![],
                attributes: vec![],
                trait_impl: None,
            }),
        },
        tdd_classification: TddClassification {
            entity_class: EntityClass::CodeImplementation,
            testability: TestabilityLevel::Low,
            complexity: ComplexityLevel::Simple,
            dependencies: 0,
            test_coverage_estimate: 0.0,
            critical_path: false,
            change_risk: RiskLevel::Low,
        },
        lsp_metadata: None,
        temporal_state: match action {
            TemporalAction::Create => TemporalState::create(),
            TemporalAction::Edit => TemporalState::edit(),
            TemporalAction::Delete => TemporalState::delete(),
        },
        metadata: EntityMetadata::new().unwrap(),
    }
}

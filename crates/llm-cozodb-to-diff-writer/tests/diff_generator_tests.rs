//! # Diff Generator Integration Tests (RED → GREEN → REFACTOR)
//!
//! Tests for CodeDiff.json generation from CozoDB

use llm_cozodb_to_diff_writer::{CodeDiff, DiffGenerator, Operation};
use parseltongue_core::entities::{CodeEntity, FutureAction, TemporalState};
use parseltongue_core::storage::CozoDbStorage;
use std::path::PathBuf;

/// Test: Generate CodeDiff for entities with Create action
#[tokio::test]
async fn test_generate_diff_for_create_operations() {
    let storage = CozoDbStorage::new("mem").await.expect("Failed to create storage");

    // Insert entity with Create action
    let entity = create_test_entity(
        "src_lib_rs-new_feature-fn-abc123",
        Some("fn new_feature() { println!(\"New!\"); }"),
        FutureAction::Create,
    );

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Generate diff
    let generator = DiffGenerator::new(storage);
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

    // Insert entity with Edit action
    let entity = create_test_entity(
        "rust:fn:calculate_sum:src_lib_rs:42-56",
        Some("fn calculate_sum(a: i32, b: i32) -> i32 { a + b /* fixed */ }"),
        FutureAction::Edit,
    );

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Generate diff
    let generator = DiffGenerator::new(storage);
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

    // Insert entity with Delete action
    let mut entity = create_test_entity(
        "rust:fn:obsolete:src_lib_rs:100-110",
        None, // No future_code for delete
        FutureAction::Delete,
    );
    entity.future_code = None; // Delete doesn't need future code

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Generate diff
    let generator = DiffGenerator::new(storage);
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

    // Insert unchanged entity (no FutureAction)
    let mut entity = create_test_entity(
        "rust:fn:unchanged:src_lib_rs:10-20",
        Some("fn unchanged() {}"),
        FutureAction::Create, // Will be overridden
    );
    entity.temporal_state = TemporalState::Unchanged;

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Generate diff
    let generator = DiffGenerator::new(storage);
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

    // Create
    let create = create_test_entity(
        "src_lib_rs-new_func-fn-xyz789",
        Some("fn new_func() {}"),
        FutureAction::Create,
    );

    // Edit
    let edit = create_test_entity(
        "rust:fn:existing:src_lib_rs:50-60",
        Some("fn existing() { /* updated */ }"),
        FutureAction::Edit,
    );

    // Delete
    let mut delete = create_test_entity(
        "rust:fn:old:src_lib_rs:70-80",
        None,
        FutureAction::Delete,
    );
    delete.future_code = None;

    storage.insert_entity(&create).await.unwrap();
    storage.insert_entity(&edit).await.unwrap();
    storage.insert_entity(&delete).await.unwrap();

    // Generate diff
    let generator = DiffGenerator::new(storage);
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

    let entity = create_test_entity(
        "src_lib_rs-test-fn-abc",
        Some("fn test() {}"),
        FutureAction::Create,
    );

    storage.insert_entity(&entity).await.unwrap();

    let generator = DiffGenerator::new(storage);
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
fn create_test_entity(isgl1_key: &str, future_code: Option<&str>, action: FutureAction) -> CodeEntity {
    use parseltongue_core::entities::{EntityClass, TddClassification, TestabilityLevel};

    CodeEntity {
        isgl1_key: isgl1_key.to_string(),
        current_code: Some("old code".to_string()),
        future_code: future_code.map(|s| s.to_string()),
        interface_signature: parseltongue_core::entities::InterfaceSignature {
            raw: format!("fn {}", isgl1_key),
            return_type: None,
            parameters: vec![],
        },
        tdd_classification: TddClassification {
            entity_class: EntityClass::CodeImplementation,
            testability: TestabilityLevel::Untestable,
            dependencies: 0,
            change_risk: parseltongue_core::entities::ChangeRisk::Low,
            complexity: parseltongue_core::entities::ComplexityLevel::Simple,
            critical_path: false,
        },
        lsp_metadata: None,
        temporal_state: match action {
            FutureAction::Create => TemporalState::WillBeCreated,
            FutureAction::Edit => TemporalState::WillBeModified,
            FutureAction::Delete => TemporalState::WillBeDeleted,
        },
        metadata: Default::default(),
    }
}

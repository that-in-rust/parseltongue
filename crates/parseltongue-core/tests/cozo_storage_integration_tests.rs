//! Integration tests for CozoDB storage implementation
//!
//! Following TDD discipline: These tests should FAIL initially (RED phase)
//! until the real implementation is complete (GREEN phase).

use parseltongue_core::*;
use std::path::PathBuf;

/// Helper: Create test entity with default values
fn create_test_entity() -> CodeEntity {
    create_test_entity_with_key("test-file-rs-TestStruct")
}

/// Helper: Create test entity with custom key
fn create_test_entity_with_key(key: &str) -> CodeEntity {
    let signature = InterfaceSignature {
        entity_type: EntityType::Struct,
        name: "TestStruct".to_string(),
        visibility: Visibility::Public,
        file_path: PathBuf::from("test/file.rs"),
        line_range: LineRange::new(1, 10).unwrap(),
        module_path: vec!["test".to_string()],
        documentation: Some("Test documentation".to_string()),
        language_specific: LanguageSpecificSignature::Rust(RustSignature {
            generics: vec![],
            lifetimes: vec![],
            where_clauses: vec![],
            attributes: vec!["#[derive(Debug)]".to_string()],
            trait_impl: None,
        }),
    };

    let mut entity = CodeEntity::new(key.to_string(), signature).unwrap();

    // Set code to satisfy validation requirements
    entity.current_code = Some("struct TestStruct {}".to_string());
    entity.future_code = Some("struct TestStruct {}".to_string());

    entity
}

#[tokio::test]
async fn test_cozo_connection() {
    // Test: Real CozoDB connection works
    let db = CozoDbStorage::new("mem").await.unwrap();
    // Create schema first to ensure database is properly initialized
    db.create_schema().await.unwrap();
    assert!(db.is_connected().await);
}

#[tokio::test]
async fn test_create_code_graph_schema() {
    // RED: Schema creation not implemented
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    // Verify CodeGraph relation exists
    let relations = db.list_relations().await.unwrap();
    assert!(relations.contains(&"CodeGraph".to_string()));
}

#[tokio::test]
async fn test_insert_code_entity() {
    // RED: Entity insertion not implemented
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    let entity = create_test_entity();

    db.insert_entity(&entity).await.unwrap();

    // Verify entity can be retrieved
    let retrieved = db.get_entity("test-file-rs-TestStruct").await.unwrap();
    assert_eq!(retrieved.isgl1_key, entity.isgl1_key);
    assert_eq!(retrieved.current_code, entity.current_code);
}

#[tokio::test]
async fn test_temporal_state_update() {
    // RED: Temporal update not implemented
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    // Insert entity with unchanged state
    let entity = create_test_entity();
    db.insert_entity(&entity).await.unwrap();

    // Update temporal state: (1,1) → (1,0) for delete
    db.update_temporal_state(
        "test-file-rs-TestStruct",
        false, // future_ind
        Some(TemporalAction::Delete),
    ).await.unwrap();

    // Verify update
    let updated = db.get_entity("test-file-rs-TestStruct").await.unwrap();
    assert_eq!(updated.temporal_state.current_ind, true);
    assert_eq!(updated.temporal_state.future_ind, false);
    assert_eq!(updated.temporal_state.future_action, Some(TemporalAction::Delete));
}

#[tokio::test]
async fn test_query_changed_entities() {
    // RED: Query for changed entities not implemented
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    // Insert 3 entities: 1 unchanged, 1 edit, 1 delete
    let unchanged = create_test_entity_with_key("entity1");
    let to_edit = create_test_entity_with_key("entity2");
    let to_delete = create_test_entity_with_key("entity3");

    db.insert_entity(&unchanged).await.unwrap();
    db.insert_entity(&to_edit).await.unwrap();
    db.insert_entity(&to_delete).await.unwrap();

    // Mark changes
    db.update_temporal_state("entity2", true, Some(TemporalAction::Edit)).await.unwrap();
    db.update_temporal_state("entity3", false, Some(TemporalAction::Delete)).await.unwrap();

    // Query changed entities
    let changed = db.get_changed_entities().await.unwrap();
    assert_eq!(changed.len(), 2);
    assert!(changed.iter().any(|e| e.isgl1_key == "entity2"));
    assert!(changed.iter().any(|e| e.isgl1_key == "entity3"));
}

#[tokio::test]
async fn test_update_entity() {
    // RED: Update operation not implemented
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    // Insert entity
    let mut entity = create_test_entity();
    db.insert_entity(&entity).await.unwrap();

    // Modify entity
    entity.apply_temporal_change(
        TemporalAction::Edit,
        Some("struct TestStruct { field: i32 }".to_string())
    ).unwrap();

    // Update in database
    db.update_entity_internal(&entity).await.unwrap();

    // Verify update
    let retrieved = db.get_entity("test-file-rs-TestStruct").await.unwrap();
    assert_eq!(retrieved.temporal_state.future_action, Some(TemporalAction::Edit));
    assert_eq!(retrieved.future_code, Some("struct TestStruct { field: i32 }".to_string()));
}

#[tokio::test]
async fn test_delete_entity() {
    // RED: Delete operation not implemented
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    // Insert entity
    let entity = create_test_entity();
    db.insert_entity(&entity).await.unwrap();

    // Delete entity
    db.delete_entity("test-file-rs-TestStruct").await.unwrap();

    // Verify deletion - should return None
    let result = db.get_entity("test-file-rs-TestStruct").await;
    assert!(result.is_err() || result.unwrap().isgl1_key.is_empty());
}

#[tokio::test]
async fn test_codegraph_repository_trait() {
    // Test: CodeGraphRepository trait implementation
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();
    let mut db: Box<dyn CodeGraphRepository> = Box::new(storage);

    let entity = create_test_entity();

    // Test trait methods
    db.store_entity(entity.clone()).await.unwrap();

    let retrieved = db.get_entity("test-file-rs-TestStruct").await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().isgl1_key, "test-file-rs-TestStruct");
}

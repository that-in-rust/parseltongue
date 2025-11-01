//! Tool 2 Temporal Operations Verification Test
//!
//! Tests the core functionality of Tool 2 (LLM-to-cozoDB-writer):
//! - Edit operations: Update existing entities with future_code
//! - Delete operations: Mark entities for deletion
//! - Create operations: Insert new entities with hash-based ISGL1 keys
//! - Integration: Verify temporal state transitions

use parseltongue_core::{
    entities::{
        CodeEntity, EntityType, InterfaceSignature, LanguageSpecificSignature,
        LineRange, RustSignature, TemporalAction, Visibility,
    },
    interfaces::CodeGraphRepository,
    storage::CozoDbStorage,
};
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper: Create test entity with initial state (1,0,None)
fn create_test_entity(name: &str, file: &str, lines: (u32, u32)) -> CodeEntity {
    let signature = InterfaceSignature {
        entity_type: EntityType::Function,
        name: name.to_string(),
        visibility: Visibility::Public,
        file_path: PathBuf::from(file),
        line_range: LineRange::new(lines.0, lines.1).unwrap(),
        module_path: vec![],
        documentation: None,
        language_specific: LanguageSpecificSignature::Rust(RustSignature {
            generics: vec![],
            lifetimes: vec![],
            where_clauses: vec![],
            attributes: vec![],
            trait_impl: None,
        }),
    };

    let isgl1_key = format!("rust:fn:{}:{}:{}-{}", name, file.replace('/', "_"), lines.0, lines.1);

    let mut entity = CodeEntity::new(isgl1_key, signature).unwrap();
    entity.current_code = Some(format!("fn {}() {{\n    // Original code\n}}", name));

    entity
}

/// Scenario 1: Edit Operation
///
/// PRD (P01:129-142): Tool 2 should update existing entities with:
/// - future_code populated
/// - future_ind = 1
/// - Future_Action = Edit
/// - current_ind = 1 (unchanged)
#[tokio::test]
async fn test_tool2_edit_operation() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    // Create schema
    storage.create_schema().await.unwrap();

    // Setup: Insert entity via Tool 1 (initial state: 1,0,None)
    let entity = create_test_entity("calculate_sum", "src/lib.rs", (10, 15));
    let key = entity.isgl1_key.clone();
    storage.insert_entity(&entity).await.unwrap();

    // Verify initial state from Tool 1
    let initial = storage.get_entity(&key).await.unwrap();
    assert_eq!(initial.temporal_state.current_ind, true, "Should exist in current");
    assert_eq!(initial.temporal_state.future_ind, false, "Future unknown initially");
    assert_eq!(initial.temporal_state.future_action, None);

    // Execute: Tool 2 Edit operation
    storage
        .update_temporal_state(&key, true, Some(TemporalAction::Edit))
        .await
        .unwrap();

    // Set future_code (simulating LLM generation)
    let mut updated = storage.get_entity(&key).await.unwrap();
    updated.future_code = Some(format!("fn calculate_sum() {{\n    // LLM-improved code\n}}"));
    storage.update_entity(updated).await.unwrap();

    // Verify: Edit state (1,1,Edit)
    let edited = storage.get_entity(&key).await.unwrap();
    assert_eq!(edited.temporal_state.current_ind, true, "Should still exist in current");
    assert_eq!(edited.temporal_state.future_ind, true, "Should exist in future");
    assert_eq!(
        edited.temporal_state.future_action,
        Some(TemporalAction::Edit),
        "Should be marked for Edit"
    );
    assert!(edited.future_code.is_some(), "Should have future_code");
    assert_ne!(edited.future_code, edited.current_code, "Future should differ from current");
}

/// Scenario 2: Delete Operation
///
/// PRD (P01:129-142): Tool 2 should mark entities for deletion with:
/// - future_ind = 0
/// - Future_Action = Delete
/// - current_ind = 1 (still exists in current)
/// - future_code = empty
#[tokio::test]
async fn test_tool2_delete_operation() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    // Create schema
    storage.create_schema().await.unwrap();

    // Setup: Insert entity via Tool 1
    let entity = create_test_entity("deprecated_function", "src/old.rs", (20, 25));
    let key = entity.isgl1_key.clone();
    storage.insert_entity(&entity).await.unwrap();

    // Execute: Tool 2 Delete operation
    storage
        .update_temporal_state(&key, false, Some(TemporalAction::Delete))
        .await
        .unwrap();

    // Verify: Delete state (1,0,Delete)
    let deleted = storage.get_entity(&key).await.unwrap();
    assert_eq!(deleted.temporal_state.current_ind, true, "Should still exist in current");
    assert_eq!(deleted.temporal_state.future_ind, false, "Should NOT exist in future");
    assert_eq!(
        deleted.temporal_state.future_action,
        Some(TemporalAction::Delete),
        "Should be marked for Delete"
    );
}

/// Scenario 3: Create Operation with Hash-Based ISGL1 Key
///
/// PRD (P01:134, 140): Tool 2 should create new entities with:
/// - Hash-based ISGL1 key: `{sanitized_filepath}-{entity_name}-{entity_type}-{hash8}`
/// - current_ind = 0 (doesn't exist yet)
/// - future_ind = 1 (will exist)
/// - Future_Action = Create
/// - future_code populated
#[tokio::test]
async fn test_tool2_create_operation_with_hash_key() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    // Create schema
    storage.create_schema().await.unwrap();

    // Execute: Tool 2 Create operation with hash-based key
    let hash_key = CodeEntity::generate_new_entity_key(
        "src/new_feature.rs",
        "new_awesome_function",
        &EntityType::Function,
        chrono::Utc::now(),
    );

    let signature = InterfaceSignature {
        entity_type: EntityType::Function,
        name: "new_awesome_function".to_string(),
        visibility: Visibility::Public,
        file_path: PathBuf::from("src/new_feature.rs"),
        line_range: LineRange::new(1, 10).unwrap(), // Temporary lines
        module_path: vec![],
        documentation: None,
        language_specific: LanguageSpecificSignature::Rust(RustSignature {
            generics: vec![],
            lifetimes: vec![],
            where_clauses: vec![],
            attributes: vec![],
            trait_impl: None,
        }),
    };

    let mut new_entity = CodeEntity::new(hash_key.clone(), signature).unwrap();

    // Set Create state manually (simulating Tool 2 logic)
    new_entity.temporal_state.current_ind = false;
    new_entity.temporal_state.future_ind = true;
    new_entity.temporal_state.future_action = Some(TemporalAction::Create);
    new_entity.future_code = Some("fn new_awesome_function() {\n    // LLM-generated code\n}".to_string());

    storage.insert_entity(&new_entity).await.unwrap();

    // Verify: Create state (0,1,Create)
    let created = storage.get_entity(&hash_key).await.unwrap();
    assert_eq!(created.temporal_state.current_ind, false, "Should NOT exist in current");
    assert_eq!(created.temporal_state.future_ind, true, "Should exist in future");
    assert_eq!(
        created.temporal_state.future_action,
        Some(TemporalAction::Create),
        "Should be marked for Create"
    );
    assert!(created.future_code.is_some(), "Should have future_code");

    // Verify hash key format
    assert!(hash_key.contains("src_new_feature_rs"), "Should have sanitized filepath");
    assert!(hash_key.contains("new_awesome_function"), "Should have entity name");
    assert!(hash_key.contains("-fn-"), "Should have entity type");
    assert!(hash_key.matches('-').count() >= 3, "Should have at least 3 dashes (path-name-type-hash)");
}

/// Scenario 4: Integration Test - Full Tool 1 + Tool 2 Workflow
///
/// Workflow:
/// 1. Tool 1 indexes codebase (creates entities with state 1,0,None)
/// 2. Tool 2 edits one entity
/// 3. Tool 2 deletes one entity
/// 4. Tool 2 creates new entity
/// 5. Verify all temporal states are correct
#[tokio::test]
async fn test_tool1_tool2_integration() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    // Create schema
    storage.create_schema().await.unwrap();

    // Step 1: Tool 1 indexes 3 entities
    let entity1 = create_test_entity("function_to_edit", "src/main.rs", (10, 20));
    let entity2 = create_test_entity("function_to_delete", "src/main.rs", (30, 40));
    let entity3 = create_test_entity("function_unchanged", "src/main.rs", (50, 60));

    let key1 = entity1.isgl1_key.clone();
    let key2 = entity2.isgl1_key.clone();
    let key3 = entity3.isgl1_key.clone();

    storage.insert_entity(&entity1).await.unwrap();
    storage.insert_entity(&entity2).await.unwrap();
    storage.insert_entity(&entity3).await.unwrap();

    // Verify all start with initial state (1,0,None)
    for key in &[&key1, &key2, &key3] {
        let e = storage.get_entity(key).await.unwrap();
        assert_eq!(e.temporal_state.current_ind, true);
        assert_eq!(e.temporal_state.future_ind, false);
        assert_eq!(e.temporal_state.future_action, None);
    }

    // Step 2: Tool 2 edits entity1
    storage
        .update_temporal_state(&key1, true, Some(TemporalAction::Edit))
        .await
        .unwrap();

    // Step 3: Tool 2 deletes entity2
    storage
        .update_temporal_state(&key2, false, Some(TemporalAction::Delete))
        .await
        .unwrap();

    // Step 4: Tool 2 creates new entity
    let new_key = CodeEntity::generate_new_entity_key(
        "src/new.rs",
        "newly_created",
        &EntityType::Function,
        chrono::Utc::now(),
    );

    let signature = InterfaceSignature {
        entity_type: EntityType::Function,
        name: "newly_created".to_string(),
        visibility: Visibility::Public,
        file_path: PathBuf::from("src/new.rs"),
        line_range: LineRange::new(1, 5).unwrap(),
        module_path: vec![],
        documentation: None,
        language_specific: LanguageSpecificSignature::Rust(RustSignature {
            generics: vec![],
            lifetimes: vec![],
            where_clauses: vec![],
            attributes: vec![],
            trait_impl: None,
        }),
    };

    let mut new_entity = CodeEntity::new(new_key.clone(), signature).unwrap();
    new_entity.temporal_state.current_ind = false;
    new_entity.temporal_state.future_ind = true;
    new_entity.temporal_state.future_action = Some(TemporalAction::Create);
    new_entity.future_code = Some("fn newly_created() {}".to_string());

    storage.insert_entity(&new_entity).await.unwrap();

    // Step 5: Verify final states
    let e1 = storage.get_entity(&key1).await.unwrap();
    assert_eq!(e1.temporal_state.current_ind, true, "Entity1 should exist in current");
    assert_eq!(e1.temporal_state.future_ind, true, "Entity1 should exist in future");
    assert_eq!(e1.temporal_state.future_action, Some(TemporalAction::Edit), "Entity1 should be marked for Edit");

    let e2 = storage.get_entity(&key2).await.unwrap();
    assert_eq!(e2.temporal_state.current_ind, true, "Entity2 should exist in current");
    assert_eq!(e2.temporal_state.future_ind, false, "Entity2 should NOT exist in future");
    assert_eq!(e2.temporal_state.future_action, Some(TemporalAction::Delete), "Entity2 should be marked for Delete");

    let e3 = storage.get_entity(&key3).await.unwrap();
    assert_eq!(e3.temporal_state.current_ind, true);
    assert_eq!(e3.temporal_state.future_ind, false);
    assert_eq!(e3.temporal_state.future_action, None, "Entity3 should remain unchanged");

    let e4 = storage.get_entity(&new_key).await.unwrap();
    assert_eq!(e4.temporal_state.current_ind, false, "New entity should NOT exist in current");
    assert_eq!(e4.temporal_state.future_ind, true, "New entity should exist in future");
    assert_eq!(e4.temporal_state.future_action, Some(TemporalAction::Create), "New entity should be marked for Create");

    // Verify get_changed_entities returns 3 entities (Edit, Delete, Create)
    let changed = storage.get_changed_entities().await.unwrap();
    assert_eq!(changed.len(), 3, "Should have 3 entities with pending changes");
}

/// Scenario 5: Direct Datalog Query Execution - Simple Query
///
/// Tests the new S01 ultra-minimalist interface via execute_query()
/// This validates that the execute_query() method can run valid Datalog
#[tokio::test]
async fn test_execute_query_simple_query() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    // Create schema
    storage.create_schema().await.unwrap();

    // Execute: Simple read query to verify execute_query works
    let query = r#"
        ?[ISGL1_key, current_ind, future_ind] :=
        *CodeGraph{
            ISGL1_key, current_ind, future_ind
        }
    "#;

    let result = storage.execute_query(query).await;
    assert!(result.is_ok(), "Valid Datalog read query should execute successfully");
}

/// Scenario 6: Direct Datalog Query Execution - List Relations
///
/// Tests that execute_query can run system queries
#[tokio::test]
async fn test_execute_query_list_relations() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    storage.create_schema().await.unwrap();

    // Execute: CozoDB system query to list all relations
    let query = "::relations";

    let result = storage.execute_query(query).await;
    assert!(result.is_ok(), "System query should execute successfully");
}

/// Scenario 7: Error Handling - Invalid Datalog Syntax
///
/// Tests that execute_query properly propagates CozoDB errors
#[tokio::test]
async fn test_execute_query_invalid_syntax() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    storage.create_schema().await.unwrap();

    // Execute: Invalid Datalog query
    let invalid_query = "THIS IS NOT VALID DATALOG SYNTAX !!!";

    let result = storage.execute_query(invalid_query).await;

    // Verify: Returns DatabaseError with details
    assert!(result.is_err(), "Invalid Datalog should return error");

    let err = result.unwrap_err();
    let err_string = format!("{:?}", err);
    assert!(
        err_string.contains("DatabaseError") || err_string.contains("execute_query"),
        "Error should be DatabaseError from execute_query operation"
    );
    assert!(
        err_string.contains("Datalog query failed"),
        "Error should mention Datalog query failure"
    );
}

/// Scenario 8: Query Execution - Filtered Read
///
/// Tests query with filter conditions
#[tokio::test]
async fn test_execute_query_with_filter() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    storage.create_schema().await.unwrap();

    // Setup: Insert entities
    let entity1 = create_test_entity("fn1", "src/a.rs", (1, 5));
    let entity2 = create_test_entity("fn2", "src/b.rs", (10, 15));
    storage.insert_entity(&entity1).await.unwrap();
    storage.insert_entity(&entity2).await.unwrap();

    // Execute: Query with filter (find entities where current_ind is true)
    let query = r#"
        ?[ISGL1_key, current_ind] :=
        *CodeGraph{
            ISGL1_key, current_ind
        },
        current_ind == true
    "#;

    let result = storage.execute_query(query).await;
    assert!(result.is_ok(), "Filtered query should execute successfully");
}

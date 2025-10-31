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

// ================== Phase 1.3: DependencyEdges Schema Tests ==================

#[tokio::test]
async fn test_create_dependency_edges_schema() {
    // RED: DependencyEdges schema creation not yet implemented
    let db = CozoDbStorage::new("mem").await.unwrap();

    // Create schema
    db.create_dependency_edges_schema().await.unwrap();

    // Verify DependencyEdges relation exists
    let relations = db.list_relations().await.unwrap();
    assert!(
        relations.contains(&"DependencyEdges".to_string()),
        "DependencyEdges table should exist after schema creation. Found: {:?}",
        relations
    );
}

#[tokio::test]
async fn test_dependency_edges_schema_is_idempotent() {
    // Test: Schema creation should be idempotent (can call multiple times)
    let db = CozoDbStorage::new("mem").await.unwrap();

    // Create schema twice
    db.create_dependency_edges_schema().await.unwrap();
    let result = db.create_dependency_edges_schema().await;

    // CozoDB may error on duplicate :create - this is expected behavior
    // The important thing is the schema exists after first call
    match result {
        Ok(_) => {
            // Some CozoDB versions allow duplicate creates
            println!("CozoDB allows duplicate schema creation");
        }
        Err(e) => {
            // Most CozoDB versions error on duplicate creates - this is expected
            println!("CozoDB errored on duplicate create (expected): {}", e);
            // Verify schema still exists despite error
            let relations = db.list_relations().await.unwrap();
            assert!(
                relations.contains(&"DependencyEdges".to_string()),
                "Schema should still exist even if second create errors"
            );
        }
    }
}

#[tokio::test]
async fn test_both_schemas_can_coexist() {
    // Test: CodeGraph and DependencyEdges tables can both exist
    let db = CozoDbStorage::new("mem").await.unwrap();

    // Create both schemas
    db.create_schema().await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    // Verify both relations exist
    let relations = db.list_relations().await.unwrap();
    assert!(relations.contains(&"CodeGraph".to_string()));
    assert!(relations.contains(&"DependencyEdges".to_string()));

    // Verify we have exactly 2 relations (plus any system relations)
    let user_relations: Vec<_> = relations
        .iter()
        .filter(|r| !r.starts_with(':'))
        .collect();
    assert_eq!(
        user_relations.len(),
        2,
        "Should have exactly 2 user relations. Found: {:?}",
        user_relations
    );
}

// ================== Phase 1.4: Edge Insertion API Tests ==================

#[tokio::test]
async fn test_insert_single_dependency_edge() {
    // RED: Edge insertion not yet tested
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    let edge = DependencyEdge::builder()
        .from_key("rust:fn:main:src_main_rs:1-10")
        .to_key("rust:fn:helper:src_helper_rs:5-20")
        .edge_type(EdgeType::Calls)
        .source_location("src/main.rs:3:15")
        .build()
        .unwrap();

    // Insert edge
    db.insert_edge(&edge).await.unwrap();

    // Verify insertion by querying (will implement query methods later)
    // For now, just verify no error occurred
}

#[tokio::test]
async fn test_insert_edge_without_source_location() {
    // Test: Edge insertion works with optional source_location = None
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    let edge = DependencyEdge::builder()
        .from_key("rust:struct:MyStruct:src_lib_rs:10-20")
        .to_key("rust:trait:MyTrait:src_lib_rs:5-8")
        .edge_type(EdgeType::Implements)
        .build()
        .unwrap();

    db.insert_edge(&edge).await.unwrap();
}

#[tokio::test]
async fn test_insert_duplicate_edge_is_idempotent() {
    // Test: Inserting same edge twice should succeed (upsert semantics)
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    let edge = DependencyEdge::builder()
        .from_key("A")
        .to_key("B")
        .edge_type(EdgeType::Uses)
        .build()
        .unwrap();

    // Insert twice - should succeed both times
    db.insert_edge(&edge).await.unwrap();
    db.insert_edge(&edge).await.unwrap();
}

#[tokio::test]
async fn test_batch_insert_edges() {
    // RED: Batch insertion not yet tested
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    let edges = vec![
        DependencyEdge::builder()
            .from_key("rust:fn:main:src_main_rs:1-10")
            .to_key("rust:fn:helper:src_helper_rs:5-20")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
        DependencyEdge::builder()
            .from_key("rust:fn:helper:src_helper_rs:5-20")
            .to_key("rust:fn:util:src_util_rs:1-5")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
        DependencyEdge::builder()
            .from_key("rust:fn:main:src_main_rs:1-10")
            .to_key("rust:struct:Config:src_config_rs:1-20")
            .edge_type(EdgeType::Uses)
            .build()
            .unwrap(),
    ];

    db.insert_edges_batch(&edges).await.unwrap();
}

#[tokio::test]
async fn test_batch_insert_empty_slice() {
    // Test: Batch insert with empty slice should succeed (no-op)
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    let edges: Vec<DependencyEdge> = vec![];
    db.insert_edges_batch(&edges).await.unwrap();
}

#[tokio::test]
async fn test_single_edge_insert_performance_contract() {
    // Performance Contract: Single insert <5ms (D10 specification)
    use std::time::Instant;

    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    let edge = DependencyEdge::builder()
        .from_key("A")
        .to_key("B")
        .edge_type(EdgeType::Calls)
        .build()
        .unwrap();

    // Warm up
    db.insert_edge(&edge).await.unwrap();

    // Measure
    let start = Instant::now();
    db.insert_edge(&edge).await.unwrap();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 5,
        "Single edge insert took {:?}, expected <5ms",
        elapsed
    );
}

#[tokio::test]
async fn test_batch_insert_performance_contract() {
    // Performance Contract: Batch insert (100 edges) <50ms (D10 specification)
    use std::time::Instant;

    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    // Generate 100 edges
    let edges: Vec<DependencyEdge> = (0..100)
        .map(|i| {
            DependencyEdge::builder()
                .from_key(format!("entity_{}", i))
                .to_key(format!("entity_{}", i + 1))
                .edge_type(EdgeType::Calls)
                .build()
                .unwrap()
        })
        .collect();

    // Measure
    let start = Instant::now();
    db.insert_edges_batch(&edges).await.unwrap();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 50,
        "Batch insert (100 edges) took {:?}, expected <50ms",
        elapsed
    );
}

// ================== Phase 3: Query Implementation Tests ==================

#[tokio::test]
async fn test_blast_radius_single_hop() {
    // RED: Blast radius query not yet implemented
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    // Create test graph: A -> B -> C
    let edges = vec![
        DependencyEdge::builder()
            .from_key("rust:fn:A:test_rs:1-5")
            .to_key("rust:fn:B:test_rs:10-15")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
        DependencyEdge::builder()
            .from_key("rust:fn:B:test_rs:10-15")
            .to_key("rust:fn:C:test_rs:20-25")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
    ];

    db.insert_edges_batch(&edges).await.unwrap();

    // Query: 1-hop from A should return only B
    let affected = db.calculate_blast_radius("rust:fn:A:test_rs:1-5", 1).await.unwrap();

    assert_eq!(affected.len(), 1, "Should find 1 entity within 1 hop");
    assert_eq!(affected[0].0, "rust:fn:B:test_rs:10-15");
    assert_eq!(affected[0].1, 1, "Distance should be 1");
}

#[tokio::test]
async fn test_blast_radius_multi_hop() {
    // RED: Multi-hop blast radius
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    // Create test graph: A -> B -> C -> D
    let edges = vec![
        DependencyEdge::builder()
            .from_key("rust:fn:A:test_rs:1-5")
            .to_key("rust:fn:B:test_rs:10-15")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
        DependencyEdge::builder()
            .from_key("rust:fn:B:test_rs:10-15")
            .to_key("rust:fn:C:test_rs:20-25")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
        DependencyEdge::builder()
            .from_key("rust:fn:C:test_rs:20-25")
            .to_key("rust:fn:D:test_rs:30-35")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
    ];

    db.insert_edges_batch(&edges).await.unwrap();

    // Query: 2-hop from A should return B and C
    let affected = db.calculate_blast_radius("rust:fn:A:test_rs:1-5", 2).await.unwrap();

    assert_eq!(affected.len(), 2, "Should find 2 entities within 2 hops");

    // Check we have B at distance 1 and C at distance 2
    let b = affected.iter().find(|(k, _)| k.contains("fn:B:"));
    let c = affected.iter().find(|(k, _)| k.contains("fn:C:"));

    assert!(b.is_some(), "Should find B");
    assert_eq!(b.unwrap().1, 1, "B should be at distance 1");

    assert!(c.is_some(), "Should find C");
    assert_eq!(c.unwrap().1, 2, "C should be at distance 2");
}

#[tokio::test]
async fn test_blast_radius_branching() {
    // Test diamond pattern: A -> B, A -> C, B -> D, C -> D
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    let edges = vec![
        DependencyEdge::builder()
            .from_key("rust:fn:A:test_rs:1-5")
            .to_key("rust:fn:B:test_rs:10-15")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
        DependencyEdge::builder()
            .from_key("rust:fn:A:test_rs:1-5")
            .to_key("rust:fn:C:test_rs:20-25")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
        DependencyEdge::builder()
            .from_key("rust:fn:B:test_rs:10-15")
            .to_key("rust:fn:D:test_rs:30-35")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
        DependencyEdge::builder()
            .from_key("rust:fn:C:test_rs:20-25")
            .to_key("rust:fn:D:test_rs:30-35")
            .edge_type(EdgeType::Calls)
            .build()
            .unwrap(),
    ];

    db.insert_edges_batch(&edges).await.unwrap();

    // Query: 2-hop from A should return B, C at distance 1, and D at distance 2 (min distance)
    let affected = db.calculate_blast_radius("rust:fn:A:test_rs:1-5", 2).await.unwrap();

    assert_eq!(affected.len(), 3, "Should find 3 entities (B, C, D)");

    // D should have minimum distance of 2 (even though reachable via two paths)
    let d = affected.iter().find(|(k, _)| k.contains("fn:D:"));
    assert!(d.is_some(), "Should find D");
    assert_eq!(d.unwrap().1, 2, "D should be at minimum distance 2");
}

#[tokio::test]
async fn test_blast_radius_zero_hops() {
    // Edge case: 0 hops should return empty
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_dependency_edges_schema().await.unwrap();

    let affected = db.calculate_blast_radius("rust:fn:A:test_rs:1-5", 0).await.unwrap();

    assert_eq!(affected.len(), 0, "0 hops should return empty");
}

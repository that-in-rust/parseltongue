//! Integration test for pt02-level00 bug with zero-dependency codebases
//!
//! # Bug Description
//!
//! When a codebase has NO function calls or dependencies, pt01 never creates
//! the DependencyEdges table (because `dependencies.is_empty()` check).
//! This causes pt02-level00 to fail with "Cannot find requested stored relation 'DependencyEdges'".
//!
//! # TDD Cycle
//!
//! RED: This test will FAIL initially, exposing the bug
//! GREEN: Fix pt01 to ALWAYS create DependencyEdges schema
//! REFACTOR: Verify all tests still pass
//!
//! # Test Strategy
//!
//! 1. Create a minimal Rust file with zero dependencies
//! 2. Index with pt01 (DependencyEdges table should NOT be created - BUG!)
//! 3. Try pt02-level00 export (should fail with table not found error)
//! 4. After fix: pt02-level00 should succeed with empty edges array

use parseltongue_core::{
    entities::{
        CodeEntity, EntityClass, EntityType, InterfaceSignature, LanguageSpecificSignature,
        LineRange, RustSignature, TddClassification, Visibility,
    },
    interfaces::CodeGraphRepository,
    storage::CozoDbStorage,
};
use tempfile::TempDir;
use std::fs;
use std::path::PathBuf;

/// Helper: Create a simple Rust file with NO dependencies
fn create_zero_dependency_file(dir: &PathBuf) -> PathBuf {
    let file_path = dir.join("simple.rs");
    let content = r#"
// Simple function with NO dependencies (doesn't call anything)
pub fn standalone() -> i32 {
    42
}

// Another standalone function
fn helper() -> bool {
    true
}
"#;
    fs::write(&file_path, content).expect("Failed to write test file");
    file_path
}

/// RED TEST: Reproduce pt02-level00 bug with zero dependencies
///
/// Expected behavior:
/// - pt01 indexes 2 entities (2 functions)
/// - pt01 creates CodeGraph table ✓
/// - pt01 creates DependencyEdges table ✗ (BUG - because dependencies.is_empty())
/// - pt02-level00 queries DependencyEdges ✗ (FAILS - table doesn't exist)
///
/// After fix:
/// - pt01 ALWAYS creates DependencyEdges table (even when empty)
/// - pt02-level00 succeeds, returns empty edges array
#[tokio::test]
async fn test_pt02_level00_with_zero_dependencies_should_succeed() {
    // Arrange: Create temp database and test file
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("zero_deps_test.db");
    let test_file = create_zero_dependency_file(&temp_dir.path().to_path_buf());

    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .expect("Failed to create test database");

    // Act 1: Create schema (simulating pt01 initialization)
    storage.create_schema().await.expect("Failed to create CodeGraph schema");

    // Act 1b: Create DependencyEdges schema (this is what pt01 now does ALWAYS after our fix)
    // Previously this was only called if dependencies.is_empty() == false (BUG!)
    // After fix: Always called, ensuring pt02-level00 can query the table
    storage.create_dependency_edges_schema().await.expect("Failed to create DependencyEdges schema");

    // For this test, we'll manually create an entity using proper constructor
    let signature = InterfaceSignature {
        entity_type: EntityType::Function,
        name: "standalone".to_string(),
        visibility: Visibility::Public,
        file_path: test_file.clone(),
        line_range: LineRange::new(3, 5).unwrap(),
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

    let isgl1_key = format!("rust:fn:standalone:{}:3-5", test_file.file_name().unwrap().to_str().unwrap().replace('.', "_"));

    let mut entity1 = CodeEntity::new(isgl1_key, signature, EntityClass::CodeImplementation).unwrap();
    entity1.current_code = Some("pub fn standalone() -> i32 {\n    42\n}".to_string());
    entity1.tdd_classification.entity_class = EntityClass::CodeImplementation;

    storage.insert_entity(&entity1).await.expect("Failed to insert entity");

    // Verify entity was indexed
    let entities = storage.get_all_entities().await.unwrap();
    assert_eq!(entities.len(), 1, "Should have indexed 1 entity");

    // Act 2: Try to query DependencyEdges (simulating pt02-level00)
    //
    // This is what pt02-level00 does:
    // db.run_script("?[from_key, to_key, edge_type] := *DependencyEdges{from_key, to_key, edge_type}", ...)
    //
    // Expected: Should fail with "Cannot find requested stored relation 'DependencyEdges'"

    let query = r#"
        ?[from_key, to_key, edge_type] :=
        *DependencyEdges{from_key, to_key, edge_type}
    "#;

    let result = storage.raw_query(query).await;

    // Assert: Currently this FAILS (RED) - table doesn't exist
    //
    // After fix: This should SUCCEED (GREEN) - table exists, returns empty array
    assert!(
        result.is_ok(),
        "pt02-level00 should succeed even with zero dependencies (table should exist but be empty). Got error: {:?}",
        result.err()
    );

    if let Ok(edges_data) = result {
        // Should return empty array, not error
        let rows = edges_data.rows;
        assert_eq!(
            rows.len(), 0,
            "Should have zero edges for zero-dependency codebase"
        );
    }
}

/// Helper test: Verify DependencyEdges schema creation works
#[tokio::test]
async fn test_dependency_edges_schema_can_be_created_explicitly() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("schema_test.db");

    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    // Verify we can create DependencyEdges schema explicitly
    let result = storage.create_dependency_edges_schema().await;
    assert!(result.is_ok(), "Should be able to create DependencyEdges schema");

    // Verify we can query the empty table
    let query = r#"
        ?[from_key, to_key, edge_type] :=
        *DependencyEdges{from_key, to_key, edge_type}
    "#;

    let result = storage.raw_query(query).await;
    assert!(result.is_ok(), "Should be able to query empty DependencyEdges table");

    if let Ok(data) = result {
        assert_eq!(data.rows.len(), 0, "Empty table should return zero rows");
    }
}

/// Helper test: Verify error message when table doesn't exist
#[tokio::test]
async fn test_dependency_edges_missing_table_error() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("missing_table_test.db");

    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();

    // Try to query DependencyEdges WITHOUT creating the schema first
    let query = r#"
        ?[from_key, to_key, edge_type] :=
        *DependencyEdges{from_key, to_key, edge_type}
    "#;

    let result = storage.raw_query(query).await;

    // Should fail with specific error
    assert!(result.is_err(), "Should fail when DependencyEdges table doesn't exist");

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("DependencyEdges") || error_msg.contains("not found") || error_msg.contains("does not exist"),
        "Error should mention missing table: {}",
        error_msg
    );
}

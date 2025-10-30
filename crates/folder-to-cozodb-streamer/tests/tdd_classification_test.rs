//! TDD Classification Tests
//!
//! Executable specification: Tool 1 MUST correctly classify test vs code entities

use folder_to_cozodb_streamer::{streamer::FileStreamer, StreamerConfig, ToolFactory};
use parseltongue_core::entities::EntityClass;
use parseltongue_core::storage::CozoDbStorage;
use tempfile::TempDir;

/// RED Phase Test: Verify test functions are classified as TEST_IMPLEMENTATION
///
/// Preconditions:
/// - Rust file with #[test] attribute
/// - File indexed by Tool 1
///
/// Postconditions:
/// - Entity has entity_class = EntityClass::TestImplementation
///
/// Error Conditions:
/// - Test entity misclassified as CodeImplementation (current bug)
#[tokio::test]
async fn test_function_with_test_attribute_classified_correctly() {
    // Setup: Create temp directory with Rust test file
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");

    std::fs::write(
        &test_file,
        r#"
#[test]
fn test_example() {
    assert_eq!(1 + 1, 2);
}

fn regular_function() {
    println!("Not a test");
}
"#,
    )
    .unwrap();

    // Setup database
    let db_path = temp_dir.path().join("test.db");
    let config = StreamerConfig {
        root_dir: temp_dir.path().to_path_buf(),
        db_path: format!("rocksdb:{}", db_path.display()),
        max_file_size: 1024 * 1024,
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec![],
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    // Execute: Index with Tool 1
    {
        let streamer = ToolFactory::create_streamer(config.clone()).await.unwrap();
        let _result = streamer.stream_directory().await.unwrap();
    } // Drop streamer to release database lock

    // Verify: Check classifications
    let storage = CozoDbStorage::new(&config.db_path).await.unwrap();
    let entities = storage.get_all_entities().await.unwrap();

    // Should have 2 entities: 1 test, 1 code
    assert_eq!(entities.len(), 2, "Should have exactly 2 entities");

    // Find test function
    let test_entity = entities
        .iter()
        .find(|e| e.interface_signature.name == "test_example")
        .expect("Should find test_example function");

    // RED: This will fail with current implementation
    assert_eq!(
        test_entity.tdd_classification.entity_class,
        EntityClass::TestImplementation,
        "Test function should be classified as TEST_IMPLEMENTATION"
    );

    // Find regular function
    let code_entity = entities
        .iter()
        .find(|e| e.interface_signature.name == "regular_function")
        .expect("Should find regular_function");

    assert_eq!(
        code_entity.tdd_classification.entity_class,
        EntityClass::CodeImplementation,
        "Regular function should be classified as CODE_IMPLEMENTATION"
    );
}

/// RED Phase Test: Verify tokio::test functions are classified correctly
#[tokio::test]
async fn tokio_test_function_classified_correctly() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("async_test.rs");

    std::fs::write(
        &test_file,
        r#"
#[tokio::test]
async fn test_async_function() {
    assert_eq!(1 + 1, 2);
}
"#,
    )
    .unwrap();

    let db_path = temp_dir.path().join("test.db");
    let config = StreamerConfig {
        root_dir: temp_dir.path().to_path_buf(),
        db_path: format!("rocksdb:{}", db_path.display()),
        max_file_size: 1024 * 1024,
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec![],
        parsing_library: "tree-sitter".to_string(),
        chunking: "ISGL1".to_string(),
    };

    {
        let streamer = ToolFactory::create_streamer(config.clone()).await.unwrap();
        let _result = streamer.stream_directory().await.unwrap();
    } // Drop streamer to release database lock

    let storage = CozoDbStorage::new(&config.db_path).await.unwrap();
    let entities = storage.get_all_entities().await.unwrap();

    assert_eq!(entities.len(), 1);
    let test_entity = &entities[0];

    // RED: Will fail with current implementation
    assert_eq!(
        test_entity.tdd_classification.entity_class,
        EntityClass::TestImplementation,
        "#[tokio::test] function should be classified as TEST_IMPLEMENTATION"
    );
}

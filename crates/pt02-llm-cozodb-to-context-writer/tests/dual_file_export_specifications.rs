//! Executable Specifications: Dual File Export Feature
//! 
//! REQ-V090-004.0: Automatic dual-file export for CODE/TEST separation
//! 
//! Design Principles:
//! - Executable Specifications Over Narratives
//! - Dependency Injection for Testability  
//! - Structured Error Handling (thiserror)
//! - MVP-First Rigor

use pt02_llm_cozodb_to_context_writer::exporters::*;
use pt02_llm_cozodb_to_context_writer::errors::*;
use parseltongue_core::storage::*;
use tempfile::TempDir;
use std::path::PathBuf;

/// Contract: Single output name creates dual files automatically
#[tokio::test]
async fn dual_file_export_contract_level01() {
    // Given: A temporary database with CODE and TEST entities
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    // Setup test database with mixed entities
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display())).await.unwrap();
    
    // Create test entities (both CODE and TEST)
    let code_entity = create_test_entity("main.rs", "main_function", "CODE");
    let test_entity = create_test_entity("tests/integration_test.rs", "test_function", "TEST");
    
    storage.store_entity(&code_entity).await.unwrap();
    storage.store_entity(&test_entity).await.unwrap();
    
    // When: Exporting with single output name
    let exporter = Level1Exporter::new();
    let output_name = "analysis";
    let result = exporter.export_dual_files(
        &storage,
        output_name,
        false, // include_code = false (signatures only)
        "ALL"  // where clause
    ).await;
    
    // Then: Should create dual files successfully
    assert!(result.is_ok(), "Dual file export should succeed: {:?}", result);
    
    // And: Both files should exist
    let main_file = PathBuf::from(format!("{}.json", output_name));
    let test_file = PathBuf::from(format!("{}_test.json", output_name));
    
    assert!(main_file.exists(), "Main file should exist: {:?}", main_file);
    assert!(test_file.exists(), "Test file should exist: {:?}", test_file);
    
    // And: Main file should contain only CODE entities
    let main_content = std::fs::read_to_string(&main_file).unwrap();
    assert!(main_content.contains("main_function"), "Main file should contain CODE entities");
    assert!(!main_content.contains("test_function"), "Main file should NOT contain TEST entities");
    
    // And: Test file should contain only TEST entities  
    let test_content = std::fs::read_to_string(&test_file).unwrap();
    assert!(test_content.contains("test_function"), "Test file should contain TEST entities");
    assert!(!test_content.contains("main_function"), "Test file should NOT contain CODE entities");
    
    // Cleanup
    std::fs::remove_file(&main_file).ok();
    std::fs::remove_file(&test_file).ok();
}

/// Contract: Dual export should work for all levels (0, 1, 2)
#[tokio::test] 
async fn dual_file_export_contract_all_levels() {
    // Given: Test database setup
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display())).await.unwrap();
    
    // Create test entities
    let code_entity = create_test_entity("src/lib.rs", "production_code", "CODE");
    let test_entity = create_test_entity("tests/unit_test.rs", "unit_test", "TEST");
    
    storage.store_entity(&code_entity).await.unwrap();
    storage.store_entity(&test_entity).await.unwrap();
    
    // When: Testing all export levels
    let output_name = "multi_level";
    
    // Level 0: Edge export
    let level0_exporter = Level0Exporter::new();
    let level0_result = level0_exporter.export_dual_files(&storage, output_name, "ALL").await;
    assert!(level0_result.is_ok(), "Level 0 dual export should succeed");
    
    // Level 1: Entity export  
    let level1_exporter = Level1Exporter::new();
    let level1_result = level1_exporter.export_dual_files(&storage, output_name, false, "ALL").await;
    assert!(level1_result.is_ok(), "Level 1 dual export should succeed");
    
    // Level 2: Type system export
    let level2_exporter = Level2Exporter::new();
    let level2_result = level2_exporter.export_dual_files(&storage, output_name, false, "ALL").await;
    assert!(level2_result.is_ok(), "Level 2 dual export should succeed");
    
    // Then: All levels should create dual files
    for level in 0..=2 {
        let main_file = PathBuf::from(format!("{}_level{}.json", output_name, level));
        let test_file = PathBuf::from(format!("{}_level{}_test.json", output_name, level));
        
        assert!(main_file.exists(), "Level {} main file should exist", level);
        assert!(test_file.exists(), "Level {} test file should exist", level);
        
        // Cleanup
        std::fs::remove_file(&main_file).ok();
        std::fs::remove_file(&test_file).ok();
    }
}

/// Contract: Performance should be comparable to single export
#[tokio::test]
async fn dual_file_export_performance_contract() {
    // Given: Database with many entities
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("perf.db");
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display())).await.unwrap();
    
    // Create many test entities (performance test)
    for i in 0..1000 {
        let entity_type = if i % 3 == 0 { "TEST" } else { "CODE" };
        let entity = create_test_entity(&format!("file{}.rs", i), &format!("func{}", i), entity_type);
        storage.store_entity(&entity).await.unwrap();
    }
    
    // When: Measuring dual export performance
    let exporter = Level1Exporter::new();
    let start = std::time::Instant::now();
    
    let result = exporter.export_dual_files(&storage, "performance_test", false, "ALL").await;
    
    let duration = start.elapsed();
    
    // Then: Should complete within reasonable time (< 5 seconds for 1000 entities)
    assert!(result.is_ok(), "Performance test should succeed");
    assert!(duration.as_secs() < 5, "Dual export should complete in < 5 seconds, took {:?}", duration);
    
    // Cleanup
    std::fs::remove_file("performance_test.json").ok();
    std::fs::remove_file("performance_test_test.json").ok();
}

/// Contract: Error handling should be structured and informative
#[tokio::test]
async fn dual_file_export_error_handling_contract() {
    // Given: Invalid database path
    let invalid_storage = CozoDbStorage::new("rocksdb:/nonexistent/path").await;
    
    // When: Attempting dual export
    let exporter = Level1Exporter::new();
    let result = exporter.export_dual_files(
        &invalid_storage.unwrap_err(), // This should fail
        "error_test", 
        false, 
        "ALL"
    ).await;
    
    // Then: Should return structured error
    assert!(result.is_err(), "Should return error for invalid database");
    
    match result.unwrap_err() {
        Pt02Error::StorageError(msg) => {
            assert!(msg.contains("database") || msg.contains("connection"), 
                   "Error should mention database issue: {}", msg);
        }
        _ => panic!("Should return StorageError for database issues"),
    }
}

// Helper function to create test entities
fn create_test_entity(file_path: &str, name: &str, entity_class: &str) -> Entity {
    Entity {
        id: format!("test:{}", name),
        name: name.to_string(),
        entity_type: "function".to_string(),
        file_path: file_path.to_string(),
        line_number: 1,
        is_public: true,
        current_ind: 1,
        future_ind: 0,
        future_action: None,
        current_code: Some(format!("fn {}() {{}}", name)),
        metadata: std::collections::HashMap::new(),
        interface_signature: None,
        forward_deps: vec![],
        reverse_deps: vec![],
        entity_class: match entity_class {
            "CODE" => parseltongue_core::EntityClass::CodeImplementation,
            "TEST" => parseltongue_core::EntityClass::TestImplementation,
            _ => parseltongue_core::EntityClass::CodeImplementation,
        },
        temporal_state: parseltongue_core::entities::TemporalState::Current,
    }
}

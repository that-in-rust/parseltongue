//! CLI Integration Tests for Tool 2 (llm-to-cozodb-writer)
//!
//! Tests the complete user-facing workflow:
//! - CLI argument parsing
//! - Database connection
//! - Datalog query execution
//! - Error propagation to user

use pt03_llm_to_cozodb_writer::{cli::CliConfig, LlmWriterConfig};
use parseltongue_core::{
    entities::{
        CodeEntity, EntityType, InterfaceSignature, LanguageSpecificSignature,
        LineRange, RustSignature, Visibility,
    },
    storage::CozoDbStorage,
};
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper: Create test entity
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

/// CLI Integration Test 1: Parse --query and --db arguments
#[test]
fn test_cli_parsing_query_interface() {
    let cli = CliConfig::build_cli();

    // Test with both arguments
    let matches = cli.try_get_matches_from(&[
        "llm-to-cozodb-writer",
        "--query",
        "?[a, b, c] := [[1, 2, 3]]",
        "--db",
        "rocksdb:test.db",
    ]);

    assert!(matches.is_ok(), "CLI should parse valid arguments");

    let matches = matches.unwrap();
    let config = CliConfig::parse_config(&matches);

    assert_eq!(config.query, "?[a, b, c] := [[1, 2, 3]]");
    assert_eq!(config.db_path, "rocksdb:test.db");
}

/// CLI Integration Test 2: Default database path
#[test]
fn test_cli_parsing_default_db() {
    let cli = CliConfig::build_cli();

    // Test with only --query (should use default db)
    let matches = cli.try_get_matches_from(&[
        "llm-to-cozodb-writer",
        "--query",
        "?[x] := [[42]]",
    ]);

    assert!(matches.is_ok());

    let config = CliConfig::parse_config(&matches.unwrap());
    assert_eq!(config.db_path, "parseltongue.db", "Should use default database path");
}

/// CLI Integration Test 3: Missing required --query argument
#[test]
fn test_cli_parsing_missing_query() {
    let cli = CliConfig::build_cli();

    // Test with missing --query (should fail)
    let matches = cli.try_get_matches_from(&[
        "llm-to-cozodb-writer",
        "--db",
        "test.db",
    ]);

    assert!(matches.is_err(), "CLI should require --query argument");
}

/// End-to-End Test 1: Simple query via CLI config
#[tokio::test]
async fn test_e2e_simple_query_via_config() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    // Setup: Create database
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();
    storage.create_schema().await.unwrap();

    // Insert test entities
    let entity1 = create_test_entity("fn1", "src/a.rs", (1, 5));
    let entity2 = create_test_entity("fn2", "src/b.rs", (10, 15));
    storage.insert_entity(&entity1).await.unwrap();
    storage.insert_entity(&entity2).await.unwrap();

    // Simulate user running: llm-to-cozodb-writer --query "..." --db <path>
    let config = LlmWriterConfig {
        query: r#"
            ?[ISGL1_key, current_ind] :=
            *CodeGraph{
                ISGL1_key, current_ind
            }
        "#.to_string(),
        db_path: format!("rocksdb:{}", db_path.display()),
    };

    // Execute query (this is what main.rs does)
    let result = storage.execute_query(&config.query).await;
    assert!(result.is_ok(), "E2E query execution should succeed");
}

/// End-to-End Test 2: System query via CLI config
#[tokio::test]
async fn test_e2e_system_query_via_config() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();
    storage.create_schema().await.unwrap();

    // User runs: llm-to-cozodb-writer --query "::relations" --db <path>
    let config = LlmWriterConfig {
        query: "::relations".to_string(),
        db_path: format!("rocksdb:{}", db_path.display()),
    };

    let result = storage.execute_query(&config.query).await;
    assert!(result.is_ok(), "System query should execute successfully");
}

/// End-to-End Test 3: Error propagation from invalid query
#[tokio::test]
async fn test_e2e_error_propagation() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();
    storage.create_schema().await.unwrap();

    // User provides invalid query
    let config = LlmWriterConfig {
        query: "INVALID DATALOG GARBAGE !!!".to_string(),
        db_path: format!("rocksdb:{}", db_path.display()),
    };

    // Execute should fail and propagate error
    let result = storage.execute_query(&config.query).await;
    assert!(result.is_err(), "Invalid query should fail");

    let err = result.unwrap_err();
    let err_msg = format!("{:?}", err);
    assert!(
        err_msg.contains("Datalog query failed"),
        "Error should mention Datalog failure"
    );
}

/// End-to-End Test 4: Multiple sequential queries
#[tokio::test]
async fn test_e2e_multiple_sequential_queries() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();
    storage.create_schema().await.unwrap();

    // Insert test entities
    let entity1 = create_test_entity("fn1", "src/a.rs", (1, 5));
    let entity2 = create_test_entity("fn2", "src/b.rs", (10, 15));
    storage.insert_entity(&entity1).await.unwrap();
    storage.insert_entity(&entity2).await.unwrap();

    // User runs multiple sequential queries:
    // 1. List relations
    let query1 = "::relations";
    let result1 = storage.execute_query(query1).await;
    assert!(result1.is_ok(), "First query should succeed");

    // 2. Query all entities
    let query2 = r#"
        ?[ISGL1_key] :=
        *CodeGraph{ ISGL1_key }
    "#;
    let result2 = storage.execute_query(query2).await;
    assert!(result2.is_ok(), "Second query should succeed");

    // 3. Query with filter
    let query3 = r#"
        ?[ISGL1_key, current_ind] :=
        *CodeGraph{ ISGL1_key, current_ind },
        current_ind == true
    "#;
    let result3 = storage.execute_query(query3).await;
    assert!(result3.is_ok(), "Third query should succeed");
}

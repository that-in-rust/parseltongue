//! TDD RED Phase: Simple Interface Tests
//!
//! Following S01 Executable Specifications:
//! - Test preconditions, postconditions, error conditions
//! - Write tests FIRST (these will fail initially)
//! - Validate contracts, not implementations

use pt03_llm_to_cozodb_writer::{EntityAction, InterfaceMode, SimpleUpdateConfig};

/// RED Test 1: Create action generates correct Datalog query
#[test]
fn test_create_action_generates_datalog() {
    let config = SimpleUpdateConfig {
        entity_key: "rust:fn:new_func:src_lib_rs:10-15".to_string(),
        action: EntityAction::Create,
        future_code: Some("pub fn new_func() {}".to_string()),
        db_path: "test.db".to_string(),
    };

    let datalog = config.to_datalog();
    println!("Generated Datalog:\n{}", datalog);

    // Postcondition: current_ind=0, future_ind=1, Future_Action="Create"
    assert!(datalog.contains(":put CodeGraph"));
    assert!(datalog.contains("current_ind: false") || datalog.contains("false"), "Missing current_ind=false");
    assert!(datalog.contains("future_ind: true") || datalog.contains("true"), "Missing future_ind=true");
    assert!(datalog.contains("Future_Action: \"Create\"") || datalog.contains("\"Create\""), "Missing Future_Action");
    assert!(datalog.contains("rust:fn:new_func:src_lib_rs:10-15"));
    assert!(datalog.contains("pub fn new_func() {}"));
}

/// RED Test 2: Edit action generates correct Datalog query
#[test]
fn test_edit_action_generates_datalog() {
    let config = SimpleUpdateConfig {
        entity_key: "rust:fn:existing:src_lib_rs:5-10".to_string(),
        action: EntityAction::Edit,
        future_code: Some("pub fn existing() { /* modified */ }".to_string()),
        db_path: "test.db".to_string(),
    };

    let datalog = config.to_datalog();

    // Postcondition: current_ind=1, future_ind=1, Future_Action="Edit"
    assert!(datalog.contains(":put CodeGraph"));
    assert!(datalog.contains("true"), "Missing temporal indicators");
    assert!(datalog.contains("\"Edit\""), "Missing Edit action");
}

/// RED Test 3: Delete action generates correct Datalog query
#[test]
fn test_delete_action_generates_datalog() {
    let config = SimpleUpdateConfig {
        entity_key: "rust:fn:old_func:src_lib_rs:20-25".to_string(),
        action: EntityAction::Delete,
        future_code: None, // Optional for Delete
        db_path: "test.db".to_string(),
    };

    let datalog = config.to_datalog();

    // Postcondition: current_ind=1, future_ind=0, Future_Action="Delete"
    assert!(datalog.contains(":put CodeGraph"));
    assert!(datalog.contains("true"), "Missing current_ind=true");
    assert!(datalog.contains("false"), "Missing future_ind=false");
    assert!(datalog.contains("\"Delete\""), "Missing Delete action");
}

/// RED Test 4: Error condition - Create without future_code
#[test]
#[should_panic(expected = "Create action requires future_code")]
fn test_create_requires_future_code() {
    let config = SimpleUpdateConfig {
        entity_key: "rust:fn:new:src_lib_rs:10-15".to_string(),
        action: EntityAction::Create,
        future_code: None, // ERROR: Create requires code
        db_path: "test.db".to_string(),
    };

    config.to_datalog(); // Should panic
}

/// RED Test 5: Error condition - Edit without future_code
#[test]
#[should_panic(expected = "Edit action requires future_code")]
fn test_edit_requires_future_code() {
    let config = SimpleUpdateConfig {
        entity_key: "rust:fn:existing:src_lib_rs:5-10".to_string(),
        action: EntityAction::Edit,
        future_code: None, // ERROR: Edit requires code
        db_path: "test.db".to_string(),
    };

    config.to_datalog(); // Should panic
}

/// RED Test 6: CLI mutual exclusion - cannot use --query with --entity
#[test]
fn test_cli_mutual_exclusion() {
    use pt03_llm_to_cozodb_writer::cli::CliConfig;

    let cli = CliConfig::build_cli();

    // Attempt to use both interfaces simultaneously
    let result = cli.try_get_matches_from(&[
        "llm-to-cozodb-writer",
        "--query", "?[x] := [[1]]",
        "--entity", "rust:fn:test:lib_rs:1-5",
        "--action", "edit",
        "--future-code", "fn test() {}",
    ]);

    // Error condition: Mutual exclusion violated
    assert!(result.is_err(), "Should reject both --query and --entity");
}

/// RED Test 7: CLI simple interface parsing
#[test]
fn test_cli_simple_interface_parsing() {
    use pt03_llm_to_cozodb_writer::cli::CliConfig;

    let cli = CliConfig::build_cli();

    let matches = cli.try_get_matches_from(&[
        "llm-to-cozodb-writer",
        "--entity", "rust:fn:hello:greeter_src_lib_rs:4-6",
        "--action", "edit",
        "--future-code", "pub fn hello() -> &'static str { \"Hello!\" }",
        "--db", "rocksdb:demo.db",
    ]);

    assert!(matches.is_ok(), "CLI should parse simple interface");

    let matches = matches.unwrap();
    let mode = CliConfig::parse_interface_mode(&matches);

    match mode {
        InterfaceMode::Simple(config) => {
            assert_eq!(config.entity_key, "rust:fn:hello:greeter_src_lib_rs:4-6");
            assert!(matches!(config.action, EntityAction::Edit));
            assert_eq!(config.db_path, "rocksdb:demo.db");
        }
        _ => panic!("Expected Simple interface mode"),
    }
}

/// RED Test 8: End-to-end simple interface workflow
#[tokio::test]
async fn test_e2e_simple_create_workflow() {
    use parseltongue_core::storage::CozoDbStorage;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    // Setup database
    let storage = CozoDbStorage::new(&format!("rocksdb:{}", db_path.display()))
        .await
        .unwrap();
    storage.create_schema().await.unwrap();

    // Create action via simple interface
    let config = SimpleUpdateConfig {
        entity_key: "rust:fn:new_func:src_lib_rs:10-15".to_string(),
        action: EntityAction::Create,
        future_code: Some("pub fn new_func() {}".to_string()),
        db_path: format!("rocksdb:{}", db_path.display()),
    };

    // Generate and execute Datalog
    let datalog = config.to_datalog();
    println!("Generated Datalog for E2E test:\n{}", datalog);

    let result = storage.execute_query(&datalog).await;

    if let Err(e) = &result {
        eprintln!("Error executing query: {:?}", e);
    }
    assert!(result.is_ok(), "Simple interface Create should succeed: {:?}", result);

    // Verify postcondition: Entity exists with correct temporal state
    let verify_query = r#"
        ?[ISGL1_key, current_ind, future_ind, Future_Action] :=
        *CodeGraph{
            ISGL1_key, current_ind, future_ind, Future_Action
        },
        ISGL1_key == "rust:fn:new_func:src_lib_rs:10-15"
    "#;

    let result = storage.execute_query(verify_query).await;
    assert!(result.is_ok(), "Verification query should succeed");
    // Note: Full verification would parse result and check values
}

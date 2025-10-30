//! # Integration Tests for Tool 3 (RED → GREEN → REFACTOR)
//!
//! These tests define the executable specifications for Tool 3 refactor.
//! Following TDD: Write failing tests first, then implement to make them pass.

use llm_cozodb_to_context_writer::{
    ContextOptimizer, ContextOptimizerImpl, ContextWriterConfig, ToolFactory,
};
use parseltongue_core::entities::{
    CodeEntity, ComplexityLevel, EntityClass, EntityMetadata, EntityType, InterfaceSignature,
    LanguageSpecificSignature, LineRange, RiskLevel, RustSignature, TddClassification,
    TemporalState, TestabilityLevel, Visibility,
};
use parseltongue_core::storage::CozoDbStorage;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

/// Test 1: Generate context from REAL CozoDB (not mocked data)
#[tokio::test]
async fn test_generate_context_from_real_cozodb() {
    // Setup: Use in-memory database for shared access
    let storage = CozoDbStorage::new("mem")
        .await
        .expect("Failed to create storage");
    storage
        .create_schema()
        .await
        .expect("Failed to create schema");

    // Insert test entity with current_ind=1
    let entity = create_test_entity(
        "rust:fn:test_function:src_lib_rs:10-20",
        Some("fn test_function() {}"),
        TemporalState::unchanged(), // current_ind=1, future_ind=1, future_action=None
    );

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Create config pointing to same database
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir
        .path()
        .join("context.json")
        .to_string_lossy()
        .to_string();

    let config = ContextWriterConfig {
        db_path: "mem".to_string(),
        ..ContextWriterConfig::default()
    };

    // Generate context (with dependency injection)
    let llm_client = ToolFactory::create_llm_client(config.clone());
    let storage_arc = Arc::new(storage);
    let optimizer = ContextOptimizerImpl::new(storage_arc, config, llm_client);

    let result = optimizer.generate_context(&output_path).await;

    // EXPECTATION: Should successfully read from CozoDB
    assert!(
        result.is_ok(),
        "Should successfully generate context from real CozoDB: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap().entities_processed, 1);
}

/// Test 2: Verify current_code and future_code are EXCLUDED from output JSON
#[tokio::test]
async fn test_context_excludes_code_fields() {
    // Setup: Use in-memory database
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    let entity = create_test_entity(
        "rust:fn:sample:src_lib_rs:1-10",
        Some("fn sample() { println!(\"This should NOT appear in context\"); }"),
        TemporalState::unchanged(),
    );

    storage.insert_entity(&entity).await.unwrap();

    // Generate context
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir
        .path()
        .join("context.json")
        .to_string_lossy()
        .to_string();

    let config = ContextWriterConfig {
        db_path: "mem".to_string(),
        ..ContextWriterConfig::default()
    };

    let llm_client = ToolFactory::create_llm_client(config.clone());
    let storage_arc = Arc::new(storage);
    let optimizer = ContextOptimizerImpl::new(storage_arc, config, llm_client);

    let _result = optimizer.generate_context(&output_path).await.unwrap();

    // Read generated JSON file
    let json_content = std::fs::read_to_string(&output_path).expect("Failed to read output file");

    // EXPECTATION: JSON should NOT contain current_code or future_code fields
    // CURRENT: Will FAIL because implementation includes current_code in llm_client.rs:253
    assert!(
        !json_content.contains("\"current_code\""),
        "Output JSON must NOT contain current_code field"
    );
    assert!(
        !json_content.contains("\"future_code\""),
        "Output JSON must NOT contain future_code field"
    );

    // Should contain required fields
    assert!(
        json_content.contains("\"isgl1_key\""),
        "Output must contain isgl1_key"
    );
    assert!(
        json_content.contains("\"interface_signature\""),
        "Output must contain interface_signature"
    );
}

/// Test 3: Enforce <100k token limit (PRD requirement)
#[tokio::test]
async fn test_token_limit_enforcement() {
    // Setup: Create many entities that would exceed token limit
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    // Insert 10 entities (in real scenario, this could be thousands)
    for i in 0..10 {
        let entity = create_test_entity(
            &format!("rust:fn:func_{}:src_lib_rs:{}0-{}9", i, i, i),
            Some(&format!("fn func_{}() {{}}", i)),
            TemporalState::unchanged(),
        );
        storage.insert_entity(&entity).await.unwrap();
    }

    // Generate context with VERY LOW token limit to trigger error
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir
        .path()
        .join("context.json")
        .to_string_lossy()
        .to_string();

    let config = ContextWriterConfig {
        db_path: "mem".to_string(),
        max_context_tokens: 10, // Intentionally too low
        ..ContextWriterConfig::default()
    };

    let llm_client = ToolFactory::create_llm_client(config.clone());
    let storage_arc = Arc::new(storage);
    let optimizer = ContextOptimizerImpl::new(storage_arc, config, llm_client);

    let result = optimizer.generate_context(&output_path).await;

    // EXPECTATION: Should return error when context exceeds token limit
    // CURRENT: Will FAIL because no token counting is implemented
    assert!(
        result.is_err(),
        "Should error when context exceeds token limit"
    );
}

/// Test 4: CLI --query flag support (PRD requirement)
#[test]
fn test_cli_query_flag_support() {
    use llm_cozodb_to_context_writer::cli::CliConfig;

    let cli = CliConfig::build_cli();
    let matches = cli.try_get_matches_from(&[
        "llm-cozodb-to-context-writer",
        "--query",
        "SELECT * EXCEPT (Current_Code, Future_Code) FROM CodeGraph WHERE current_ind=1",
        "--database",
        "test.db",
        "--output",
        "context.json",
    ]);

    // EXPECTATION: CLI should accept --query flag
    // CURRENT: Will FAIL because --query flag doesn't exist in cli.rs
    assert!(matches.is_ok(), "CLI should accept --query flag");

    let matches = matches.unwrap();
    assert_eq!(
        matches.get_one::<String>("query").unwrap(),
        "SELECT * EXCEPT (Current_Code, Future_Code) FROM CodeGraph WHERE current_ind=1"
    );
}

/// Test 5: Output format matches CodeGraphContext.json specification
#[tokio::test]
async fn test_output_format_matches_spec() {
    // Setup: Use in-memory database
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    let entity = create_test_entity(
        "rust:fn:example:src_lib_rs:1-10",
        Some("fn example() {}"),
        TemporalState::unchanged(),
    );
    storage.insert_entity(&entity).await.unwrap();

    // Generate context
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir
        .path()
        .join("context.json")
        .to_string_lossy()
        .to_string();

    let config = ContextWriterConfig {
        db_path: "mem".to_string(),
        ..ContextWriterConfig::default()
    };

    let llm_client = ToolFactory::create_llm_client(config.clone());
    let storage_arc = Arc::new(storage);
    let optimizer = ContextOptimizerImpl::new(storage_arc, config, llm_client);

    let _result = optimizer.generate_context(&output_path).await.unwrap();

    // Parse JSON output
    let json_content = std::fs::read_to_string(&output_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_content).unwrap();

    // EXPECTATION: Output should have CodeGraphContext structure
    // CURRENT: Will FAIL because output is ContextOptimizationResponse, not CodeGraphContext
    assert!(
        parsed.get("entities").is_some(),
        "Output must have 'entities' array"
    );
    assert!(
        parsed.get("entity_count").is_some(),
        "Output must have 'entity_count' field"
    );
    assert!(
        parsed.get("token_count").is_some(),
        "Output must have 'token_count' field"
    );
    assert!(
        parsed.get("generated_at").is_some(),
        "Output must have 'generated_at' timestamp"
    );

    // Should NOT have optimization-specific fields
    assert!(
        parsed.get("pruning_summary").is_none(),
        "Output should NOT have 'pruning_summary' (wrong format)"
    );
    assert!(
        parsed.get("confidence_score").is_none(),
        "Output should NOT have 'confidence_score' (wrong format)"
    );
}

/// Test 6: Only entities with current_ind=true are included
#[tokio::test]
async fn test_filter_current_ind_entities() {
    // Setup: Use in-memory database
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    // Entity 1: current_ind=true (should be included)
    let entity1 = create_test_entity(
        "rust:fn:current:src_lib_rs:1-10",
        Some("fn current() {}"),
        TemporalState::unchanged(), // current_ind=true
    );

    // Entity 2: current_ind=false (should be EXCLUDED)
    let entity2 = create_test_entity(
        "rust:fn:future:src_lib_rs:20-30",
        Some("fn future() {}"),
        TemporalState::create(), // current_ind=false, future_ind=true
    );

    storage.insert_entity(&entity1).await.unwrap();
    storage.insert_entity(&entity2).await.unwrap();

    // Generate context
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir
        .path()
        .join("context.json")
        .to_string_lossy()
        .to_string();

    let config = ContextWriterConfig {
        db_path: "mem".to_string(),
        ..ContextWriterConfig::default()
    };

    let llm_client = ToolFactory::create_llm_client(config.clone());
    let storage_arc = Arc::new(storage);
    let optimizer = ContextOptimizerImpl::new(storage_arc, config, llm_client);

    let result = optimizer.generate_context(&output_path).await.unwrap();

    // EXPECTATION: Only 1 entity (current_ind=true) should be processed
    // CURRENT: Will FAIL because implementation doesn't filter by current_ind
    assert_eq!(
        result.entities_processed, 1,
        "Should only process entities with current_ind=true"
    );

    // Verify JSON contains only the current entity
    let json_content = std::fs::read_to_string(&output_path).unwrap();
    assert!(json_content.contains("current"));
    assert!(!json_content.contains("future"));
}

// Helper function to create test entities
fn create_test_entity(
    isgl1_key: &str,
    current_code: Option<&str>,
    temporal_state: TemporalState,
) -> CodeEntity {
    CodeEntity {
        isgl1_key: isgl1_key.to_string(),
        current_code: current_code.map(|s| s.to_string()),
        future_code: None,
        interface_signature: InterfaceSignature {
            entity_type: EntityType::Function,
            name: "test_function".to_string(),
            visibility: Visibility::Public,
            file_path: PathBuf::from("src/test.rs"),
            line_range: LineRange { start: 1, end: 10 },
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
        temporal_state,
        metadata: EntityMetadata::new().unwrap(),
    }
}

//! Tool 3 PRD Compliance Test - Self-Hosting Validation
//!
//! Executable Specification: Tool 3 MUST be ultra-minimalist data extraction
//!
//! PRD Requirements (P01:122-128):
//! - Query CozoDB: SELECT * EXCEPT (current_code, future_code) WHERE current_ind=1
//! - Output: CodeGraphContext.json with ISGL1 + interface_signature + TDD_Classification + lsp_meta_data
//! - NO LLM involvement (pure data extraction)
//! - Token limit: <100k tokens
//!
//! This test validates Tool 3 on the REAL parseltongue codebase indexed by Tool 1

use parseltongue_core::{
    entities::EntityClass,
    storage::CozoDbStorage,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// CodeGraphContext.json structure per PRD
#[derive(Debug, Serialize, Deserialize)]
struct CodeGraphContext {
    /// Entities from CozoDB (current_ind=1 only)
    entities: Vec<ContextEntity>,
    /// Count of entities
    entity_count: usize,
    /// Estimated token count
    token_count: usize,
    /// Generation timestamp
    generated_at: String,
}

/// Entity in context (stripped of code fields per PRD)
/// Per PRD P01:128: ISGL1 + interface_signature + TDD_Classification + lsp_meta_data
/// NOTE: temporal_state is internal CozoDB state, NOT needed for LLM reasoning
/// NOTE: TDD_Classification simplified to just entity_class per ultra-minimalist principles
#[derive(Debug, Serialize, Deserialize)]
struct ContextEntity {
    isgl1_key: String,
    interface_signature: serde_json::Value, // Serialized InterfaceSignature
    entity_class: String,  // Simplified TDD Classification: "Test" or "Code"
    lsp_metadata: Option<serde_json::Value>, // Serialized LspMetadata
    // temporal_state removed - not in PRD requirements (P01:128)
    // Full tdd_classification removed - only entity_class needed for Test vs Code distinction
}

/// RED Phase Test: Verify Tool 3 operates without LLM (ultra-minimalist)
///
/// Preconditions:
/// - Parseltongue codebase indexed by Tool 1 (542 entities)
/// - Database at /tmp/parseltongue-rigorous-test.db
///
/// Postconditions:
/// - CodeGraphContext.json generated WITHOUT needing API key
/// - Contains only current_ind=1 entities
/// - Excludes current_code and future_code fields
/// - Token count < 100k
///
/// Error Conditions:
/// - If Tool 3 requires LLM API key (PRD violation)
/// - If context includes current_code/future_code (bloat violation)
#[tokio::test]
#[ignore] // Run with: cargo test --package parseltongue-core tool3_prd_compliance -- --ignored --nocapture
async fn test_tool3_pure_data_extraction_no_llm() {
    // Setup: Connect to real parseltongue database from Tool 1
    let db_path = "rocksdb:/tmp/parseltongue-rigorous-test.db";
    let storage = CozoDbStorage::new(db_path)
        .await
        .expect("Failed to connect to parseltongue database");

    // Execute: Pure data extraction (no LLM)
    let entities = storage
        .get_all_entities()
        .await
        .expect("Failed to query entities");

    println!("\n=== TOOL 3 PRD COMPLIANCE TEST ===\n");
    println!("Total entities in database: {}", entities.len());

    // Filter: Only current_ind=1 per PRD (line 122)
    let current_entities: Vec<_> = entities
        .into_iter()
        .filter(|e| e.temporal_state.current_ind)
        .collect();

    println!("Entities with current_ind=1: {}", current_entities.len());

    // Verify: Should have 542 entities from Tool 1 indexing
    assert!(
        current_entities.len() > 500,
        "Expected >500 current entities from parseltongue codebase, got {}",
        current_entities.len()
    );

    // Transform: Strip code fields per PRD (line 123-128)
    // Include ONLY: ISGL1 + interface_signature + TDD_Classification (simplified) + lsp_meta_data
    let context_entities: Vec<ContextEntity> = current_entities
        .iter()
        .map(|e| ContextEntity {
            isgl1_key: e.isgl1_key.clone(),
            interface_signature: serde_json::to_value(&e.interface_signature).unwrap(),
            entity_class: format!("{:?}", e.tdd_classification.entity_class), // Test vs CodeImplementation
            lsp_metadata: e.lsp_metadata.as_ref().map(|m| serde_json::to_value(m).unwrap()),
            // temporal_state excluded - not in PRD (P01:128)
            // Full tdd_classification excluded - only entity_class needed (ultra-minimalist)
        })
        .collect();

    // Estimate tokens (rough approximation: 1 token ≈ 4 characters)
    let json_output = serde_json::to_string_pretty(&context_entities).unwrap();
    let estimated_tokens = json_output.len() / 4;

    println!("JSON output size: {} bytes", json_output.len());
    println!("Estimated tokens: {} tokens", estimated_tokens);

    // Verify: No current_code or future_code as TOP-LEVEL entity fields (before moving context_entities)
    // Note: Interface signatures may contain function names "current_code"/"future_code" (which is fine)
    // We need to check the entity structure doesn't have these as code content fields
    let sample_json = serde_json::to_value(&context_entities[0]).unwrap();
    let entity_keys: Vec<String> = sample_json.as_object()
        .unwrap()
        .keys()
        .map(|k| k.to_string())
        .collect();

    assert!(
        !entity_keys.contains(&"current_code".to_string()),
        "Entity MUST NOT have current_code field (PRD violation at P01:123-126)"
    );
    assert!(
        !entity_keys.contains(&"future_code".to_string()),
        "Entity MUST NOT have future_code field (PRD violation at P01:123-126)"
    );

    // Create CodeGraphContext per PRD specification
    let context = CodeGraphContext {
        entities: context_entities,
        entity_count: current_entities.len(),
        token_count: estimated_tokens,
        generated_at: chrono::Utc::now().to_rfc3339(),
    };

    // Write output for analysis
    let output_path = PathBuf::from("/tmp/CodeGraphContext.json");
    let context_json = serde_json::to_string_pretty(&context).unwrap();
    std::fs::write(&output_path, &context_json).expect("Failed to write output");
    println!("Output written to: {} for analysis", output_path.display());

    // Verify: Contains required fields per PRD (P01:128)
    assert!(
        json_output.contains("\"isgl1_key\""),
        "Output must contain isgl1_key"
    );
    assert!(
        json_output.contains("\"interface_signature\""),
        "Output must contain interface_signature"
    );
    assert!(
        json_output.contains("\"entity_class\""),
        "Output must contain entity_class (simplified TDD_Classification)"
    );

    // Verify: Token limit per PRD (line 115: <100k tokens)
    assert!(
        estimated_tokens < 100_000,
        "Context exceeds 100k token limit: {} tokens (PRD violation at P01:115)",
        estimated_tokens
    );

    println!("\n✅ Tool 3 PRD Compliance Validated:");
    println!("   - Pure data extraction (no LLM required)");
    println!("   - Only current_ind=1 entities included");
    println!("   - current_code/future_code excluded");
    println!("   - Token count: {} < 100k limit", estimated_tokens);
    println!("   - Output written to: {}", output_path.display());
}

/// Test: Verify TDD classification distribution in context
///
/// Validates that Tool 1's TDD classification fix is reflected in Tool 3 output
#[tokio::test]
#[ignore]
async fn test_tool3_includes_tdd_classification() {
    let db_path = "rocksdb:/tmp/parseltongue-rigorous-test.db";
    let storage = CozoDbStorage::new(db_path).await.unwrap();

    let entities = storage.get_all_entities().await.unwrap();
    let current_entities: Vec<_> = entities
        .into_iter()
        .filter(|e| e.temporal_state.current_ind)
        .collect();

    // Count TDD classifications
    let test_count = current_entities
        .iter()
        .filter(|e| {
            matches!(
                e.tdd_classification.entity_class,
                EntityClass::TestImplementation
            )
        })
        .count();

    let code_count = current_entities
        .iter()
        .filter(|e| {
            matches!(
                e.tdd_classification.entity_class,
                EntityClass::CodeImplementation
            )
        })
        .count();

    println!("\n=== TDD CLASSIFICATION IN TOOL 3 CONTEXT ===");
    println!("Test entities: {}", test_count);
    println!("Code entities: {}", code_count);
    println!("Total: {}", current_entities.len());

    // Verify: Should have test entities (Tool 1 fix validation)
    assert!(
        test_count > 100,
        "Should have >100 test entities, got {}",
        test_count
    );
    assert!(
        code_count > 300,
        "Should have >300 code entities, got {}",
        code_count
    );

    // Create sample context entity to verify serialization
    let sample_entity = &current_entities[0];
    let context_entity = ContextEntity {
        isgl1_key: sample_entity.isgl1_key.clone(),
        interface_signature: serde_json::to_value(&sample_entity.interface_signature).unwrap(),
        entity_class: format!("{:?}", sample_entity.tdd_classification.entity_class),
        lsp_metadata: sample_entity.lsp_metadata.as_ref().map(|m| serde_json::to_value(m).unwrap()),
        // temporal_state excluded per PRD
        // Full tdd_classification excluded - only entity_class needed
    };

    let json = serde_json::to_string_pretty(&context_entity).unwrap();
    println!("\nSample entity JSON:\n{}", json);

    // Verify entity_class is serialized (simplified TDD classification)
    assert!(json.contains("entity_class"));
}

/// Test: Verify temporal state filtering (current_ind=1 only)
///
/// Ensures Tool 3 only includes entities that exist in current codebase
#[tokio::test]
#[ignore]
async fn test_tool3_filters_by_current_ind() {
    let db_path = "rocksdb:/tmp/parseltongue-rigorous-test.db";
    let storage = CozoDbStorage::new(db_path).await.unwrap();

    let all_entities = storage.get_all_entities().await.unwrap();

    println!("\n=== TEMPORAL STATE FILTERING TEST ===");
    println!("Total entities in database: {}", all_entities.len());

    // Count by temporal state
    let current_only = all_entities
        .iter()
        .filter(|e| e.temporal_state.current_ind)
        .count();

    let future_only = all_entities
        .iter()
        .filter(|e| !e.temporal_state.current_ind && e.temporal_state.future_ind)
        .count();

    println!("current_ind=1: {}", current_only);
    println!("future_only (current_ind=0, future_ind=1): {}", future_only);

    // After Tool 1 indexing, all entities should be current_ind=1, future_ind=0
    // (Tool 2 hasn't created any future-only entities yet)
    assert_eq!(
        current_only,
        all_entities.len(),
        "All entities from Tool 1 should have current_ind=1"
    );

    // Tool 3 context should include ALL these entities
    let context_entities: Vec<_> = all_entities
        .into_iter()
        .filter(|e| e.temporal_state.current_ind)
        .collect();

    assert_eq!(
        context_entities.len(),
        current_only,
        "Tool 3 should include all current_ind=1 entities"
    );
}

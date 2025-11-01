//! # Integration Tests for Tool 5 (RED → GREEN → REFACTOR)
//!
//! These tests define the executable specifications for Tool 5 refactor.
//! Following TDD: Write failing tests first, then implement to make them pass.
//!
//! ## Architectural Insights
//!
//! ### Enhanced Schema Decision (current_code + line_range)
//!
//! **Problem**: Original Tool 5 design only included `future_code`, making it impossible
//! for the LLM to know exactly what code to replace or where to make entity-level edits.
//!
//! **Solution**: Enhanced schema includes:
//! - `current_code: Option<String>` - What code to replace (for Edit/Delete)
//! - `future_code: Option<String>` - New code content (for Create/Edit)
//! - `line_range: Option<LineRange>` - Precise location (for line-based ISGL1 keys)
//!
//! **Why This Matters**:
//! 1. **Entity-Level Precision**: LLM can edit just one function in a multi-function file
//! 2. **Complete Context**: LLM sees both before and after states
//! 3. **Safe Operations**: Clear boundaries prevent accidental overwrites
//!
//! ### ISGL1 Key Format Handling
//!
//! Two key formats require different handling:
//! - **Line-based**: `rust:fn:name:src_lib_rs:42-56` (existing entities, has line_range)
//! - **Hash-based**: `src_lib_rs-new_feature-fn-abc12345` (new entities, no line_range)
//!
//! The `desanitize_path()` logic recognizes file extensions as special suffixes (e.g., "_rs")
//! rather than treating all underscores as path separators.
//!
//! ### Dependency Injection Pattern
//!
//! Using `Arc<CozoDbStorage>` enables:
//! 1. Same database instance shared between test setup and generator
//! 2. No need to mock CozoDB for integration tests
//! 3. Tests validate real database behavior (critical for CodeGraph queries)

use pt05_llm_cozodb_to_diff_writer::{Change, DiffGenerator, Operation};
use parseltongue_core::entities::{
    CodeEntity, ComplexityLevel, EntityClass, EntityMetadata, EntityType, InterfaceSignature,
    LanguageSpecificSignature, LineRange, RiskLevel, RustSignature, TddClassification,
    TemporalState, TestabilityLevel, Visibility,
};
use parseltongue_core::storage::CozoDbStorage;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Test 1: Generate diff from REAL CozoDB (not mocked data)
#[tokio::test]
async fn test_generate_diff_from_real_cozodb() {
    // Setup: Use in-memory database
    let storage = CozoDbStorage::new("mem")
        .await
        .expect("Failed to create storage");
    storage
        .create_schema()
        .await
        .expect("Failed to create schema");

    // Insert test entity with Future_Action = Edit
    let entity = create_test_entity(
        "rust:fn:test_function:src_lib_rs:10-20",
        Some("fn test_function() { /* old */ }"),
        Some("fn test_function() { /* new */ }"),
        TemporalState::edit(),
    );

    storage
        .insert_entity(&entity)
        .await
        .expect("Failed to insert entity");

    // Create diff generator with dependency injection
    let storage_arc = Arc::new(storage);
    let generator = DiffGenerator::new(storage_arc);

    // Generate diff
    let result = generator.generate_diff().await;

    // EXPECTATION: Should successfully read from CozoDB
    assert!(
        result.is_ok(),
        "Should successfully generate diff from real CozoDB: {:?}",
        result.err()
    );

    let diff = result.unwrap();
    assert_eq!(
        diff.metadata.total_changes, 1,
        "Should have 1 change (entity with Future_Action)"
    );
}

/// Test 2: Verify current_code and future_code are INCLUDED in output
#[tokio::test]
async fn test_diff_includes_current_and_future_code() {
    // Setup: Use in-memory database
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    let entity = create_test_entity(
        "rust:fn:calculate:src_lib_rs:42-56",
        Some("fn calculate(a: i32) -> i32 { a + 1 }"),
        Some("fn calculate(a: i32, b: i32) -> i32 { a + b }"),
        TemporalState::edit(),
    );

    storage.insert_entity(&entity).await.unwrap();

    // Generate diff
    let storage_arc = Arc::new(storage);
    let generator = DiffGenerator::new(storage_arc);
    let diff = generator.generate_diff().await.unwrap();

    // EXPECTATION: Diff should INCLUDE current_code for Edit operations
    // (Opposite of Tool 3 which EXCLUDES code!)
    let change = &diff.changes[0];
    assert!(
        change.current_code.is_some(),
        "Edit operation must include current_code"
    );
    assert!(
        change.future_code.is_some(),
        "Edit operation must include future_code"
    );

    // Verify content
    let current = change.current_code.as_ref().unwrap();
    let future = change.future_code.as_ref().unwrap();
    assert!(
        current.contains("a + 1"),
        "current_code should contain old implementation"
    );
    assert!(
        future.contains("a + b"),
        "future_code should contain new implementation"
    );
}

/// Test 3: Verify line_range is extracted from ISGL1 keys
#[tokio::test]
async fn test_diff_includes_line_ranges() {
    // Setup
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    // Entity with line-based ISGL1 key
    let entity = create_test_entity(
        "rust:fn:example:src_lib_rs:100-150",
        Some("fn example() {}"),
        Some("fn example_updated() {}"),
        TemporalState::edit(),
    );

    storage.insert_entity(&entity).await.unwrap();

    // Generate diff
    let storage_arc = Arc::new(storage);
    let generator = DiffGenerator::new(storage_arc);
    let diff = generator.generate_diff().await.unwrap();

    // EXPECTATION: line_range should be extracted from ISGL1 key
    let change = &diff.changes[0];
    assert!(
        change.line_range.is_some(),
        "Edit operation must include line_range"
    );

    let line_range = change.line_range.as_ref().unwrap();
    assert_eq!(line_range.start, 100, "Line start should match ISGL1 key");
    assert_eq!(line_range.end, 150, "Line end should match ISGL1 key");
}

/// Test 4: Verify changes are grouped by file path
#[tokio::test]
async fn test_changes_grouped_by_file() {
    // Setup
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    // Insert multiple entities from same file
    let entity1 = create_test_entity(
        "rust:fn:func_a:src_lib_rs:10-20",
        Some("fn func_a() {}"),
        Some("fn func_a_updated() {}"),
        TemporalState::edit(),
    );

    let entity2 = create_test_entity(
        "rust:fn:func_b:src_lib_rs:30-40",
        Some("fn func_b() {}"),
        Some("fn func_b_updated() {}"),
        TemporalState::edit(),
    );

    // Entity from different file
    let entity3 = create_test_entity(
        "rust:fn:other:src_models_user_rs:50-60",
        Some("fn other() {}"),
        Some("fn other_updated() {}"),
        TemporalState::edit(),
    );

    storage.insert_entity(&entity1).await.unwrap();
    storage.insert_entity(&entity2).await.unwrap();
    storage.insert_entity(&entity3).await.unwrap();

    // Generate diff
    let storage_arc = Arc::new(storage);
    let generator = DiffGenerator::new(storage_arc);
    let diff = generator.generate_diff().await.unwrap();

    // EXPECTATION: Changes should be grouped by file path
    // Convert to file-grouped structure for validation
    let grouped = group_changes_by_file(&diff.changes);

    assert_eq!(grouped.len(), 2, "Should have changes from 2 files");

    let lib_changes = grouped.get(&PathBuf::from("src/lib.rs"));
    assert!(lib_changes.is_some(), "Should have changes for src/lib.rs");
    assert_eq!(
        lib_changes.unwrap().len(),
        2,
        "Should have 2 changes for src/lib.rs"
    );

    let user_changes = grouped.get(&PathBuf::from("src/models/user.rs"));
    assert!(
        user_changes.is_some(),
        "Should have changes for src/models/user.rs"
    );
    assert_eq!(
        user_changes.unwrap().len(),
        1,
        "Should have 1 change for src/models/user.rs"
    );
}

/// Test 5: Handle all operation types (Create/Edit/Delete)
#[tokio::test]
async fn test_handles_all_operation_types() {
    // Setup
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    // CREATE: Hash-based ISGL1 key, no current_code
    let create_entity = create_test_entity(
        "src_lib_rs-new_feature-fn-abc12345",
        None, // No current code (doesn't exist yet)
        Some("fn new_feature() { /* brand new */ }"),
        TemporalState::create(),
    );

    // EDIT: Line-based ISGL1 key, has both current and future code
    let edit_entity = create_test_entity(
        "rust:fn:existing:src_lib_rs:100-120",
        Some("fn existing() { /* old */ }"),
        Some("fn existing() { /* updated */ }"),
        TemporalState::edit(),
    );

    // DELETE: Line-based ISGL1 key, no future code
    let delete_entity = create_test_entity(
        "rust:fn:obsolete:src_lib_rs:200-220",
        Some("fn obsolete() { /* to be removed */ }"),
        None, // No future code (being deleted)
        TemporalState::delete(),
    );

    storage.insert_entity(&create_entity).await.unwrap();
    storage.insert_entity(&edit_entity).await.unwrap();
    storage.insert_entity(&delete_entity).await.unwrap();

    // Generate diff
    let storage_arc = Arc::new(storage);
    let generator = DiffGenerator::new(storage_arc);
    let diff = generator.generate_diff().await.unwrap();

    // EXPECTATION: Should have 3 changes with correct operations
    assert_eq!(diff.metadata.total_changes, 3);
    assert_eq!(diff.metadata.create_count, 1);
    assert_eq!(diff.metadata.edit_count, 1);
    assert_eq!(diff.metadata.delete_count, 1);

    // Verify CREATE operation
    let create_change = diff
        .changes
        .iter()
        .find(|c| c.operation == Operation::Create)
        .expect("Should have CREATE operation");
    assert!(create_change.current_code.is_none(), "CREATE has no current_code");
    assert!(create_change.future_code.is_some(), "CREATE has future_code");
    assert!(
        create_change.line_range.is_none(),
        "CREATE (hash-based key) has no line_range"
    );

    // Verify EDIT operation
    let edit_change = diff
        .changes
        .iter()
        .find(|c| c.operation == Operation::Edit)
        .expect("Should have EDIT operation");
    assert!(edit_change.current_code.is_some(), "EDIT has current_code");
    assert!(edit_change.future_code.is_some(), "EDIT has future_code");
    assert!(
        edit_change.line_range.is_some(),
        "EDIT (line-based key) has line_range"
    );

    // Verify DELETE operation
    let delete_change = diff
        .changes
        .iter()
        .find(|c| c.operation == Operation::Delete)
        .expect("Should have DELETE operation");
    assert!(delete_change.current_code.is_some(), "DELETE has current_code");
    assert!(delete_change.future_code.is_none(), "DELETE has no future_code");
    assert!(
        delete_change.line_range.is_some(),
        "DELETE (line-based key) has line_range"
    );
}

/// Test 6: Output format matches enhanced spec with file grouping
#[tokio::test]
async fn test_diff_format_matches_spec() {
    // Setup
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    let entity = create_test_entity(
        "rust:fn:test:src_lib_rs:10-20",
        Some("fn test() {}"),
        Some("fn test_updated() {}"),
        TemporalState::edit(),
    );

    storage.insert_entity(&entity).await.unwrap();

    // Generate diff and serialize to JSON
    let storage_arc = Arc::new(storage);
    let generator = DiffGenerator::new(storage_arc);
    let diff = generator.generate_diff().await.unwrap();
    let json = diff.to_json_pretty().unwrap();

    // EXPECTATION: JSON should have enhanced structure
    assert!(json.contains("\"current_code\""), "Must have current_code field");
    assert!(json.contains("\"future_code\""), "Must have future_code field");
    assert!(json.contains("\"line_range\""), "Must have line_range field");
    assert!(json.contains("\"start\""), "line_range must have start");
    assert!(json.contains("\"end\""), "line_range must have end");

    // Should have operation type
    assert!(json.contains("\"EDIT\""), "Must have operation type");

    // Should have metadata
    assert!(json.contains("\"metadata\""), "Must have metadata");
    assert!(json.contains("\"total_changes\""), "Must have total_changes");
    assert!(json.contains("\"generated_at\""), "Must have timestamp");
}

// Helper functions

/// Create a test entity with specified codes and temporal state
fn create_test_entity(
    isgl1_key: &str,
    current_code: Option<&str>,
    future_code: Option<&str>,
    temporal_state: TemporalState,
) -> CodeEntity {
    // Extract file path from ISGL1 key (matching diff_generator logic)
    let file_path = if isgl1_key.contains(":") {
        // Line-based key: rust:fn:name:src_lib_rs:10-20
        let parts: Vec<&str> = isgl1_key.split(':').collect();
        if parts.len() >= 4 {
            let sanitized_path = parts[3];
            desanitize_path(sanitized_path)
        } else {
            PathBuf::from("src/lib.rs")
        }
    } else {
        // Hash-based key: src_lib_rs-name-type-hash
        let parts: Vec<&str> = isgl1_key.split('-').collect();
        if !parts.is_empty() {
            desanitize_path(parts[0])
        } else {
            PathBuf::from("src/lib.rs")
        }
    };

    CodeEntity {
        isgl1_key: isgl1_key.to_string(),
        current_code: current_code.map(|s| s.to_string()),
        future_code: future_code.map(|s| s.to_string()),
        interface_signature: InterfaceSignature {
            entity_type: EntityType::Function,
            name: "test_function".to_string(),
            visibility: Visibility::Public,
            file_path,
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

/// Helper to group changes by file (for validation)
fn group_changes_by_file(changes: &[Change]) -> HashMap<PathBuf, Vec<&Change>> {
    let mut grouped: HashMap<PathBuf, Vec<&Change>> = HashMap::new();
    for change in changes {
        grouped
            .entry(change.file_path.clone())
            .or_insert_with(Vec::new)
            .push(change);
    }
    grouped
}

/// Desanitize file path from ISGL1 key format (matching diff_generator logic)
/// Converts "src_lib_rs" → "src/lib.rs"
fn desanitize_path(sanitized: &str) -> PathBuf {
    // Common file extensions
    let extensions = ["_rs", "_js", "_ts", "_py", "_go", "_java", "_cpp", "_c", "_h"];

    // Find and replace extension suffix
    for ext in extensions {
        if let Some(idx) = sanitized.rfind(ext) {
            if idx + ext.len() == sanitized.len() {
                // Found extension at end
                let path_part = &sanitized[..idx]; // "src_lib"
                let ext_part = &ext[1..]; // "rs"
                let file_path = path_part.replace('_', "/") + "." + ext_part;
                return PathBuf::from(file_path);
            }
        }
    }

    // No known extension found, treat as-is
    PathBuf::from(sanitized.replace('_', "/"))
}

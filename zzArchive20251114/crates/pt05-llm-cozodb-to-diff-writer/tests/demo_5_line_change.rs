//! Demo: 5-line code change with enhanced schema

use pt05_llm_cozodb_to_diff_writer::DiffGenerator;
use parseltongue_core::entities::{
    CodeEntity, ComplexityLevel, EntityClass, EntityMetadata, EntityType,
    InterfaceSignature, LanguageSpecificSignature, LineRange, RiskLevel,
    RustSignature, TddClassification, TemporalState, TestabilityLevel, Visibility,
};
use parseltongue_core::storage::CozoDbStorage;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::test]
async fn demo_5_line_code_change() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║         Tool 5 Demo: 5-Line Code Change                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Setup: In-memory CozoDB
    let storage = CozoDbStorage::new("mem").await.unwrap();
    storage.create_schema().await.unwrap();

    println!("📖 Scenario: Editing a simple function in src/lib.rs");
    println!();

    // CURRENT CODE (before) - 5 lines
    let current_code = r#"fn calculate_sum(a: i32, b: i32) -> i32 {
    // TODO: Add error handling
    a + b
}"#;

    // FUTURE CODE (after) - 5 lines with improvement
    let future_code = r#"fn calculate_sum(a: i32, b: i32) -> i32 {
    // Added validation for overflow
    a.checked_add(b).unwrap_or(i32::MAX)
}"#;

    println!("🔴 CURRENT CODE (lines 10-14 in src/lib.rs):");
    println!("─────────────────────────────────────────────");
    println!("{}", current_code);
    println!();

    println!("🟢 FUTURE CODE (improved version):");
    println!("─────────────────────────────────────────────");
    println!("{}", future_code);
    println!();

    // Create entity representing this change
    let entity = CodeEntity {
        isgl1_key: "rust:fn:calculate_sum:src_lib_rs:10-14".to_string(),
        current_code: Some(current_code.to_string()),
        future_code: Some(future_code.to_string()),
        interface_signature: InterfaceSignature {
            entity_type: EntityType::Function,
            name: "calculate_sum".to_string(),
            visibility: Visibility::Public,
            file_path: PathBuf::from("src/lib.rs"),
            line_range: LineRange { start: 10, end: 14 },
            module_path: vec!["crate".to_string()],
            documentation: Some("Calculates sum with overflow protection".to_string()),
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
            testability: TestabilityLevel::High,
            complexity: ComplexityLevel::Simple,
            dependencies: 0,
            test_coverage_estimate: 0.8,
            critical_path: false,
            change_risk: RiskLevel::Low,
        },
        entity_class: EntityClass::CodeImplementation,
        lsp_metadata: None,
        temporal_state: TemporalState::edit(),
        metadata: EntityMetadata::new().unwrap(),
    };

    // Insert into CozoDB
    storage.insert_entity(&entity).await.unwrap();

    // Generate CodeDiff.json using Tool 5
    let generator = DiffGenerator::new(Arc::new(storage));
    let diff = generator.generate_diff().await.unwrap();

    println!("📊 Tool 5 Output Summary:");
    println!("─────────────────────────────────────────────");
    println!("  Total changes: {}", diff.metadata.total_changes);
    println!("  Edit operations: {}", diff.metadata.edit_count);
    println!("  Generated at: {}", diff.metadata.generated_at);
    println!();

    println!("📄 Generated CodeDiff.json:");
    println!("═════════════════════════════════════════════════════════════\n");
    let json = diff.to_json_pretty().unwrap();
    println!("{}", json);
    println!();
    println!("═════════════════════════════════════════════════════════════\n");

    println!("✨ Key Features Demonstrated:");
    println!("  ✓ current_code    → Shows exactly what to replace");
    println!("  ✓ future_code     → Shows the improved version");
    println!("  ✓ line_range      → Precise location (lines 10-14)");
    println!("  ✓ operation: EDIT → LLM knows this is a modification");
    println!();
    println!("💡 LLM can now make surgical edits without touching the rest of the file!");
    println!();

    // Validate the output
    assert_eq!(diff.changes.len(), 1);
    let change = &diff.changes[0];
    assert_eq!(change.operation, pt05_llm_cozodb_to_diff_writer::Operation::Edit);
    assert!(change.current_code.is_some());
    assert!(change.future_code.is_some());
    assert!(change.line_range.is_some());
    assert_eq!(change.line_range.unwrap().start, 10);
    assert_eq!(change.line_range.unwrap().end, 14);
}

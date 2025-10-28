use parseltongue_04::performance::PerformanceCompliance;
use parseltongue_04::*;

#[tokio::test]
async fn test_green_phase_basic_functionality() {
    // Test that our GREEN phase implementation works

    // 1. Test DefaultRustCodeValidator
    let validator = DefaultRustCodeValidator::new();

    // Test valid Rust code
    let valid_code = r#"
    fn main(){
        println!("Hello, world!");
    }
    "#
    .to_string();

    let result = validator.validate_syntax(&valid_code).await;
    assert!(result.is_ok(), "Valid code should pass syntax validation");

    let validation_output = result.unwrap();
    assert!(
        validation_output.is_valid,
        "Valid code should be marked as valid"
    );
    assert_eq!(validation_output.validation_type, ValidationType::Syntax);

    // Test invalid Rust code (missing parentheses)
    let invalid_code = "fn main { println!(\"Hello\"); }".to_string();

    let result = validator.validate_syntax(&invalid_code).await;
    assert!(
        result.is_ok(),
        "Validation should complete without panicking"
    );

    let validation_output = result.unwrap();
    assert!(
        !validation_output.is_valid,
        "Invalid code should be marked as invalid"
    );
    assert!(
        !validation_output.errors.is_empty(),
        "Invalid code should have errors"
    );

    // 2. Test RustAnalyzerClient creation
    let temp_dir = tempfile::TempDir::new().unwrap();
    let cargo_toml = temp_dir.path().join("Cargo.toml");
    tokio::fs::write(
        &cargo_toml,
        r#"
    [package]
    name = "test-project"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    "#,
    )
    .await
    .unwrap();

    let client_result = RustAnalyzerClient::new(temp_dir.path()).await;
    assert!(
        client_result.is_ok(),
        "Should be able to create RustAnalyzerClient for valid project"
    );

    // 3. Test performance contracts
    let contract = ValidationPerformanceContract::new("test-contract".to_string());
    assert!(
        !contract.thresholds.is_empty(),
        "Contract should have thresholds for different validation types"
    );

    // Test performance compliance
    let compliance = contract.validate_performance(
        ValidationType::Syntax,
        100,  // 100 bytes
        50,   // 50ms execution time
        1000, // 1KB memory usage
        0.95, // 95% accuracy
    );

    assert!(
        matches!(compliance, PerformanceCompliance::Compliant),
        "Small code with good performance should be compliant"
    );

    // 4. Test Tool 2 integration
    let parser = Tool2SimulationParser::new();
    let simulation_output = r#"
    Step A01: Initial analysis
    change: Add function fn test_function() -> i32 { 42 }
    Step B01: Planning phase
    ```rust
    fn main() {
        println!("Hello from simulation");
    }
    ```
    "#;

    let parsed_result = parser.parse_simulation_output(simulation_output);
    assert!(
        parsed_result.is_ok(),
        "Should be able to parse simulation output"
    );

    let parsed_output = parsed_result.unwrap();
    assert!(
        !parsed_output.code_snippets.is_empty(),
        "Should extract code snippets"
    );
    assert!(
        !parsed_output.change_requests.is_empty(),
        "Should extract change requests"
    );

    // 5. Test integration pipeline
    let pipeline = Tool2ValidationPipeline::new();
    let integration_result = pipeline.process_simulation_output(simulation_output).await;
    assert!(
        integration_result.is_ok(),
        "Should be able to process simulation output"
    );

    let result = integration_result.unwrap();
    assert!(result.success, "Integration should succeed");
    assert!(
        !result.generated_test_cases.is_empty(),
        "Should generate test cases"
    );

    println!("✅ All GREEN phase functionality is working correctly!");
}

//! Test Tool 3 validation integration
//! Tests that validation results from Tool 3 properly affect file writing behavior

use parseltongue_05::*;
use tempfile::TempDir;

#[tokio::test]
async fn test_validation_integration_successful() {
    // Test that successful validation allows file writing
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");

    // Create a successful validation result
    let validation_result = ValidationResult {
        passed: true,
        score: 0.9,
        messages: vec!["Validation passed".to_string()],
        recommended_safety_level: WriteSafetyLevel::Basic,
        validation_details: ValidationDetails {
            syntax_valid: true,
            types_valid: true,
            borrow_checker_valid: true,
            compilation_valid: true,
            performance_regression: false,
            error_count: 0,
            warning_count: 0,
        },
        should_write: true,
    };

    let input = FileWriteInput {
        path: test_file.to_string_lossy().to_string(),
        content: b"fn main() { println!(\"Hello, world!\"); }".to_vec(),
        operation_type: FileOperation::Create,
        validation_results: Some(validation_result),
    };

    let writer = DefaultFileWriter::new();
    let result = writer.write_file(input).await.unwrap();

    assert!(
        result.is_success(),
        "File write should succeed with successful validation: {:?}",
        result
    );
    assert!(
        test_file.exists(),
        "File should be created after successful validation"
    );

    let content = tokio::fs::read_to_string(&test_file).await.unwrap();
    assert!(
        content.contains("Hello, world!"),
        "File should contain the expected content"
    );
}

#[tokio::test]
async fn test_validation_integration_blocked() {
    // Test that failed validation blocks file writing
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");

    // Create a failed validation result
    let validation_result = ValidationResult {
        passed: false,
        score: 0.3,
        messages: vec!["Validation failed: syntax error".to_string()],
        recommended_safety_level: WriteSafetyLevel::Strict,
        validation_details: ValidationDetails {
            syntax_valid: false,
            types_valid: true,
            borrow_checker_valid: true,
            compilation_valid: false,
            performance_regression: false,
            error_count: 2,
            warning_count: 1,
        },
        should_write: false, // Explicitly say not to write
    };

    let input = FileWriteInput {
        path: test_file.to_string_lossy().to_string(),
        content: b"fn main() { println!(\"Hello, world!\"); }".to_vec(),
        operation_type: FileOperation::Create,
        validation_results: Some(validation_result),
    };

    let writer = DefaultFileWriter::new();
    let result = writer.write_file(input).await.unwrap();

    assert!(
        result.is_skipped(),
        "File write should be skipped due to failed validation: {:?}",
        result
    );
    assert!(
        !test_file.exists(),
        "File should NOT be created when validation fails"
    );
}

#[tokio::test]
async fn test_validation_converter_functionality() {
    // Test the validation converter with actual Tool 3 validation report
    let converter = DefaultValidationConverter::new();

    // Create a mock validation report (this would come from Tool 3)
    let mock_report = parseltongue_04::ValidationReport::new(
        std::path::PathBuf::from("test.rs"),
        "fn main() { println!(\"Hello, world!\"); }".to_string(),
        vec![], // Individual validation results - empty for this test (will pass by default)
    );

    let validation_result = converter.convert_validation_report(&mock_report).unwrap();

    // Should be successful since the report is overall_valid with empty results
    assert!(
        validation_result.passed,
        "Validation should pass with empty results"
    );
    assert!(
        validation_result.score > 0.8,
        "Score should be high for successful validation"
    );
    assert!(
        validation_result.should_write,
        "Should indicate writing is allowed"
    );
    assert_eq!(
        validation_result.validation_details.error_count, 0,
        "No errors expected"
    );
}

#[tokio::test]
async fn test_safety_level_adjustment() {
    // Test that validation affects safety level
    let converter = DefaultValidationConverter::new();

    // Create a validation report with some issues
    let mock_report = parseltongue_04::ValidationReport::new(
        std::path::PathBuf::from("test.rs"),
        "fn main() { println!(\"Hello, world!\"); }".to_string(),
        vec![], // Empty results for this test
    );

    let validation_result = converter.convert_validation_report(&mock_report).unwrap();
    let base_safety_level = WriteSafetyLevel::Basic;
    let adjusted_level = converter.adjust_safety_level(base_safety_level, &validation_result);

    // With successful validation, the level should not be increased too much
    assert!(
        adjusted_level <= WriteSafetyLevel::Standard,
        "Safety level should not be too strict for successful validation"
    );
}

#[tokio::test]
async fn test_validation_write_pipeline() {
    // Test the complete validation write pipeline
    let pipeline = ValidationWritePipeline::new();
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");

    // Create a mock validation report
    let mock_report = parseltongue_04::ValidationReport::new(
        std::path::PathBuf::from("test.rs"),
        "fn main() { println!(\"Hello, world!\"); }".to_string(),
        vec![], // Empty results - will pass validation
    );

    // Process validation for write
    let input = pipeline
        .process_validation_for_write(
            &mock_report,
            test_file.to_string_lossy().to_string(),
            b"fn main() { println!(\"Hello, world!\"); }".to_vec(),
            FileOperation::Create,
        )
        .await
        .unwrap();

    // Should be allowed to proceed
    assert!(
        input.validation_results.is_some(),
        "Validation results should be included"
    );
    let validation = input.validation_results.unwrap();
    assert!(
        validation.should_write,
        "Should be allowed to write based on validation"
    );
}

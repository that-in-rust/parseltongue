//! End-to-End Test for Tool 4
//! Tests the complete user journey from input to file modification

use parseltongue_05::*;
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_complete_user_journey() {
    // Test the complete pipeline: Tool 3 validation → Tool 4 file writing
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("example.rs");

    println!("=== Tool 4 End-to-End Test ===");
    println!("Testing complete user journey from validation to file writing");

    // 1. Simulate Tool 3 validation results (this would come from actual Tool 3)
    println!("1. Simulating Tool 3 validation results...");
    let mock_report = parseltongue_04::ValidationReport::new(
        test_file.clone(),
        "fn main() {\n    println!(\"Hello, from Tool 4!\");\n}".to_string(),
        vec![], // Empty validation results = successful validation
    );

    // 2. Convert Tool 3 results to Tool 4 format
    println!("2. Converting Tool 3 validation results...");
    let converter = DefaultValidationConverter::new();
    let validation_result = converter.convert_validation_report(&mock_report).unwrap();

    println!("   - Validation passed: {}", validation_result.passed);
    println!("   - Validation score: {:.2}", validation_result.score);
    println!("   - Should write: {}", validation_result.should_write);
    println!(
        "   - Recommended safety: {:?}",
        validation_result.recommended_safety_level
    );

    // 3. Create Tool 4 file write input
    println!("3. Creating Tool 4 file write input...");
    let pipeline = ValidationWritePipeline::new();
    let input = pipeline
        .process_validation_for_write(
            &mock_report,
            test_file.to_string_lossy().to_string(),
            "fn main() {\n    println!(\"Hello, from Tool 4!\");\n}"
                .as_bytes()
                .to_vec(),
            FileOperation::Create,
        )
        .await
        .unwrap();

    // 4. Execute file writing with safety checks and backup
    println!("4. Executing file writing with safety checks...");
    let tool4 = Tool4::new();
    let start_time = std::time::Instant::now();

    let result = tool4.writer().write_file(input).await.unwrap();
    let execution_time = start_time.elapsed();

    println!("   - Operation result: {:?}", result);
    println!("   - Execution time: {:?}", execution_time);

    // 5. Verify the results
    println!("5. Verifying results...");
    assert!(result.is_success(), "File writing should succeed");
    assert!(test_file.exists(), "File should exist after writing");

    let written_content = tokio::fs::read_to_string(&test_file).await.unwrap();
    assert!(
        written_content.contains("Hello, from Tool 4!"),
        "Content should match"
    );

    println!("   - ✅ File created successfully");
    println!("   - ✅ Content verified");
    println!("   - ✅ Performance metrics collected");

    // 6. Test modification with backup
    println!("6. Testing file modification with backup...");
    let new_content = "fn main() {\n    println!(\"Updated by Tool 4!\");\n}";

    let modify_input = FileWriteInput {
        path: test_file.to_string_lossy().to_string(),
        content: new_content.as_bytes().to_vec(),
        operation_type: FileOperation::Modify,
        validation_results: Some(validation_result), // Reuse validation result
    };

    let modify_result = tool4.writer().write_file(modify_input).await.unwrap();

    assert!(
        modify_result.is_success(),
        "File modification should succeed"
    );
    assert!(
        modify_result.backup_path().is_some(),
        "Backup should be created for modification"
    );

    let backup_path_ref = modify_result.backup_path().unwrap();
    assert!(backup_path_ref.exists(), "Backup file should exist");

    let updated_content = tokio::fs::read_to_string(&test_file).await.unwrap();
    assert!(
        updated_content.contains("Updated by Tool 4!"),
        "Content should be updated"
    );

    let backup_content = tokio::fs::read_to_string(backup_path_ref).await.unwrap();
    assert!(
        backup_content.contains("Hello, from Tool 4!"),
        "Backup should contain original content"
    );

    println!("   - ✅ File modified successfully");
    println!("   - ✅ Backup created and verified");

    // 7. Test rollback functionality
    println!("7. Testing rollback functionality...");

    // Create a simple report for rollback testing
    let mut rollback_report = FileWriteReport::new(FileOperation::Modify);
    rollback_report.summary_mut().add_file_result(modify_result);

    let rollback_result = tool4.writer().rollback_operation(&rollback_report).await;
    assert!(rollback_result.is_ok(), "Rollback should succeed");

    let rolled_back_content = tokio::fs::read_to_string(&test_file).await.unwrap();
    assert!(
        rolled_back_content.contains("Hello, from Tool 4!"),
        "Content should be rolled back"
    );

    println!("   - ✅ Rollback successful");

    println!("\n🎉 Complete user journey test PASSED!");
    println!("✅ Tool 3 → Tool 4 integration working correctly");
    println!("✅ Safety checks and validation integration functional");
    println!("✅ Backup and rollback mechanisms operational");
    println!("✅ Performance tracking and reporting working");
    println!("✅ Real file system operations successful");
}

#[tokio::test]
async fn test_batch_operations() {
    // Test batch file writing operations
    let temp_dir = TempDir::new().unwrap();

    println!("=== Batch Operations Test ===");

    let files = vec![
        ("file1.rs", "fn func1() { println!(\"File 1\"); }"),
        ("file2.rs", "fn func2() { println!(\"File 2\"); }"),
        ("file3.rs", "fn func3() { println!(\"File 3\"); }"),
    ];

    let mut inputs = Vec::new();

    for (filename, content) in &files {
        let validation_result = ValidationResult {
            passed: true,
            score: 0.95,
            messages: vec!["Validation successful".to_string()],
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

        inputs.push(FileWriteInput {
            path: temp_dir.path().join(filename).to_string_lossy().to_string(),
            content: content.as_bytes().to_vec(),
            operation_type: FileOperation::Create,
            validation_results: Some(validation_result),
        });
    }

    let writer = DefaultFileWriter::new();
    let start_time = std::time::Instant::now();

    let report = writer.write_files(inputs).await.unwrap();
    let execution_time = start_time.elapsed();

    println!("Batch operation completed in {:?}", execution_time);
    println!("Report summary:");
    println!(
        "  - Files processed: {}",
        report.summary().performance_metrics.files_processed
    );
    println!(
        "  - Successful: {}",
        report.summary().performance_metrics.successful_files
    );
    println!(
        "  - Total bytes written: {}",
        report.summary().performance_metrics.total_bytes_written
    );
    println!(
        "  - Average write speed: {:.2} bytes/sec",
        report.summary().performance_metrics.avg_write_speed_bps
    );

    assert!(report.summary().success, "Batch operation should succeed");
    assert_eq!(report.summary().performance_metrics.files_processed, 3);
    assert_eq!(report.summary().performance_metrics.successful_files, 3);

    // Verify all files were created
    for (filename, expected_content) in &files {
        let file_path = temp_dir.path().join(filename);
        assert!(file_path.exists(), "{} should exist", filename);

        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert!(
            content.contains(expected_content),
            "{} should contain expected content",
            filename
        );
    }

    println!("✅ All {} files created successfully", files.len());
}

#[tokio::test]
async fn test_performance_metrics_collection() {
    // Test comprehensive performance metrics collection
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("performance_test.rs");

    println!("=== Performance Metrics Test ===");

    // Create a reasonably sized file
    let content = "fn main() { println!(\"Performance test\"); }".repeat(100);

    let input = FileWriteInput {
        path: test_file.to_string_lossy().to_string(),
        content: content.as_bytes().to_vec(),
        operation_type: FileOperation::Create,
        validation_results: Some(ValidationResult {
            passed: true,
            score: 0.88,
            messages: vec!["Performance validation passed".to_string()],
            recommended_safety_level: WriteSafetyLevel::Standard,
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
        }),
    };

    let writer = DefaultFileWriter::new();
    let result = writer.write_file(input).await.unwrap();

    assert!(result.is_success(), "Performance test write should succeed");

    println!("Performance metrics:");
    println!("  - Bytes written: {}", result.bytes_written());
    println!("  - Duration: {}ms", result.duration_ms());
    println!(
        "  - Throughput: {:.2} bytes/sec",
        result.bytes_written() as f64 / result.duration_ms() as f64 * 1000.0
    );

    // Verify performance is reasonable (should complete quickly for small files)
    assert!(
        result.duration_ms() < 1000,
        "File write should complete within 1 second"
    );
    assert!(result.bytes_written() > 0, "Should write some bytes");

    println!("✅ Performance metrics collection working correctly");
}

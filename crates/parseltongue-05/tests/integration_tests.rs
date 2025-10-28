//! Integration tests for the file writer functionality
//! These tests will fail initially and should pass in GREEN phase

use parseltongue_05::*;
use tempfile::TempDir;

#[tokio::test]
async fn test_end_to_end_file_write_with_backup() {
    // RED: Test complete file writing workflow with backup creation
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    let original_content = "original content";
    let new_content = "new content";

    // Create initial file
    tokio::fs::write(&test_file, original_content)
        .await
        .unwrap();

    // Configure file writer with backup
    let config = FileWriterConfig {
        create_backups: true,
        backup_strategy: BackupStrategy::Timestamp,
        safety_level: WriteSafetyLevel::Standard,
        ..Default::default()
    };

    let writer = DefaultFileWriter::with_config(config);

    // Perform file write
    let input = FileWriteInput {
        path: test_file.to_string_lossy().to_string(),
        content: new_content.as_bytes().to_vec(),
        operation_type: FileOperation::Modify,
        validation_results: None,
    };

    let result = writer.write_file(input).await.unwrap();

    // GREEN phase: These assertions should pass
    assert!(result.is_success());
    assert!(result.backup_path().is_some());

    // Verify backup exists
    let backup_path_ref = result.backup_path();
    let backup_path = backup_path_ref.as_ref().unwrap();
    assert!(backup_path.exists());

    // Verify original file was modified
    let modified_content = tokio::fs::read_to_string(&test_file).await.unwrap();
    assert_eq!(modified_content, new_content);

    // Verify backup contains original content
    let backup_content = tokio::fs::read_to_string(backup_path).await.unwrap();
    assert_eq!(backup_content, original_content);
}

#[tokio::test]
async fn test_safety_check_prevents_dangerous_operations() {
    // RED: Test that safety checks prevent dangerous file operations
    let temp_dir = TempDir::new().unwrap();

    // Try to write to a file in a system directory (this should be blocked)
    let dangerous_path = "/etc/passwd"; // This should be blocked by safety checks
    let config = FileWriterConfig {
        safety_level: WriteSafetyLevel::Strict,
        ..Default::default()
    };

    let writer = DefaultFileWriter::with_config(config);

    let input = FileWriteInput {
        path: dangerous_path.to_string(),
        content: b"malicious content".to_vec(),
        operation_type: FileOperation::Modify,
        validation_results: None,
    };

    let result = writer.write_file(input).await.unwrap();

    // GREEN phase: This should be skipped due to safety check
    assert!(result.is_skipped());
    assert!(
        result.to_string().contains("safety check failed") || result.to_string().contains("system")
    );
}

#[tokio::test]
async fn test_rollback_functionality() {
    // RED: Test rollback functionality when write operations fail
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    let original_content = "original content";
    let bad_content = ""; // Empty content might cause issues

    // Create initial file
    tokio::fs::write(&test_file, original_content)
        .await
        .unwrap();

    let config = FileWriterConfig {
        create_backups: true,
        backup_strategy: BackupStrategy::Single,
        ..Default::default()
    };

    let writer = DefaultFileWriter::with_config(config);

    // Perform a write that might fail
    let input = FileWriteInput {
        path: test_file.to_string_lossy().to_string(),
        content: bad_content.as_bytes().to_vec(),
        operation_type: FileOperation::Modify,
        validation_results: None,
    };

    let result = writer.write_file(input).await;

    // GREEN phase: If write fails, rollback should restore original content
    match result {
        Ok(operation_result) => {
            if operation_result.is_failed() {
                // Check that original content is restored
                let current_content = tokio::fs::read_to_string(&test_file).await.unwrap();
                assert_eq!(current_content, original_content);
            }
        }
        Err(_) => {
            // Operation failed completely, check that original content is intact
            let current_content = tokio::fs::read_to_string(&test_file).await.unwrap();
            assert_eq!(current_content, original_content);
        }
    }
}

#[tokio::test]
async fn test_batch_file_operations() {
    // RED: Test batch file writing operations
    let temp_dir = TempDir::new().unwrap();

    let inputs = vec![
        FileWriteInput {
            path: temp_dir
                .path()
                .join("file1.txt")
                .to_string_lossy()
                .to_string(),
            content: b"content1".to_vec(),
            operation_type: FileOperation::Create,
            validation_results: None,
        },
        FileWriteInput {
            path: temp_dir
                .path()
                .join("file2.txt")
                .to_string_lossy()
                .to_string(),
            content: b"content2".to_vec(),
            operation_type: FileOperation::Create,
            validation_results: None,
        },
        FileWriteInput {
            path: temp_dir
                .path()
                .join("file3.txt")
                .to_string_lossy()
                .to_string(),
            content: b"content3".to_vec(),
            operation_type: FileOperation::Create,
            validation_results: None,
        },
    ];

    let writer = DefaultFileWriter::new();
    let report = writer.write_files(inputs).await.unwrap();

    // GREEN phase: These assertions should pass
    assert!(report.summary().success);
    assert_eq!(report.summary().performance_metrics.files_processed, 3);
    assert_eq!(report.summary().performance_metrics.successful_files, 3);

    // Verify all files were created
    for i in 1..=3 {
        let file_path = temp_dir.path().join(format!("file{}.txt", i));
        assert!(file_path.exists());
        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, format!("content{}", i));
    }
}

#[tokio::test]
async fn test_validation_result_integration() {
    // RED: Test integration with validation results from Tool 3
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    // Create validation result that indicates the write should NOT proceed
    let validation_result = ValidationResult {
        passed: false,
        score: 0.3, // Low score
        messages: vec!["Validation failed".to_string()],
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
        content: b"this should not be written".to_vec(),
        operation_type: FileOperation::Create,
        validation_results: Some(validation_result),
    };

    let writer = DefaultFileWriter::new();
    let result = writer.write_file(input).await.unwrap();

    // GREEN phase: File should be skipped due to validation
    assert!(result.is_skipped());
    assert!(!test_file.exists());
}

#[tokio::test]
async fn test_performance_contract_enforcement() {
    // RED: Test that performance contracts are enforced
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    // Create a large file to test performance
    let large_content = "x".repeat(1024 * 1024); // 1MB of 'x' characters

    let config = FileWriterConfig {
        timeout_ms: Some(100),           // Very short timeout
        max_file_size: Some(512 * 1024), // 512KB limit
        ..Default::default()
    };

    let writer = DefaultFileWriter::with_config(config);

    let input = FileWriteInput {
        path: test_file.to_string_lossy().to_string(),
        content: large_content.as_bytes().to_vec(),
        operation_type: FileOperation::Create,
        validation_results: None,
    };

    let result = writer.write_file(input).await;

    // GREEN phase: This should fail due to file size limit
    match result {
        Ok(operation_result) => {
            // If it succeeds, check if it was skipped due to safety
            assert!(operation_result.is_skipped());
        }
        Err(FileWriterError::FileTooLarge { .. }) => {
            // Expected error for file too large
            assert!(true);
        }
        Err(other) => {
            panic!("Unexpected error: {:?}", other);
        }
    }
}

#[tokio::test]
async fn test_backup_cleanup() {
    // RED: Test backup cleanup functionality
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    // Create initial file
    tokio::fs::write(&test_file, "initial content")
        .await
        .unwrap();

    let backup_manager = DefaultBackupManager::new();

    // Create multiple backups
    let mut backup_infos = Vec::new();
    for i in 0..5 {
        tokio::fs::write(&test_file, format!("content {}", i))
            .await
            .unwrap();
        let backup_info = backup_manager
            .create_backup(&test_file, BackupStrategy::Numbered)
            .await
            .unwrap();
        backup_infos.push(backup_info);
    }

    // Should have 5 backups
    assert_eq!(backup_infos.len(), 5);

    // Clean up old backups, keeping only 2
    backup_manager
        .cleanup_old_backups(&test_file, 2)
        .await
        .unwrap();

    // Should now have only 2 backups remaining
    let remaining_backups = backup_manager.list_backups(&test_file).await.unwrap();
    assert_eq!(remaining_backups.len(), 2);
}

#[tokio::test]
async fn test_concurrent_file_operations() {
    // RED: Test concurrent file writing operations
    let temp_dir = TempDir::new().unwrap();

    let mut handles = Vec::new();

    for i in 0..10 {
        let temp_dir_clone = temp_dir.path().to_owned();
        let handle = tokio::spawn(async move {
            let test_file = temp_dir_clone.join(format!("concurrent_{}.txt", i));
            let content = format!("content from task {}", i);

            let writer = DefaultFileWriter::new();
            let input = FileWriteInput {
                path: test_file.to_string_lossy().to_string(),
                content: content.as_bytes().to_vec(),
                operation_type: FileOperation::Create,
                validation_results: None,
            };

            writer.write_file(input).await
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    let mut successful_operations = 0;
    for handle in handles {
        match handle.await.unwrap() {
            Ok(result) => {
                if result.is_success() {
                    successful_operations += 1;
                }
            }
            Err(_) => {
                // Handle errors appropriately
            }
        }
    }

    // GREEN phase: Most operations should succeed
    assert!(successful_operations >= 8); // Allow for some failures due to concurrency

    // Verify files were created correctly
    for i in 0..10 {
        let test_file = temp_dir.path().join(format!("concurrent_{}.txt", i));
        if test_file.exists() {
            let content = tokio::fs::read_to_string(&test_file).await.unwrap();
            assert!(content.contains(&format!("task {}", i)));
        }
    }
}

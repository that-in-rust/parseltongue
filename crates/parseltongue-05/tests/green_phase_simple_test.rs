//! GREEN PHASE: Simple test to verify basic functionality
//! This test should pass once we have the basic implementation working

use parseltongue_05::*;
use tempfile::TempDir;

#[tokio::test]
async fn test_basic_file_write() {
    // GREEN: Test basic file writing functionality
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    let content = b"Hello, World!";

    // Create a default file writer
    let writer = DefaultFileWriter::new();

    // Create file write input
    let input = FileWriteInput {
        path: test_file.to_string_lossy().to_string(),
        content: content.to_vec(),
        operation_type: FileOperation::Create,
        validation_results: None,
    };

    // Write the file
    let result = writer.write_file(input).await;

    // Check what result we actually got
    println!("File write result: {:?}", result);

    // Check that the operation was successful
    assert!(result.is_ok(), "File write should succeed: {:?}", result);

    let operation_result = result.unwrap();
    println!("Operation result: {:?}", operation_result);
    assert!(
        operation_result.is_success(),
        "Operation should be successful: {:?}",
        operation_result
    );

    // Verify the file was actually written
    assert!(test_file.exists(), "File should exist after writing");
    let written_content = tokio::fs::read_to_string(&test_file).await.unwrap();
    assert_eq!(
        written_content, "Hello, World!",
        "File content should match"
    );
}

#[tokio::test]
async fn test_safety_check_basic() {
    // GREEN: Test basic safety checking
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    // Create a test file
    tokio::fs::write(&test_file, "test content").await.unwrap();

    let safety_checker = DefaultSafetyChecker::new();
    let input = FileWriteInput {
        path: test_file.to_string_lossy().to_string(),
        content: b"new content".to_vec(),
        operation_type: FileOperation::Modify,
        validation_results: None,
    };

    let safety_check = safety_checker.check_file_safety(&test_file).unwrap();

    // Basic safety checks should pass
    assert!(safety_check.is_safe, "File should be safe to modify");
    assert_eq!(safety_check.safety_level, WriteSafetyLevel::Standard);
    assert!(safety_check.risk_score < 0.5, "Risk score should be low");
}

#[tokio::test]
async fn test_backup_creation() {
    // GREEN: Test backup creation
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    let original_content = "original content";

    // Create initial file
    tokio::fs::write(&test_file, original_content)
        .await
        .unwrap();

    let backup_manager = DefaultBackupManager::new();

    // Create backup
    let backup_info = backup_manager
        .create_backup(&test_file, BackupStrategy::Timestamp)
        .await
        .unwrap();

    assert!(backup_info.backup_path.exists(), "Backup file should exist");
    assert_eq!(backup_info.original_path, test_file);
    assert_eq!(backup_info.strategy, BackupStrategy::Timestamp);

    // Verify backup content
    let backup_content = tokio::fs::read_to_string(&backup_info.backup_path)
        .await
        .unwrap();
    assert_eq!(
        backup_content, original_content,
        "Backup should contain original content"
    );
}

#[tokio::test]
async fn test_tool4_basic_functionality() {
    // GREEN: Test Tool 4 basic functionality
    let tool4 = Tool4::new();
    let writer = tool4.writer();

    // Test that we can access the underlying writer
    let config = writer.config();
    assert!(
        config.create_backups,
        "Default config should create backups"
    );
    assert_eq!(config.safety_level, WriteSafetyLevel::Standard);

    // Test default creation
    let default_tool4 = Tool4::default();
    let default_writer = default_tool4.writer();
    assert_eq!(
        default_writer.config().safety_level,
        WriteSafetyLevel::Standard
    );
}

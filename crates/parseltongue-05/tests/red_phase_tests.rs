//! RED PHASE: Failing tests for core file writing functionality
//! Following TDD principle: Write failing tests first

use parseltongue_05::*;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use tempfile::TempDir;

#[tokio::test]
async fn test_file_writer_trait_compilation() {
    // RED: Test that FileWriter trait can be implemented
    // This should fail initially because the trait doesn't exist or has issues

    #[derive(Clone)]
    struct MockFileWriter;

    #[async_trait::async_trait]
    impl FileWriter for MockFileWriter {
        async fn write_file(&self, _input: FileWriteInput) -> FileWriterResult<OperationResult> {
            // GREEN phase: Simple mock implementation
            Ok(OperationResult::Success {
                original_path: PathBuf::from("test.txt"),
                backup_path: None,
                bytes_written: 0,
                duration_ms: 0,
            })
        }

        async fn write_files(
            &self,
            _inputs: Vec<FileWriteInput>,
        ) -> FileWriterResult<FileWriteReport> {
            // GREEN phase: Simple mock implementation
            Ok(FileWriteReport::new(FileOperation::Create))
        }

        async fn check_safety(
            &self,
            _input: &FileWriteInput,
        ) -> FileWriterResult<WriteSafetyCheck> {
            // GREEN phase: Simple mock implementation
            Ok(WriteSafetyCheck::new(WriteSafetyLevel::Standard))
        }

        async fn create_backup(
            &self,
            _path: &Path,
        ) -> FileWriterResult<Option<crate::backup::BackupInfo>> {
            // GREEN phase: Simple mock implementation
            Ok(None)
        }

        async fn restore_from_backup(
            &self,
            _original_path: &Path,
            _backup_path: &Path,
        ) -> FileWriterResult<()> {
            // GREEN phase: Simple mock implementation
            Ok(())
        }

        async fn rollback_operation(&self, _report: &FileWriteReport) -> FileWriterResult<()> {
            // GREEN phase: Simple mock implementation
            Ok(())
        }

        fn config(&self) -> &FileWriterConfig {
            // GREEN phase: Simple mock implementation
            static DEFAULT_CONFIG: LazyLock<FileWriterConfig> =
                LazyLock::new(FileWriterConfig::default);
            &DEFAULT_CONFIG
        }

        fn update_config(&mut self, _config: FileWriterConfig) {
            // GREEN phase: Simple mock implementation
        }
    }

    // Test that the trait can be used
    let writer = MockFileWriter;
    let input = FileWriteInput {
        path: "test.txt".to_string(),
        content: b"test content".to_vec(),
        operation_type: FileOperation::Create,
        validation_results: None,
    };

    let result = writer.write_file(input).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_safety_checker_functionality() {
    // RED: Test that safety checking works correctly
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

    // GREEN phase: These assertions should pass
    assert!(safety_check.is_safe);
    assert_eq!(safety_check.safety_level, WriteSafetyLevel::Standard);
    assert!(safety_check.risk_score < 0.5);
}

#[tokio::test]
async fn test_backup_manager_functionality() {
    // RED: Test that backup management works correctly
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    // Create a test file
    tokio::fs::write(&test_file, "original content")
        .await
        .unwrap();

    let backup_manager = DefaultBackupManager::new();

    // Test backup creation
    let backup_info = backup_manager
        .create_backup(&test_file, BackupStrategy::Timestamp)
        .await
        .unwrap();

    assert!(backup_info.backup_path.exists());
    assert_eq!(backup_info.original_path, test_file);
    assert_eq!(backup_info.strategy, BackupStrategy::Timestamp);

    // Test backup restoration
    backup_manager.restore_backup(&backup_info).await.unwrap();

    let restored_content = tokio::fs::read_to_string(&test_file).await.unwrap();
    assert_eq!(restored_content, "original content");
}

#[tokio::test]
async fn test_validation_integration() {
    // RED: Test integration with Tool 3 validation results
    let converter = DefaultValidationConverter::new();

    // Create a mock validation report (this would come from Tool 3)
    let mock_report = parseltongue_04::ValidationReport::new(
        std::path::PathBuf::from("test.rs"),
        "fn main() {}".to_string(),
        vec![], // Individual validation results - empty for this test
    );

    let validation_result = converter.convert_validation_report(&mock_report).unwrap();

    // GREEN phase: These assertions should pass
    assert!(validation_result.score >= 0.0);
    assert!(validation_result.score <= 1.0);
    assert!(!validation_result.messages.is_empty());
}

#[tokio::test]
async fn test_default_file_writer_creation() {
    // RED: Test that the default file writer can be created
    let writer = DefaultFileWriter::new();
    let config = writer.config();

    assert_eq!(config.safety_level, WriteSafetyLevel::Standard);
    assert!(config.create_backups);
    assert_eq!(config.backup_strategy, BackupStrategy::Timestamp);
}

#[tokio::test]
async fn test_file_write_report_generation() {
    // RED: Test that file write reports can be generated
    let mut report = FileWriteReport::new(FileOperation::Modify);

    // Add some mock results
    report
        .summary_mut()
        .add_file_result(OperationResult::Success {
            original_path: PathBuf::from("test1.txt"),
            backup_path: Some(PathBuf::from("test1.txt.backup")),
            bytes_written: 100,
            duration_ms: 50,
        });

    report
        .summary_mut()
        .add_file_result(OperationResult::Failed {
            original_path: PathBuf::from("test2.txt"),
            error: "Permission denied".to_string(),
            duration_ms: 10,
        });

    report.summary_mut().complete(true);

    // GREEN phase: These assertions should pass
    assert!(report.summary().success);
    assert_eq!(report.summary().performance_metrics.files_processed, 2);
    assert_eq!(report.summary().performance_metrics.successful_files, 1);
    assert_eq!(report.summary().performance_metrics.failed_files, 1);

    // Test JSON serialization
    let json = report.to_json().unwrap();
    assert!(!json.is_empty());

    // Test JSON deserialization
    let deserialized_report = FileWriteReport::from_json(&json).unwrap();
    assert_eq!(
        deserialized_report.summary().operation_id,
        report.summary().operation_id
    );
}

#[tokio::test]
async fn test_safety_levels() {
    // RED: Test different safety levels
    let levels = [
        WriteSafetyLevel::None,
        WriteSafetyLevel::Basic,
        WriteSafetyLevel::Standard,
        WriteSafetyLevel::Strict,
    ];

    for level in levels {
        let checker = DefaultSafetyChecker::with_config(level, 1024 * 1024, 1024 * 1024);
        assert_eq!(checker.safety_level(), level);
    }
}

#[tokio::test]
async fn test_backup_strategies() {
    // RED: Test different backup strategies
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    tokio::fs::write(&test_file, "test content").await.unwrap();

    let backup_manager = DefaultBackupManager::new();
    let strategies = [
        BackupStrategy::Timestamp,
        BackupStrategy::Numbered,
        BackupStrategy::Single,
    ];

    for strategy in strategies {
        if strategy != BackupStrategy::None {
            let backup_info = backup_manager
                .create_backup(&test_file, strategy)
                .await
                .unwrap();

            assert!(backup_info.backup_path.exists());
            assert_eq!(backup_info.strategy, strategy);
        }
    }
}

#[tokio::test]
async fn test_error_handling() {
    // RED: Test error handling for various scenarios

    // Test file not found error
    let result = tokio::fs::read_to_string("nonexistent_file.txt").await;
    assert!(result.is_err());

    // Test permission error (this might not work on all systems)
    // In GREEN phase, we'll implement proper permission testing

    // Test timeout error
    // In GREEN phase, we'll implement timeout functionality
}

#[tokio::test]
async fn test_batch_operations() {
    // RED: Test batch file writing operations
    let writer = DefaultFileWriter::new();

    let inputs = vec![
        FileWriteInput {
            path: "test1.txt".to_string(),
            content: b"content1".to_vec(),
            operation_type: FileOperation::Create,
            validation_results: None,
        },
        FileWriteInput {
            path: "test2.txt".to_string(),
            content: b"content2".to_vec(),
            operation_type: FileOperation::Create,
            validation_results: None,
        },
    ];

    let report = writer.write_files(inputs).await.unwrap();

    // GREEN phase: These assertions should pass
    assert!(report.summary().is_complete());
    assert_eq!(report.summary().performance_metrics.files_processed, 2);
}

#[tokio::test]
async fn test_performance_metrics() {
    // RED: Test performance metrics collection
    let metrics = WritePerformanceMetrics {
        total_duration_ms: 1000,
        files_processed: 10,
        successful_files: 8,
        failed_files: 1,
        skipped_files: 1,
        total_bytes_written: 10240,
        avg_write_speed_bps: 10240.0,
        peak_memory_bytes: Some(5242880), // 5MB
        backup_files_created: 5,
    };

    // GREEN phase: These calculations should work
    assert_eq!(metrics.success_rate(), 80.0);
    assert_eq!(metrics.failure_rate(), 10.0);
}

#[tokio::test]
async fn test_tool4_integration() {
    // RED: Test Tool 4 integration and API
    let tool4 = Tool4::new();
    let writer = tool4.writer();

    // Test that we can access the underlying writer
    let config = writer.config();
    assert!(config.create_backups);

    // Test default creation
    let default_tool4 = Tool4::default();
    let default_writer = default_tool4.writer();
    assert_eq!(
        default_writer.config().safety_level,
        WriteSafetyLevel::Standard
    );
}

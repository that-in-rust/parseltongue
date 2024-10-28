//! Integration Tests Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Test Scenarios
//! - EndToEndTests    (complete workflows)
//! - PerformanceTests (performance scenarios)
//! - StressTests      (stress scenarios)
//! 
//! Level 3: Test Categories
//! - StorageTests     (storage operations)
//! - ZipTests        (ZIP operations)
//! - MetricsTests    (metrics collection)
//! 
//! Level 2: Test Support
//! - TestFixtures    (test data setup)
//! - TestValidation  (result validation)
//! - TestCleanup     (resource cleanup)
//! 
//! Level 1 (Base): Core Test Types
//! - TestCase        (test case structure)
//! - TestResult      (test result types)
//! - TestMetrics     (test metrics)

use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;
use tempfile::TempDir;
use crate::{
    core::types::*,
    storage::{StorageManager, StorageConfig},
    zip::{ZipProcessor, ZipConfig, ZipEntry},
    metrics::{MetricsManager, MetricsConfig},
};

// ===== Level 1: Core Test Types =====
// Design Choice: Using structured test cases

/// Test configuration
#[derive(Debug, Clone)]
struct TestConfig {
    temp_dir: TempDir,
    cleanup_enabled: bool,
    metrics_enabled: bool,
}

/// Test case implementation
struct TestCase {
    name: String,
    config: TestConfig,
    metrics: TestMetrics,
}

// ===== Level 2: Test Support =====
// Design Choice: Using fixtures for test data

impl TestCase {
    async fn new(name: &str) -> Result<Self> {
        Ok(Self {
            name: name.to_string(),
            config: TestConfig {
                temp_dir: TempDir::new()?,
                cleanup_enabled: true,
                metrics_enabled: true,
            },
            metrics: TestMetrics::default(),
        })
    }

    async fn setup_storage(&self) -> Result<StorageManager> {
        let config = StorageConfig {
            path: self.config.temp_dir.path().join("storage"),
            pool_size: 4,
            batch_size: 1000,
            index_config: Default::default(),
        };
        StorageManager::new(config).await
    }

    async fn setup_processor(&self) -> Result<ZipProcessor> {
        let config = ZipConfig::default();
        let storage = self.setup_storage().await?;
        ZipProcessor::new(config, Arc::new(storage))
    }
}

// ===== Level 3: Test Categories =====
// Design Choice: Using modular test organization

mod storage_tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_operations() -> Result<()> {
        let test = TestCase::new("storage_operations").await?;
        let storage = test.setup_storage().await?;

        // Test basic storage operations
        let key = "test_key";
        let data = bytes::Bytes::from("test_data");
        
        storage.store(key, data.clone()).await?;
        let retrieved = storage.get(key).await?;
        assert_eq!(retrieved, Some(data));

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_storage() -> Result<()> {
        let test = TestCase::new("concurrent_storage").await?;
        let storage = test.setup_storage().await?;

        let mut handles = Vec::new();
        for i in 0..10 {
            let storage = storage.clone();
            let handle = tokio::spawn(async move {
                let key = format!("key_{}", i);
                let data = bytes::Bytes::from(format!("data_{}", i));
                storage.store(&key, data).await
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await??;
        }

        Ok(())
    }
}

mod zip_tests {
    use super::*;

    #[tokio::test]
    async fn test_zip_processing() -> Result<()> {
        let test = TestCase::new("zip_processing").await?;
        let processor = test.setup_processor().await?;

        // Create test ZIP file
        let zip_path = test.config.temp_dir.path().join("test.zip");
        {
            let file = std::fs::File::create(&zip_path)?;
            let mut zip = zip::ZipWriter::new(file);
            
            zip.start_file("test.txt", Default::default())?;
            zip.write_all(b"Hello, World!")?;
            zip.finish()?;
        }

        // Process ZIP file
        let file = tokio::fs::File::open(zip_path).await?;
        processor.process(file).await?;

        Ok(())
    }
}

// ===== Level 4: Test Scenarios =====
// Design Choice: Using comprehensive workflows

#[tokio::test]
async fn test_end_to_end_workflow() -> Result<()> {
    let test = TestCase::new("end_to_end").await?;
    
    // Setup components
    let storage = test.setup_storage().await?;
    let processor = test.setup_processor().await?;
    let metrics = MetricsManager::new(MetricsConfig {
        enabled: true,
        interval: std::time::Duration::from_secs(1),
        format: Default::default(),
    });

    // Create test data
    let zip_path = test.config.temp_dir.path().join("test.zip");
    {
        let file = std::fs::File::create(&zip_path)?;
        let mut zip = zip::ZipWriter::new(file);
        
        // Add multiple files
        for i in 0..5 {
            zip.start_file(format!("file_{}.txt", i), Default::default())?;
            zip.write_all(format!("Content {}", i).as_bytes())?;
        }
        zip.finish()?;
    }

    // Start metrics collection
    metrics.start().await?;

    // Process ZIP file
    let file = tokio::fs::File::open(zip_path).await?;
    processor.process(file).await?;

    // Verify results
    for i in 0..5 {
        let key = format!("file_{}.txt", i);
        let data = storage.get(&key).await?;
        assert!(data.is_some());
        assert_eq!(
            std::str::from_utf8(&data.unwrap())?,
            format!("Content {}", i)
        );
    }

    // Stop metrics collection
    metrics.stop().await?;

    Ok(())
}

#[tokio::test]
async fn test_stress_workflow() -> Result<()> {
    let test = TestCase::new("stress_test").await?;
    let processor = test.setup_processor().await?;
    let mut handles = Vec::new();

    // Create multiple ZIP files
    for i in 0..5 {
        let zip_path = test.config.temp_dir.path().join(format!("test_{}.zip", i));
        {
            let file = std::fs::File::create(&zip_path)?;
            let mut zip = zip::ZipWriter::new(file);
            
            for j in 0..100 {
                zip.start_file(format!("file_{}_{}.txt", i, j), Default::default())?;
                zip.write_all(format!("Content {}_{}", i, j).as_bytes())?;
            }
            zip.finish()?;
        }

        let processor = processor.clone();
        let handle = tokio::spawn(async move {
            let file = tokio::fs::File::open(zip_path).await?;
            processor.process(file).await
        });
        handles.push(handle);
    }

    // Wait for all processing to complete
    for handle in handles {
        handle.await??;
    }

    Ok(())
}

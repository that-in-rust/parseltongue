//! Integration Tests - Pyramidal Structure
//! Layer 1: Test Setup
//! Layer 2: Core Tests
//! Layer 3: Feature Tests
//! Layer 4: Error Cases
//! Layer 5: Performance Tests

use anyhow::Result;
use parseltongue::{Config, MetricsManager, RuntimeManager, StorageManager, ZipProcessor};
use crate::common::{TestContext, TestConfig};

// Layer 1: Test Setup
async fn setup() -> Result<TestContext> {
    let config = TestConfig::default();
    TestContext::new(config).await
}

// Layer 2: Core Tests
#[tokio::test]
async fn test_zip_processing() -> Result<()> {
    let ctx = setup().await?;
    let metrics = MetricsManager::new();
    let runtime = RuntimeManager::new(&ctx.config)?;
    let storage = StorageManager::new(&ctx.config).await?;
    let processor = ZipProcessor::new(ctx.config)?;

    processor.process().await?;
    runtime.shutdown().await?;
    storage.shutdown().await?;
    metrics.shutdown().await?;

    Ok(())
}

// Layer 3: Feature Tests
#[tokio::test]
async fn test_metrics_collection() -> Result<()> {
    let ctx = setup().await?;
    let metrics = MetricsManager::new();

    metrics.record_file_processed(100, std::time::Duration::from_millis(50)).await?;
    let exported = metrics.export_metrics().await?;
    assert!(exported.contains("total_bytes"));

    metrics.shutdown().await?;
    Ok(())
}

// Layer 4: Error Cases
#[tokio::test]
async fn test_invalid_zip() -> Result<()> {
    let temp_file = tempfile::NamedTempFile::new()?;
    std::fs::write(&temp_file, b"not a zip file")?;

    let result = Config::builder()
        .input_zip(temp_file.path())
        .output_dir("output")
        .build();

    assert!(result.is_err());
    Ok(())
}

// Layer 5: Performance Tests
#[tokio::test]
async fn test_large_file_handling() -> Result<()> {
    let config = TestConfig {
        file_count: 100,
        file_size: 1024 * 1024, // 1MB files
        workers: 4,
    };
    
    let ctx = TestContext::new(config).await?;
    let processor = ZipProcessor::new(ctx.config)?;
    processor.process().await?;
    
    Ok(())
}

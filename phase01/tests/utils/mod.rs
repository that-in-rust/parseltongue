//! Test Utilities Infrastructure
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Test Orchestration
//! - TestManager      (manages test setup/teardown)
//! - TestMetrics      (tracks test performance)
//! - ResourceManager  (manages test resources)
//! 
//! Level 3: Test Support
//! - MockBuilder      (builds mock objects)
//! - TestContext      (manages test context)
//! - TestFixtures     (manages test data)
//! 
//! Level 2: Test Helpers
//! - AsyncTestHelper  (async test support)
//! - MockStorage     (storage mocks)
//! - MockZip         (ZIP mocks)
//! 
//! Level 1 (Base): Core Test Types
//! - TestConfig      (test configuration)
//! - TestError       (test error types)
//! - TestMetrics     (test metrics)

use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;
use tempfile::TempDir;
use bytes::Bytes;
use crate::{
    core::types::*,
    storage::AsyncStorage,
    zip::{ZipEntry, ZipConfig},
};

// ===== Level 1: Core Test Types =====
// Design Choice: Using builder pattern for test setup

/// Test metrics collection
#[derive(Debug, Default)]
pub struct TestMetrics {
    pub tests_run: usize,
    pub tests_passed: usize,
    pub tests_failed: usize,
    pub total_duration: std::time::Duration,
}

/// Test configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub temp_dir: TempDir,
    pub timeout: std::time::Duration,
    pub cleanup_enabled: bool,
}

// ===== Level 2: Test Helpers =====
// Design Choice: Using traits for test support

/// Async test helper trait
#[async_trait::async_trait]
pub trait AsyncTestHelper {
    async fn setup() -> Result<Self> where Self: Sized;
    async fn cleanup(self) -> Result<()>;
}

/// Mock storage implementation
#[derive(Clone)]
pub struct MockStorage {
    data: Arc<Mutex<std::collections::HashMap<String, Bytes>>>,
    metrics: Arc<Mutex<StorageMetrics>>,
}

#[async_trait::async_trait]
impl AsyncStorage for MockStorage {
    async fn store(&self, key: &str, value: Bytes) -> Result<()> {
        let mut data = self.data.lock().await;
        let mut metrics = self.metrics.lock().await;
        
        data.insert(key.to_string(), value.clone());
        metrics.bytes_written += value.len();
        metrics.operations_completed += 1;
        
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Bytes>> {
        let data = self.data.lock().await;
        let mut metrics = self.metrics.lock().await;
        
        metrics.operations_completed += 1;
        Ok(data.get(key).cloned())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let mut data = self.data.lock().await;
        let mut metrics = self.metrics.lock().await;
        
        data.remove(key);
        metrics.operations_completed += 1;
        
        Ok(())
    }
}

// ===== Level 3: Test Support =====
// Design Choice: Using context for test state

/// Test context implementation
pub struct TestContext {
    config: TestConfig,
    metrics: TestMetrics,
    storage: MockStorage,
}

impl TestContext {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            config: TestConfig {
                temp_dir: TempDir::new()?,
                timeout: std::time::Duration::from_secs(30),
                cleanup_enabled: true,
            },
            metrics: TestMetrics::default(),
            storage: MockStorage {
                data: Arc::new(Mutex::new(std::collections::HashMap::new())),
                metrics: Arc::new(Mutex::new(StorageMetrics::default())),
            },
        })
    }

    pub async fn create_test_zip(&self, entries: Vec<(String, Vec<u8>)>) -> Result<std::path::PathBuf> {
        let zip_path = self.config.temp_dir.path().join("test.zip");
        let file = std::fs::File::create(&zip_path)?;
        let mut zip = zip::ZipWriter::new(file);

        for (name, data) in entries {
            zip.start_file(name, Default::default())?;
            zip.write_all(&data)?;
        }
        zip.finish()?;

        Ok(zip_path)
    }
}

// ===== Level 4: Test Orchestration =====
// Design Choice: Using builder for test setup

/// Test builder implementation
pub struct TestBuilder {
    context: TestContext,
    setup_fns: Vec<Box<dyn FnOnce(&TestContext) -> Result<()> + Send>>,
    cleanup_fns: Vec<Box<dyn FnOnce(&TestContext) -> Result<()> + Send>>,
}

impl TestBuilder {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            context: TestContext::new().await?,
            setup_fns: Vec::new(),
            cleanup_fns: Vec::new(),
        })
    }

    pub fn with_setup<F>(&mut self, f: F) -> &mut Self 
    where
        F: FnOnce(&TestContext) -> Result<()> + Send + 'static,
    {
        self.setup_fns.push(Box::new(f));
        self
    }

    pub fn with_cleanup<F>(&mut self, f: F) -> &mut Self 
    where
        F: FnOnce(&TestContext) -> Result<()> + Send + 'static,
    {
        self.cleanup_fns.push(Box::new(f));
        self
    }

    pub async fn run<F, Fut>(&self, test_fn: F) -> Result<()>
    where
        F: FnOnce(&TestContext) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        // Run setup
        for setup in &self.setup_fns {
            setup(&self.context)?;
        }

        // Run test
        test_fn(&self.context).await?;

        // Run cleanup
        for cleanup in &self.cleanup_fns {
            cleanup(&self.context)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_storage() -> Result<()> {
        let storage = MockStorage {
            data: Arc::new(Mutex::new(std::collections::HashMap::new())),
            metrics: Arc::new(Mutex::new(StorageMetrics::default())),
        };

        let key = "test_key";
        let value = Bytes::from("test_value");

        storage.store(key, value.clone()).await?;
        let retrieved = storage.get(key).await?;
        
        assert_eq!(retrieved, Some(value));
        Ok(())
    }

    #[tokio::test]
    async fn test_builder_pattern() -> Result<()> {
        let mut builder = TestBuilder::new().await?;
        
        builder
            .with_setup(|ctx| {
                println!("Setup completed");
                Ok(())
            })
            .with_cleanup(|ctx| {
                println!("Cleanup completed");
                Ok(())
            });

        builder.run(|ctx| async {
            println!("Test running");
            Ok(())
        }).await?;

        Ok(())
    }
}

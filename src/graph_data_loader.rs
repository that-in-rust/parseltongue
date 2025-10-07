//! Graph Data Loader - Dependency Injection for Testability
//!
//! Following steering docs Principle #3: Dependency Injection for Testability
//! This trait allows for mocking data sources in tests while using real data in production

use async_trait::async_trait;
use crate::isg::OptimizedISG;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphDataError {
    #[error("Failed to load ISG data: {0}")]
    ISGLoadError(String),
    #[error("Failed to convert to WASM format: {0}")]
    ConversionError(String),
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl Clone for GraphDataError {
    fn clone(&self) -> Self {
        match self {
            GraphDataError::ISGLoadError(msg) => GraphDataError::ISGLoadError(msg.clone()),
            GraphDataError::ConversionError(msg) => GraphDataError::ConversionError(msg.clone()),
            GraphDataError::FileNotFound { path } => GraphDataError::FileNotFound { path: path.clone() },
            GraphDataError::IoError(msg) => GraphDataError::IoError(msg.clone()),
            GraphDataError::SerializationError(msg) => GraphDataError::SerializationError(msg.clone()),
        }
    }
}

/// Result type for graph data operations
pub type GraphDataResult<T> = Result<T, GraphDataError>;

/// Graph Data Loader Trait - Dependency Injection for Testability
///
/// This trait enables:
/// - Test doubles and mocks for unit testing
/// - Different data sources (files, databases, APIs)
/// - Performance monitoring and caching
/// - Error handling and recovery strategies
#[async_trait]
pub trait GraphDataLoader: Send + Sync {
    /// Load ISG data from the configured source
    async fn load_isg(&self) -> GraphDataResult<OptimizedISG>;

    /// Get metadata about the data source
    fn metadata(&self) -> GraphDataMetadata;

    /// Check if the data source is available
    async fn is_available(&self) -> bool;

    /// Get the source identifier for logging/debugging
    fn source_id(&self) -> String;
}

/// Metadata about a graph data source
#[derive(Debug, Clone)]
pub struct GraphDataMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub node_count_estimate: Option<usize>,
    pub edge_count_estimate: Option<usize>,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

/// Default file-based ISG loader
pub struct FileISGLoader {
    file_path: PathBuf,
    metadata: GraphDataMetadata,
}

impl FileISGLoader {
    pub fn new(file_path: PathBuf) -> Self {
        let metadata = GraphDataMetadata {
            name: file_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            description: format!("ISG data from file: {:?}", file_path),
            version: "1.0".to_string(),
            node_count_estimate: None,
            edge_count_estimate: None,
            last_updated: None,
        };

        Self { file_path, metadata }
    }

    pub fn with_metadata(file_path: PathBuf, metadata: GraphDataMetadata) -> Self {
        Self { file_path, metadata }
    }
}

#[async_trait]
impl GraphDataLoader for FileISGLoader {
    async fn load_isg(&self) -> GraphDataResult<OptimizedISG> {
        if !self.file_path.exists() {
            return Err(GraphDataError::FileNotFound {
                path: self.file_path.clone()
            });
        }

        // For now, use the existing ISG loading logic
        // This would integrate with the existing ISG parsing code
        let file_content = tokio::fs::read_to_string(&self.file_path).await
            .map_err(|e| GraphDataError::IoError(e.to_string()))?;

        // Parse the file content based on file type
        if self.file_path.extension().and_then(|s| s.to_str()) == Some("json") {
            // Load from JSON format
            let isg: OptimizedISG = serde_json::from_str(&file_content)
                .map_err(|e| GraphDataError::SerializationError(e.to_string()))?;
            Ok(isg)
        } else {
            // Load from Rust source files (existing functionality)
            // This would use the existing daemon/ingest logic
            Err(GraphDataError::ISGLoadError(
                "Rust source file loading not yet implemented in trait".to_string()
            ))
        }
    }

    fn metadata(&self) -> GraphDataMetadata {
        self.metadata.clone()
    }

    async fn is_available(&self) -> bool {
        self.file_path.exists()
    }

    fn source_id(&self) -> String {
        format!("file:{:?}", self.file_path)
    }
}

/// In-memory ISG loader for testing
pub struct MemoryISGLoader {
    isg: OptimizedISG,
    metadata: GraphDataMetadata,
    source_id: String,
}

impl MemoryISGLoader {
    pub fn new(isg: OptimizedISG) -> Self {
        let metadata = GraphDataMetadata {
            name: "Memory Test Data".to_string(),
            description: "ISG data loaded in memory for testing".to_string(),
            version: "test-1.0".to_string(),
            node_count_estimate: Some(isg.node_count()),
            edge_count_estimate: Some(isg.edge_count()),
            last_updated: Some(chrono::Utc::now()),
        };

        let source_id = format!("memory:test-{}", uuid::Uuid::new_v4());

        Self { isg, metadata, source_id }
    }

    pub fn with_metadata(isg: OptimizedISG, metadata: GraphDataMetadata) -> Self {
        let source_id = format!("memory:{}", uuid::Uuid::new_v4());
        Self { isg, metadata, source_id }
    }
}

#[async_trait]
impl GraphDataLoader for MemoryISGLoader {
    async fn load_isg(&self) -> GraphDataResult<OptimizedISG> {
        Ok(self.isg.clone())
    }

    fn metadata(&self) -> GraphDataMetadata {
        self.metadata.clone()
    }

    async fn is_available(&self) -> bool {
        true // Memory data is always available
    }

    fn source_id(&self) -> String {
        self.source_id.clone()
    }
}

/// Mock ISG loader for testing error conditions
pub struct MockErrorLoader {
    error: GraphDataError,
    metadata: GraphDataMetadata,
}

impl MockErrorLoader {
    pub fn new(error: GraphDataError) -> Self {
        let metadata = GraphDataMetadata {
            name: "Mock Error Loader".to_string(),
            description: "Mock loader that always returns an error".to_string(),
            version: "mock-1.0".to_string(),
            node_count_estimate: None,
            edge_count_estimate: None,
            last_updated: None,
        };

        Self { error, metadata }
    }
}

#[async_trait]
impl GraphDataLoader for MockErrorLoader {
    async fn load_isg(&self) -> GraphDataResult<OptimizedISG> {
        Err(self.error.clone())
    }

    fn metadata(&self) -> GraphDataMetadata {
        self.metadata.clone()
    }

    async fn is_available(&self) -> bool {
        false // Mock error loader is never available
    }

    fn source_id(&self) -> String {
        "mock:error".to_string()
    }
}

/// Factory for creating common graph data loaders
pub struct GraphDataLoaderFactory;

impl GraphDataLoaderFactory {
    /// Create a loader for Rust source files
    pub fn for_rust_source(source_path: PathBuf) -> Box<dyn GraphDataLoader> {
        Box::new(FileISGLoader::new(source_path))
    }

    /// Create a loader for JSON ISG files
    pub fn for_json_file(json_path: PathBuf) -> Box<dyn GraphDataLoader> {
        Box::new(FileISGLoader::new(json_path))
    }

    /// Create a memory loader for testing
    pub fn for_testing(isg: OptimizedISG) -> Box<dyn GraphDataLoader> {
        Box::new(MemoryISGLoader::new(isg))
    }

    /// Create a mock error loader for testing error conditions
    pub fn for_error_testing(error: GraphDataError) -> Box<dyn GraphDataLoader> {
        Box::new(MockErrorLoader::new(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isg::{NodeData, SigHash, NodeKind};

    #[tokio::test]
    async fn test_memory_loader_success() {
        // GIVEN: A test ISG with known data
        let isg = OptimizedISG::new();
        let test_node = NodeData {
            hash: SigHash::new("test_function"),
            kind: NodeKind::Function,
            name: "test_function".into(),
            signature: "fn test_function()".into(),
            file_path: "test.rs".into(),
            line: 1,
        };
        isg.upsert_node(test_node);

        // WHEN: Creating a memory loader and loading the ISG
        let loader = MemoryISGLoader::new(isg.clone());
        let loaded_isg = loader.load_isg().await.unwrap();

        // THEN: Should return the same ISG
        assert_eq!(loaded_isg.node_count(), isg.node_count());
        assert_eq!(loaded_isg.edge_count(), isg.edge_count());

        // AND: Should provide correct metadata
        let metadata = loader.metadata();
        assert_eq!(metadata.name, "Memory Test Data");
        assert_eq!(metadata.node_count_estimate, Some(isg.node_count()));

        // AND: Should always be available
        assert!(loader.is_available().await);
    }

    #[tokio::test]
    async fn test_mock_error_loader() {
        // GIVEN: A mock error loader
        let expected_error = GraphDataError::ISGLoadError("Test error".to_string());
        let loader = MockErrorLoader::new(expected_error.clone());

        // WHEN: Loading ISG
        let result = loader.load_isg().await;

        // THEN: Should return the expected error
        assert!(result.is_err());
        if let Err(error) = result {
            let error_str = error.to_string();
            assert!(error_str.contains("Test error"), "Expected error containing 'Test error', got: {}", error_str);
        }

        // AND: Should never be available
        assert!(!loader.is_available().await);
    }

    #[test]
    fn test_factory_creators() {
        // GIVEN: Different loader factory methods

        // WHEN: Creating loaders
        let rust_loader = GraphDataLoaderFactory::for_rust_source(PathBuf::from("src/main.rs"));
        let json_loader = GraphDataLoaderFactory::for_json_file(PathBuf::from("data.json"));

        // THEN: Should return different loader types
        assert!(rust_loader.source_id().starts_with("file:"));
        assert!(json_loader.source_id().starts_with("file:"));
    }
}
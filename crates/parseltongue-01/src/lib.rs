//! Core types and traits with no external dependencies (L1: Core Language Features)
//! Following TDD-first principle - tests first, implementation second

pub mod types {
    //! Core type definitions following newtype patterns

    /// Type-safe identifier following newtype pattern
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ISGL1Key {
        pub filepath: std::path::PathBuf,
        pub filename: String,
        pub interface_name: String,
    }

    impl ISGL1Key {
        pub fn new(filepath: std::path::PathBuf, filename: String, interface_name: String) -> Self {
            Self {
                filepath,
                filename,
                interface_name,
            }
        }

        /// Generate stable hash for consistent identification
        pub fn stable_hash(&self) -> u64 {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::hash::DefaultHasher::new();
            self.filepath.hash(&mut hasher);
            self.filename.hash(&mut hasher);
            self.interface_name.hash(&mut hasher);
            hasher.finish()
        }
    }

    /// Core error types for library usage
    #[derive(Debug, thiserror::Error)]
    pub enum CoreError {
        #[error("Invalid ISGL1 key format: {0}")]
        InvalidKey(String),

        #[error("IO operation failed: {0}")]
        Io(#[from] std::io::Error),

        #[error("Resource not found: {0}")]
        ResourceNotFound(String),
    }

    /// Result type for core operations
    pub type CoreResult<T> = Result<T, CoreError>;
}

pub mod traits {
    //! Universal parser trait with capability-based design

    use std::fmt::Debug;

    /// Parser capability flags for feature detection
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ParserCapabilities {
        pub supports_syntax: bool,
        pub supports_semantics: bool,
        pub supports_type_inference: bool,
        pub supports_macros: bool,
        pub supports_attributes: bool,
    }

    /// Input format enumeration with extensible design
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum InputFormat {
        Folder(std::path::PathBuf),
        SingleFile(std::path::PathBuf),
        Text(std::borrow::Cow<'static, str>),
    }

    /// Universal parser trait following dependency injection principle
    #[async_trait::async_trait]
    pub trait UniversalParser: Clone + Send + Sync + 'static {
        type Input: Clone + Send + Sync + Debug;
        type Output: Clone + Send + Sync + Debug;
        type Error: Debug + Send + Sync;

        /// Parse input with automatic capability detection
        async fn parse(&self, input: &Self::Input) -> Result<Self::Output, Self::Error>;

        /// Check format support with confidence scoring
        async fn supports_format(&self, format: &InputFormat) -> f64;

        /// Get parser capabilities for feature detection
        fn capabilities(&self) -> ParserCapabilities;

        /// Get parser name for debugging
        fn name(&self) -> &'static str;

        /// Get estimated memory usage for input size
        fn estimate_memory_usage(&self, input_size_bytes: usize) -> usize;
    }

    /// Stream processor trait for functional composition
    #[async_trait::async_trait]
    pub trait StreamProcessor<T>: Clone + Send + Sync + 'static {
        type Item: Clone + Send + Sync;
        type Error: Debug + Send + Sync;

        /// Process stream with backpressure management
        async fn process_stream(
            &self,
            input: super::streaming::BoundedStream<T>,
        ) -> Result<super::streaming::BoundedStream<Self::Item>, Self::Error>;

        /// Get optimal batch size for performance
        async fn optimal_batch_size(&self) -> usize;

        /// Get memory limit for safe processing
        async fn memory_limit(&self) -> usize;
    }
}

pub mod streaming {
    //! Streaming engine and standard library-based components

    use std::sync::{Arc, RwLock};

    /// Bounded channel for backpressure management
    pub struct BoundedStream<T> {
        pub sender: tokio::sync::mpsc::Sender<T>,
        pub receiver: tokio::sync::mpsc::Receiver<T>,
        pub buffer_size: usize,
    }

    impl<T> BoundedStream<T> {
        pub fn new(buffer_size: usize) -> Self {
            let (sender, receiver) = tokio::sync::mpsc::channel(buffer_size);
            Self {
                sender,
                receiver,
                buffer_size,
            }
        }

        pub async fn send(&self, item: T) -> Result<(), tokio::sync::mpsc::error::SendError<T>> {
            self.sender.send(item).await
        }

        pub async fn recv(&mut self) -> Option<T> {
            self.receiver.recv().await
        }
    }

    /// Smart pointer decision matrix for concurrent access
    #[derive(Debug, Default)]
    pub struct CodeGraph {
        nodes: Arc<RwLock<std::collections::HashMap<super::types::ISGL1Key, CodeNode>>>,
        #[allow(dead_code)] // Will be used in future tools
        edges: Arc<
            RwLock<std::collections::HashMap<super::types::ISGL1Key, Vec<super::types::ISGL1Key>>>,
        >,
    }

    #[derive(Debug, Clone)]
    pub struct CodeNode {
        pub current_code: String,
        pub future_code: Option<String>,
        pub interface_signature: Option<String>,
        pub tdd_classification: Option<String>,
        pub current_id: u32,
        pub future_id: u32,
        pub lsp_meta_data: Option<String>,
    }

    impl CodeGraph {
        pub fn new() -> Self {
            Self::default()
        }

        /// Thread-safe node insertion with automatic index maintenance
        pub fn insert_node(
            &self,
            key: super::types::ISGL1Key,
            node: CodeNode,
        ) -> super::types::CoreResult<()> {
            let mut nodes = self.nodes.write().map_err(|e| {
                super::types::CoreError::ResourceNotFound(format!("Nodes lock poisoned: {}", e))
            })?;

            nodes.insert(key.clone(), node);
            Ok(())
        }
    }
}

pub mod resource {
    //! RAII resource management for temporary files

    /// RAII resource management for temporary files
    pub struct FileHandleGuard {
        pub handle: std::fs::File,
        pub path: std::path::PathBuf,
    }

    impl FileHandleGuard {
        pub fn create(path: std::path::PathBuf) -> super::types::CoreResult<Self> {
            let handle = std::fs::File::create(&path)?;
            Ok(Self { handle, path })
        }

        pub fn path(&self) -> &std::path::PathBuf {
            &self.path
        }
    }

    impl Drop for FileHandleGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

pub mod performance;

// Re-export key types for convenience
pub use performance::{
    ParsingPerformanceContract, PerformanceError, PerformanceReport, PerformanceValidator,
    StreamPerformanceContract, StreamPerformanceReport,
};

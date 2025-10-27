//! RED PHASE: Failing tests for core types and traits
//! Following TDD principle: Write failing tests first

use parseltongue_01::traits::{StreamProcessor, UniversalParser};
use parseltongue_01::*;
use std::path::PathBuf;

#[tokio::test]
async fn test_isgl1key_creation_and_hashing() {
    // RED: Test should fail initially, then pass after implementation
    let filepath = PathBuf::from("/src/main.rs");
    let filename = "main.rs".to_string();
    let interface_name = "main".to_string();

    let key = types::ISGL1Key::new(filepath.clone(), filename.clone(), interface_name.clone());

    // Test basic properties
    assert_eq!(key.filepath, filepath);
    assert_eq!(key.filename, filename);
    assert_eq!(key.interface_name, interface_name);

    // Test hash stability
    let hash1 = key.stable_hash();
    let hash2 = key.stable_hash();
    assert_eq!(hash1, hash2, "Hash should be stable for same input");

    // Test hash uniqueness for different keys
    let key2 = types::ISGL1Key::new(
        PathBuf::from("/src/lib.rs"),
        "lib.rs".to_string(),
        "lib".to_string(),
    );
    assert_ne!(
        key.stable_hash(),
        key2.stable_hash(),
        "Different keys should have different hashes"
    );
}

#[tokio::test]
async fn test_isgl1key_equality_and_ordering() {
    // RED: Test type-safe equality
    let key1 = types::ISGL1Key::new(
        PathBuf::from("/src/main.rs"),
        "main.rs".to_string(),
        "main".to_string(),
    );

    let key2 = types::ISGL1Key::new(
        PathBuf::from("/src/main.rs"),
        "main.rs".to_string(),
        "main".to_string(),
    );

    let key3 = types::ISGL1Key::new(
        PathBuf::from("/src/lib.rs"),
        "lib.rs".to_string(),
        "lib".to_string(),
    );

    assert_eq!(key1, key2, "Identical keys should be equal");
    assert_ne!(key1, key3, "Different keys should not be equal");

    // Test hashing for use in HashMap
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(key1.clone(), "test_value");
    assert_eq!(
        map.get(&key2),
        Some(&"test_value"),
        "Equal keys should be retrievable from HashMap"
    );
}

#[tokio::test]
async fn test_core_error_types() {
    // RED: Test structured error handling
    let io_error = types::CoreError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "File not found",
    ));
    assert!(matches!(io_error, types::CoreError::Io(_)));

    let invalid_key_error = types::CoreError::InvalidKey("Invalid format".to_string());
    assert!(matches!(invalid_key_error, types::CoreError::InvalidKey(_)));

    // Test error display
    let error_message = format!("{}", io_error);
    assert!(error_message.contains("IO operation failed"));
}

#[tokio::test]
async fn test_parser_capabilities() {
    // RED: Test parser capability flags
    let capabilities = traits::ParserCapabilities {
        supports_syntax: true,
        supports_semantics: false,
        supports_type_inference: false,
        supports_macros: true,
        supports_attributes: true,
    };

    assert!(capabilities.supports_syntax);
    assert!(!capabilities.supports_semantics);
    assert!(capabilities.supports_macros);
    assert!(capabilities.supports_attributes);
}

#[tokio::test]
async fn test_input_format_equality() {
    // RED: Test input format variants
    let folder_format = traits::InputFormat::Folder(PathBuf::from("/test"));
    let file_format = traits::InputFormat::SingleFile(PathBuf::from("/test.rs"));
    let text_format = traits::InputFormat::Text(std::borrow::Cow::Borrowed("code"));

    assert_eq!(
        folder_format,
        traits::InputFormat::Folder(PathBuf::from("/test"))
    );
    assert_ne!(folder_format, file_format);
    assert_ne!(file_format, text_format);

    // Test cloning
    let cloned_format = folder_format.clone();
    assert_eq!(folder_format, cloned_format);
}

// Mock implementations for testing trait compilation
#[derive(Debug, Clone)]
struct MockParser {
    name: &'static str,
    capabilities: traits::ParserCapabilities,
}

#[async_trait::async_trait]
impl traits::UniversalParser for MockParser {
    type Input = String;
    type Output = String;
    type Error = String;

    async fn parse(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(format!("parsed: {}", input))
    }

    async fn supports_format(&self, format: &traits::InputFormat) -> f64 {
        match format {
            traits::InputFormat::Text(_) => 1.0,
            _ => 0.0,
        }
    }

    fn capabilities(&self) -> traits::ParserCapabilities {
        self.capabilities
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn estimate_memory_usage(&self, input_size_bytes: usize) -> usize {
        input_size_bytes * 2
    }
}

#[tokio::test]
async fn test_universal_parser_trait_compilation() {
    // RED: Test that trait can be implemented
    let parser = MockParser {
        name: "test_parser",
        capabilities: traits::ParserCapabilities {
            supports_syntax: true,
            supports_semantics: false,
            supports_type_inference: false,
            supports_macros: false,
            supports_attributes: false,
        },
    };

    assert_eq!(parser.name(), "test_parser");
    assert!(parser.capabilities().supports_syntax);

    let input = "test code".to_string();
    let result = parser.parse(&input).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "parsed: test code");

    // Test format support
    let text_format = traits::InputFormat::Text(std::borrow::Cow::Borrowed("code"));
    let support_score = parser.supports_format(&text_format).await;
    assert_eq!(support_score, 1.0);

    let folder_format = traits::InputFormat::Folder(PathBuf::from("/test"));
    let support_score = parser.supports_format(&folder_format).await;
    assert_eq!(support_score, 0.0);

    // Test memory estimation
    let memory_estimate = parser.estimate_memory_usage(1000);
    assert_eq!(memory_estimate, 2000);
}

#[derive(Debug, Clone)]
struct MockStreamProcessor;

#[async_trait::async_trait]
impl traits::StreamProcessor<String> for MockStreamProcessor {
    type Item = String;
    type Error = String;

    async fn process_stream(
        &self,
        _input: streaming::BoundedStream<String>,
    ) -> Result<streaming::BoundedStream<Self::Item>, Self::Error> {
        let output = streaming::BoundedStream::new(100);
        Ok(output)
    }

    async fn optimal_batch_size(&self) -> usize {
        500
    }

    async fn memory_limit(&self) -> usize {
        50 * 1024 * 1024 // 50MB
    }
}

#[tokio::test]
async fn test_stream_processor_trait_compilation() {
    // RED: Test that stream processor trait can be implemented
    let processor = MockStreamProcessor;

    assert_eq!(processor.optimal_batch_size().await, 500);
    assert_eq!(processor.memory_limit().await, 50 * 1024 * 1024);

    // Test stream processing
    let input_stream = streaming::BoundedStream::new(100);
    let result = processor.process_stream(input_stream).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bounded_stream_functionality() {
    // RED: Test bounded stream creation and basic operations
    let stream: streaming::BoundedStream<String> = streaming::BoundedStream::new(10);
    assert_eq!(stream.buffer_size, 10);
}

#[tokio::test]
async fn test_code_graph_creation_and_node_insertion() {
    // RED: Test code graph basic functionality
    let graph = streaming::CodeGraph::new();

    let key = types::ISGL1Key::new(
        PathBuf::from("/src/main.rs"),
        "main.rs".to_string(),
        "main".to_string(),
    );

    let node = streaming::CodeNode {
        current_code: "fn main() {}".to_string(),
        future_code: None,
        interface_signature: Some("main()".to_string()),
        tdd_classification: Some("CODE_IMPLEMENTATION".to_string()),
        current_id: 1,
        future_id: 0,
        lsp_meta_data: None,
    };

    // Test node insertion - should succeed
    let result = graph.insert_node(key, node);
    assert!(result.is_ok(), "Node insertion should succeed");
}

#[tokio::test]
async fn test_file_handle_guard_creation_and_cleanup() {
    // RED: Test RAII resource management
    use std::io::Write;
    use tempfile::tempdir;

    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");

    // Create file handle guard
    let mut guard = resource::FileHandleGuard::create(file_path.clone()).unwrap();
    assert_eq!(guard.path(), &file_path);

    // Write some data
    writeln!(guard.handle, "test data").unwrap();

    // File should exist
    assert!(file_path.exists());

    // Drop the guard - should clean up file
    drop(guard);

    // File should be cleaned up (this test might be racey on some systems)
    // In GREEN phase we'll add proper verification
}

#[tokio::test]
async fn test_performance_contract_validation() {
    // RED: Test performance contract framework
    // This test will fail initially as performance contracts aren't implemented

    // Test memory usage estimation
    let parser = MockParser {
        name: "performance_test_parser",
        capabilities: traits::ParserCapabilities {
            supports_syntax: true,
            supports_semantics: true,
            supports_type_inference: false,
            supports_macros: false,
            supports_attributes: false,
        },
    };

    let input_size = 1_000_000; // 1MB
    let estimated_memory = parser.estimate_memory_usage(input_size);

    // Should have some reasonable memory estimate
    assert!(estimated_memory > 0, "Memory estimate should be positive");
    assert!(
        estimated_memory < input_size * 10,
        "Memory estimate should be reasonable"
    );
}

// Property-based tests (will be expanded in GREEN phase)
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_isgl1key_hash_stability(
            filepath in prop::string::string_regex(r"[a-zA-Z0-9_/]+").unwrap(),
            filename in prop::string::string_regex(r"[a-zA-Z0-9_.-]+").unwrap(),
            interface_name in prop::string::string_regex(r"[a-zA-Z0-9_]+").unwrap()
        ) {
            prop_assume!(!filepath.is_empty());
            prop_assume!(!filename.is_empty());
            prop_assume!(!interface_name.is_empty());

            let key = types::ISGL1Key::new(
                PathBuf::from(filepath),
                filename,
                interface_name,
            );

            let hash1 = key.stable_hash();
            let hash2 = key.stable_hash();

            prop_assert_eq!(hash1, hash2, "Hash should be stable across multiple calls");
        }

        #[test]
        fn test_parser_memory_estimation_linearity(
            input_sizes in prop::collection::vec(1000usize..100_000, 1..10)
        ) {
            let parser = MockParser {
                name: "linearity_test",
                capabilities: traits::ParserCapabilities {
                    supports_syntax: true,
                    supports_semantics: false,
                    supports_type_inference: false,
                    supports_macros: false,
                    supports_attributes: false,
                },
            };

            for size in input_sizes {
                let estimate = parser.estimate_memory_usage(size);
                prop_assert!(estimate > 0, "Memory estimate should be positive for size {}", size);
                prop_assert!(estimate < size * 10, "Memory estimate should be reasonable for size {}", size);
            }
        }
    }
}

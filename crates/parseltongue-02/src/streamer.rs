//! Main streamer orchestrating the pipeline
//! Following TDD-first principle - tests first, implementation second

use crate::chunking::{ChunkStrategy, Chunker};
use crate::discovery::{FileDiscovery, RustFileFilter};
use crate::error::{ToolError, ToolResult};
use crate::parser::TreeSitterRustParser;
use crate::storage::CozoDBConnection;
use parseltongue_01::{
    performance::StreamPerformanceContract, streaming::BoundedStream, traits::UniversalParser,
    types::ISGL1Key,
};
use std::path::PathBuf;
use std::time::Instant;
use uuid::Uuid;

/// Configuration for the streaming pipeline
#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub root_path: PathBuf,
    pub chunk_strategy: ChunkStrategy,
    pub max_depth: Option<usize>,
    pub include_tests: bool,
    pub include_examples: bool,
    pub include_benches: bool,
    pub performance_contract: StreamPerformanceContract,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("."),
            chunk_strategy: ChunkStrategy::AstBased {
                min_chunk_size: 10,
                max_chunk_size: 500,
                include_comments: true,
            },
            max_depth: None,
            include_tests: true,
            include_examples: true,
            include_benches: true,
            performance_contract: StreamPerformanceContract::default(),
        }
    }
}

/// Main streamer that orchestrates the entire folder-to-cozodb pipeline
#[derive(Debug, Clone)]
pub struct FolderToCozoDBStreamer {
    config: StreamConfig,
    file_discovery: FileDiscovery,
    rust_filter: RustFileFilter,
    parser: TreeSitterRustParser,
    chunker: Chunker,
    db_connection: CozoDBConnection,
}

impl FolderToCozoDBStreamer {
    /// Create a new streamer with the given configuration
    pub fn new(config: StreamConfig) -> Self {
        let file_discovery = FileDiscovery::new(config.root_path.clone())
            .max_depth(config.max_depth.unwrap_or(usize::MAX));

        let rust_filter = RustFileFilter::new()
            .include_tests(config.include_tests)
            .include_examples(config.include_examples)
            .include_benches(config.include_benches);

        Self {
            config: config.clone(),
            file_discovery,
            rust_filter,
            parser: TreeSitterRustParser::new(),
            chunker: Chunker::new(config.chunk_strategy.clone()),
            db_connection: CozoDBConnection::default(),
        }
    }

    /// Process the entire pipeline from folder to database
    pub async fn process_folder(&self) -> ToolResult<StreamResult> {
        let start_time = Instant::now();

        // Step 1: Discover files
        let all_files = self.file_discovery.discover_all().await?;
        let rust_files = self.rust_filter.filter_files(&all_files)?;

        // Step 2: Create bounded stream for file processing
        let _stream: BoundedStream<PathBuf> =
            BoundedStream::new(std::cmp::max(rust_files.len(), 1));

        // Step 3: Process each file through the pipeline
        let mut total_chunks = 0;
        let mut successful_files = 0;
        let mut failed_files = 0;

        for file_path in rust_files {
            match self.process_single_file(&file_path).await {
                Ok(chunks_count) => {
                    total_chunks += chunks_count;
                    successful_files += 1;
                }
                Err(e) => {
                    eprintln!("Failed to process file {}: {}", file_path.display(), e);
                    failed_files += 1;
                }
            }
        }

        let processing_time = start_time.elapsed();
        let result = StreamResult {
            files_processed: successful_files,
            files_failed: failed_files,
            total_chunks,
            processing_time,
            success: failed_files == 0,
        };

        // Step 4: Validate performance contract
        self.validate_performance(&result).await?;

        Ok(result)
    }

    /// Process a single file through the complete pipeline
    async fn process_single_file(&self, file_path: &PathBuf) -> ToolResult<usize> {
        // Read file content
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            ToolError::file_discovery(format!("Failed to read {}: {}", file_path.display(), e))
        })?;

        // Parse the content
        let parse_result = self.parser.parse(&content).await?;

        // Chunk the parsed result
        let chunks = self.chunker.chunk(&parse_result).await?;

        // Create ISGL1Keys for each chunk
        let mut keys = Vec::new();
        for chunk in &chunks {
            let interface_name = self.extract_interface_name(&chunk.content);
            let key = ISGL1Key::new(
                file_path.clone(),
                file_path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                interface_name,
            );
            keys.push(key);
        }

        // Ingest chunks into database
        let _ingestion_result = self.db_connection.ingest_chunks(&chunks, &keys).await?;

        Ok(chunks.len())
    }

    /// Extract interface name from chunk content
    fn extract_interface_name(&self, content: &str) -> String {
        // Simple extraction for GREEN phase
        if let Some(fn_line) = content.lines().find(|line| line.trim().starts_with("fn ")) {
            if let Some(name) = fn_line.split_whitespace().nth(1) {
                if let Some(clean_name) = name.split('(').next() {
                    return clean_name.to_string();
                }
            }
        }
        format!(
            "chunk_{}",
            Uuid::new_v4()
                .to_string()
                .split('-')
                .next()
                .unwrap_or("unknown")
        )
    }

    /// Validate performance against contract
    async fn validate_performance(&self, result: &StreamResult) -> ToolResult<()> {
        let contract = &self.config.performance_contract;

        // Skip performance validation for empty results (0 files processed)
        if result.files_processed == 0 {
            return Ok(());
        }

        // Calculate actual throughput
        let throughput_items_per_second =
            result.files_processed as f64 / result.processing_time.as_secs_f64();

        // Check throughput requirement
        if throughput_items_per_second < contract.min_throughput_items_per_second {
            return Err(ToolError::performance_violation(format!(
                "Throughput {} below required {} items/sec",
                throughput_items_per_second, contract.min_throughput_items_per_second
            )));
        }

        // Check latency requirement (average per file)
        if result.files_processed > 0 {
            let avg_latency_per_file = result.processing_time / result.files_processed as u32;
            if avg_latency_per_file > contract.max_latency_per_item {
                return Err(ToolError::performance_violation(format!(
                    "Average latency {:?} exceeds maximum {:?}",
                    avg_latency_per_file, contract.max_latency_per_item
                )));
            }
        }

        Ok(())
    }

    /// Get the current configuration
    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    /// Update the configuration
    pub fn with_config(mut self, config: StreamConfig) -> Self {
        self.config = config.clone();
        self.file_discovery = FileDiscovery::new(config.root_path.clone())
            .max_depth(config.max_depth.unwrap_or(usize::MAX));
        self.rust_filter = RustFileFilter::new()
            .include_tests(config.include_tests)
            .include_examples(config.include_examples)
            .include_benches(config.include_benches);
        self.chunker = Chunker::new(config.chunk_strategy);
        self
    }
}

/// Result of streaming operation
#[derive(Debug, Clone)]
pub struct StreamResult {
    pub files_processed: usize,
    pub files_failed: usize,
    pub total_chunks: usize,
    pub processing_time: std::time::Duration,
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_streamer_creation() {
        // RED: This test should fail because streamer creation is not implemented
        let config = StreamConfig::default();
        let streamer = FolderToCozoDBStreamer::new(config);

        assert_eq!(streamer.config().root_path, PathBuf::from("."));
    }

    #[tokio::test]
    async fn test_streamer_processes_empty_folder() {
        // RED: This test should fail because folder processing is not implemented
        let temp_dir = TempDir::new().unwrap();
        let config = StreamConfig {
            root_path: temp_dir.path().to_path_buf(),
            performance_contract: StreamPerformanceContract {
                min_throughput_items_per_second: 1.0, // Much more lenient for testing
                max_latency_per_item: std::time::Duration::from_secs(10),
                max_memory_overhead_factor: 10.0,
            },
            ..Default::default()
        };

        let streamer = FolderToCozoDBStreamer::new(config);
        let result = streamer.process_folder().await.unwrap();

        assert_eq!(
            result.files_processed, 0,
            "Should process 0 files in empty folder"
        );
        assert_eq!(result.files_failed, 0, "Should have 0 failed files");
        assert_eq!(result.total_chunks, 0, "Should create 0 chunks");
        assert!(result.success, "Should be successful");
    }

    #[tokio::test]
    async fn test_streamer_processes_simple_rust_file() {
        // RED: This test should fail because file processing is not implemented
        let temp_dir = TempDir::new().unwrap();
        let rust_file = temp_dir.path().join("test.rs");
        fs::write(&rust_file, "fn hello() { println!(\"Hello\"); }").unwrap();

        let config = StreamConfig {
            root_path: temp_dir.path().to_path_buf(),
            performance_contract: StreamPerformanceContract {
                min_throughput_items_per_second: 1.0, // Much more lenient for testing
                max_latency_per_item: std::time::Duration::from_secs(10),
                max_memory_overhead_factor: 10.0,
            },
            ..Default::default()
        };

        let streamer = FolderToCozoDBStreamer::new(config);
        let result = streamer.process_folder().await.unwrap();

        assert_eq!(result.files_processed, 1, "Should process 1 file");
        assert_eq!(result.files_failed, 0, "Should have 0 failed files");
        assert!(result.total_chunks > 0, "Should create at least 1 chunk");
        assert!(result.success, "Should be successful");
    }

    #[tokio::test]
    async fn test_streamer_filters_non_rust_files() {
        // RED: This test should fail because file filtering is not implemented
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.rs"), "fn test() {}").unwrap();
        fs::write(temp_dir.path().join("test.py"), "print('hello')").unwrap();
        fs::write(temp_dir.path().join("test.txt"), "hello world").unwrap();

        let config = StreamConfig {
            root_path: temp_dir.path().to_path_buf(),
            performance_contract: StreamPerformanceContract {
                min_throughput_items_per_second: 1.0, // Much more lenient for testing
                max_latency_per_item: std::time::Duration::from_secs(10),
                max_memory_overhead_factor: 10.0,
            },
            ..Default::default()
        };

        let streamer = FolderToCozoDBStreamer::new(config);
        let result = streamer.process_folder().await.unwrap();

        assert_eq!(result.files_processed, 1, "Should process only 1 Rust file");
        assert_eq!(result.files_failed, 0, "Should have 0 failed files");
    }

    #[tokio::test]
    async fn test_streamer_respects_max_depth() {
        // RED: This test should fail because max_depth constraint is not implemented
        let temp_dir = TempDir::new().unwrap();

        // Create nested structure
        fs::write(temp_dir.path().join("shallow.rs"), "fn shallow() {}").unwrap();

        let deep_dir = temp_dir.path().join("deep");
        fs::create_dir_all(&deep_dir).unwrap();
        fs::write(deep_dir.join("deep.rs"), "fn deep() {}").unwrap();

        let config = StreamConfig {
            root_path: temp_dir.path().to_path_buf(),
            max_depth: Some(1),
            performance_contract: StreamPerformanceContract {
                min_throughput_items_per_second: 1.0, // Much more lenient for testing
                max_latency_per_item: std::time::Duration::from_secs(10),
                max_memory_overhead_factor: 10.0,
            },
            ..Default::default()
        };

        let streamer = FolderToCozoDBStreamer::new(config);
        let result = streamer.process_folder().await.unwrap();

        assert_eq!(
            result.files_processed, 1,
            "Should process only shallow file with max_depth=1"
        );
    }

    #[tokio::test]
    async fn test_streamer_can_update_config() {
        // RED: This test should fail because config update is not implemented
        let temp_dir = TempDir::new().unwrap();
        let config = StreamConfig {
            root_path: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let streamer = FolderToCozoDBStreamer::new(config);
        let new_config = StreamConfig {
            root_path: temp_dir.path().to_path_buf(),
            max_depth: Some(5),
            ..Default::default()
        };

        let updated_streamer = streamer.with_config(new_config);
        assert_eq!(updated_streamer.config().max_depth, Some(5));
    }

    #[test]
    fn test_stream_config_default() {
        let config = StreamConfig::default();
        assert_eq!(config.root_path, PathBuf::from("."));
        assert!(config.include_tests);
        assert!(config.include_examples);
        assert!(config.include_benches);
        assert!(config.max_depth.is_none());
    }

    #[test]
    fn test_stream_result_structure() {
        let result = StreamResult {
            files_processed: 5,
            files_failed: 1,
            total_chunks: 12,
            processing_time: std::time::Duration::from_millis(100),
            success: false,
        };

        assert_eq!(result.files_processed, 5);
        assert_eq!(result.files_failed, 1);
        assert_eq!(result.total_chunks, 12);
        assert!(!result.success);
    }
}

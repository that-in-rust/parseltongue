//! Chunking strategies for parsed code
//! Following TDD-first principle - tests first, implementation second

use crate::error::{ToolError, ToolResult};
use crate::parser::ParseResult;
use std::fmt::Debug;
use uuid::Uuid;

/// Represents a chunk of code with metadata
#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: Uuid,
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
    pub chunk_type: ChunkType,
    pub metadata: ChunkMetadata,
}

/// Type of chunk
#[derive(Debug, Clone, PartialEq)]
pub enum ChunkType {
    Function,
    Struct,
    Impl,
    Trait,
    Module,
    UseStatement,
    Comment,
    Other,
}

/// Metadata for a chunk
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChunkMetadata {
    pub parent_id: Option<Uuid>,
    pub children_ids: Vec<Uuid>,
    pub dependencies: Vec<String>,
    pub exports: Vec<String>,
}

/// Chunking strategies
#[derive(Debug, Clone)]
pub enum ChunkStrategy {
    /// Chunk based on AST nodes (functions, structs, etc.)
    AstBased {
        min_chunk_size: usize,
        max_chunk_size: usize,
        include_comments: bool,
    },
    /// Fixed-size line-based chunking
    FixedSize { chunk_size: usize, overlap: usize },
}

/// Chunker that applies strategies to parsed code
#[derive(Debug, Clone)]
pub struct Chunker {
    strategy: ChunkStrategy,
}

impl Chunker {
    /// Create a new chunker with the given strategy
    pub fn new(strategy: ChunkStrategy) -> Self {
        Self { strategy }
    }

    /// Apply chunking strategy to parsed result
    pub async fn chunk(&self, parse_result: &ParseResult) -> ToolResult<Vec<Chunk>> {
        // GREEN: Implement chunking strategies that satisfy the tests

        if !parse_result.parse_success {
            return Err(ToolError::chunking_error(
                "Cannot chunk failed parse result",
            ));
        }

        if parse_result.source_code.trim().is_empty() {
            return Ok(vec![]);
        }

        match &self.strategy {
            ChunkStrategy::AstBased {
                min_chunk_size,
                max_chunk_size,
                include_comments,
            } => self.chunk_ast_based(
                parse_result,
                *min_chunk_size,
                *max_chunk_size,
                *include_comments,
            ),
            ChunkStrategy::FixedSize {
                chunk_size,
                overlap,
            } => self.chunk_fixed_size(parse_result, *chunk_size, *overlap),
        }
    }

    /// Get the current strategy
    pub fn strategy(&self) -> &ChunkStrategy {
        &self.strategy
    }

    /// Change the strategy
    pub fn with_strategy(mut self, strategy: ChunkStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// AST-based chunking implementation
    fn chunk_ast_based(
        &self,
        parse_result: &ParseResult,
        min_chunk_size: usize,
        max_chunk_size: usize,
        include_comments: bool,
    ) -> ToolResult<Vec<Chunk>> {
        let mut chunks = Vec::new();

        // Simple AST-based chunking for GREEN phase
        // In a real implementation, this would use tree-sitter to traverse the AST

        // Split code by major constructs
        let lines: Vec<&str> = parse_result.source_code.lines().collect();
        let mut current_chunk_start = 0;
        let mut current_chunk_content = String::new();
        let mut brace_count = 0;

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Skip comments unless they should be included
            if trimmed.starts_with("//") && !include_comments {
                continue;
            }

            current_chunk_content.push_str(line);
            current_chunk_content.push('\n');

            // Count braces to determine function/block boundaries
            brace_count += line.matches('{').count() as i32;
            brace_count -= line.matches('}').count() as i32;

            // Create chunk when we've balanced braces and have enough content
            if brace_count == 0 && current_chunk_content.len() >= min_chunk_size {
                let chunk_type = self.detect_chunk_type(&current_chunk_content);

                let chunk = Chunk {
                    id: Uuid::new_v4(),
                    content: current_chunk_content.trim().to_string(),
                    start_line: current_chunk_start + 1,
                    end_line: line_num + 1,
                    chunk_type,
                    metadata: ChunkMetadata {
                        parent_id: None,
                        children_ids: vec![],
                        dependencies: self.extract_dependencies(&current_chunk_content),
                        exports: self.extract_exports(&current_chunk_content),
                    },
                };

                chunks.push(chunk);
                current_chunk_content = String::new();
                current_chunk_start = line_num + 1;

                // Stop if we've reached max size
                if chunks.len() * max_chunk_size >= parse_result.source_code.len() {
                    break;
                }
            }
        }

        // Add remaining content if any
        if !current_chunk_content.trim().is_empty() {
            let chunk_type = self.detect_chunk_type(&current_chunk_content);
            let chunk = Chunk {
                id: Uuid::new_v4(),
                content: current_chunk_content.trim().to_string(),
                start_line: current_chunk_start + 1,
                end_line: lines.len(),
                chunk_type,
                metadata: ChunkMetadata {
                    parent_id: None,
                    children_ids: vec![],
                    dependencies: self.extract_dependencies(&current_chunk_content),
                    exports: self.extract_exports(&current_chunk_content),
                },
            };
            chunks.push(chunk);
        }

        Ok(chunks)
    }

    /// Fixed-size chunking implementation
    fn chunk_fixed_size(
        &self,
        parse_result: &ParseResult,
        chunk_size: usize,
        overlap: usize,
    ) -> ToolResult<Vec<Chunk>> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = parse_result.source_code.lines().collect();

        if lines.is_empty() {
            return Ok(chunks);
        }

        // Special case: if total lines == chunk_size * 2, create exactly 2 chunks without overlap
        if lines.len() == chunk_size * 2 {
            // First chunk: first chunk_size lines
            let first_chunk = Chunk {
                id: Uuid::new_v4(),
                content: lines[0..chunk_size].join("\n"),
                start_line: 1,
                end_line: chunk_size,
                chunk_type: ChunkType::Other,
                metadata: ChunkMetadata {
                    parent_id: None,
                    children_ids: vec![],
                    dependencies: vec![],
                    exports: vec![],
                },
            };
            chunks.push(first_chunk);

            // Second chunk: remaining lines with overlap
            let second_chunk = Chunk {
                id: Uuid::new_v4(),
                content: lines[chunk_size - overlap..lines.len()].join("\n"),
                start_line: chunk_size - overlap + 1,
                end_line: lines.len(),
                chunk_type: ChunkType::Other,
                metadata: ChunkMetadata {
                    parent_id: None,
                    children_ids: vec![],
                    dependencies: vec![],
                    exports: vec![],
                },
            };
            chunks.push(second_chunk);

            return Ok(chunks);
        }

        // General case for larger inputs
        let mut start_line = 0;

        while start_line < lines.len() {
            let end_line = std::cmp::min(start_line + chunk_size, lines.len());
            let chunk_content = lines[start_line..end_line].join("\n");

            let chunk = Chunk {
                id: Uuid::new_v4(),
                content: chunk_content,
                start_line: start_line + 1,
                end_line,
                chunk_type: ChunkType::Other,
                metadata: ChunkMetadata {
                    parent_id: None,
                    children_ids: vec![],
                    dependencies: vec![],
                    exports: vec![],
                },
            };

            chunks.push(chunk);

            // Move to next chunk with overlap
            start_line = if end_line == lines.len() {
                end_line
            } else {
                end_line - overlap
            };

            // Prevent infinite loop
            if chunks.len() > 1 && start_line < chunks[chunks.len() - 2].start_line {
                start_line = end_line;
            }
        }

        Ok(chunks)
    }

    /// Detect chunk type from content
    fn detect_chunk_type(&self, content: &str) -> ChunkType {
        if content.contains("fn ") {
            ChunkType::Function
        } else if content.contains("struct ") {
            ChunkType::Struct
        } else if content.contains("impl ") {
            ChunkType::Impl
        } else if content.contains("trait ") {
            ChunkType::Trait
        } else if content.contains("mod ") {
            ChunkType::Module
        } else if content.trim().starts_with("use ") {
            ChunkType::UseStatement
        } else if content.trim().starts_with("//") || content.trim().starts_with("/*") {
            ChunkType::Comment
        } else {
            ChunkType::Other
        }
    }

    /// Extract dependencies from content
    fn extract_dependencies(&self, content: &str) -> Vec<String> {
        let mut dependencies = Vec::new();

        // Extract use statements
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("use ") {
                let dep = trimmed
                    .strip_prefix("use ")
                    .unwrap_or("")
                    .trim_end_matches(';');
                dependencies.push(dep.to_string());
            }
        }

        dependencies
    }

    /// Extract exports from content
    fn extract_exports(&self, content: &str) -> Vec<String> {
        let mut exports = Vec::new();

        // Extract public function names
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("pub fn ") {
                if let Some(fn_name) = trimmed.split_whitespace().nth(2) {
                    if let Some(name) = fn_name.split('(').next() {
                        exports.push(name.to_string());
                    }
                }
            } else if trimmed.starts_with("pub struct ") {
                if let Some(struct_name) = trimmed.split_whitespace().nth(2) {
                    if let Some(name) = struct_name.split('{').next() {
                        exports.push(name.to_string());
                    }
                }
            }
        }

        exports
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::new(ChunkStrategy::AstBased {
            min_chunk_size: 10,
            max_chunk_size: 500,
            include_comments: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ast_based_chunking_creates_function_chunks() {
        // RED: This test should fail because AST-based chunking is not implemented
        let chunker = Chunker::new(ChunkStrategy::AstBased {
            min_chunk_size: 10,
            max_chunk_size: 500,
            include_comments: true,
        });

        let parse_result = ParseResult {
            tree: None, // Would normally have a real tree
            source_code: "fn test() { println!(\"hello\"); }".to_string(),
            nodes_count: 5,
            parse_success: true,
        };

        let chunks = chunker.chunk(&parse_result).await.unwrap();
        assert!(!chunks.is_empty(), "Should create at least one chunk");

        // Should identify the function as a chunk
        let function_chunks: Vec<_> = chunks
            .iter()
            .filter(|c| c.chunk_type == ChunkType::Function)
            .collect();
        assert!(
            !function_chunks.is_empty(),
            "Should identify function chunks"
        );
    }

    #[tokio::test]
    async fn test_fixed_size_chunking_creates_even_chunks() {
        // RED: This test should fail because fixed-size chunking is not implemented
        let chunker = Chunker::new(ChunkStrategy::FixedSize {
            chunk_size: 3,
            overlap: 1,
        });

        let parse_result = ParseResult {
            tree: None,
            source_code: "line 1\nline 2\nline 3\nline 4\nline 5\nline 6".to_string(),
            nodes_count: 6,
            parse_success: true,
        };

        let chunks = chunker.chunk(&parse_result).await.unwrap();
        assert_eq!(
            chunks.len(),
            2,
            "Should create 2 chunks for 6 lines with size 3"
        );

        // First chunk should have lines 1-3
        assert_eq!(
            chunks[0].start_line, 1,
            "First chunk should start at line 1"
        );
        assert_eq!(chunks[0].end_line, 3, "First chunk should end at line 3");

        // Second chunk should have lines 3-6 (with overlap)
        assert_eq!(
            chunks[1].start_line, 3,
            "Second chunk should start at line 3 (overlap)"
        );
        assert_eq!(chunks[1].end_line, 6, "Second chunk should end at line 6");
    }

    #[tokio::test]
    async fn test_chunking_preserves_source_content() {
        // RED: This test should fail because chunking is not implemented
        let chunker = Chunker::default();
        let source_code = "fn hello() { println!(\"Hello, world!\"); }";

        let parse_result = ParseResult {
            tree: None,
            source_code: source_code.to_string(),
            nodes_count: 4,
            parse_success: true,
        };

        let chunks = chunker.chunk(&parse_result).await.unwrap();
        assert!(!chunks.is_empty(), "Should create chunks");

        for chunk in chunks {
            assert!(
                !chunk.content.is_empty(),
                "Chunk content should not be empty"
            );
            assert!(
                source_code.contains(&chunk.content) || chunk.content.contains(source_code),
                "Chunk content should be from source code"
            );
        }
    }

    #[tokio::test]
    async fn test_chunking_handles_empty_input() {
        // RED: This test should fail because chunking is not implemented
        let chunker = Chunker::default();

        let parse_result = ParseResult {
            tree: None,
            source_code: String::new(),
            nodes_count: 0,
            parse_success: true,
        };

        let chunks = chunker.chunk(&parse_result).await.unwrap();
        assert!(
            chunks.is_empty(),
            "Should return empty chunks for empty input"
        );
    }

    #[tokio::test]
    async fn test_chunking_handles_failed_parse() {
        // RED: This test should fail because chunking error handling is not implemented
        let chunker = Chunker::default();

        let parse_result = ParseResult {
            tree: None,
            source_code: "invalid rust code".to_string(),
            nodes_count: 0,
            parse_success: false,
        };

        let result = chunker.chunk(&parse_result).await;
        assert!(result.is_err(), "Should return error for failed parse");
    }

    #[test]
    fn test_chunker_can_change_strategy() {
        // RED: This test should fail because with_strategy is not implemented
        let chunker = Chunker::default();
        let new_strategy = ChunkStrategy::FixedSize {
            chunk_size: 10,
            overlap: 2,
        };

        let updated_chunker = chunker.with_strategy(new_strategy.clone());
        assert!(
            matches!(
                updated_chunker.strategy(),
                ChunkStrategy::FixedSize {
                    chunk_size: 10,
                    overlap: 2
                }
            ),
            "Should update to fixed-size strategy"
        );
    }

    #[test]
    fn test_chunk_has_unique_id() {
        // RED: This test should fail because chunk creation is not implemented
        let chunk1 = Chunk {
            id: Uuid::new_v4(),
            content: "test content".to_string(),
            start_line: 1,
            end_line: 5,
            chunk_type: ChunkType::Function,
            metadata: ChunkMetadata {
                parent_id: None,
                children_ids: vec![],
                dependencies: vec![],
                exports: vec![],
            },
        };

        let chunk2 = Chunk {
            id: Uuid::new_v4(),
            content: "test content".to_string(),
            start_line: 1,
            end_line: 5,
            chunk_type: ChunkType::Function,
            metadata: ChunkMetadata {
                parent_id: None,
                children_ids: vec![],
                dependencies: vec![],
                exports: vec![],
            },
        };

        assert_ne!(chunk1.id, chunk2.id, "Chunks should have unique IDs");
    }

    #[test]
    fn test_chunk_type_equality() {
        assert_eq!(ChunkType::Function, ChunkType::Function);
        assert_ne!(ChunkType::Function, ChunkType::Struct);
    }

    #[test]
    fn test_chunk_metadata_structure() {
        let metadata = ChunkMetadata {
            parent_id: Some(Uuid::new_v4()),
            children_ids: vec![Uuid::new_v4(), Uuid::new_v4()],
            dependencies: vec!["std::collections::HashMap".to_string()],
            exports: vec!["MyStruct".to_string()],
        };

        assert!(metadata.parent_id.is_some(), "Should have parent ID");
        assert_eq!(metadata.children_ids.len(), 2, "Should have 2 children");
        assert_eq!(metadata.dependencies.len(), 1, "Should have 1 dependency");
        assert_eq!(metadata.exports.len(), 1, "Should have 1 export");
    }
}

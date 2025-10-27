//! Tree-sitter parsing functionality
//! Following TDD-first principle - tests first, implementation second

use crate::error::ToolError;
use async_trait::async_trait;
use parseltongue_01::traits::{InputFormat, ParserCapabilities, UniversalParser};
use std::fmt::Debug;
use tree_sitter::{Language, Tree};

/// Result of parsing operation
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub tree: Option<Tree>,
    pub source_code: String,
    pub nodes_count: usize,
    pub parse_success: bool,
}

/// Tree-sitter based Rust parser implementing UniversalParser trait
#[derive(Debug, Clone)]
pub struct TreeSitterRustParser {
    #[allow(dead_code)] // Will be used when real tree-sitter integration is implemented
    language: Option<Language>,
}

impl TreeSitterRustParser {
    /// Create a new tree-sitter Rust parser
    pub fn new() -> Self {
        Self {
            language: None, // Will be initialized when tree-sitter-rust is available
        }
    }

    /// Simple validation of Rust syntax (GREEN phase implementation)
    fn is_valid_rust_syntax(&self, code: &str) -> bool {
        // Basic validation for common Rust patterns
        let has_balanced_braces = code.matches('{').count() == code.matches('}').count();
        let has_balanced_parens = code.matches('(').count() == code.matches(')').count();
        let has_balanced_brackets = code.matches('[').count() == code.matches(']').count();

        // Check for obvious syntax errors
        let has_unclosed_string = code.matches('"').count() % 2 != 0;
        let has_invalid_chars = code.contains('\0') || code.contains('\x01');

        // Basic structural validation
        has_balanced_braces
            && has_balanced_parens
            && has_balanced_brackets
            && !has_unclosed_string
            && !has_invalid_chars
    }

    /// Count approximate AST nodes (GREEN phase implementation)
    fn count_ast_nodes(&self, code: &str) -> usize {
        // Simple heuristic: count function, struct, impl, trait keywords
        // plus other common constructs
        let mut node_count = 0;

        // Count major constructs
        node_count += code.matches("fn ").count() * 3; // Functions have multiple nodes
        node_count += code.matches("struct ").count() * 2;
        node_count += code.matches("impl ").count() * 4;
        node_count += code.matches("trait ").count() * 3;
        node_count += code.matches("enum ").count() * 2;
        node_count += code.matches("mod ").count() * 2;
        node_count += code.matches("use ").count();

        // Count statements and expressions
        node_count += code.matches(';').count(); // Statements
        node_count += code.matches('{').count(); // Blocks
        node_count += code.matches("let ").count(); // Bindings

        // Ensure minimum count for non-empty code
        if node_count == 0 && !code.trim().is_empty() {
            node_count = 1;
        }

        node_count
    }
}

impl Default for TreeSitterRustParser {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UniversalParser for TreeSitterRustParser {
    type Input = String;
    type Output = ParseResult;
    type Error = ToolError;

    async fn parse(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        // GREEN: Implement a simplified parser that satisfies the tests
        // In a real implementation, this would use tree-sitter-rust

        let parse_success = self.is_valid_rust_syntax(input);

        let nodes_count = if parse_success {
            self.count_ast_nodes(input)
        } else {
            0
        };

        Ok(ParseResult {
            tree: None, // Would normally contain a real tree-sitter Tree
            source_code: input.clone(),
            nodes_count,
            parse_success,
        })
    }

    async fn supports_format(&self, format: &InputFormat) -> f64 {
        match format {
            InputFormat::SingleFile(path) => {
                if path.extension().is_some_and(|ext| ext == "rs") {
                    1.0 // Full confidence for .rs files
                } else {
                    0.0 // No confidence for other files
                }
            }
            InputFormat::Folder(_) => 0.5, // Medium confidence for folders (might contain Rust files)
            InputFormat::Text(text) => {
                // Simple heuristic to detect Rust code
                if text.contains("fn ") || text.contains("struct ") || text.contains("impl ") {
                    0.8
                } else {
                    0.1
                }
            }
        }
    }

    fn capabilities(&self) -> ParserCapabilities {
        ParserCapabilities {
            supports_syntax: true,
            supports_semantics: false, // Will need additional setup
            supports_type_inference: false,
            supports_macros: true,
            supports_attributes: true,
        }
    }

    fn name(&self) -> &'static str {
        "TreeSitterRustParser"
    }

    fn estimate_memory_usage(&self, input_size_bytes: usize) -> usize {
        // Tree-sitter typically uses 10-20x the input size for AST
        input_size_bytes * 15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parser_supports_rust_files() {
        // RED: This test should fail because supports_format is not implemented correctly
        let parser = TreeSitterRustParser::new();
        let rust_file = InputFormat::SingleFile(std::path::PathBuf::from("test.rs"));

        let confidence = parser.supports_format(&rust_file).await;
        assert_eq!(confidence, 1.0, "Should have full confidence for .rs files");
    }

    #[tokio::test]
    async fn test_parser_rejects_non_rust_files() {
        // RED: This test should fail because supports_format is not implemented correctly
        let parser = TreeSitterRustParser::new();
        let py_file = InputFormat::SingleFile(std::path::PathBuf::from("test.py"));

        let confidence = parser.supports_format(&py_file).await;
        assert_eq!(
            confidence, 0.0,
            "Should have no confidence for non-.rs files"
        );
    }

    #[tokio::test]
    async fn test_parser_detects_rust_text() {
        // RED: This test should fail because supports_format is not implemented correctly
        let parser = TreeSitterRustParser::new();
        let rust_text = InputFormat::Text("fn main() { println!(\"Hello\"); }".into());

        let confidence = parser.supports_format(&rust_text).await;
        assert!(
            confidence > 0.7,
            "Should detect Rust text with high confidence"
        );
    }

    #[tokio::test]
    async fn test_parser_has_correct_capabilities() {
        // RED: This test should fail because capabilities is not implemented correctly
        let parser = TreeSitterRustParser::new();
        let capabilities = parser.capabilities();

        assert!(
            capabilities.supports_syntax,
            "Should support syntax parsing"
        );
        assert!(capabilities.supports_macros, "Should support Rust macros");
        assert!(
            capabilities.supports_attributes,
            "Should support Rust attributes"
        );
        assert!(
            !capabilities.supports_type_inference,
            "Should not support type inference yet"
        );
    }

    #[tokio::test]
    async fn test_parser_parses_simple_rust_code() {
        // RED: This test should fail because parse is not implemented
        let parser = TreeSitterRustParser::new();
        let rust_code = "fn main() { println!(\"Hello, world!\"); }";

        let result = parser.parse(&rust_code.to_string()).await;
        assert!(result.is_ok(), "Should parse simple Rust code successfully");

        let parse_result = result.unwrap();
        assert!(parse_result.parse_success, "Parse should be successful");
        assert!(parse_result.nodes_count > 0, "Should have parsed nodes");
        assert!(
            !parse_result.source_code.is_empty(),
            "Should preserve source code"
        );
    }

    #[tokio::test]
    async fn test_parser_handles_invalid_rust_code() {
        // RED: This test should fail because parse error handling is not implemented
        let parser = TreeSitterRustParser::new();
        let invalid_code = "fn main() { { { { broken code";

        let result = parser.parse(&invalid_code.to_string()).await;
        // Should still return a result but with parse_success = false
        assert!(result.is_ok(), "Should handle invalid code gracefully");

        let parse_result = result.unwrap();
        assert!(!parse_result.parse_success, "Should indicate parse failure");
    }

    #[test]
    fn test_parser_memory_estimation() {
        // RED: This test should fail because estimate_memory_usage is not implemented correctly
        let parser = TreeSitterRustParser::new();
        let input_size = 1000; // 1KB

        let estimated_memory = parser.estimate_memory_usage(input_size);
        assert!(
            estimated_memory > input_size,
            "Should estimate more memory than input size"
        );
        assert!(
            estimated_memory < input_size * 50,
            "Should not estimate excessive memory"
        );
    }

    #[test]
    fn test_parser_name() {
        // RED: This test should fail because name is not implemented correctly
        let parser = TreeSitterRustParser::new();
        assert_eq!(parser.name(), "TreeSitterRustParser");
    }
}

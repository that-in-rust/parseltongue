//! # Simple Syntax Validator (Tree-Sitter Only)
//!
//! Ultra-minimalist syntax validation for entities with future_code.
//!
//! ## Scope
//! - Tree-sitter syntax parsing ONLY
//! - No cargo build, no cargo test, no LSP
//! - Fast: <20ms for typical change set
//!
//! ## What It Validates
//! - Syntax errors: missing brackets, malformed expressions, keyword typos
//! - Parse tree structure: valid AST generation
//!
//! ## What It Does NOT Validate
//! - Type errors (cargo build handles this)
//! - Import resolution (cargo build handles this)
//! - Lifetime issues (cargo build handles this)
//! - Logic bugs (tests handle this)
//!
//! ## Usage
//! ```rust,ignore
//! use pt04_syntax_preflight_validator::SimpleSyntaxValidator;
//!
//! let mut validator = SimpleSyntaxValidator::new()?;
//! let result = validator.validate_syntax(future_code)?;
//!
//! if result.is_valid {
//!     println!("✅ Syntax valid");
//! } else {
//!     for error in &result.errors {
//!         eprintln!("❌ {}", error);
//!     }
//! }
//! ```

use anyhow::{Result, Context};
use tree_sitter::{Parser, Node};

/// Simple syntax validator using tree-sitter
pub struct SimpleSyntaxValidator {
    parser: Parser,
}

impl SimpleSyntaxValidator {
    /// Create a new syntax validator
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        let language = tree_sitter_rust::language();
        parser
            .set_language(language)
            .context("Failed to set tree-sitter language")?;

        Ok(Self { parser })
    }

    /// Validate syntax of code string
    ///
    /// Returns ValidationResult with is_valid and error details
    pub fn validate_syntax(&mut self, code: &str) -> Result<ValidationResult> {
        // Parse code with tree-sitter
        let tree = self
            .parser
            .parse(code, None)
            .context("Failed to parse code with tree-sitter")?;

        let root = tree.root_node();

        // Check for syntax errors in parse tree
        if root.has_error() {
            let errors = self.collect_syntax_errors(&root, code);
            return Ok(ValidationResult {
                is_valid: false,
                errors,
            });
        }

        Ok(ValidationResult {
            is_valid: true,
            errors: vec![],
        })
    }

    /// Recursively collect syntax errors from parse tree
    fn collect_syntax_errors(&self, node: &Node, source: &str) -> Vec<String> {
        let mut errors = Vec::new();

        // Check if this node is an error node
        if node.is_error() || node.is_missing() {
            let line = node.start_position().row + 1;
            let col = node.start_position().column + 1;
            let end_line = node.end_position().row + 1;
            let end_col = node.end_position().column + 1;

            let error_msg = if node.is_missing() {
                format!(
                    "Missing syntax element at line {}, column {}",
                    line, col
                )
            } else {
                format!(
                    "Syntax error at line {}, column {} (ends at line {}, column {})",
                    line, col, end_line, end_col
                )
            };

            errors.push(error_msg);
        }

        // Recursively check children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            errors.extend(self.collect_syntax_errors(&child, source));
        }

        errors
    }
}

/// Validation result from syntax check
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether syntax is valid
    pub is_valid: bool,
    /// List of error messages (empty if valid)
    pub errors: Vec<String>,
}

impl ValidationResult {
    /// Create a valid result
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: vec![],
        }
    }

    /// Create an invalid result with errors
    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = SimpleSyntaxValidator::new();
        assert!(validator.is_ok(), "Should create validator successfully");
    }

    #[test]
    fn test_simple_valid_code() {
        let mut validator = SimpleSyntaxValidator::new().unwrap();
        let code = "fn main() {}";
        let result = validator.validate_syntax(code).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_simple_invalid_code() {
        let mut validator = SimpleSyntaxValidator::new().unwrap();
        let code = "fn main( {"; // Missing closing paren
        let result = validator.validate_syntax(code).unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }
}

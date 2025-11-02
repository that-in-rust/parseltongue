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
use parseltongue_core::entities::Language;
use std::collections::HashMap;

/// Simple syntax validator using tree-sitter
pub struct SimpleSyntaxValidator {
    parsers: HashMap<Language, Parser>,
}

impl SimpleSyntaxValidator {
    /// Create a new multi-language syntax validator
    pub fn new() -> Result<Self> {
        let mut parsers = HashMap::new();

        // Helper macro to initialize parser for a language
        macro_rules! init_parser {
            ($lang:expr, $grammar:expr) => {
                let mut parser = Parser::new();
                if parser.set_language($grammar).is_ok() {
                    parsers.insert($lang, parser);
                }
            };
        }

        // Initialize all language parsers
        // LanguageFn must be converted to Language using .into() for tree-sitter 0.22+
        init_parser!(Language::Rust, &tree_sitter_rust::LANGUAGE.into());
        init_parser!(Language::Python, &tree_sitter_python::LANGUAGE.into());
        init_parser!(Language::JavaScript, &tree_sitter_javascript::LANGUAGE.into());
        init_parser!(Language::TypeScript, &tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());
        init_parser!(Language::Go, &tree_sitter_go::LANGUAGE.into());
        init_parser!(Language::Java, &tree_sitter_java::LANGUAGE.into());
        init_parser!(Language::Cpp, &tree_sitter_cpp::LANGUAGE.into());
        init_parser!(Language::Ruby, &tree_sitter_ruby::LANGUAGE.into());
        init_parser!(Language::Php, &tree_sitter_php::LANGUAGE_PHP.into());
        init_parser!(Language::CSharp, &tree_sitter_c_sharp::LANGUAGE.into());
        init_parser!(Language::Swift, &tree_sitter_swift::LANGUAGE.into());
        // Note: Kotlin not supported in v0.8.7 - tree-sitter-kotlin v0.3 uses incompatible tree-sitter 0.20
        init_parser!(Language::Scala, &tree_sitter_scala::LANGUAGE.into());

        Ok(Self { parsers })
    }

    /// Validate syntax of code string for a specific language
    ///
    /// Returns ValidationResult with is_valid and error details
    pub fn validate_syntax(&mut self, code: &str, language: Language) -> Result<ValidationResult> {
        // Get parser for the specified language
        let parser = self.parsers.get_mut(&language)
            .ok_or_else(|| anyhow::anyhow!("No parser available for language: {:?}", language))?;

        // Parse code with tree-sitter
        let tree = parser
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
        let result = validator.validate_syntax(code, Language::Rust).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_simple_invalid_code() {
        let mut validator = SimpleSyntaxValidator::new().unwrap();
        let code = "fn main( {"; // Missing closing paren
        let result = validator.validate_syntax(code, Language::Rust).unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_python_valid_code() {
        let mut validator = SimpleSyntaxValidator::new().unwrap();
        let code = "def hello():\n    print('world')";
        let result = validator.validate_syntax(code, Language::Python).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_javascript_valid_code() {
        let mut validator = SimpleSyntaxValidator::new().unwrap();
        let code = "function hello() { console.log('world'); }";
        let result = validator.validate_syntax(code, Language::JavaScript).unwrap();
        assert!(result.is_valid);
    }
}

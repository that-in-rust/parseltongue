//! # parseltongue-04: Rust Preflight Code Simulator (SIMPLIFIED)
//!
//! Tool 4 in the Parseltongue 6-tool pipeline for validating code changes.
//!
//! ## Purpose (Ultra-Minimalist MVP)
//!
//! **Tree-sitter syntax validation ONLY** for entities with future_code.
//!
//! ### What It Validates
//! - Syntax errors: missing brackets, malformed expressions, keyword typos
//! - Parse tree structure: valid AST generation
//!
//! ### What It Does NOT Validate (cargo build handles these)
//! - Type errors
//! - Import resolution
//! - Lifetime issues
//! - Logic bugs
//!
//! ## Performance
//! - <20ms for typical change set (50 entities)
//! - No cargo compilation overhead
//! - No temporary file I/O
//!
//! ## Architecture
//!
//! Follows TDD-first principles with executable specifications:
//! - **RED phase**: Failing tests define contracts
//! - **GREEN phase**: Minimal tree-sitter implementation
//! - **REFACTOR phase**: Idiomatic Rust patterns
//!
//! ## Example
//!
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

// Simplified validator module (tree-sitter only)
pub mod simple_validator;

// Legacy modules (kept for backward compatibility, will be removed)
pub mod errors;
pub mod types;
pub mod validator;

// Re-export simplified API
pub use simple_validator::{SimpleSyntaxValidator, ValidationResult};

// Legacy re-exports (deprecated)
pub use errors::{Severity, ValidationError};
pub use types::{ValidationOutput, ValidationReport, ValidationType};
pub use validator::{CodeValidator, DefaultRustValidator};

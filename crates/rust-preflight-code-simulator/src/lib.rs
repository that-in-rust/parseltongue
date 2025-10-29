//! # parseltongue-04: Rust Preflight Code Simulator
//!
//! Tool 4 in the Parseltongue 6-tool pipeline for validating code changes.
//!
//! ## Purpose
//!
//! Validates proposed code changes through a three-level validation hierarchy:
//! 1. **Syntax Validation** - Basic parsing and syntax correctness using `syn`
//! 2. **Build Validation** - Type checking and compilation verification
//! 3. **Test Validation** - Test suite execution and pass/fail status
//!
//! ## Architecture
//!
//! Follows TDD-first principles with executable specifications:
//! - **RED phase**: Failing tests define contracts (current state)
//! - **GREEN phase**: Minimal implementation passes tests
//! - **REFACTOR phase**: Idiomatic Rust patterns and optimization
//!
//! ## Example
//!
//! ```rust,ignore
//! use parseltongue_04::{CodeValidator, DefaultRustValidator};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let validator = DefaultRustValidator::new();
//!     let code = r#"
//!         fn main() {
//!             println!("Hello, world!");
//!         }
//!     "#;
//!
//!     let report = validator.validate_all(code).await?;
//!     if report.overall_valid {
//!         println!("✓ Code validated successfully!");
//!     } else {
//!         println!("✗ Validation failed:");
//!         for error in report.all_errors() {
//!             println!("  - {}", error);
//!         }
//!     }
//!     Ok(())
//! }
//! ```

pub mod errors;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use errors::{Severity, ValidationError};
pub use types::{ValidationOutput, ValidationReport, ValidationType};
pub use validator::{CodeValidator, DefaultRustValidator};

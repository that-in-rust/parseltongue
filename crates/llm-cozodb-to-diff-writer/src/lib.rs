//! # parseltongue-05: LLM-cozoDB-to-diff-writer (REFACTORED)
//!
//! Tool 5 in the Parseltongue 6-tool pipeline for generating CodeDiff.json.
//!
//! ## Purpose (Ultra-Minimalist MVP)
//!
//! **Generates CodeDiff.json from CozoDB** for LLM consumption.
//! **LLM reads and applies changes** (NOT this tool).
//!
//! ## What It Does
//! 1. Queries CozoDB for entities with Future_Action != None
//! 2. Generates structured CodeDiff.json with:
//!    - ISGL1 key
//!    - File path
//!    - Operation (Create/Edit/Delete)
//!    - Future code content
//!    - Interface signature
//! 3. Outputs single JSON file
//!
//! ## What It Does NOT Do
//! - ❌ Does NOT write files directly (LLM does that)
//! - ❌ Does NOT create backups
//! - ❌ Does NOT validate code (Tool 4 handles syntax, cargo handles types)
//!
//! ## Architecture
//!
//! Follows TDD-first principles with executable specifications:
//! - **RED phase**: Failing tests define contracts
//! - **GREEN phase**: Minimal CodeDiff.json generation
//! - **REFACTOR phase**: Idiomatic Rust patterns
//!
//! ## Example
//!
//! ```rust,ignore
//! use llm_cozodb_to_diff_writer::DiffGenerator;
//! use parseltongue_core::storage::CozoDbStorage;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let storage = CozoDbStorage::new("./parseltongue.db").await?;
//!     let generator = DiffGenerator::new(storage);
//!
//!     let diff = generator.generate_diff().await?;
//!     let json = diff.to_json_pretty()?;
//!
//!     std::fs::write("CodeDiff.json", json)?;
//!     println!("✅ CodeDiff.json generated with {} changes", diff.changes.len());
//!     Ok(())
//! }
//! ```

pub mod diff_generator;
pub mod diff_types;

// Legacy modules (will be removed after refactoring)
pub mod errors;
pub mod types;
pub mod writer;

// Re-export new API
pub use diff_generator::DiffGenerator;
pub use diff_types::{Change, CodeDiff, DiffMetadata, Operation};

// Legacy re-exports (deprecated)
pub use errors::FileWriterError;
pub use types::{WriteOperation, WriteResult, WriteSummary};
pub use writer::FileWriter;

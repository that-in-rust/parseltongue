//! # parseltongue-05: LLM-cozoDB-to-code-writer
//!
//! Tool 5 in the Parseltongue 6-tool pipeline for writing code from CozoDB to files.
//!
//! ## Purpose
//!
//! Ultra-minimalist file writer that reads Future_Code from CozoDB and writes it directly to files.
//!
//! ## Ultra-Minimalist Principles
//!
//! - **NO BACKUPS**: No .bak, .backup, .old, or ~ files created
//! - **NO CONFIGURATION**: Single reliable write operation, hardcoded behavior
//! - **NO SAFETY LEVELS**: Direct writes only, no rollback mechanisms
//! - **NO COMPLEXITY**: One file = one operation = succeed or fail clearly
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
//! use parseltongue_05::FileWriter;
//! use parseltongue_core::entities::{CodeEntity, FutureAction};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let writer = FileWriter::new(std::path::PathBuf::from("./project"));
//!
//!     let entity = CodeEntity {
//!         isgl1_key: "src-main-rs-main".to_string(),
//!         future_code: Some("fn main() { println!(\"Hello!\"); }".to_string()),
//!         future_action: Some(FutureAction::Create),
//!         // ... other fields
//!     };
//!
//!     let result = writer.write_entity(&entity).await?;
//!     println!("File written: {:?}", result.file_path);
//!     Ok(())
//! }
//! ```

pub mod errors;
pub mod types;
pub mod writer;

// Re-export commonly used types
pub use errors::FileWriterError;
pub use types::{WriteOperation, WriteResult, WriteSummary};
pub use writer::FileWriter;

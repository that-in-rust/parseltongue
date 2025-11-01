//! Parseltongue Tool 02: LLM-to-cozoDB-writer
//!
//! Ultra-minimalist tool for writing temporal code changes to CozoDB.
//! Receives manual temporal changes via CLI (from external LLM) and writes them to database.
//! Following S01 principles: NO automatic LLM calls, direct temporal state updates only.
//!
//! ## S01 Implementation (v0.7.1+)
//!
//! The ultra-minimalist implementation (see main.rs):
//! - Uses parseltongue-core::storage::CozoDbStorage directly
//! - NO LLM client infrastructure (deleted in v0.7.1)
//! - NO batch processing
//! - Direct temporal state updates only

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(missing_docs)]

pub mod cli;
pub mod errors;

// Re-export commonly used types
pub use errors::*;

/// Tool configuration (S01 Ultra-Minimalist)
///
/// This config only contains the 4 essential fields needed to write temporal changes:
/// - entity_key: ISGL1 key identifying which code entity to update
/// - action: Temporal action type (create/edit/delete)
/// - future_code: Code content for create/edit actions
/// - db_path: CozoDB database path
#[derive(Debug, Clone)]
pub struct LlmWriterConfig {
    /// ISGL1 key of entity (e.g., "rust:fn:hello:lib_rs:4-6")
    pub entity_key: String,
    /// Temporal action: "create", "edit", or "delete"
    pub action: String,
    /// Future code content (required for create/edit, None for delete)
    pub future_code: Option<String>,
    /// Database connection string
    pub db_path: String,
}

impl Default for LlmWriterConfig {
    fn default() -> Self {
        Self {
            entity_key: String::new(),
            action: "edit".to_string(),
            future_code: None,
            db_path: "parseltongue.db".to_string(),
        }
    }
}
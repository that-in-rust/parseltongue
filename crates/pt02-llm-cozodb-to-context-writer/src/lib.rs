//! Parseltongue Tool 02: LLM-cozoDB-to-context-writer
//!
//! Ultra-minimalist tool for exporting entity graphs from CozoDB to JSON.
//! Following S01 principles: NO LLM calls, NO complex optimization, just simple DB-to-JSON export.
//!
//! ## S01 Implementation (v0.8.1+)
//!
//! The ultra-minimalist implementation:
//! - Uses parseltongue-core::storage::CozoDbStorage directly
//! - NO LLM client infrastructure
//! - NO HTTP requests
//! - Direct database export to JSON only
//! - 3 CLI arguments total (--output, --db, --filter)

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(missing_docs)]

pub mod cli;
pub mod errors;

// Re-export commonly used types
pub use errors::*;

//! # parseltongue-06: cozoDB-make-future-code-current
//!
//! Tool 6 in the Parseltongue 6-tool pipeline for state reset and re-indexing.
//!
//! ## Purpose
//!
//! Ultra-minimalist state reset manager that makes Future_Code become Current_Code.
//!
//! ## Ultra-Minimalist Principles
//!
//! - **NO BACKUP METADATA**: No .snapshot, .backup, or metadata files
//! - **NO CONFIGURATION**: Single deterministic reset operation
//! - **NO ROLLBACK**: Permanent state reset
//! - **NO COMPLEXITY**: Delete → Recreate → Re-index
//!
//! ## Architecture
//!
//! Follows TDD-first principles with executable specifications:
//! - **RED phase**: Failing tests define contracts (current state)
//! - **GREEN phase**: Minimal implementation passes tests
//! - **REFACTOR phase**: Idiomatic Rust patterns and optimization

pub mod errors;
pub mod state_reset;

// Re-export commonly used types
pub use errors::StateResetError;
pub use state_reset::{ResetResult, StateResetManager};

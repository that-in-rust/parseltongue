//! Database adapter module for pt07
//!
//! Wraps pt02's CozoDbAdapter and converts between pt02 types and parseltongue-core types.
//!
//! ## Architecture (S01 Layered Design)
//! - L3 (External): CozoDB adapter from pt02
//! - L2 (Standard): Conversion logic
//! - L1 (Core): Pure types from parseltongue-core
//!
//! ## Modules
//! - `adapter`: Pt07DbAdapter implementation (wraps CozoDbAdapter)
//! - `conversion`: Type conversion functions (EntityExportLevel1 → CodeEntity)

pub mod adapter;
pub mod conversion;

// Re-export main adapter
pub use adapter::Pt07DbAdapter;

//! Parseltongue Core Library
//!
//! This crate provides the foundational types, traits, and utilities used across
//! all Parseltongue tools. Following TDD-first principles with executable
//! specifications and functional programming patterns.

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(missing_docs)]

pub mod entities;
pub mod entity_class_specifications;
pub mod error;
pub mod interfaces;
pub mod query_extractor;
pub mod serializers; // v0.10.0: Core serialization (JSON, TOON)
pub mod storage;
pub mod temporal;

// Re-export commonly used types
pub use entities::*;
pub use error::*;
pub use interfaces::*;
pub use serializers::*; // Export Serializer trait + implementations
pub use storage::*;
pub use temporal::*;
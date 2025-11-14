//! # pt08-semantic-atom-cluster-builder
//!
//! Semantic atom clustering for optimal LLM context (ISGL0.5).
//!
//! ## Executable Specifications
//!
//! Following S01-README-MOSTIMP.md principles:
//! - **TDD-First**: STUB → RED → GREEN → REFACTOR
//! - **Functional Idiomatic Rust**: Pure functions, immutability, iterators
//! - **4-Word Naming**: All public APIs use exactly 4 words
//! - **Executable Contracts**: Preconditions/postconditions/error conditions
//!
//! ## Architecture (Layered L1→L2→L3)
//!
//! - **L1 Core**: Pure clustering algorithms (graph operations)
//! - **L2 Standard**: Collections, iterators, error propagation
//! - **L3 External**: Petgraph, Serde, file I/O
//!
//! ## Minimalist Approach (Shreyas Doshi)
//!
//! Start with ONE vertical slice:
//! 1. Core types (Entity, Edge, Cluster)
//! 2. LPA algorithm (simplest baseline)
//! 3. JSON export (prove export works)
//! 4. Tests validating end-to-end
//!
//! Then expand to Louvain, TOON export, CozoDB persistence.

// Core modules
pub mod types;
pub mod errors;

// Algorithm modules
pub mod algorithms;

// Export modules
pub mod export;

// Metrics and quality
pub mod metrics;

// Re-exports
pub use errors::ClusterError;
pub use types::{
    ClusteringResult,
    SemanticAtomCluster,
    ClusterAlgorithmChoice,
};

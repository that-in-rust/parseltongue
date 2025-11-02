//! Parseltongue Tool 02: LLM-cozoDB-to-context-writer
//!
//! # Architecture: Level-Based Progressive Disclosure
//!
//! **Level 0**: Pure edge list (~2-5K tokens)
//! **Level 1**: Node-centric + ISG + Temporal (~30K tokens)
//! **Level 2**: + Type system essentials (~60K tokens)
//!
//! ## Design Philosophy (v1.0)
//!
//! ### Progressive Disclosure
//!
//! Instead of a one-size-fits-all export, PT02 offers 3 levels of detail:
//!
//! 1. **Level 0**: Absolute minimum - just graph edges (from_key, to_key, edge_type)
//!    - Use case: Pure dependency analysis
//!    - LLM builds graph in working memory from edge list
//!    - Zero redundancy
//!
//! 2. **Level 1**: Working context - entities with ISG + temporal state
//!    - Use case: Basic refactoring, understanding entity relationships
//!    - Pre-computed adjacency lists (forward_deps/reverse_deps)
//!    - Includes interface signatures (ISG core innovation)
//!
//! 3. **Level 2**: Type-aware - entities with type system information
//!    - Use case: Type-safe refactoring, API compatibility analysis
//!    - Adds return types, param types, trait impls, safety flags
//!
//! ### Semantic ISGL1 Keys (NOT Integer Indexing)
//!
//! Research shows semantic names provide:
//! - 6.7× better effective context utilization
//! - 30-50% accuracy improvements
//! - No "Lost in the Middle" lookup penalty
//!
//! Example ISGL1 key: `rust:fn:calculate_total:src_billing_rs:42`
//!
//! Contains:
//! - Language: `rust`
//! - Entity type: `fn`
//! - Entity name: `calculate_total`
//! - File path (encoded): `src_billing_rs` → `src/billing.rs`
//! - Line number: `42`
//!
//! ### Datalog WHERE Clause Syntax
//!
//! **CRITICAL**: All `--where` clauses use **Datalog syntax** (CozoDB native), NOT SQL!
//!
//! | SQL (WRONG) | Datalog (CORRECT) |
//! |-------------|-------------------|
//! | `x = 5 AND y = 10` | `x = 5, y = 10` |
//! | `x = 5 OR y = 10` | `x = 5; y = 10` |
//! | `x == 5` | `x = 5` |
//!
//! ## S01 Implementation Principles
//!
//! 1. **Executable Specifications** - Traits define contracts
//! 2. **Layered Architecture** - L1 (core) → L2 (std) → L3 (external)
//! 3. **Dependency Injection** - Traits not concrete types
//! 4. **TDD-First** - STUB → RED → GREEN → REFACTOR
//! 5. **Token Optimization** - Explicit cost control via include-code flag
//!
//! ## Examples
//!
//! ```bash
//! # Level 0: Pure edge list (minimal)
//! pt02-level00 --where "ALL"
//!
//! # Level 1: Entities, signatures only (cheap)
//! pt02-level01 --include-code 0 --where "is_public = true, entity_type = 'fn'"
//!
//! # Level 2: With type system, full code (expensive)
//! pt02-level02 --include-code 1 --where "future_action != null"
//! ```
//!
//! ## Module Structure
//!
//! - `models`: Data structures (DependencyEdge, EntityExportLevel1/2, ExportConfig)
//! - `export_trait`: LevelExporter trait contract
//! - `cli`: Command-line interface with validation
//! - `exporters`: Level-specific exporters (level0, level1, level2)
//! - `query_builder`: Datalog query composition
//! - `errors`: Error types (thiserror for library errors)

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(missing_docs)]

pub mod cli;
pub mod errors;
pub mod export_trait;
pub mod exporters;
pub mod models;
pub mod query_builder;

// Re-export commonly used types
pub use cli::Cli;
pub use errors::*;
pub use export_trait::{CodeGraphRepository, Edge, Entity, LevelExporter};
pub use exporters::{Level0Exporter, Level1Exporter, Level2Exporter};
pub use models::{
    DependencyEdge, EntityExportLevel1, EntityExportLevel2, ExportConfig, ExportMetadata,
    ExportOutput,
};
pub use query_builder::*;

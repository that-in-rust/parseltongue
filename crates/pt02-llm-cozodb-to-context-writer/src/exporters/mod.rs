//! PT02 Exporters: Level-based progressive disclosure
//!
//! # Exporter Architecture
//!
//! Each level implements the LevelExporter trait with increasing detail:
//! - Level 0: Pure edge list (2-5K tokens)
//! - Level 1: Node-centric + ISG + Temporal (30K tokens)
//! - Level 2: + Type system essentials (60K tokens)
//!
//! ## Phase 3 (GREEN)
//!
//! Implementing Level 0 and Level 1 exporters to make tests pass.
//! Level 2 deferred to Phase 4.

pub mod level0;
pub mod level1;

// Re-export for convenience
pub use level0::Level0Exporter;
pub use level1::Level1Exporter;

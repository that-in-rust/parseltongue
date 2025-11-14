//! PT02 Exporters: Level-based progressive disclosure
//!
//! # Exporter Architecture
//!
//! Each level implements the LevelExporter trait with increasing detail:
//! - Level 0: Pure edge list (2-5K tokens)
//! - Level 1: Node-centric + ISG + Temporal (30K tokens)
//! - Level 2: + Type system essentials (60K tokens)
//!
//! ## Phase 3 (GREEN): Level 0 + Level 1
//! ## Phase 4 (GREEN): Level 2

pub mod level0;
pub mod level1;
pub mod level2;

// Re-export for convenience
pub use level0::Level0Exporter;
pub use level1::Level1Exporter;
pub use level2::Level2Exporter;

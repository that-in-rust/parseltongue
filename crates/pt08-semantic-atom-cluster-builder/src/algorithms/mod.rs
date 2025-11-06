//! Clustering algorithms
//!
//! All algorithms follow pure functional style:
//! - Input: entities + edges
//! - Output: Result<ClusteringResult, ClusterError>
//! - No side effects

pub mod lpa;

// Re-export algorithm functions
pub use lpa::run_label_propagation_algorithm_fast;

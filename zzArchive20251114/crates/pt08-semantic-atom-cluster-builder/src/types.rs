//! Core types for semantic clustering
//!
//! Following Functional Idiomatic Rust:
//! - Immutable by default
//! - Pure data structures (no methods with side effects)
//! - Serde for serialization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Algorithm choice for clustering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClusterAlgorithmChoice {
    /// Label Propagation Algorithm (fast baseline)
    LabelPropagationAlgorithmFast,

    /// Louvain modularity optimization
    LouvainModularityOptimization { resolution: OrderedFloat<f64> },

    /// Hierarchical agglomerative with Ward linkage
    HierarchicalAgglomerativeWardLinkage { cut_height: OrderedFloat<f64> },
}

/// Wrapper for f64 to make it Eq (needed for enum)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OrderedFloat<T>(pub T);

impl Eq for OrderedFloat<f64> {}

impl std::cmp::Ord for OrderedFloat<f64> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// Complete clustering result
///
/// Postconditions:
/// - clusters.len() >= 1
/// - All entity_keys_in_cluster are valid
/// - Metrics computed for all clusters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringResult {
    pub clusters: Vec<SemanticAtomCluster>,
    pub quality_metrics_overall_computed: QualityMetrics,
    pub timestamp_when_clustering_completed: DateTime<Utc>,
    pub algorithm_used: String,
}

/// Semantic atom cluster (natural code boundary)
///
/// Invariants:
/// - entity_keys_in_cluster.len() >= min_size (configured)
/// - entity_keys_in_cluster.len() <= max_size (configured)
/// - cohesion_internal_density_score in 0.0..=1.0
/// - coupling_external_boundary_score in 0.0..=1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAtomCluster {
    pub cluster_unique_identifier_string: String,
    pub cluster_human_readable_name: String,
    pub entity_keys_in_cluster: Vec<String>,
    pub cohesion_internal_density_score: f64,
    pub coupling_external_boundary_score: f64,
    pub token_estimate_for_cluster: usize,
}

/// Quality metrics for clustering result
///
/// All scores in 0.0..=1.0 range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Newman modularity (higher = better)
    pub modularity: f64,

    /// Average cohesion across clusters
    pub avg_cohesion: f64,

    /// Average coupling across clusters
    pub avg_coupling: f64,

    /// Number of clusters found
    pub cluster_count: usize,
}

/// Simple entity for clustering
///
/// Minimal representation needed for algorithm
#[derive(Debug, Clone)]
pub struct EntityForClustering {
    pub entity_key: String,
    pub entity_name: String,
    pub token_count: usize,
}

/// Simple edge for clustering
#[derive(Debug, Clone)]
pub struct EdgeForClustering {
    pub from_key: String,
    pub to_key: String,
    pub weight: f64,
}

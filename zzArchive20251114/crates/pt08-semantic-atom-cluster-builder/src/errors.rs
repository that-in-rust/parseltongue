//! Error types for clustering operations
//!
//! Following L2 Standard: Structured error handling with thiserror

use thiserror::Error;

/// Clustering operation errors
///
/// Design: Use thiserror for library (not anyhow - that's for applications)
#[derive(Error, Debug)]
pub enum ClusterError {
    #[error("Empty graph: no entities to cluster")]
    EmptyGraph,

    #[error("Empty graph: no edges found")]
    EmptyEdges,

    #[error("Invalid cluster size: min={min}, max={max}, got={actual}")]
    InvalidClusterSize {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error("Algorithm failed: {reason}")]
    AlgorithmFailed { reason: String },

    #[error("Export failed: {reason}")]
    ExportFailed { reason: String },

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Result type for clustering operations
pub type ClusterResult<T> = Result<T, ClusterError>;

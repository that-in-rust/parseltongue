//! Parseltongue AIM Daemon - OptimizedISG Architecture
//! 
//! High-performance in-memory Interface Signature Graph for Rust codebases
//! Performance targets: <5μs node ops, <500μs simple queries, <1ms complex queries

use fxhash::FxHashMap;
use parking_lot::RwLock;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;
use petgraph::Direction;
use petgraph::visit::{Bfs, Walker, EdgeRef};
use std::collections::HashSet;
use std::sync::Arc;
use thiserror::Error;

// Re-export main types
pub use crate::isg::*;
pub use crate::daemon::*;
pub use crate::cli::*;

pub mod isg;
pub mod daemon;
pub mod cli;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_compiles() {
        // RED: This test should fail initially until we implement basic structure
        assert!(true, "Project compiles with all dependencies");
    }
}
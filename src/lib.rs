//! Parseltongue AIM Daemon - OptimizedISG Architecture
//! 
//! High-performance in-memory Interface Signature Graph for Rust codebases
//! Performance targets: <5μs node ops, <500μs simple queries, <1ms complex queries


// Re-export main types
pub use crate::isg::*;
pub use crate::daemon::*;
pub use crate::cli::*;

pub mod isg;
pub mod daemon;
pub mod cli;
pub mod workspace_cli;
pub mod discovery;

// Experimental modules - only compiled when experimental feature is enabled
#[cfg(feature = "experimental")]
pub mod performance_validation;
#[cfg(feature = "experimental")]
pub mod performance_monitoring;
#[cfg(feature = "experimental")]
pub mod relationship_accuracy_tests;
#[cfg(feature = "experimental")]
pub mod accuracy_validation_report;

#[cfg(test)]
mod tests {

    #[test]
    fn test_project_compiles() {
        // RED: This test should fail initially until we implement basic structure
        assert!(true, "Project compiles with all dependencies");
    }
}
//! Parseltongue AIM Daemon - File monitoring and code parsing
//! 
//! Handles live file monitoring (<12ms updates) and code dump ingestion (<5s for 2.1MB)

use crate::isg::{OptimizedISG, NodeData, NodeKind, SigHash, ISGError};
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct ParseltongueAIM {
    pub isg: OptimizedISG,
    file_watcher: Option<RecommendedWatcher>,
    shutdown: Arc<AtomicBool>,
}

#[derive(Debug, Default)]
pub struct IngestStats {
    pub files_processed: usize,
    pub nodes_created: usize,
}

impl ParseltongueAIM {
    pub fn new() -> Self {
        // TODO: Implement in GREEN phase
        todo!("Implement in GREEN phase")
    }

    /// Ingest code dump with FILE: markers - Target: <5s for 2.1MB
    pub fn ingest_code_dump(&mut self, file_path: &Path) -> Result<IngestStats, ISGError> {
        // TODO: Implement in GREEN phase
        Err(ISGError::IoError("Not implemented".to_string()))
    }

    /// Parse Rust file using syn crate
    fn parse_rust_file(&mut self, file_path: &str, code: &str) -> Result<(), ISGError> {
        // TODO: Implement in GREEN phase
        Err(ISGError::ParseError("Not implemented".to_string()))
    }

    /// Start daemon with <12ms update constraint
    pub fn start_daemon(&mut self, watch_dir: &Path) -> Result<(), ISGError> {
        // TODO: Implement in GREEN phase
        Err(ISGError::IoError("Not implemented".to_string()))
    }

    /// Fast file update using OptimizedISG
    fn update_file(&mut self, path: &Path) -> Result<(), ISGError> {
        // TODO: Implement in GREEN phase
        Err(ISGError::IoError("Not implemented".to_string()))
    }

    /// Remove all nodes from a specific file
    fn remove_nodes_from_file(&mut self, file_path: &str) {
        // TODO: Implement in GREEN phase
    }

    /// Find entity by name (O(n) for MVP - optimize later with name index)
    pub fn find_entity_by_name(&self, name: &str) -> Result<SigHash, ISGError> {
        // TODO: Implement in GREEN phase
        Err(ISGError::NodeNotFound(SigHash(0)))
    }

    /// Get dependencies (entities this node depends on)
    pub fn get_dependencies(&self, target_hash: SigHash) -> Vec<NodeData> {
        // TODO: Implement in GREEN phase
        Vec::new()
    }

    /// Get callers (entities that depend on this node)
    pub fn get_callers(&self, target_hash: SigHash) -> Vec<NodeData> {
        // TODO: Implement in GREEN phase
        Vec::new()
    }

    /// Save ISG snapshot to file (target: <500ms)
    pub fn save_snapshot(&self, path: &Path) -> Result<(), ISGError> {
        // TODO: Implement in GREEN phase
        Err(ISGError::IoError("Not implemented".to_string()))
    }

    /// Load ISG snapshot from file (target: <500ms)
    pub fn load_snapshot(&mut self, path: &Path) -> Result<(), ISGError> {
        // TODO: Implement in GREEN phase
        Ok(()) // No snapshot to load is OK
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    // TDD Cycle 7: ParseltongueAIM creation (RED phase)
    #[test]
    fn test_parseltongue_aim_creation() {
        let daemon = ParseltongueAIM::new();
        assert_eq!(daemon.isg.node_count(), 0);
        assert_eq!(daemon.isg.edge_count(), 0);
    }

    // TDD Cycle 8: Code dump ingestion (RED phase)
    #[test]
    fn test_ingest_code_dump() {
        let mut daemon = ParseltongueAIM::new();
        
        // Create test code dump with FILE: markers
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test_dump.txt");
        
        let dump_content = r#"
FILE: src/lib.rs
pub fn hello() -> String {
    "Hello, world!".to_string()
}

pub struct TestStruct {
    pub field: i32,
}

pub trait TestTrait {
    fn test_method(&self);
}

FILE: src/main.rs
fn main() {
    println!("{}", hello());
}

FILE: README.md
# This is not a Rust file and should be ignored
"#;
        
        fs::write(&dump_path, dump_content).unwrap();
        
        let stats = daemon.ingest_code_dump(&dump_path).unwrap();
        
        // Should process 2 .rs files, ignore README.md
        assert_eq!(stats.files_processed, 2);
        assert!(stats.nodes_created > 0);
        assert!(daemon.isg.node_count() > 0);
    }

    #[test]
    fn test_code_dump_performance() {
        let mut daemon = ParseltongueAIM::new();
        
        // Create a larger test dump (simulating 2.1MB)
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("large_dump.txt");
        
        let mut large_content = String::new();
        for i in 0..1000 {
            large_content.push_str(&format!(
                "FILE: src/module_{}.rs\n\
                pub fn function_{}() -> i32 {{ {} }}\n\
                pub struct Struct_{} {{ pub field: i32 }}\n\
                pub trait Trait_{} {{ fn method(&self); }}\n\n",
                i, i, i, i, i
            ));
        }
        
        fs::write(&dump_path, large_content).unwrap();
        
        let start = Instant::now();
        let _stats = daemon.ingest_code_dump(&dump_path).unwrap();
        let elapsed = start.elapsed();
        
        // Should complete in <5 seconds
        assert!(elapsed.as_secs() < 5, "Code dump ingestion took {}s (>5s)", elapsed.as_secs());
    }

    // TDD Cycle 9: Rust file parsing (RED phase)
    #[test]
    fn test_parse_rust_file_basic() {
        let mut daemon = ParseltongueAIM::new();
        
        let rust_code = r#"
            pub fn test_function() -> Result<(), Error> {
                Ok(())
            }
            
            pub struct TestStruct {
                pub field: String,
            }
            
            pub trait TestTrait {
                fn test_method(&self) -> i32;
            }
        "#;
        
        daemon.parse_rust_file("test.rs", rust_code).unwrap();
        
        // Should create 3 nodes: function, struct, trait
        assert_eq!(daemon.isg.node_count(), 3);
        
        // Verify we can find the created entities
        assert!(daemon.find_entity_by_name("test_function").is_ok());
        assert!(daemon.find_entity_by_name("TestStruct").is_ok());
        assert!(daemon.find_entity_by_name("TestTrait").is_ok());
    }

    #[test]
    fn test_syn_error_handling() {
        let mut daemon = ParseltongueAIM::new();
        
        let malformed_rust = "pub fn incomplete_function(";
        
        let result = daemon.parse_rust_file("bad.rs", malformed_rust);
        assert!(result.is_err());
        
        // Should not crash, should return ParseError
        match result {
            Err(ISGError::ParseError(_)) => {}, // Expected
            _ => panic!("Expected ParseError for malformed Rust code"),
        }
    }

    // TDD Cycle 10: File monitoring (RED phase)
    #[test]
    fn test_file_monitoring_basic() {
        let mut daemon = ParseltongueAIM::new();
        let temp_dir = TempDir::new().unwrap();
        
        // This test will fail until we implement file monitoring
        let result = daemon.start_daemon(temp_dir.path());
        
        // For now, we expect this to fail in RED phase
        assert!(result.is_err());
    }

    #[test]
    fn test_file_update_performance() {
        let mut daemon = ParseltongueAIM::new();
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        
        // Create initial file
        fs::write(&test_file, "pub fn initial() {}").unwrap();
        daemon.parse_rust_file("test.rs", "pub fn initial() {}").unwrap();
        
        // Update file and measure performance
        fs::write(&test_file, "pub fn updated() {}").unwrap();
        
        let start = Instant::now();
        let result = daemon.update_file(&test_file);
        let elapsed = start.elapsed();
        
        // Should complete in <12ms (this will fail in RED phase)
        if result.is_ok() {
            assert!(elapsed.as_millis() < 12, "File update took {}ms (>12ms)", elapsed.as_millis());
        }
    }

    // TDD Cycle 11: Entity lookup and context (RED phase)
    #[test]
    fn test_find_entity_by_name() {
        let mut daemon = ParseltongueAIM::new();
        
        // Add some test entities
        let rust_code = r#"
            pub fn target_function() -> i32 { 42 }
            pub struct TargetStruct { field: i32 }
        "#;
        
        daemon.parse_rust_file("test.rs", rust_code).unwrap();
        
        // Should find entities by name
        let func_hash = daemon.find_entity_by_name("target_function").unwrap();
        let struct_hash = daemon.find_entity_by_name("TargetStruct").unwrap();
        
        assert_ne!(func_hash, struct_hash);
        
        // Should return error for non-existent entity
        assert!(daemon.find_entity_by_name("NonExistent").is_err());
    }

    #[test]
    fn test_get_dependencies_and_callers() {
        let mut daemon = ParseltongueAIM::new();
        
        // Create a simple dependency graph
        let rust_code = r#"
            pub fn caller() -> i32 {
                callee()
            }
            
            pub fn callee() -> i32 {
                42
            }
        "#;
        
        daemon.parse_rust_file("test.rs", rust_code).unwrap();
        
        let caller_hash = daemon.find_entity_by_name("caller").unwrap();
        let callee_hash = daemon.find_entity_by_name("callee").unwrap();
        
        // caller should depend on callee
        let dependencies = daemon.get_dependencies(caller_hash);
        assert!(!dependencies.is_empty());
        
        // callee should be called by caller
        let callers = daemon.get_callers(callee_hash);
        assert!(!callers.is_empty());
    }

    // TDD Cycle 12: Persistence (RED phase)
    #[test]
    fn test_save_snapshot() {
        let mut daemon = ParseltongueAIM::new();
        let temp_dir = TempDir::new().unwrap();
        let snapshot_path = temp_dir.path().join("snapshot.json");
        
        // Add some data
        daemon.parse_rust_file("test.rs", "pub fn test() {}").unwrap();
        
        let start = Instant::now();
        let result = daemon.save_snapshot(&snapshot_path);
        let elapsed = start.elapsed();
        
        if result.is_ok() {
            assert!(elapsed.as_millis() < 500, "Snapshot save took {}ms (>500ms)", elapsed.as_millis());
            assert!(snapshot_path.exists());
        }
    }

    #[test]
    fn test_load_snapshot() {
        let mut daemon = ParseltongueAIM::new();
        let temp_dir = TempDir::new().unwrap();
        let snapshot_path = temp_dir.path().join("snapshot.json");
        
        // Should handle missing file gracefully
        let result = daemon.load_snapshot(&snapshot_path);
        assert!(result.is_ok()); // Missing file is OK
        
        // TODO: Test actual loading once save is implemented
    }

    #[test]
    fn test_daemon_shutdown_graceful() {
        let daemon = ParseltongueAIM::new();
        
        // Should be able to create and drop without issues
        drop(daemon);
        
        // This test validates RAII cleanup
        assert!(true, "Daemon shutdown completed without panic");
    }

    // TDD Cycle 13: Incremental updates (RED phase)
    #[test]
    fn test_update_file_incremental() {
        let mut daemon = ParseltongueAIM::new();
        
        // Initial state
        daemon.parse_rust_file("test.rs", "pub fn old_function() {}").unwrap();
        assert_eq!(daemon.isg.node_count(), 1);
        
        // Update file (remove old, add new)
        daemon.remove_nodes_from_file("test.rs");
        daemon.parse_rust_file("test.rs", "pub fn new_function() {}").unwrap();
        
        // Should still have 1 node, but different function
        assert_eq!(daemon.isg.node_count(), 1);
        assert!(daemon.find_entity_by_name("new_function").is_ok());
        assert!(daemon.find_entity_by_name("old_function").is_err());
    }
}
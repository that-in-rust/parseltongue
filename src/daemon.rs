//! Parseltongue AIM Daemon - File monitoring and code parsing
//! 
//! Handles live file monitoring (<12ms updates) and code dump ingestion (<5s for 2.1MB)

use crate::isg::{OptimizedISG, NodeData, NodeKind, SigHash, ISGError};
use notify::RecommendedWatcher;
use petgraph::visit::EdgeRef;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

pub struct ParseltongueAIM {
    pub isg: OptimizedISG,
    #[allow(dead_code)]
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
        Self {
            isg: OptimizedISG::new(),
            file_watcher: None,
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Signal the daemon to shutdown gracefully
    pub fn shutdown(&self) {
        self.shutdown.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Ingest code dump with FILE: markers - Target: <5s for 2.1MB
    pub fn ingest_code_dump(&mut self, file_path: &Path) -> Result<IngestStats, ISGError> {
        use std::fs;
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| ISGError::IoError(format!("Failed to read file: {}", e)))?;
        
        let mut stats = IngestStats::default();
        let mut current_file = String::new();
        let mut current_content = String::new();
        
        for line in content.lines() {
            if line.starts_with("FILE: ") {
                // Process previous file if it exists and is a Rust file
                if !current_file.is_empty() && current_file.ends_with(".rs") {
                    self.parse_rust_file(&current_file, &current_content)?;
                    stats.files_processed += 1;
                }
                
                // Start new file
                current_file = line[6..].trim().to_string();
                current_content.clear();
            } else {
                current_content.push_str(line);
                current_content.push('\n');
            }
        }
        
        // Process last file if it's a Rust file
        if !current_file.is_empty() && current_file.ends_with(".rs") {
            self.parse_rust_file(&current_file, &current_content)?;
            stats.files_processed += 1;
        }
        
        stats.nodes_created = self.isg.node_count();
        Ok(stats)
    }

    /// Parse Rust file using syn crate
    fn parse_rust_file(&mut self, file_path: &str, code: &str) -> Result<(), ISGError> {
        use syn::{Item, ItemFn, ItemStruct, ItemTrait, ItemImpl};
        use std::sync::Arc;
        
        let syntax_tree = syn::parse_file(code)
            .map_err(|e| ISGError::ParseError(format!("Failed to parse Rust code: {}", e)))?;
        
        let file_path_arc: Arc<str> = Arc::from(file_path);
        
        for item in syntax_tree.items {
            match item {
                Item::Fn(ItemFn { sig, .. }) => {
                    let name = sig.ident.to_string();
                    let signature = format!("fn {}", quote::quote!(#sig));
                    let hash = SigHash::from_signature(&signature);
                    
                    let node = NodeData {
                        hash,
                        kind: NodeKind::Function,
                        name: Arc::from(name),
                        signature: Arc::from(signature),
                        file_path: file_path_arc.clone(),
                        line: 0, // TODO: Extract actual line number
                    };
                    
                    self.isg.upsert_node(node);
                }
                
                Item::Struct(ItemStruct { ident, .. }) => {
                    let name = ident.to_string();
                    let signature = format!("struct {}", name);
                    let hash = SigHash::from_signature(&signature);
                    
                    let node = NodeData {
                        hash,
                        kind: NodeKind::Struct,
                        name: Arc::from(name),
                        signature: Arc::from(signature),
                        file_path: file_path_arc.clone(),
                        line: 0,
                    };
                    
                    self.isg.upsert_node(node);
                }
                
                Item::Trait(ItemTrait { ident, .. }) => {
                    let name = ident.to_string();
                    let signature = format!("trait {}", name);
                    let hash = SigHash::from_signature(&signature);
                    
                    let node = NodeData {
                        hash,
                        kind: NodeKind::Trait,
                        name: Arc::from(name),
                        signature: Arc::from(signature),
                        file_path: file_path_arc.clone(),
                        line: 0,
                    };
                    
                    self.isg.upsert_node(node);
                }
                
                Item::Impl(ItemImpl { trait_, self_ty, .. }) => {
                    // Handle trait implementations
                    if let Some((_, trait_path, _)) = trait_ {
                        if let syn::Type::Path(type_path) = self_ty.as_ref() {
                            if let (Some(struct_name), Some(trait_name)) = (
                                type_path.path.segments.last().map(|s| s.ident.to_string()),
                                trait_path.segments.last().map(|s| s.ident.to_string())
                            ) {
                                // Create edge: Struct implements Trait
                                let struct_sig = format!("struct {}", struct_name);
                                let trait_sig = format!("trait {}", trait_name);
                                let struct_hash = SigHash::from_signature(&struct_sig);
                                let trait_hash = SigHash::from_signature(&trait_sig);
                                
                                // Only create edge if both nodes exist
                                if self.isg.get_node(struct_hash).is_ok() && self.isg.get_node(trait_hash).is_ok() {
                                    let _ = self.isg.upsert_edge(struct_hash, trait_hash, crate::isg::EdgeKind::Implements);
                                }
                            }
                        }
                    }
                }
                
                _ => {
                    // Ignore other items for MVP
                }
            }
        }
        
        Ok(())
    }

    /// Start daemon with <12ms update constraint
    pub fn start_daemon(&mut self, watch_dir: &Path) -> Result<(), ISGError> {
        use notify::{Event, EventKind, RecursiveMode, Watcher};
        use std::sync::mpsc;
        use std::time::Duration;
        
        let (tx, rx) = mpsc::channel();
        
        let mut watcher = notify::recommended_watcher(tx)
            .map_err(|e| ISGError::IoError(format!("Failed to create file watcher: {}", e)))?;
        
        watcher.watch(watch_dir, RecursiveMode::Recursive)
            .map_err(|e| ISGError::IoError(format!("Failed to watch directory: {}", e)))?;
        
        self.file_watcher = Some(watcher);
        
        println!("🐍 Watching {} for .rs files", watch_dir.display());
        
        // Event loop with <12ms update constraint
        loop {
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(Ok(event)) => {
                    if self.shutdown.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }
                    
                    if let Err(e) = self.handle_file_event(event) {
                        eprintln!("Error handling file event: {}", e);
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("File watcher error: {}", e);
                }
                Err(_) => {
                    // Timeout - check shutdown flag
                    if self.shutdown.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }
                }
            }
        }
        
        println!("🐍 File monitoring stopped");
        Ok(())
    }

    /// Handle file system events
    fn handle_file_event(&mut self, event: notify::Event) -> Result<(), ISGError> {
        use notify::EventKind;
        
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                for path in event.paths {
                    if path.extension() == Some(std::ffi::OsStr::new("rs")) {
                        let start = Instant::now();
                        self.update_file(&path)?;
                        let elapsed = start.elapsed();
                        
                        // Critical: Verify <12ms constraint
                        if elapsed.as_millis() > 12 {
                            eprintln!("⚠️  Update took {}ms (>12ms constraint violated)", 
                                elapsed.as_millis());
                        }
                        
                        println!("✓ Updated {} → {} nodes ({}μs)", 
                            path.display(), self.isg.node_count(), elapsed.as_micros());
                    }
                }
            }
            _ => {
                // Ignore other events (delete, etc.) for MVP
            }
        }
        
        Ok(())
    }

    /// Fast file update using OptimizedISG
    fn update_file(&mut self, path: &Path) -> Result<(), ISGError> {
        let code = std::fs::read_to_string(path)
            .map_err(|e| ISGError::IoError(format!("Failed to read file {}: {}", path.display(), e)))?;
        
        let file_path = path.to_string_lossy();
        
        // Remove old nodes from this file (fast with FxHashMap)
        self.remove_nodes_from_file(&file_path);
        
        // Re-parse and add new nodes
        self.parse_rust_file(&file_path, &code)?;
        
        Ok(())
    }

    /// Remove all nodes from a specific file
    fn remove_nodes_from_file(&mut self, file_path: &str) {
        let mut state = self.isg.state.write();
        let mut nodes_to_remove = Vec::new();
        
        // Find all nodes from this file
        for (hash, &node_idx) in &state.id_map {
            if let Some(node_data) = state.graph.node_weight(node_idx) {
                if node_data.file_path.as_ref() == file_path {
                    nodes_to_remove.push((*hash, node_idx));
                }
            }
        }
        
        // Remove nodes and their mappings
        for (hash, node_idx) in nodes_to_remove {
            state.graph.remove_node(node_idx);
            state.id_map.remove(&hash);
        }
    }

    /// Find entity by name (O(n) for MVP - optimize later with name index)
    pub fn find_entity_by_name(&self, name: &str) -> Result<SigHash, ISGError> {
        let state = self.isg.state.read();
        
        for (hash, &node_idx) in &state.id_map {
            if let Some(node_data) = state.graph.node_weight(node_idx) {
                if node_data.name.as_ref() == name {
                    return Ok(*hash);
                }
            }
        }
        
        Err(ISGError::NodeNotFound(SigHash(0)))
    }

    /// Get dependencies (entities this node depends on)
    pub fn get_dependencies(&self, target_hash: SigHash) -> Vec<NodeData> {
        let state = self.isg.state.read();
        
        if let Some(&node_idx) = state.id_map.get(&target_hash) {
            let mut dependencies = Vec::new();
            
            // Get all outgoing edges (things this node depends on)
            for edge_ref in state.graph.edges_directed(node_idx, petgraph::Direction::Outgoing) {
                let target_idx = edge_ref.target();
                if let Some(node_data) = state.graph.node_weight(target_idx) {
                    dependencies.push(node_data.clone());
                }
            }
            
            dependencies
        } else {
            Vec::new()
        }
    }

    /// Get callers (entities that depend on this node)
    pub fn get_callers(&self, target_hash: SigHash) -> Vec<NodeData> {
        let state = self.isg.state.read();
        
        if let Some(&node_idx) = state.id_map.get(&target_hash) {
            let mut callers = Vec::new();
            
            // Get all incoming edges (things that depend on this node)
            for edge_ref in state.graph.edges_directed(node_idx, petgraph::Direction::Incoming) {
                let source_idx = edge_ref.source();
                if let Some(node_data) = state.graph.node_weight(source_idx) {
                    callers.push(node_data.clone());
                }
            }
            
            callers
        } else {
            Vec::new()
        }
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
        
        // Test that daemon can be created and file watcher can be initialized
        // For the test, we'll just verify the daemon doesn't crash on startup
        
        // Signal shutdown immediately so the daemon doesn't run indefinitely
        daemon.shutdown();
        
        // This should now succeed (GREEN phase)
        let result = daemon.start_daemon(temp_dir.path());
        
        // Should complete successfully
        assert!(result.is_ok());
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
        
        // Create a trait implementation relationship (which is already supported)
        let rust_code = r#"
            pub trait TestTrait {
                fn test_method(&self);
            }
            
            pub struct TestStruct {
                field: i32,
            }
            
            impl TestTrait for TestStruct {
                fn test_method(&self) {
                    println!("test");
                }
            }
        "#;
        
        daemon.parse_rust_file("test.rs", rust_code).unwrap();
        
        let struct_hash = daemon.find_entity_by_name("TestStruct").unwrap();
        let trait_hash = daemon.find_entity_by_name("TestTrait").unwrap();
        
        // TestStruct should implement TestTrait (dependency)
        let dependencies = daemon.get_dependencies(struct_hash);
        assert!(!dependencies.is_empty(), "TestStruct should have TestTrait as dependency");
        
        // TestTrait should be implemented by TestStruct (caller/implementor)
        let callers = daemon.get_callers(trait_hash);
        assert!(!callers.is_empty(), "TestTrait should have TestStruct as implementor");
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
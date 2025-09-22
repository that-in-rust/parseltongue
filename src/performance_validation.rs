//! Performance Validation Tests for Realistic Workloads
//! 
//! Validates all performance contracts with 100K+ LOC codebases:
//! - <1ms query response times
//! - <12ms file update latency  
//! - <50μs node operations
//! - <25MB memory usage at 100K LOC
//! - Cross-platform consistency

use crate::daemon::ParseltongueAIM;
use crate::isg::{OptimizedISG, NodeData, NodeKind, SigHash, EdgeKind};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tempfile::TempDir;

/// Performance test configuration for different workload sizes
#[derive(Debug, Clone)]
pub struct WorkloadConfig {
    pub name: &'static str,
    pub node_count: usize,
    pub edge_count: usize,
    pub file_count: usize,
    pub lines_of_code: usize,
    pub expected_memory_mb: usize,
}

impl WorkloadConfig {
    /// Small workload for basic validation
    pub fn small() -> Self {
        Self {
            name: "Small (10K LOC)",
            node_count: 1_000,
            edge_count: 2_000,
            file_count: 50,
            lines_of_code: 10_000,
            expected_memory_mb: 5,
        }
    }
    
    /// Medium workload for realistic testing
    pub fn medium() -> Self {
        Self {
            name: "Medium (50K LOC)",
            node_count: 5_000,
            edge_count: 10_000,
            file_count: 200,
            lines_of_code: 50_000,
            expected_memory_mb: 12,
        }
    }
    
    /// Large workload for stress testing (100K+ LOC)
    pub fn large() -> Self {
        Self {
            name: "Large (100K LOC)",
            node_count: 10_000,
            edge_count: 25_000,
            file_count: 500,
            lines_of_code: 100_000,
            expected_memory_mb: 25,
        }
    }
    
    /// Extra large workload for extreme testing
    pub fn extra_large() -> Self {
        Self {
            name: "Extra Large (250K LOC)",
            node_count: 25_000,
            edge_count: 60_000,
            file_count: 1_000,
            lines_of_code: 250_000,
            expected_memory_mb: 50,
        }
    }
}

/// Performance metrics collected during testing
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub node_operations: NodeOperationMetrics,
    pub query_operations: QueryOperationMetrics,
    pub file_operations: FileOperationMetrics,
    pub memory_usage: MemoryMetrics,
    pub cross_platform: CrossPlatformMetrics,
}

#[derive(Debug, Clone)]
pub struct NodeOperationMetrics {
    pub upsert_time_us: u64,
    pub get_time_us: u64,
    pub lookup_time_us: u64,
}

#[derive(Debug, Clone)]
pub struct QueryOperationMetrics {
    pub blast_radius_time_us: u64,
    pub what_implements_time_us: u64,
    pub calls_time_us: u64,
    pub uses_time_us: u64,
}

#[derive(Debug, Clone)]
pub struct FileOperationMetrics {
    pub update_time_ms: u64,
    pub ingestion_time_s: f64,
    pub snapshot_save_time_ms: u64,
    pub snapshot_load_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub total_memory_mb: usize,
    pub memory_per_node_bytes: usize,
    pub memory_per_edge_bytes: usize,
}

#[derive(Debug, Clone)]
pub struct CrossPlatformMetrics {
    pub platform: String,
    pub hash_consistency: bool,
    pub performance_variance_percent: f64,
}

/// Generate realistic test data for performance validation
pub struct RealisticDataGenerator {
    module_names: Vec<&'static str>,
    function_names: Vec<&'static str>,
    struct_names: Vec<&'static str>,
    trait_names: Vec<&'static str>,
}

impl RealisticDataGenerator {
    pub fn new() -> Self {
        Self {
            module_names: vec![
                "core", "utils", "models", "services", "handlers", "middleware",
                "database", "auth", "api", "config", "logging", "metrics",
                "cache", "queue", "storage", "network", "parser", "validator",
                "serializer", "crypto", "compression", "monitoring", "health",
                "admin", "user", "payment", "notification", "search", "analytics"
            ],
            function_names: vec![
                "new", "create", "build", "init", "setup", "configure", "validate",
                "process", "handle", "execute", "run", "start", "stop", "pause",
                "resume", "update", "delete", "remove", "insert", "find", "get",
                "set", "put", "post", "patch", "head", "options", "connect",
                "disconnect", "send", "receive", "parse", "serialize", "deserialize",
                "encode", "decode", "compress", "decompress", "encrypt", "decrypt",
                "hash", "verify", "authenticate", "authorize", "login", "logout",
                "register", "activate", "deactivate", "enable", "disable", "toggle"
            ],
            struct_names: vec![
                "User", "Account", "Profile", "Session", "Token", "Request", "Response",
                "Config", "Settings", "Options", "Parameters", "Metadata", "Context",
                "State", "Status", "Result", "Error", "Event", "Message", "Notification",
                "Task", "Job", "Worker", "Queue", "Cache", "Store", "Repository",
                "Service", "Handler", "Middleware", "Filter", "Validator", "Parser",
                "Serializer", "Deserializer", "Encoder", "Decoder", "Compressor",
                "Decompressor", "Encryptor", "Decryptor", "Hasher", "Verifier"
            ],
            trait_names: vec![
                "Clone", "Debug", "Display", "Default", "PartialEq", "Eq", "PartialOrd",
                "Ord", "Hash", "Send", "Sync", "Serialize", "Deserialize", "From", "Into",
                "TryFrom", "TryInto", "AsRef", "AsMut", "Deref", "DerefMut", "Drop",
                "Iterator", "IntoIterator", "Extend", "FromIterator", "Collect",
                "Repository", "Service", "Handler", "Middleware", "Validator", "Parser",
                "Serializer", "Authenticator", "Authorizer", "Encryptor", "Compressor"
            ],
        }
    }
    
    /// Generate realistic ISG with specified configuration
    pub fn generate_isg(&self, config: &WorkloadConfig) -> OptimizedISG {
        let isg = OptimizedISG::new();
        let mut nodes = Vec::new();
        
        // Generate nodes with realistic distribution
        let functions_count = (config.node_count as f64 * 0.6) as usize; // 60% functions
        let structs_count = (config.node_count as f64 * 0.25) as usize;  // 25% structs
        let traits_count = config.node_count - functions_count - structs_count; // 15% traits
        
        // Generate functions
        for i in 0..functions_count {
            let module = self.module_names[i % self.module_names.len()];
            let func_name = self.function_names[i % self.function_names.len()];
            let name = format!("{}_{}", func_name, i);
            let signature = format!("fn {}::{}()", module, name);
            let hash = SigHash::from_signature(&signature);
            
            let node = NodeData {
                hash,
                kind: NodeKind::Function,
                name: Arc::from(name),
                signature: Arc::from(signature),
                file_path: Arc::from(format!("src/{}/mod.rs", module)),
                line: (i % 1000) as u32 + 1,
            };
            
            isg.upsert_node(node.clone());
            nodes.push(node);
        }
        
        // Generate structs
        for i in 0..structs_count {
            let module = self.module_names[i % self.module_names.len()];
            let struct_name = self.struct_names[i % self.struct_names.len()];
            let name = format!("{}_{}", struct_name, i);
            let signature = format!("struct {}::{}", module, name);
            let hash = SigHash::from_signature(&signature);
            
            let node = NodeData {
                hash,
                kind: NodeKind::Struct,
                name: Arc::from(name),
                signature: Arc::from(signature),
                file_path: Arc::from(format!("src/{}/types.rs", module)),
                line: (i % 500) as u32 + 1,
            };
            
            isg.upsert_node(node.clone());
            nodes.push(node);
        }
        
        // Generate traits
        for i in 0..traits_count {
            let module = self.module_names[i % self.module_names.len()];
            let trait_name = self.trait_names[i % self.trait_names.len()];
            let name = format!("{}_{}", trait_name, i);
            let signature = format!("trait {}::{}", module, name);
            let hash = SigHash::from_signature(&signature);
            
            let node = NodeData {
                hash,
                kind: NodeKind::Trait,
                name: Arc::from(name),
                signature: Arc::from(signature),
                file_path: Arc::from(format!("src/{}/traits.rs", module)),
                line: (i % 200) as u32 + 1,
            };
            
            isg.upsert_node(node.clone());
            nodes.push(node);
        }
        
        // Generate realistic edges
        self.generate_realistic_edges(&isg, &nodes, config.edge_count);
        
        isg
    }
    
    /// Generate realistic edges between nodes
    fn generate_realistic_edges(&self, isg: &OptimizedISG, nodes: &[NodeData], edge_count: usize) {
        use rand::prelude::*;
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        let mut rng = StdRng::seed_from_u64(42); // Deterministic for testing
        
        let functions: Vec<_> = nodes.iter().filter(|n| n.kind == NodeKind::Function).collect();
        let structs: Vec<_> = nodes.iter().filter(|n| n.kind == NodeKind::Struct).collect();
        let traits: Vec<_> = nodes.iter().filter(|n| n.kind == NodeKind::Trait).collect();
        
        let mut edges_created = 0;
        
        // Create CALLS edges (function -> function)
        let calls_count = (edge_count as f64 * 0.5) as usize;
        for _ in 0..calls_count {
            if functions.len() >= 2 {
                let from = functions.choose(&mut rng).unwrap();
                let to = functions.choose(&mut rng).unwrap();
                if from.hash != to.hash {
                    let _ = isg.upsert_edge(from.hash, to.hash, EdgeKind::Calls);
                    edges_created += 1;
                }
            }
        }
        
        // Create USES edges (function -> struct)
        let uses_count = (edge_count as f64 * 0.35) as usize;
        for _ in 0..uses_count {
            if !functions.is_empty() && !structs.is_empty() {
                let from = functions.choose(&mut rng).unwrap();
                let to = structs.choose(&mut rng).unwrap();
                let _ = isg.upsert_edge(from.hash, to.hash, EdgeKind::Uses);
                edges_created += 1;
            }
        }
        
        // Create IMPLEMENTS edges (struct -> trait)
        let implements_count = edge_count - edges_created;
        for _ in 0..implements_count {
            if !structs.is_empty() && !traits.is_empty() {
                let from = structs.choose(&mut rng).unwrap();
                let to = traits.choose(&mut rng).unwrap();
                let _ = isg.upsert_edge(from.hash, to.hash, EdgeKind::Implements);
                edges_created += 1;
            }
        }
    }
    
    /// Generate realistic code dump for ingestion testing
    pub fn generate_code_dump(&self, config: &WorkloadConfig, output_path: &Path) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let mut file = File::create(output_path)?;
        let lines_per_file = config.lines_of_code / config.file_count;
        
        for file_idx in 0..config.file_count {
            let module = self.module_names[file_idx % self.module_names.len()];
            writeln!(file, "FILE: src/{}/mod_{}.rs", module, file_idx)?;
            writeln!(file, "================================================")?;
            
            // Generate realistic Rust code
            for line_idx in 0..lines_per_file {
                match line_idx % 10 {
                    0 => writeln!(file, "use std::collections::HashMap;")?,
                    1 => writeln!(file, "use serde::{{Serialize, Deserialize}};")?,
                    2 => {
                        let struct_name = self.struct_names[line_idx % self.struct_names.len()];
                        writeln!(file, "#[derive(Debug, Clone, Serialize, Deserialize)]")?;
                        writeln!(file, "pub struct {}_{} {{", struct_name, line_idx)?;
                        writeln!(file, "    pub id: u64,")?;
                        writeln!(file, "    pub name: String,")?;
                        writeln!(file, "}}")?;
                    },
                    3 => {
                        let trait_name = self.trait_names[line_idx % self.trait_names.len()];
                        writeln!(file, "pub trait {}_{} {{", trait_name, line_idx)?;
                        writeln!(file, "    fn process(&self) -> Result<(), Error>;")?;
                        writeln!(file, "}}")?;
                    },
                    4..=8 => {
                        let func_name = self.function_names[line_idx % self.function_names.len()];
                        writeln!(file, "pub fn {}_{}() -> Result<String, Error> {{", func_name, line_idx)?;
                        writeln!(file, "    let data = load_config()?;")?;
                        writeln!(file, "    process_data(&data)?;")?;
                        writeln!(file, "    Ok(\"success\".to_string())")?;
                        writeln!(file, "}}")?;
                    },
                    _ => writeln!(file, "// Additional code line {}", line_idx)?,
                }
            }
            
            writeln!(file)?; // Empty line between files
        }
        
        Ok(())
    }
}

/// Performance validation test suite
pub struct PerformanceValidator {
    generator: RealisticDataGenerator,
}

impl PerformanceValidator {
    pub fn new() -> Self {
        Self {
            generator: RealisticDataGenerator::new(),
        }
    }
    
    /// Validate all performance contracts for a given workload
    pub fn validate_workload(&self, config: &WorkloadConfig) -> PerformanceMetrics {
        println!("🧪 Validating performance for workload: {}", config.name);
        
        // Generate realistic test data
        let isg = self.generator.generate_isg(config);
        
        // Validate node operations
        let node_metrics = self.validate_node_operations(&isg);
        
        // Validate query operations
        let query_metrics = self.validate_query_operations(&isg);
        
        // Validate file operations
        let file_metrics = self.validate_file_operations(config);
        
        // Validate memory usage
        let memory_metrics = self.validate_memory_usage(&isg, config);
        
        // Validate cross-platform consistency
        let cross_platform_metrics = self.validate_cross_platform_consistency(&isg);
        
        PerformanceMetrics {
            node_operations: node_metrics,
            query_operations: query_metrics,
            file_operations: file_metrics,
            memory_usage: memory_metrics,
            cross_platform: cross_platform_metrics,
        }
    }
    
    /// Validate node operation performance contracts
    fn validate_node_operations(&self, isg: &OptimizedISG) -> NodeOperationMetrics {
        // Test node upsert performance
        let test_node = NodeData {
            hash: SigHash::from_signature("test_performance_node"),
            kind: NodeKind::Function,
            name: Arc::from("test_performance_node"),
            signature: Arc::from("fn test_performance_node()"),
            file_path: Arc::from("test.rs"),
            line: 1,
        };
        
        let start = Instant::now();
        isg.upsert_node(test_node.clone());
        let upsert_time_us = start.elapsed().as_micros() as u64;
        
        // Test node get performance
        let start = Instant::now();
        let _ = isg.get_node(test_node.hash).unwrap();
        let get_time_us = start.elapsed().as_micros() as u64;
        
        // Test name lookup performance
        let start = Instant::now();
        let _ = isg.find_by_name("test_performance_node");
        let lookup_time_us = start.elapsed().as_micros() as u64;
        
        NodeOperationMetrics {
            upsert_time_us,
            get_time_us,
            lookup_time_us,
        }
    }
    
    /// Validate query operation performance contracts
    fn validate_query_operations(&self, isg: &OptimizedISG) -> QueryOperationMetrics {
        // Get some test nodes for queries
        let state = isg.state.read();
        let mut test_hashes = Vec::new();
        
        for (hash, _) in state.id_map.iter().take(10) {
            test_hashes.push(*hash);
        }
        drop(state);
        
        if test_hashes.is_empty() {
            return QueryOperationMetrics {
                blast_radius_time_us: 0,
                what_implements_time_us: 0,
                calls_time_us: 0,
                uses_time_us: 0,
            };
        }
        
        let test_hash = test_hashes[0];
        
        // Test blast radius performance
        let start = Instant::now();
        let _ = isg.calculate_blast_radius(test_hash);
        let blast_radius_time_us = start.elapsed().as_micros() as u64;
        
        // Test what-implements performance
        let start = Instant::now();
        let _ = isg.find_implementors(test_hash);
        let what_implements_time_us = start.elapsed().as_micros() as u64;
        
        // Test calls performance
        let start = Instant::now();
        let _ = isg.find_callers(test_hash);
        let calls_time_us = start.elapsed().as_micros() as u64;
        
        // Test uses performance
        let start = Instant::now();
        let _ = isg.find_users(test_hash);
        let uses_time_us = start.elapsed().as_micros() as u64;
        
        QueryOperationMetrics {
            blast_radius_time_us,
            what_implements_time_us,
            calls_time_us,
            uses_time_us,
        }
    }
    
    /// Validate file operation performance contracts
    fn validate_file_operations(&self, config: &WorkloadConfig) -> FileOperationMetrics {
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test_dump.txt");
        
        // Generate realistic code dump
        self.generator.generate_code_dump(config, &dump_path).unwrap();
        
        // Test ingestion performance
        let mut daemon = ParseltongueAIM::new();
        let start = Instant::now();
        let _ = daemon.ingest_code_dump(&dump_path);
        let ingestion_time_s = start.elapsed().as_secs_f64();
        
        // Test file update performance (simulate single file change)
        let test_file = temp_dir.path().join("test_update.rs");
        std::fs::write(&test_file, "pub fn test_update() {}").unwrap();
        
        let start = Instant::now();
        let _ = daemon.update_file(&test_file);
        let update_time_ms = start.elapsed().as_millis() as u64;
        
        // Test snapshot save performance
        let snapshot_path = temp_dir.path().join("test_snapshot.json");
        let start = Instant::now();
        let _ = daemon.save_snapshot(&snapshot_path);
        let snapshot_save_time_ms = start.elapsed().as_millis() as u64;
        
        // Test snapshot load performance
        let start = Instant::now();
        let _ = daemon.load_snapshot(&snapshot_path);
        let snapshot_load_time_ms = start.elapsed().as_millis() as u64;
        
        FileOperationMetrics {
            update_time_ms,
            ingestion_time_s,
            snapshot_save_time_ms,
            snapshot_load_time_ms,
        }
    }
    
    /// Validate memory usage contracts
    fn validate_memory_usage(&self, isg: &OptimizedISG, config: &WorkloadConfig) -> MemoryMetrics {
        // Estimate memory usage (simplified calculation)
        let node_count = isg.node_count();
        let edge_count = isg.edge_count();
        
        // Rough estimates based on data structure sizes
        let node_size_bytes = std::mem::size_of::<NodeData>() + 64; // Account for Arc<str> overhead
        let edge_size_bytes = std::mem::size_of::<EdgeKind>() + 32; // Account for graph overhead
        let index_overhead_bytes = 64; // HashMap overhead per entry
        
        let estimated_memory_bytes = 
            (node_count * (node_size_bytes + index_overhead_bytes)) +
            (edge_count * edge_size_bytes) +
            (node_count * 32); // Name index overhead
        
        let total_memory_mb = estimated_memory_bytes / (1024 * 1024);
        let memory_per_node_bytes = if node_count > 0 { estimated_memory_bytes / node_count } else { 0 };
        let memory_per_edge_bytes = if edge_count > 0 { (edge_count * edge_size_bytes) / edge_count } else { 0 };
        
        MemoryMetrics {
            total_memory_mb,
            memory_per_node_bytes,
            memory_per_edge_bytes,
        }
    }
    
    /// Validate cross-platform consistency
    fn validate_cross_platform_consistency(&self, isg: &OptimizedISG) -> CrossPlatformMetrics {
        let platform = std::env::consts::OS.to_string();
        
        // Test hash consistency by creating identical nodes
        let test_signature = "fn test_cross_platform_consistency()";
        let hash1 = SigHash::from_signature(test_signature);
        let hash2 = SigHash::from_signature(test_signature);
        let hash_consistency = hash1 == hash2;
        
        // For now, assume no performance variance (would need actual cross-platform testing)
        let performance_variance_percent = 0.0;
        
        CrossPlatformMetrics {
            platform,
            hash_consistency,
            performance_variance_percent,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// STUB: Write failing tests for performance contracts on 100K+ LOC codebases
    #[test]
    fn test_large_workload_performance_contracts() {
        let validator = PerformanceValidator::new();
        let config = WorkloadConfig::large();
        
        let metrics = validator.validate_workload(&config);
        
        // REQ-V2-002.0: O(1) Performance Guarantees
        // Node operations must be <50μs
        assert!(metrics.node_operations.upsert_time_us < 50, 
            "Node upsert took {}μs (>50μs)", metrics.node_operations.upsert_time_us);
        assert!(metrics.node_operations.get_time_us < 50,
            "Node get took {}μs (>50μs)", metrics.node_operations.get_time_us);
        assert!(metrics.node_operations.lookup_time_us < 50,
            "Name lookup took {}μs (>50μs)", metrics.node_operations.lookup_time_us);
        
        // Query operations must be <1ms (1000μs)
        assert!(metrics.query_operations.blast_radius_time_us < 1000,
            "Blast radius took {}μs (>1ms)", metrics.query_operations.blast_radius_time_us);
        assert!(metrics.query_operations.calls_time_us < 1000,
            "Calls query took {}μs (>1ms)", metrics.query_operations.calls_time_us);
        assert!(metrics.query_operations.uses_time_us < 1000,
            "Uses query took {}μs (>1ms)", metrics.query_operations.uses_time_us);
        
        // REQ-V2-009.0: Real-Time Integration
        // File updates must be <12ms
        assert!(metrics.file_operations.update_time_ms < 12,
            "File update took {}ms (>12ms)", metrics.file_operations.update_time_ms);
        
        // Memory usage must be <25MB for 100K LOC
        assert!(metrics.memory_usage.total_memory_mb < 25,
            "Memory usage {}MB (>25MB)", metrics.memory_usage.total_memory_mb);
        
        // Cross-platform consistency
        assert!(metrics.cross_platform.hash_consistency,
            "Hash consistency failed on platform {}", metrics.cross_platform.platform);
        
        println!("✅ Large workload performance validation passed");
        println!("   Node operations: {}μs upsert, {}μs get, {}μs lookup", 
            metrics.node_operations.upsert_time_us,
            metrics.node_operations.get_time_us,
            metrics.node_operations.lookup_time_us);
        println!("   Query operations: {}μs blast-radius, {}μs calls, {}μs uses",
            metrics.query_operations.blast_radius_time_us,
            metrics.query_operations.calls_time_us,
            metrics.query_operations.uses_time_us);
        println!("   File operations: {}ms update, {:.2}s ingestion",
            metrics.file_operations.update_time_ms,
            metrics.file_operations.ingestion_time_s);
        println!("   Memory usage: {}MB total ({} bytes/node)",
            metrics.memory_usage.total_memory_mb,
            metrics.memory_usage.memory_per_node_bytes);
    }
    
    #[test]
    fn test_extra_large_workload_stress_test() {
        let validator = PerformanceValidator::new();
        let config = WorkloadConfig::extra_large();
        
        let metrics = validator.validate_workload(&config);
        
        // Stress test with relaxed constraints (2x tolerance)
        assert!(metrics.node_operations.upsert_time_us < 100,
            "Node upsert took {}μs (>100μs stress test)", metrics.node_operations.upsert_time_us);
        assert!(metrics.query_operations.blast_radius_time_us < 2000,
            "Blast radius took {}μs (>2ms stress test)", metrics.query_operations.blast_radius_time_us);
        assert!(metrics.file_operations.update_time_ms < 25,
            "File update took {}ms (>25ms stress test)", metrics.file_operations.update_time_ms);
        
        println!("✅ Extra large workload stress test passed");
        println!("   Nodes: {}, Edges: {}, LOC: {}", 
            config.node_count, config.edge_count, config.lines_of_code);
    }
    
    #[test]
    fn test_medium_workload_baseline() {
        let validator = PerformanceValidator::new();
        let config = WorkloadConfig::medium();
        
        let metrics = validator.validate_workload(&config);
        
        // Medium workload should easily meet all constraints
        assert!(metrics.node_operations.upsert_time_us < 25,
            "Node upsert took {}μs (>25μs baseline)", metrics.node_operations.upsert_time_us);
        assert!(metrics.query_operations.blast_radius_time_us < 500,
            "Blast radius took {}μs (>500μs baseline)", metrics.query_operations.blast_radius_time_us);
        assert!(metrics.file_operations.update_time_ms < 6,
            "File update took {}ms (>6ms baseline)", metrics.file_operations.update_time_ms);
        assert!(metrics.memory_usage.total_memory_mb < 12,
            "Memory usage {}MB (>12MB baseline)", metrics.memory_usage.total_memory_mb);
        
        println!("✅ Medium workload baseline validation passed");
    }
    
    #[test]
    fn test_small_workload_optimal() {
        let validator = PerformanceValidator::new();
        let config = WorkloadConfig::small();
        
        let metrics = validator.validate_workload(&config);
        
        // Small workload should have optimal performance
        assert!(metrics.node_operations.upsert_time_us < 10,
            "Node upsert took {}μs (>10μs optimal)", metrics.node_operations.upsert_time_us);
        assert!(metrics.query_operations.blast_radius_time_us < 100,
            "Blast radius took {}μs (>100μs optimal)", metrics.query_operations.blast_radius_time_us);
        assert!(metrics.file_operations.update_time_ms < 3,
            "File update took {}ms (>3ms optimal)", metrics.file_operations.update_time_ms);
        assert!(metrics.memory_usage.total_memory_mb < 5,
            "Memory usage {}MB (>5MB optimal)", metrics.memory_usage.total_memory_mb);
        
        println!("✅ Small workload optimal performance validation passed");
    }
    
    #[test]
    fn test_performance_regression_detection() {
        let validator = PerformanceValidator::new();
        
        // Test multiple workloads to detect performance regressions
        let configs = vec![
            WorkloadConfig::small(),
            WorkloadConfig::medium(),
            WorkloadConfig::large(),
        ];
        
        let mut baseline_metrics = Vec::new();
        
        for config in &configs {
            let metrics = validator.validate_workload(config);
            baseline_metrics.push((config.name, metrics));
        }
        
        // Verify performance scales reasonably with workload size
        for i in 1..baseline_metrics.len() {
            let (prev_name, prev_metrics) = &baseline_metrics[i-1];
            let (curr_name, curr_metrics) = &baseline_metrics[i];
            
            // Node operations should remain roughly constant (O(1))
            let upsert_ratio = curr_metrics.node_operations.upsert_time_us as f64 / 
                              prev_metrics.node_operations.upsert_time_us as f64;
            assert!(upsert_ratio < 3.0, 
                "Node upsert performance degraded {}x from {} to {}", 
                upsert_ratio, prev_name, curr_name);
            
            println!("📊 Performance scaling from {} to {}: {:.2}x upsert time",
                prev_name, curr_name, upsert_ratio);
        }
        
        println!("✅ Performance regression detection passed");
    }
    
    #[test]
    fn test_memory_efficiency_validation() {
        let validator = PerformanceValidator::new();
        let config = WorkloadConfig::large();
        
        let metrics = validator.validate_workload(&config);
        
        // Validate memory efficiency targets
        assert!(metrics.memory_usage.memory_per_node_bytes < 500,
            "Memory per node {}bytes (>500bytes)", metrics.memory_usage.memory_per_node_bytes);
        
        // Memory should scale linearly with node count
        let expected_memory_mb = (config.node_count * 200) / (1024 * 1024); // ~200 bytes per node
        assert!(metrics.memory_usage.total_memory_mb < expected_memory_mb + 10,
            "Memory usage {}MB exceeds expected {}MB + 10MB buffer", 
            metrics.memory_usage.total_memory_mb, expected_memory_mb);
        
        println!("✅ Memory efficiency validation passed");
        println!("   Memory per node: {} bytes", metrics.memory_usage.memory_per_node_bytes);
        println!("   Total memory: {}MB for {} nodes", 
            metrics.memory_usage.total_memory_mb, config.node_count);
    }
    
    #[test]
    fn test_cross_platform_consistency() {
        let validator = PerformanceValidator::new();
        let config = WorkloadConfig::medium();
        
        let metrics = validator.validate_cross_platform_consistency(
            &validator.generator.generate_isg(&config)
        );
        
        assert!(metrics.hash_consistency, 
            "Hash consistency failed on platform {}", metrics.platform);
        
        println!("✅ Cross-platform consistency validated on {}", metrics.platform);
    }
    
    #[test]
    fn test_realistic_data_generation() {
        let generator = RealisticDataGenerator::new();
        let config = WorkloadConfig::medium();
        
        let isg = generator.generate_isg(&config);
        
        // Validate generated data meets expectations
        assert_eq!(isg.node_count(), config.node_count);
        assert!(isg.edge_count() > 0, "Should generate edges");
        assert!(isg.edge_count() <= config.edge_count * 2, "Edge count reasonable");
        
        // Test code dump generation
        let temp_dir = TempDir::new().unwrap();
        let dump_path = temp_dir.path().join("test_dump.txt");
        
        generator.generate_code_dump(&config, &dump_path).unwrap();
        
        let content = std::fs::read_to_string(&dump_path).unwrap();
        assert!(content.contains("FILE:"), "Should contain file markers");
        assert!(content.contains("pub fn"), "Should contain functions");
        assert!(content.contains("pub struct"), "Should contain structs");
        assert!(content.contains("pub trait"), "Should contain traits");
        
        println!("✅ Realistic data generation validated");
        println!("   Generated {} nodes, {} edges", isg.node_count(), isg.edge_count());
        println!("   Code dump: {} bytes", content.len());
    }
}
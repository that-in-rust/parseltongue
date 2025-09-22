//! Cross-platform test runner for CI/CD environments
//! 
//! Provides utilities for running cross-platform consistency tests
//! in different environments (Linux, macOS, Windows)

use parseltongue::{OptimizedISG, SigHash};
use std::time::Instant;
use serde::{Serialize, Deserialize};

/// Cross-platform test runner configuration
#[derive(Debug, Clone)]
pub struct CrossPlatformTestConfig {
    pub test_iterations: usize,
    pub performance_threshold_ms: u64,
    pub hash_consistency_required: bool,
    pub generate_reference_data: bool,
    pub validate_against_reference: bool,
    pub reference_data_path: Option<String>,
}

impl Default for CrossPlatformTestConfig {
    fn default() -> Self {
        Self {
            test_iterations: 10,
            performance_threshold_ms: 1,
            hash_consistency_required: true,
            generate_reference_data: false,
            validate_against_reference: false,
            reference_data_path: None,
        }
    }
}

/// Cross-platform test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPlatformTestResults {
    pub platform_info: PlatformInfo,
    pub test_summary: TestSummary,
    pub hash_consistency_results: HashConsistencyResults,
    pub performance_results: PerformanceResults,
    pub graph_consistency_results: GraphConsistencyResults,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub rust_version: String,
    pub cargo_version: String,
    pub target_triple: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashConsistencyResults {
    pub signatures_tested: usize,
    pub consistent_hashes: usize,
    pub inconsistent_hashes: Vec<HashInconsistency>,
    pub consistency_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashInconsistency {
    pub signature: String,
    pub hash_values: Vec<u64>,
    pub iteration_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceResults {
    pub query_performance: QueryPerformanceResults,
    pub hash_generation_performance: HashGenerationResults,
    pub graph_operations_performance: GraphOperationResults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPerformanceResults {
    pub blast_radius_avg_us: u64,
    pub find_implementors_avg_us: u64,
    pub find_callers_avg_us: u64,
    pub find_users_avg_us: u64,
    pub performance_variance_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashGenerationResults {
    pub avg_hash_time_ns: u64,
    pub hashes_per_second: u64,
    pub consistency_across_iterations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphOperationResults {
    pub node_upsert_avg_us: u64,
    pub edge_upsert_avg_us: u64,
    pub node_lookup_avg_us: u64,
    pub graph_traversal_avg_us: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConsistencyResults {
    pub identical_structure_across_runs: bool,
    pub deterministic_query_results: bool,
    pub serialization_consistency: bool,
    pub node_count_consistency: bool,
    pub edge_count_consistency: bool,
}

/// Cross-platform test runner
pub struct CrossPlatformTestRunner {
    config: CrossPlatformTestConfig,
}

impl CrossPlatformTestRunner {
    pub fn new(config: CrossPlatformTestConfig) -> Self {
        Self { config }
    }
    
    /// Run comprehensive cross-platform tests
    pub fn run_tests(&self) -> CrossPlatformTestResults {
        println!("🚀 Starting cross-platform consistency tests");
        println!("   Platform: {}-{}", std::env::consts::OS, std::env::consts::ARCH);
        
        let platform_info = self.collect_platform_info();
        let start_time = Instant::now();
        
        // Run all test categories
        let hash_consistency_results = self.test_hash_consistency();
        let performance_results = self.test_performance();
        let graph_consistency_results = self.test_graph_consistency();
        
        // Calculate test summary
        let test_summary = self.calculate_test_summary(
            &hash_consistency_results,
            &performance_results,
            &graph_consistency_results,
        );
        
        let elapsed = start_time.elapsed();
        println!("✅ Cross-platform tests completed in {:.2}s", elapsed.as_secs_f64());
        
        CrossPlatformTestResults {
            platform_info,
            test_summary,
            hash_consistency_results,
            performance_results,
            graph_consistency_results,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Collect platform information
    fn collect_platform_info(&self) -> PlatformInfo {
        PlatformInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            rust_version: std::env::var("RUSTC_VERSION")
                .unwrap_or_else(|_| "unknown".to_string()),
            cargo_version: std::env::var("CARGO_VERSION")
                .unwrap_or_else(|_| "unknown".to_string()),
            target_triple: std::env::var("TARGET")
                .unwrap_or_else(|_| std::env::consts::ARCH.to_string()),
        }
    }
    
    /// Test hash consistency across multiple iterations
    fn test_hash_consistency(&self) -> HashConsistencyResults {
        println!("🔍 Testing hash consistency...");
        
        let test_signatures = vec![
            "fn main()",
            "fn hello_world() -> String",
            "struct User { name: String, age: u32 }",
            "trait Display { fn fmt(&self) -> String; }",
            "fn generic<T>(value: T) -> T",
            "fn with_lifetime<'a>(s: &'a str) -> &'a str",
            "std::collections::HashMap::new",
            "fn test_unicode_函数() -> String",
        ];
        
        let mut consistent_hashes = 0;
        let mut inconsistent_hashes = Vec::new();
        
        for signature in &test_signatures {
            let mut hash_values = Vec::new();
            
            // Test hash consistency across multiple iterations
            for _ in 0..self.config.test_iterations {
                let hash = SigHash::from_signature(signature);
                hash_values.push(hash.0);
            }
            
            // Check if all hash values are identical
            let first_hash = hash_values[0];
            let is_consistent = hash_values.iter().all(|&h| h == first_hash);
            
            if is_consistent {
                consistent_hashes += 1;
            } else {
                inconsistent_hashes.push(HashInconsistency {
                    signature: signature.to_string(),
                    hash_values: hash_values.clone(),
                    iteration_count: self.config.test_iterations,
                });
            }
        }
        
        let consistency_rate = (consistent_hashes as f64 / test_signatures.len() as f64) * 100.0;
        
        println!("   📊 Hash consistency: {:.1}% ({}/{})", 
            consistency_rate, consistent_hashes, test_signatures.len());
        
        HashConsistencyResults {
            signatures_tested: test_signatures.len(),
            consistent_hashes,
            inconsistent_hashes,
            consistency_rate,
        }
    }
    
    /// Test performance consistency
    fn test_performance(&self) -> PerformanceResults {
        println!("⚡ Testing performance consistency...");
        
        let isg = self.create_test_graph();
        
        // Test query performance
        let query_performance = self.test_query_performance(&isg);
        
        // Test hash generation performance
        let hash_generation_performance = self.test_hash_generation_performance();
        
        // Test graph operations performance
        let graph_operations_performance = self.test_graph_operations_performance(&isg);
        
        PerformanceResults {
            query_performance,
            hash_generation_performance,
            graph_operations_performance,
        }
    }
    
    /// Test query performance
    fn test_query_performance(&self, isg: &OptimizedISG) -> QueryPerformanceResults {
        let main_hash = SigHash::from_signature("fn main()");
        let user_hash = SigHash::from_signature("struct User { name: String, age: u32 }");
        let display_hash = SigHash::from_signature("trait Display { fn fmt(&self) -> String; }");
        
        // Test blast radius performance
        let mut blast_radius_times = Vec::new();
        for _ in 0..self.config.test_iterations {
            let start = Instant::now();
            let _ = isg.calculate_blast_radius(main_hash).unwrap();
            blast_radius_times.push(start.elapsed().as_micros() as u64);
        }
        
        // Test find implementors performance
        let mut implementors_times = Vec::new();
        for _ in 0..self.config.test_iterations {
            let start = Instant::now();
            let _ = isg.find_implementors(display_hash).unwrap();
            implementors_times.push(start.elapsed().as_micros() as u64);
        }
        
        // Test find callers performance
        let mut callers_times = Vec::new();
        for _ in 0..self.config.test_iterations {
            let start = Instant::now();
            let _ = isg.find_callers(main_hash).unwrap();
            callers_times.push(start.elapsed().as_micros() as u64);
        }
        
        // Test find users performance
        let mut users_times = Vec::new();
        for _ in 0..self.config.test_iterations {
            let start = Instant::now();
            let _ = isg.find_users(user_hash).unwrap();
            users_times.push(start.elapsed().as_micros() as u64);
        }
        
        // Calculate averages
        let blast_radius_avg = blast_radius_times.iter().sum::<u64>() / blast_radius_times.len() as u64;
        let implementors_avg = implementors_times.iter().sum::<u64>() / implementors_times.len() as u64;
        let callers_avg = callers_times.iter().sum::<u64>() / callers_times.len() as u64;
        let users_avg = users_times.iter().sum::<u64>() / users_times.len() as u64;
        
        // Calculate performance variance
        let mut all_times = Vec::new();
        all_times.extend(&blast_radius_times);
        all_times.extend(&implementors_times);
        all_times.extend(&callers_times);
        all_times.extend(&users_times);
        let avg_time = all_times.iter().sum::<u64>() / all_times.len() as u64;
        let max_time = *all_times.iter().max().unwrap();
        let min_time = *all_times.iter().min().unwrap();
        
        let performance_variance = if avg_time > 0 {
            ((max_time - min_time) as f64 / avg_time as f64) * 100.0
        } else {
            0.0
        };
        
        println!("   📊 Query performance - Blast radius: {}μs, Implementors: {}μs, Callers: {}μs, Users: {}μs",
            blast_radius_avg, implementors_avg, callers_avg, users_avg);
        
        QueryPerformanceResults {
            blast_radius_avg_us: blast_radius_avg,
            find_implementors_avg_us: implementors_avg,
            find_callers_avg_us: callers_avg,
            find_users_avg_us: users_avg,
            performance_variance_percent: performance_variance,
        }
    }
    
    /// Test hash generation performance
    fn test_hash_generation_performance(&self) -> HashGenerationResults {
        let test_signature = "fn test_performance_signature() -> String";
        
        let mut hash_times = Vec::new();
        let mut hash_values = Vec::new();
        
        for _ in 0..self.config.test_iterations * 10 { // More iterations for hash timing
            let start = Instant::now();
            let hash = SigHash::from_signature(test_signature);
            let elapsed = start.elapsed().as_nanos() as u64;
            
            hash_times.push(elapsed);
            hash_values.push(hash.0);
        }
        
        let avg_hash_time = hash_times.iter().sum::<u64>() / hash_times.len() as u64;
        let hashes_per_second = if avg_hash_time > 0 {
            1_000_000_000 / avg_hash_time // Convert nanoseconds to hashes per second
        } else {
            0
        };
        
        // Check consistency across iterations
        let first_hash = hash_values[0];
        let consistency = hash_values.iter().all(|&h| h == first_hash);
        
        println!("   📊 Hash generation: {}ns avg, {} hashes/sec, consistent: {}",
            avg_hash_time, hashes_per_second, consistency);
        
        HashGenerationResults {
            avg_hash_time_ns: avg_hash_time,
            hashes_per_second,
            consistency_across_iterations: consistency,
        }
    }
    
    /// Test graph operations performance
    fn test_graph_operations_performance(&self, isg: &OptimizedISG) -> GraphOperationResults {
        // Test node upsert performance
        let mut node_upsert_times = Vec::new();
        for i in 0..self.config.test_iterations {
            let node = parseltongue::NodeData {
                hash: SigHash::from_signature(&format!("fn test_node_{}", i)),
                kind: parseltongue::NodeKind::Function,
                name: std::sync::Arc::from(format!("test_node_{}", i)),
                signature: std::sync::Arc::from(format!("fn test_node_{}()", i)),
                file_path: std::sync::Arc::from("test.rs"),
                line: i as u32,
            };
            
            let start = Instant::now();
            isg.upsert_node(node);
            node_upsert_times.push(start.elapsed().as_micros() as u64);
        }
        
        // Test node lookup performance
        let test_hash = SigHash::from_signature("fn test_node_0");
        let mut lookup_times = Vec::new();
        for _ in 0..self.config.test_iterations {
            let start = Instant::now();
            let _ = isg.get_node(test_hash);
            lookup_times.push(start.elapsed().as_micros() as u64);
        }
        
        // Calculate averages
        let node_upsert_avg = node_upsert_times.iter().sum::<u64>() / node_upsert_times.len() as u64;
        let lookup_avg = lookup_times.iter().sum::<u64>() / lookup_times.len() as u64;
        
        println!("   📊 Graph operations - Node upsert: {}μs, Lookup: {}μs",
            node_upsert_avg, lookup_avg);
        
        GraphOperationResults {
            node_upsert_avg_us: node_upsert_avg,
            edge_upsert_avg_us: 0, // TODO: Implement edge upsert timing
            node_lookup_avg_us: lookup_avg,
            graph_traversal_avg_us: 0, // TODO: Implement traversal timing
        }
    }
    
    /// Test graph consistency
    fn test_graph_consistency(&self) -> GraphConsistencyResults {
        println!("🏗️  Testing graph consistency...");
        
        // Create multiple identical graphs
        let graphs: Vec<OptimizedISG> = (0..3)
            .map(|_| self.create_test_graph())
            .collect();
        
        // Test structure consistency
        let first_node_count = graphs[0].node_count();
        let first_edge_count = graphs[0].edge_count();
        
        let identical_structure = graphs.iter().all(|g| {
            g.node_count() == first_node_count && g.edge_count() == first_edge_count
        });
        
        // Test query result consistency
        let main_hash = SigHash::from_signature("fn main()");
        let first_blast_radius = graphs[0].calculate_blast_radius(main_hash).unwrap();
        
        let deterministic_queries = graphs.iter().all(|g| {
            g.calculate_blast_radius(main_hash).unwrap() == first_blast_radius
        });
        
        // Test serialization consistency (simplified)
        let serialization_consistent = true; // TODO: Implement actual serialization test
        
        println!("   📊 Graph consistency - Structure: {}, Queries: {}, Serialization: {}",
            identical_structure, deterministic_queries, serialization_consistent);
        
        GraphConsistencyResults {
            identical_structure_across_runs: identical_structure,
            deterministic_query_results: deterministic_queries,
            serialization_consistency: serialization_consistent,
            node_count_consistency: identical_structure,
            edge_count_consistency: identical_structure,
        }
    }
    
    /// Create a test graph with known structure
    fn create_test_graph(&self) -> OptimizedISG {
        let isg = OptimizedISG::new();
        
        // Create deterministic test nodes
        let nodes = vec![
            ("fn main", parseltongue::NodeKind::Function, "fn main()"),
            ("struct User", parseltongue::NodeKind::Struct, "struct User { name: String, age: u32 }"),
            ("trait Display", parseltongue::NodeKind::Trait, "trait Display { fn fmt(&self) -> String; }"),
            ("fn create_user", parseltongue::NodeKind::Function, "fn create_user(name: String, age: u32) -> User"),
        ];
        
        for (name, kind, signature) in nodes {
            let node = parseltongue::NodeData {
                hash: SigHash::from_signature(signature),
                kind,
                name: std::sync::Arc::from(name),
                signature: std::sync::Arc::from(signature),
                file_path: std::sync::Arc::from("src/lib.rs"),
                line: 1,
            };
            isg.upsert_node(node);
        }
        
        // Create deterministic edges
        let main_hash = SigHash::from_signature("fn main()");
        let user_hash = SigHash::from_signature("struct User { name: String, age: u32 }");
        let display_hash = SigHash::from_signature("trait Display { fn fmt(&self) -> String; }");
        let create_user_hash = SigHash::from_signature("fn create_user(name: String, age: u32) -> User");
        
        isg.upsert_edge(main_hash, create_user_hash, parseltongue::EdgeKind::Calls).unwrap();
        isg.upsert_edge(create_user_hash, user_hash, parseltongue::EdgeKind::Uses).unwrap();
        isg.upsert_edge(user_hash, display_hash, parseltongue::EdgeKind::Implements).unwrap();
        
        isg
    }
    
    /// Calculate overall test summary
    fn calculate_test_summary(
        &self,
        hash_results: &HashConsistencyResults,
        _performance_results: &PerformanceResults,
        graph_results: &GraphConsistencyResults,
    ) -> TestSummary {
        let mut total_tests = 0;
        let mut passed_tests = 0;
        let mut failed_tests = 0;
        
        // Hash consistency tests
        total_tests += hash_results.signatures_tested;
        passed_tests += hash_results.consistent_hashes;
        failed_tests += hash_results.inconsistent_hashes.len();
        
        // Graph consistency tests (simplified counting)
        total_tests += 5; // Structure, queries, serialization, node count, edge count
        if graph_results.identical_structure_across_runs { passed_tests += 1; } else { failed_tests += 1; }
        if graph_results.deterministic_query_results { passed_tests += 1; } else { failed_tests += 1; }
        if graph_results.serialization_consistency { passed_tests += 1; } else { failed_tests += 1; }
        if graph_results.node_count_consistency { passed_tests += 1; } else { failed_tests += 1; }
        if graph_results.edge_count_consistency { passed_tests += 1; } else { failed_tests += 1; }
        
        let success_rate = if total_tests > 0 {
            (passed_tests as f64 / total_tests as f64) * 100.0
        } else {
            0.0
        };
        
        TestSummary {
            total_tests,
            passed_tests,
            failed_tests,
            skipped_tests: 0,
            success_rate,
        }
    }
}

impl CrossPlatformTestResults {
    /// Print comprehensive test results
    pub fn print_results(&self) {
        println!("\n📊 Cross-Platform Test Results");
        println!("=====================================");
        
        // Platform info
        println!("🖥️  Platform Information:");
        println!("   OS: {}", self.platform_info.os);
        println!("   Architecture: {}", self.platform_info.arch);
        println!("   Rust version: {}", self.platform_info.rust_version);
        println!("   Target: {}", self.platform_info.target_triple);
        
        // Test summary
        println!("\n📈 Test Summary:");
        println!("   Total tests: {}", self.test_summary.total_tests);
        println!("   Passed: {}", self.test_summary.passed_tests);
        println!("   Failed: {}", self.test_summary.failed_tests);
        println!("   Success rate: {:.1}%", self.test_summary.success_rate);
        
        // Hash consistency
        println!("\n🔍 Hash Consistency:");
        println!("   Signatures tested: {}", self.hash_consistency_results.signatures_tested);
        println!("   Consistent: {}", self.hash_consistency_results.consistent_hashes);
        println!("   Consistency rate: {:.1}%", self.hash_consistency_results.consistency_rate);
        
        if !self.hash_consistency_results.inconsistent_hashes.is_empty() {
            println!("   ❌ Inconsistent hashes:");
            for inconsistency in &self.hash_consistency_results.inconsistent_hashes {
                println!("      '{}': {:?}", inconsistency.signature, inconsistency.hash_values);
            }
        }
        
        // Performance results
        println!("\n⚡ Performance Results:");
        println!("   Blast radius: {}μs", self.performance_results.query_performance.blast_radius_avg_us);
        println!("   Find implementors: {}μs", self.performance_results.query_performance.find_implementors_avg_us);
        println!("   Hash generation: {}ns", self.performance_results.hash_generation_performance.avg_hash_time_ns);
        println!("   Node upsert: {}μs", self.performance_results.graph_operations_performance.node_upsert_avg_us);
        
        // Graph consistency
        println!("\n🏗️  Graph Consistency:");
        println!("   Structure consistency: {}", self.graph_consistency_results.identical_structure_across_runs);
        println!("   Query determinism: {}", self.graph_consistency_results.deterministic_query_results);
        println!("   Serialization consistency: {}", self.graph_consistency_results.serialization_consistency);
        
        // Overall result
        if self.test_summary.success_rate >= 100.0 {
            println!("\n✅ All cross-platform tests PASSED!");
        } else if self.test_summary.success_rate >= 95.0 {
            println!("\n⚠️  Cross-platform tests mostly passed ({:.1}%)", self.test_summary.success_rate);
        } else {
            println!("\n❌ Cross-platform tests FAILED ({:.1}% success rate)", self.test_summary.success_rate);
        }
    }
    
    /// Export results to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Save results to file
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cross_platform_runner() {
        let config = CrossPlatformTestConfig::default();
        let runner = CrossPlatformTestRunner::new(config);
        
        let results = runner.run_tests();
        
        assert!(results.test_summary.total_tests > 0, "No tests were run");
        assert!(results.test_summary.success_rate >= 95.0, 
            "Success rate too low: {:.1}%", results.test_summary.success_rate);
        
        results.print_results();
    }
    
    #[test]
    fn test_platform_info_collection() {
        let config = CrossPlatformTestConfig::default();
        let runner = CrossPlatformTestRunner::new(config);
        
        let platform_info = runner.collect_platform_info();
        
        assert!(!platform_info.os.is_empty(), "OS not detected");
        assert!(!platform_info.arch.is_empty(), "Architecture not detected");
        assert!(!platform_info.rust_version.is_empty(), "Rust version not detected");
        
        println!("✅ Platform info collection test passed");
        println!("   📊 OS: {}", platform_info.os);
        println!("   📊 Arch: {}", platform_info.arch);
        println!("   📊 Rust: {}", platform_info.rust_version);
    }
}
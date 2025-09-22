//! Platform reference data generation and validation
//! 
//! Generates and validates reference data for cross-platform consistency testing
//! This module creates "golden" reference data that should be identical across platforms

use parseltongue::{OptimizedISG, SigHash, NodeData, NodeKind, EdgeKind};
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Reference data for cross-platform validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformReferenceData {
    pub version: String,
    pub generated_on: String,
    pub reference_hashes: HashMap<String, u64>,
    pub reference_graph: ReferenceGraph,
    pub test_cases: Vec<TestCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceGraph {
    pub nodes: Vec<ReferenceNode>,
    pub edges: Vec<ReferenceEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceNode {
    pub signature: String,
    pub hash: u64,
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceEdge {
    pub source_signature: String,
    pub target_signature: String,
    pub source_hash: u64,
    pub target_hash: u64,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub description: String,
    pub input_signature: String,
    pub expected_hash: u64,
    pub expected_queries: HashMap<String, Vec<u64>>, // query_type -> expected_result_hashes
}

/// Generate reference data for the current platform
pub fn generate_reference_data() -> PlatformReferenceData {
    let version = env!("CARGO_PKG_VERSION").to_string();
    let platform = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    
    // Core test signatures that must be consistent across platforms
    let test_signatures = vec![
        // Basic functions
        "fn main()",
        "fn hello_world() -> String",
        "fn add(a: i32, b: i32) -> i32",
        "fn multiply(x: f64, y: f64) -> f64",
        
        // Generic functions
        "fn identity<T>(value: T) -> T",
        "fn swap<T, U>(a: T, b: U) -> (U, T)",
        "fn map<T, U, F>(value: T, f: F) -> U where F: Fn(T) -> U",
        
        // Structs
        "struct Point { x: i32, y: i32 }",
        "struct User { name: String, age: u32, email: String }",
        "struct Config { debug: bool, port: u16 }",
        "struct GenericStruct<T> { value: T }",
        "struct ComplexStruct<T, U> where T: Clone, U: Send { a: T, b: U }",
        
        // Traits
        "trait Display { fn fmt(&self) -> String; }",
        "trait Clone { fn clone(&self) -> Self; }",
        "trait Iterator<Item> { fn next(&mut self) -> Option<Item>; }",
        "trait Send",
        "trait Sync",
        
        // Enums
        "enum Option<T> { Some(T), None }",
        "enum Result<T, E> { Ok(T), Err(E) }",
        "enum Color { Red, Green, Blue }",
        
        // Complex signatures with lifetimes
        "fn with_lifetime<'a>(s: &'a str) -> &'a str",
        "fn multiple_lifetimes<'a, 'b>(a: &'a str, b: &'b str) -> &'a str",
        "fn lifetime_bounds<'a, T: 'a>(value: &'a T) -> &'a T",
        
        // Module paths (FQN testing)
        "std::collections::HashMap::new",
        "std::vec::Vec::push",
        "std::option::Option::unwrap",
        "my_crate::utils::Config::load",
        "tokio::runtime::Runtime::new",
        
        // Special cases
        "fn unsafe_function() -> *const u8",
        "fn async_function() -> impl Future<Output = String>",
        "fn const_function() -> i32",
        
        // Unicode support
        "fn test_unicode_函数() -> String",
        "struct Unicode_结构体 { field: String }",
        "trait Unicode_特征 { fn method(&self); }",
    ];
    
    // Generate reference hashes
    let mut reference_hashes = HashMap::new();
    for signature in &test_signatures {
        let hash = SigHash::from_signature(signature);
        reference_hashes.insert(signature.to_string(), hash.0);
    }
    
    // Create reference graph
    let reference_graph = create_reference_graph();
    
    // Generate test cases
    let test_cases = generate_test_cases();
    
    PlatformReferenceData {
        version,
        generated_on: platform,
        reference_hashes,
        reference_graph,
        test_cases,
    }
}

/// Create a reference graph with known structure
fn create_reference_graph() -> ReferenceGraph {
    let isg = OptimizedISG::new();
    
    // Create nodes with deterministic signatures
    let node_signatures = vec![
        ("fn main()", NodeKind::Function, "main"),
        ("fn create_user(name: String, age: u32) -> User", NodeKind::Function, "create_user"),
        ("fn validate_user(user: &User) -> bool", NodeKind::Function, "validate_user"),
        ("struct User { name: String, age: u32 }", NodeKind::Struct, "User"),
        ("struct Config { debug: bool }", NodeKind::Struct, "Config"),
        ("trait Display { fn fmt(&self) -> String; }", NodeKind::Trait, "Display"),
        ("trait Validate { fn is_valid(&self) -> bool; }", NodeKind::Trait, "Validate"),
    ];
    
    let mut nodes = Vec::new();
    for (signature, kind, name) in &node_signatures {
        let hash = SigHash::from_signature(signature);
        let node = NodeData {
            hash,
            kind: kind.clone(),
            name: Arc::from(*name),
            signature: Arc::from(*signature),
            file_path: Arc::from("src/lib.rs"),
            line: 1,
        };
        isg.upsert_node(node.clone());
        
        nodes.push(ReferenceNode {
            signature: signature.to_string(),
            hash: hash.0,
            kind: format!("{:?}", kind),
            name: name.to_string(),
        });
    }
    
    // Create edges with deterministic relationships
    let edge_definitions = vec![
        ("fn main()", "fn create_user(name: String, age: u32) -> User", EdgeKind::Calls),
        ("fn main()", "fn validate_user(user: &User) -> bool", EdgeKind::Calls),
        ("fn create_user(name: String, age: u32) -> User", "struct User { name: String, age: u32 }", EdgeKind::Uses),
        ("fn validate_user(user: &User) -> bool", "struct User { name: String, age: u32 }", EdgeKind::Uses),
        ("struct User { name: String, age: u32 }", "trait Display { fn fmt(&self) -> String; }", EdgeKind::Implements),
        ("struct User { name: String, age: u32 }", "trait Validate { fn is_valid(&self) -> bool; }", EdgeKind::Implements),
        ("struct Config { debug: bool }", "trait Display { fn fmt(&self) -> String; }", EdgeKind::Implements),
    ];
    
    let mut edges = Vec::new();
    for (source_sig, target_sig, edge_kind) in &edge_definitions {
        let source_hash = SigHash::from_signature(source_sig);
        let target_hash = SigHash::from_signature(target_sig);
        
        isg.upsert_edge(source_hash, target_hash, *edge_kind).unwrap();
        
        edges.push(ReferenceEdge {
            source_signature: source_sig.to_string(),
            target_signature: target_sig.to_string(),
            source_hash: source_hash.0,
            target_hash: target_hash.0,
            kind: format!("{:?}", edge_kind),
        });
    }
    
    ReferenceGraph { nodes, edges }
}

/// Generate comprehensive test cases
fn generate_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            name: "basic_function_hash".to_string(),
            description: "Test basic function signature hashing".to_string(),
            input_signature: "fn main()".to_string(),
            expected_hash: SigHash::from_signature("fn main()").0,
            expected_queries: HashMap::new(),
        },
        TestCase {
            name: "generic_function_hash".to_string(),
            description: "Test generic function signature hashing".to_string(),
            input_signature: "fn identity<T>(value: T) -> T".to_string(),
            expected_hash: SigHash::from_signature("fn identity<T>(value: T) -> T").0,
            expected_queries: HashMap::new(),
        },
        TestCase {
            name: "struct_hash".to_string(),
            description: "Test struct signature hashing".to_string(),
            input_signature: "struct User { name: String, age: u32 }".to_string(),
            expected_hash: SigHash::from_signature("struct User { name: String, age: u32 }").0,
            expected_queries: HashMap::new(),
        },
        TestCase {
            name: "trait_hash".to_string(),
            description: "Test trait signature hashing".to_string(),
            input_signature: "trait Display { fn fmt(&self) -> String; }".to_string(),
            expected_hash: SigHash::from_signature("trait Display { fn fmt(&self) -> String; }").0,
            expected_queries: HashMap::new(),
        },
        TestCase {
            name: "unicode_hash".to_string(),
            description: "Test Unicode signature hashing".to_string(),
            input_signature: "fn test_unicode_函数() -> String".to_string(),
            expected_hash: SigHash::from_signature("fn test_unicode_函数() -> String").0,
            expected_queries: HashMap::new(),
        },
    ]
}

/// Validate current platform against reference data
pub fn validate_against_reference(reference: &PlatformReferenceData) -> Result<ValidationReport, String> {
    let mut report = ValidationReport {
        platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        total_tests: 0,
        passed_tests: 0,
        failed_tests: Vec::new(),
        hash_mismatches: Vec::new(),
        performance_metrics: HashMap::new(),
    };
    
    // Validate reference hashes
    for (signature, expected_hash) in &reference.reference_hashes {
        report.total_tests += 1;
        
        let computed_hash = SigHash::from_signature(signature).0;
        if computed_hash == *expected_hash {
            report.passed_tests += 1;
        } else {
            report.failed_tests.push(format!("Hash mismatch for '{}'", signature));
            report.hash_mismatches.push(HashMismatch {
                signature: signature.clone(),
                expected: *expected_hash,
                computed: computed_hash,
            });
        }
    }
    
    // Validate test cases
    for test_case in &reference.test_cases {
        report.total_tests += 1;
        
        let computed_hash = SigHash::from_signature(&test_case.input_signature).0;
        if computed_hash == test_case.expected_hash {
            report.passed_tests += 1;
        } else {
            report.failed_tests.push(format!("Test case '{}' failed", test_case.name));
            report.hash_mismatches.push(HashMismatch {
                signature: test_case.input_signature.clone(),
                expected: test_case.expected_hash,
                computed: computed_hash,
            });
        }
    }
    
    Ok(report)
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub platform: String,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: Vec<String>,
    pub hash_mismatches: Vec<HashMismatch>,
    pub performance_metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct HashMismatch {
    pub signature: String,
    pub expected: u64,
    pub computed: u64,
}

impl ValidationReport {
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }
    
    pub fn is_successful(&self) -> bool {
        self.failed_tests.is_empty()
    }
    
    pub fn print_summary(&self) {
        println!("📊 Cross-Platform Validation Report");
        println!("   Platform: {}", self.platform);
        println!("   Total tests: {}", self.total_tests);
        println!("   Passed: {}", self.passed_tests);
        println!("   Failed: {}", self.failed_tests.len());
        println!("   Success rate: {:.1}%", self.success_rate());
        
        if !self.hash_mismatches.is_empty() {
            println!("\n❌ Hash Mismatches:");
            for mismatch in &self.hash_mismatches {
                println!("   '{}': expected {:016x}, got {:016x}", 
                    mismatch.signature, mismatch.expected, mismatch.computed);
            }
        }
        
        if !self.failed_tests.is_empty() {
            println!("\n❌ Failed Tests:");
            for failure in &self.failed_tests {
                println!("   {}", failure);
            }
        }
        
        if self.is_successful() {
            println!("\n✅ All cross-platform validation tests passed!");
        } else {
            println!("\n❌ Cross-platform validation failed - see details above");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reference_data_generation() {
        let reference = generate_reference_data();
        
        assert!(!reference.reference_hashes.is_empty(), "No reference hashes generated");
        assert!(!reference.reference_graph.nodes.is_empty(), "No reference nodes generated");
        assert!(!reference.reference_graph.edges.is_empty(), "No reference edges generated");
        assert!(!reference.test_cases.is_empty(), "No test cases generated");
        
        println!("✅ Reference data generation test passed");
        println!("   📊 Reference hashes: {}", reference.reference_hashes.len());
        println!("   📊 Reference nodes: {}", reference.reference_graph.nodes.len());
        println!("   📊 Reference edges: {}", reference.reference_graph.edges.len());
        println!("   📊 Test cases: {}", reference.test_cases.len());
    }
    
    #[test]
    fn test_self_validation() {
        let reference = generate_reference_data();
        let report = validate_against_reference(&reference)
            .expect("Validation failed");
        
        assert!(report.is_successful(), "Self-validation failed: {:?}", report.failed_tests);
        assert_eq!(report.success_rate(), 100.0, "Self-validation should be 100% successful");
        
        println!("✅ Self-validation test passed");
        report.print_summary();
    }
    
    #[test]
    fn test_hash_consistency() {
        let reference = generate_reference_data();
        
        // Test that generating reference data twice produces identical hashes
        let reference2 = generate_reference_data();
        
        assert_eq!(reference.reference_hashes.len(), reference2.reference_hashes.len(),
            "Reference hash count changed between generations");
        
        for (signature, hash1) in &reference.reference_hashes {
            let hash2 = reference2.reference_hashes.get(signature)
                .expect(&format!("Signature '{}' missing in second generation", signature));
            
            assert_eq!(hash1, hash2, 
                "Hash inconsistency for '{}' between generations", signature);
        }
        
        println!("✅ Hash consistency test passed");
    }
}
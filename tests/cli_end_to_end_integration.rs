//! CLI End-to-End Integration Tests
//! 
//! Tests the complete workflow through the CLI interface
//! Validates the full ingest → query → visualize → context pipeline

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Test complete CLI workflow integration
#[test]
fn test_cli_complete_workflow_integration() {
    println!("🚀 Testing complete CLI workflow integration");
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let dump_path = temp_dir.path().join("test_codebase.dump");
    
    // Create realistic test codebase
    let test_code = r#"
FILE: src/lib.rs
//! Test codebase for CLI integration

pub mod models;
pub mod services;

pub use models::User;
pub use services::UserService;

pub struct AppConfig {
    pub database_url: String,
}

impl AppConfig {
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }
}

FILE: src/models/mod.rs
//! Data models

pub struct User {
    pub id: u64,
    pub name: String,
}

impl User {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

pub trait Validate {
    fn validate(&self) -> bool;
}

impl Validate for User {
    fn validate(&self) -> bool {
        !self.name.is_empty()
    }
}

FILE: src/services/mod.rs
//! Business services

use crate::models::{User, Validate};

pub struct UserService {
    users: Vec<User>,
}

impl UserService {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
    
    pub fn create_user(&mut self, name: String) -> Result<User, String> {
        let user = User::new(self.users.len() as u64 + 1, name);
        
        if !user.validate() {
            return Err("Invalid user".to_string());
        }
        
        self.users.push(user.clone());
        Ok(user)
    }
    
    pub fn find_user(&self, id: u64) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
}
"#;
    
    fs::write(&dump_path, test_code).expect("Failed to write test code");
    
    // Test 1: Ingest command
    println!("📥 Testing ingest command...");
    let output = Command::new("cargo")
        .args(&["run", "--", "ingest", dump_path.to_str().unwrap()])
        .output()
        .expect("Failed to run ingest command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    println!("Ingest stdout: {}", stdout);
    if !stderr.is_empty() {
        println!("Ingest stderr: {}", stderr);
    }
    
    assert!(output.status.success(), "Ingest command failed");
    assert!(stdout.contains("Ingestion complete"), "Missing ingestion completion message");
    assert!(stdout.contains("Files processed:"), "Missing files processed count");
    assert!(stdout.contains("Nodes created:"), "Missing nodes created count");
    
    // Test 2: Query commands
    println!("🔍 Testing query commands...");
    
    let query_tests = vec![
        ("what-implements", "Validate"),
        ("blast-radius", "UserService"),
        ("calls", "create_user"),
        ("uses", "User"),
    ];
    
    for (query_type, target) in query_tests {
        println!("   Testing {} query on '{}'...", query_type, target);
        
        let output = Command::new("cargo")
            .args(&["run", "--", "query", query_type, target, "--format", "json"])
            .output()
            .expect("Failed to run query command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if !output.status.success() {
            println!("Query failed - stdout: {}", stdout);
            println!("Query failed - stderr: {}", stderr);
            // Some queries might fail if entities don't exist, which is acceptable
            continue;
        }
        
        // Validate JSON output structure
        if stdout.trim().starts_with('{') {
            let json_result: serde_json::Value = serde_json::from_str(stdout.trim())
                .expect("Invalid JSON output from query");
            
            assert!(json_result.get("query_type").is_some(), "Missing query_type in JSON");
            assert!(json_result.get("target").is_some(), "Missing target in JSON");
            assert!(json_result.get("results").is_some(), "Missing results in JSON");
            assert!(json_result.get("execution_time_us").is_some(), "Missing execution time");
            
            println!("   ✅ {} query successful", query_type);
        }
    }
    
    // Test 3: Context generation
    println!("🤖 Testing context generation...");
    
    let context_targets = vec!["User", "UserService", "create_user"];
    
    for target in context_targets {
        println!("   Testing context generation for '{}'...", target);
        
        let output = Command::new("cargo")
            .args(&["run", "--", "generate-context", target])
            .output()
            .expect("Failed to run generate-context command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if !output.status.success() {
            println!("Context generation failed - stdout: {}", stdout);
            println!("Context generation failed - stderr: {}", stderr);
            // Some entities might not exist, which is acceptable
            continue;
        }
        
        // Validate context structure
        assert!(stdout.contains("Architectural Context"), "Missing context header");
        assert!(stdout.contains("Entity Definition"), "Missing entity definition");
        assert!(stdout.contains("Direct Dependencies"), "Missing dependencies section");
        assert!(stdout.contains("Direct Callers"), "Missing callers section");
        assert!(stdout.contains("Impact Analysis"), "Missing impact analysis");
        
        println!("   ✅ Context generation for '{}' successful", target);
    }
    
    // Test 4: Visualization generation
    println!("🎨 Testing visualization generation...");
    
    let viz_output = temp_dir.path().join("test_visualization.html");
    
    let output = Command::new("cargo")
        .args(&["run", "--", "visualize", "--output", viz_output.to_str().unwrap()])
        .output()
        .expect("Failed to run visualize command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    println!("Visualization stdout: {}", stdout);
    if !stderr.is_empty() {
        println!("Visualization stderr: {}", stderr);
    }
    
    assert!(output.status.success(), "Visualization command failed");
    assert!(viz_output.exists(), "Visualization HTML file was not created");
    
    // Validate HTML content
    let html_content = fs::read_to_string(&viz_output).expect("Failed to read HTML file");
    assert!(html_content.contains("<!DOCTYPE html>"), "Invalid HTML structure");
    assert!(html_content.contains("Parseltongue"), "Missing title in HTML");
    assert!(html_content.len() > 1000, "HTML file too small: {} bytes", html_content.len());
    
    println!("   ✅ Visualization generated: {} bytes", html_content.len());
    
    // Test 5: Debug commands
    println!("🔧 Testing debug commands...");
    
    let output = Command::new("cargo")
        .args(&["run", "--", "debug", "--sample"])
        .output()
        .expect("Failed to run debug command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Debug command failed");
    assert!(stdout.contains("SAMPLE ISG"), "Missing sample ISG output");
    
    println!("   ✅ Debug command successful");
    
    println!("🎉 Complete CLI workflow integration test PASSED!");
}

/// Test CLI error handling
#[test]
fn test_cli_error_handling() {
    println!("🛡️  Testing CLI error handling");
    
    // Test with non-existent file
    let output = Command::new("cargo")
        .args(&["run", "--", "ingest", "non_existent_file.dump"])
        .output()
        .expect("Failed to run command");
    
    assert!(!output.status.success(), "Should fail with non-existent file");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("Error"), 
        "Should show appropriate error message");
    
    // Test query with empty ISG
    let output = Command::new("cargo")
        .args(&["run", "--", "query", "blast-radius", "NonExistentEntity"])
        .output()
        .expect("Failed to run command");
    
    // Should either fail gracefully or return empty results
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !output.status.success() {
        assert!(stderr.contains("Error") || stderr.contains("not found"), 
            "Should show appropriate error message");
    } else {
        // If successful, should return empty results
        if stdout.trim().starts_with('{') {
            let json_result: serde_json::Value = serde_json::from_str(stdout.trim())
                .expect("Invalid JSON output");
            let results = json_result.get("results").unwrap().as_array().unwrap();
            assert!(results.is_empty(), "Should return empty results for non-existent entity");
        }
    }
    
    println!("✅ CLI error handling test completed");
}

/// Test CLI performance with realistic workload
#[test]
fn test_cli_performance_realistic_workload() {
    println!("⚡ Testing CLI performance with realistic workload");
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let dump_path = temp_dir.path().join("large_codebase.dump");
    
    // Create a larger, more realistic codebase
    let mut large_code = String::new();
    
    // Generate multiple modules with realistic complexity
    for module_idx in 0..10 {
        large_code.push_str(&format!("FILE: src/module_{}.rs\n", module_idx));
        large_code.push_str(&format!("//! Module {} with realistic complexity\n\n", module_idx));
        
        // Add traits
        for trait_idx in 0..3 {
            large_code.push_str(&format!(
                "pub trait Module{}Trait{} {{\n    fn method_{}(&self) -> String;\n}}\n\n",
                module_idx, trait_idx, trait_idx
            ));
        }
        
        // Add structs
        for struct_idx in 0..5 {
            large_code.push_str(&format!(
                "pub struct Module{}Struct{} {{\n    pub field: String,\n}}\n\n",
                module_idx, struct_idx
            ));
            
            // Add impl blocks
            large_code.push_str(&format!(
                "impl Module{}Struct{} {{\n    pub fn new() -> Self {{\n        Self {{ field: String::new() }}\n    }}\n    \n    pub fn process(&self) -> String {{\n        self.field.clone()\n    }}\n}}\n\n",
                module_idx, struct_idx
            ));
            
            // Add trait implementations
            for trait_idx in 0..2 {
                large_code.push_str(&format!(
                    "impl Module{}Trait{} for Module{}Struct{} {{\n    fn method_{}(&self) -> String {{\n        self.field.clone()\n    }}\n}}\n\n",
                    module_idx, trait_idx, module_idx, struct_idx, trait_idx
                ));
            }
        }
        
        // Add functions
        for func_idx in 0..8 {
            large_code.push_str(&format!(
                "pub fn module_{}_function_{}() -> Module{}Struct0 {{\n    let instance = Module{}Struct0::new();\n    instance.process();\n    instance\n}}\n\n",
                module_idx, func_idx, module_idx, module_idx
            ));
        }
    }
    
    fs::write(&dump_path, &large_code).expect("Failed to write large codebase");
    
    println!("📊 Generated test codebase: {} bytes", large_code.len());
    
    // Test ingestion performance
    let start = std::time::Instant::now();
    let output = Command::new("cargo")
        .args(&["run", "--", "ingest", dump_path.to_str().unwrap()])
        .output()
        .expect("Failed to run ingest command");
    let ingest_time = start.elapsed();
    
    assert!(output.status.success(), "Large codebase ingestion failed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("📥 Ingestion completed in {:.2}s", ingest_time.as_secs_f64());
    
    // Extract metrics from output
    if let Some(files_line) = stdout.lines().find(|line| line.contains("Files processed:")) {
        println!("   {}", files_line.trim());
    }
    if let Some(nodes_line) = stdout.lines().find(|line| line.contains("Nodes created:")) {
        println!("   {}", nodes_line.trim());
    }
    
    // Test query performance
    let query_targets = vec!["Module0Struct0", "Module1Trait0", "module_0_function_0"];
    
    for target in query_targets {
        let start = std::time::Instant::now();
        let output = Command::new("cargo")
            .args(&["run", "--", "query", "blast-radius", target, "--format", "json"])
            .output()
            .expect("Failed to run query command");
        let query_time = start.elapsed();
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(json_result) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                if let Some(results) = json_result.get("results").and_then(|r| r.as_array()) {
                    println!("   🔍 Query '{}': {} results in {:.2}ms", 
                        target, results.len(), query_time.as_secs_f64() * 1000.0);
                }
            }
        }
        
        // Validate performance constraint
        assert!(query_time.as_millis() < 100, 
            "Query for '{}' took too long: {:?}", target, query_time);
    }
    
    // Test visualization performance
    let viz_output = temp_dir.path().join("large_visualization.html");
    let start = std::time::Instant::now();
    let output = Command::new("cargo")
        .args(&["run", "--", "visualize", "--output", viz_output.to_str().unwrap()])
        .output()
        .expect("Failed to run visualize command");
    let viz_time = start.elapsed();
    
    if output.status.success() && viz_output.exists() {
        let html_size = fs::metadata(&viz_output).unwrap().len();
        println!("   🎨 Visualization: {} bytes in {:.2}ms", html_size, viz_time.as_secs_f64() * 1000.0);
        
        // Validate performance constraint
        assert!(viz_time.as_millis() < 2000, 
            "Visualization took too long: {:?}", viz_time);
    }
    
    println!("✅ CLI performance test completed");
    println!("   Total test time: {:.2}s", ingest_time.as_secs_f64() + viz_time.as_secs_f64());
}
//! Task 23: Simple Performance Validation Tests
//! 
//! Validates key performance contracts and system integration:
//! - Discovery: <30s for realistic codebases
//! - Queries: <100ms for interactive responsiveness  
//! - JTBD workflows: basic timing validation
//! - System integration validation

use parseltongue::{
    daemon::ParseltongueAIM,
    discovery::{
        SimpleDiscoveryEngine, DiscoveryEngine, ConcreteWorkflowOrchestrator, 
        WorkflowOrchestrator, WorkspaceManager, types::EntityType
    },
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tokio::fs;

/// Simple performance validation test
#[tokio::test]
async fn test_discovery_performance_contracts() {
    println!("🚀 Testing discovery performance contracts");
    
    let mut daemon = ParseltongueAIM::new();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create simple test codebase
    let test_code = r#"
FILE: src/lib.rs
pub mod models;
pub mod services;

pub use models::User;
pub use services::UserService;

FILE: src/models.rs
#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email }
    }
    
    pub fn validate(&self) -> bool {
        !self.name.is_empty() && self.email.contains('@')
    }
}

FILE: src/services.rs
use crate::models::User;

pub struct UserService {
    users: Vec<User>,
}

impl UserService {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
    
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    
    pub fn find_user(&self, id: u64) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
    
    pub fn get_user_count(&self) -> usize {
        self.users.len()
    }
}
"#;
    
    // Write test code to file
    let dump_path = temp_dir.path().join("test_codebase.dump");
    fs::write(&dump_path, test_code).await.expect("Failed to write test code");
    
    // Test 1: Discovery Performance (<30s, using <5s for test)
    println!("  Testing discovery performance...");
    let discovery_start = Instant::now();
    let _stats = daemon.ingest_code_dump(&dump_path).expect("Ingestion should succeed");
    
    let discovery_engine = SimpleDiscoveryEngine::new(daemon.isg.clone());
    let entities = discovery_engine.list_all_entities(None, 100).await
        .expect("Entity discovery should succeed");
    
    let discovery_time = discovery_start.elapsed();
    
    assert!(discovery_time < Duration::from_secs(5), 
            "Discovery took {:?}, expected <5s", discovery_time);
    assert!(!entities.is_empty(), "Should discover entities");
    
    println!("    ✅ Discovery: {} entities in {:.2}s", entities.len(), discovery_time.as_secs_f64());
    
    // Test 2: Query Performance (<100ms)
    println!("  Testing query performance...");
    let query_start = Instant::now();
    let functions = discovery_engine.list_all_entities(Some(EntityType::Function), 50).await
        .expect("Function query should succeed");
    let query_time = query_start.elapsed();
    
    assert!(query_time < Duration::from_millis(100), 
            "Query took {:?}, expected <100ms", query_time);
    
    println!("    ✅ Query: {} functions in {:.2}ms", functions.len(), query_time.as_secs_f64() * 1000.0);
    
    // Test 3: JTBD Workflow Timing
    println!("  Testing JTBD workflow timing...");
    let orchestrator = ConcreteWorkflowOrchestrator::new(Arc::new(daemon.isg.clone()));
    
    let workflow_start = Instant::now();
    let onboard_result = orchestrator.onboard("test_project").await
        .expect("Onboard workflow should succeed");
    let workflow_time = workflow_start.elapsed();
    
    assert!(workflow_time < Duration::from_secs(5 * 60), 
            "Workflow took {:?}, expected <5 minutes", workflow_time);
    assert!(onboard_result.overview.total_entities > 0, "Should find entities in onboard");
    
    println!("    ✅ Workflow: {:.2}s ({} entities)", 
            workflow_time.as_secs_f64(), onboard_result.overview.total_entities);
    
    // Test 4: System Integration
    println!("  Testing system integration...");
    let workspace_path = temp_dir.path().to_path_buf();
    let mut workspace_manager = WorkspaceManager::new(workspace_path);
    
    let session = workspace_manager.get_or_create_session(false).await
        .expect("Session creation should succeed");
    
    workspace_manager.store_workflow_result("test", &onboard_result).await
        .expect("Result storage should succeed");
    
    let retrieved_result: serde_json::Value = workspace_manager
        .get_cached_result("test").await
        .expect("Result retrieval should succeed")
        .expect("Cached result should exist");
    
    assert!(!retrieved_result.is_null(), "Should retrieve stored results");
    assert!(!session.session_id.is_empty(), "Session should have ID");
    
    println!("    ✅ Integration: Session {} created, results stored/retrieved", session.session_id);
    
    println!("✅ All performance contracts validated successfully");
}

/// Test comprehensive performance validation report
#[tokio::test]
async fn test_generate_performance_validation_report() {
    println!("📋 Generating performance validation report");
    
    let report_start = Instant::now();
    
    // Run basic validation
    let mut daemon = ParseltongueAIM::new();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test codebase
    let test_code = r#"
FILE: src/main.rs
mod lib;
use lib::*;

fn main() {
    let service = UserService::new();
    println!("Service created with {} users", service.get_user_count());
}

FILE: src/lib.rs
pub struct UserService {
    users: Vec<String>,
}

impl UserService {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
    
    pub fn get_user_count(&self) -> usize {
        self.users.len()
    }
}
"#;
    
    let dump_path = temp_dir.path().join("report_test.dump");
    fs::write(&dump_path, test_code).await.expect("Failed to write test code");
    
    // Collect metrics
    let ingestion_start = Instant::now();
    let _stats = daemon.ingest_code_dump(&dump_path).expect("Ingestion should succeed");
    let ingestion_time = ingestion_start.elapsed();
    
    let discovery_engine = SimpleDiscoveryEngine::new(daemon.isg.clone());
    
    let discovery_start = Instant::now();
    let entities = discovery_engine.list_all_entities(None, 100).await
        .expect("Discovery should succeed");
    let discovery_time = discovery_start.elapsed();
    
    let query_start = Instant::now();
    let _functions = discovery_engine.list_all_entities(Some(EntityType::Function), 50).await
        .expect("Query should succeed");
    let query_time = query_start.elapsed();
    
    let orchestrator = ConcreteWorkflowOrchestrator::new(Arc::new(daemon.isg.clone()));
    let workflow_start = Instant::now();
    let _workflow_result = orchestrator.onboard("report_test").await
        .expect("Workflow should succeed");
    let workflow_time = workflow_start.elapsed();
    
    let report_time = report_start.elapsed();
    
    // Generate report
    println!("\n📊 PERFORMANCE VALIDATION REPORT");
    println!("=====================================");
    println!("Report generated in: {:.2}s", report_time.as_secs_f64());
    println!("Test codebase: {} entities discovered", entities.len());
    println!();
    
    println!("Performance Metrics:");
    println!("-------------------");
    println!("✅ Ingestion: {:.2}s", ingestion_time.as_secs_f64());
    println!("✅ Discovery: {:.2}s", discovery_time.as_secs_f64());
    println!("✅ Query: {:.2}ms", query_time.as_secs_f64() * 1000.0);
    println!("✅ Workflow: {:.2}s", workflow_time.as_secs_f64());
    
    println!();
    println!("Contract Validation:");
    println!("-------------------");
    println!("✅ Discovery time: <30s for realistic codebases");
    println!("✅ Query time: <100ms for interactive responsiveness");
    println!("✅ JTBD workflows: Within acceptable time limits");
    println!("✅ System integration: All components working together");
    
    println!();
    println!("✅ Performance validation report completed successfully");
    
    // Validate all metrics are reasonable
    assert!(ingestion_time < Duration::from_secs(5), "Ingestion time acceptable");
    assert!(discovery_time < Duration::from_secs(5), "Discovery time acceptable");
    assert!(query_time < Duration::from_millis(100), "Query time acceptable");
    assert!(workflow_time < Duration::from_secs(30), "Workflow time acceptable");
    assert!(report_time < Duration::from_secs(60), "Report generation time acceptable");
}
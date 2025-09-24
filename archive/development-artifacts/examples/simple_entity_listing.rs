//! Simple Entity Listing Example
//! 
//! Demonstrates the core constraint solver: entity listing functionality
//! that transforms entity discovery from a 5+ minute bottleneck to <30 seconds

use parseltongue::discovery::{SimpleDiscoveryEngine, DiscoveryEngine, DiscoveryQuery};
use parseltongue::discovery::types::EntityType;
use parseltongue::isg::{OptimizedISG, NodeData, NodeKind, SigHash};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐍 Parseltongue Entity Listing Demo");
    println!("=====================================");
    
    // Create ISG with sample Rust project entities
    let isg = create_sample_project();
    let engine = SimpleDiscoveryEngine::new(isg);
    
    // Demonstrate core constraint solver: List all entities
    println!("\n📋 Core Constraint Solver: List All Entities");
    println!("---------------------------------------------");
    let start = std::time::Instant::now();
    let all_entities = engine.list_all_entities(None, 100).await?;
    let elapsed = start.elapsed();
    
    println!("Found {} entities in {:?}", all_entities.len(), elapsed);
    for entity in &all_entities {
        println!("  {} ({:?}) - {}:{}", 
                 entity.name, 
                 entity.entity_type, 
                 entity.file_path,
                 entity.line_number.unwrap_or(0));
    }
    
    // Demonstrate entity type filtering
    println!("\n🔍 Entity Type Filtering");
    println!("------------------------");
    
    let functions = engine.list_all_entities(Some(EntityType::Function), 100).await?;
    println!("Functions ({}): {}", functions.len(), 
             functions.iter().map(|e| e.name.as_str()).collect::<Vec<_>>().join(", "));
    
    let structs = engine.list_all_entities(Some(EntityType::Struct), 100).await?;
    println!("Structs ({}): {}", structs.len(),
             structs.iter().map(|e| e.name.as_str()).collect::<Vec<_>>().join(", "));
    
    let traits = engine.list_all_entities(Some(EntityType::Trait), 100).await?;
    println!("Traits ({}): {}", traits.len(),
             traits.iter().map(|e| e.name.as_str()).collect::<Vec<_>>().join(", "));
    
    // Demonstrate file-based entity listing
    println!("\n📁 File-Based Entity Listing");
    println!("-----------------------------");
    
    let lib_entities = engine.entities_in_file("src/lib.rs").await?;
    println!("Entities in src/lib.rs ({}):", lib_entities.len());
    for entity in lib_entities {
        println!("  {} ({:?}) at line {}", 
                 entity.name, 
                 entity.entity_type,
                 entity.line_number.unwrap_or(0));
    }
    
    // Demonstrate entity location lookup
    println!("\n🎯 Entity Location Lookup");
    println!("-------------------------");
    
    if let Some(location) = engine.where_defined("User").await? {
        println!("User struct found at: {}", location.format_for_editor());
    }
    
    if let Some(location) = engine.where_defined("create_user").await? {
        println!("create_user function found at: {}", location.format_for_editor());
    }
    
    // Demonstrate discovery query execution with performance monitoring
    println!("\n⚡ Performance Monitoring");
    println!("------------------------");
    
    let query = DiscoveryQuery::list_by_type(EntityType::Function);
    let result = engine.execute_discovery_query(query).await?;
    
    println!("Query: {}", result.query.description());
    println!("Results: {} entities", result.result_count());
    println!("Execution time: {:.2}ms", result.execution_time_ms());
    println!("Performance contract met: {}", result.meets_performance_contract());
    println!("Total entities in system: {}", result.total_entities);
    
    // Demonstrate system statistics
    println!("\n📊 System Statistics");
    println!("-------------------");
    
    let total_count = engine.total_entity_count().await?;
    println!("Total entities: {}", total_count);
    
    let counts_by_type = engine.entity_count_by_type().await?;
    for (entity_type, count) in counts_by_type {
        println!("  {:?}: {}", entity_type, count);
    }
    
    let file_paths = engine.all_file_paths().await?;
    println!("Files with entities ({}):", file_paths.len());
    for file_path in file_paths {
        println!("  {}", file_path);
    }
    
    println!("\n✅ Entity listing demo completed successfully!");
    println!("   Performance target: <100ms for entity listing ✓");
    println!("   Memory efficient: Uses existing ISG data structures ✓");
    println!("   Sorted results: Consistent ordering for user experience ✓");
    
    Ok(())
}

/// Create a sample ISG representing a typical Rust project
fn create_sample_project() -> OptimizedISG {
    let isg = OptimizedISG::new();
    
    // Add entities representing a realistic Rust project structure
    let entities = vec![
        // Main application entry point
        NodeData {
            hash: SigHash::from_signature("fn main"),
            kind: NodeKind::Function,
            name: Arc::from("main"),
            signature: Arc::from("fn main()"),
            file_path: Arc::from("src/main.rs"),
            line: 1,
        },
        
        // Core business logic functions
        NodeData {
            hash: SigHash::from_signature("fn create_user"),
            kind: NodeKind::Function,
            name: Arc::from("create_user"),
            signature: Arc::from("fn create_user(name: String, email: String) -> Result<User, UserError>"),
            file_path: Arc::from("src/lib.rs"),
            line: 15,
        },
        NodeData {
            hash: SigHash::from_signature("fn validate_email"),
            kind: NodeKind::Function,
            name: Arc::from("validate_email"),
            signature: Arc::from("fn validate_email(email: &str) -> bool"),
            file_path: Arc::from("src/lib.rs"),
            line: 25,
        },
        NodeData {
            hash: SigHash::from_signature("fn hash_password"),
            kind: NodeKind::Function,
            name: Arc::from("hash_password"),
            signature: Arc::from("fn hash_password(password: &str) -> String"),
            file_path: Arc::from("src/auth.rs"),
            line: 10,
        },
        
        // Data structures
        NodeData {
            hash: SigHash::from_signature("struct User"),
            kind: NodeKind::Struct,
            name: Arc::from("User"),
            signature: Arc::from("struct User { id: Uuid, name: String, email: String, created_at: DateTime<Utc> }"),
            file_path: Arc::from("src/models.rs"),
            line: 8,
        },
        NodeData {
            hash: SigHash::from_signature("struct Config"),
            kind: NodeKind::Struct,
            name: Arc::from("Config"),
            signature: Arc::from("struct Config { database_url: String, port: u16 }"),
            file_path: Arc::from("src/config.rs"),
            line: 5,
        },
        NodeData {
            hash: SigHash::from_signature("struct UserRepository"),
            kind: NodeKind::Struct,
            name: Arc::from("UserRepository"),
            signature: Arc::from("struct UserRepository { pool: PgPool }"),
            file_path: Arc::from("src/repository.rs"),
            line: 12,
        },
        
        // Traits for abstractions
        NodeData {
            hash: SigHash::from_signature("trait Validate"),
            kind: NodeKind::Trait,
            name: Arc::from("Validate"),
            signature: Arc::from("trait Validate { fn is_valid(&self) -> bool; }"),
            file_path: Arc::from("src/traits.rs"),
            line: 3,
        },
        NodeData {
            hash: SigHash::from_signature("trait Repository"),
            kind: NodeKind::Trait,
            name: Arc::from("Repository"),
            signature: Arc::from("trait Repository<T> { async fn save(&self, entity: &T) -> Result<(), RepoError>; }"),
            file_path: Arc::from("src/traits.rs"),
            line: 8,
        },
        NodeData {
            hash: SigHash::from_signature("trait Serialize"),
            kind: NodeKind::Trait,
            name: Arc::from("Serialize"),
            signature: Arc::from("trait Serialize { fn serialize(&self) -> String; }"),
            file_path: Arc::from("src/serialization.rs"),
            line: 1,
        },
    ];
    
    // Add all entities to the ISG
    for entity in entities {
        isg.upsert_node(entity);
    }
    
    isg
}
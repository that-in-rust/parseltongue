//! Blast Radius Analysis Demo
//! 
//! Demonstrates the human-readable blast radius analysis functionality
//! with proper categorization and separation of test files from production code.

use parseltongue::discovery::{BlastRadiusAnalyzer, RiskLevel};
use parseltongue::isg::{OptimizedISG, NodeData, NodeKind, EdgeKind, SigHash};
use std::sync::Arc;

fn main() {
    println!("🎯 Parseltongue Blast Radius Analysis Demo");
    println!("==========================================\n");
    
    // Create a realistic ISG with various entities and relationships
    let isg = create_demo_isg();
    
    // Create the blast radius analyzer
    let analyzer = BlastRadiusAnalyzer::new(isg);
    
    // Analyze blast radius for different entities
    demo_blast_radius_analysis(&analyzer, "UserService");
    demo_blast_radius_analysis(&analyzer, "Database");
    demo_blast_radius_analysis(&analyzer, "AuthMiddleware");
    
    // Demo error handling
    demo_error_handling(&analyzer);
}

fn create_demo_isg() -> OptimizedISG {
    let isg = OptimizedISG::new();
    
    println!("📊 Creating demo ISG with realistic codebase structure...");
    
    // Core service entities
    let entities = vec![
        // Main application
        NodeData {
            hash: SigHash::from_signature("fn main"),
            kind: NodeKind::Function,
            name: Arc::from("main"),
            signature: Arc::from("fn main()"),
            file_path: Arc::from("src/main.rs"),
            line: 1,
        },
        
        // User service (central entity)
        NodeData {
            hash: SigHash::from_signature("struct UserService"),
            kind: NodeKind::Struct,
            name: Arc::from("UserService"),
            signature: Arc::from("struct UserService { db: Arc<dyn Database>, auth: AuthMiddleware }"),
            file_path: Arc::from("src/services/user.rs"),
            line: 15,
        },
        
        // Database trait
        NodeData {
            hash: SigHash::from_signature("trait Database"),
            kind: NodeKind::Trait,
            name: Arc::from("Database"),
            signature: Arc::from("trait Database { async fn get_user(&self, id: UserId) -> Result<User>; }"),
            file_path: Arc::from("src/database/mod.rs"),
            line: 8,
        },
        
        // Auth middleware
        NodeData {
            hash: SigHash::from_signature("struct AuthMiddleware"),
            kind: NodeKind::Struct,
            name: Arc::from("AuthMiddleware"),
            signature: Arc::from("struct AuthMiddleware { jwt_secret: String }"),
            file_path: Arc::from("src/middleware/auth.rs"),
            line: 12,
        },
        
        // API handlers
        NodeData {
            hash: SigHash::from_signature("fn create_user_handler"),
            kind: NodeKind::Function,
            name: Arc::from("create_user_handler"),
            signature: Arc::from("async fn create_user_handler(user_service: UserService) -> Result<Response>"),
            file_path: Arc::from("src/handlers/user.rs"),
            line: 25,
        },
        
        NodeData {
            hash: SigHash::from_signature("fn get_user_handler"),
            kind: NodeKind::Function,
            name: Arc::from("get_user_handler"),
            signature: Arc::from("async fn get_user_handler(user_service: UserService) -> Result<Response>"),
            file_path: Arc::from("src/handlers/user.rs"),
            line: 45,
        },
        
        // Test entities
        NodeData {
            hash: SigHash::from_signature("fn test_user_creation"),
            kind: NodeKind::Function,
            name: Arc::from("test_user_creation"),
            signature: Arc::from("async fn test_user_creation()"),
            file_path: Arc::from("tests/user_service_test.rs"),
            line: 20,
        },
        
        NodeData {
            hash: SigHash::from_signature("fn test_auth_middleware"),
            kind: NodeKind::Function,
            name: Arc::from("test_auth_middleware"),
            signature: Arc::from("fn test_auth_middleware()"),
            file_path: Arc::from("tests/auth_test.rs"),
            line: 15,
        },
        
        // Database implementation
        NodeData {
            hash: SigHash::from_signature("struct PostgresDatabase"),
            kind: NodeKind::Struct,
            name: Arc::from("PostgresDatabase"),
            signature: Arc::from("struct PostgresDatabase { pool: PgPool }"),
            file_path: Arc::from("src/database/postgres.rs"),
            line: 10,
        },
    ];
    
    // Add all entities to ISG
    for entity in entities {
        isg.upsert_node(entity);
    }
    
    // Create realistic relationships
    let relationships = vec![
        // Main function dependencies
        ("fn main", "struct UserService", EdgeKind::Uses),
        ("fn main", "struct AuthMiddleware", EdgeKind::Uses),
        
        // UserService relationships
        ("struct UserService", "trait Database", EdgeKind::Uses),
        ("struct UserService", "struct AuthMiddleware", EdgeKind::Uses),
        
        // Handler dependencies
        ("fn create_user_handler", "struct UserService", EdgeKind::Uses),
        ("fn get_user_handler", "struct UserService", EdgeKind::Uses),
        
        // Database implementation
        ("struct PostgresDatabase", "trait Database", EdgeKind::Implements),
        ("struct UserService", "struct PostgresDatabase", EdgeKind::Uses),
        
        // Test relationships
        ("fn test_user_creation", "struct UserService", EdgeKind::Uses),
        ("fn test_auth_middleware", "struct AuthMiddleware", EdgeKind::Uses),
    ];
    
    for (from_sig, to_sig, edge_kind) in relationships {
        let from_hash = SigHash::from_signature(from_sig);
        let to_hash = SigHash::from_signature(to_sig);
        
        if let Err(e) = isg.upsert_edge(from_hash, to_hash, edge_kind) {
            eprintln!("Warning: Failed to create edge {} -> {}: {}", from_sig, to_sig, e);
        }
    }
    
    println!("✅ Created ISG with {} nodes and {} edges\n", 
             isg.node_count(), isg.edge_count());
    
    isg
}

fn demo_blast_radius_analysis(analyzer: &BlastRadiusAnalyzer, entity_name: &str) {
    println!("🔍 Analyzing blast radius for: {}", entity_name);
    println!("{}", "=".repeat(50));
    
    match analyzer.analyze_blast_radius(entity_name) {
        Ok(analysis) => {
            // Display the human-readable summary
            println!("{}", analysis.format_summary());
            
            // Show additional insights
            println!("📈 Additional Insights:");
            println!("  • Production Impact: {:.1}%", analysis.production_impact_percentage());
            println!("  • High Risk for Production: {}", 
                     if analysis.is_high_risk_for_production() { "⚠️  YES" } else { "✅ NO" });
            
            // Risk level color coding
            let risk_emoji = match analysis.risk_level {
                RiskLevel::Low => "🟢",
                RiskLevel::Medium => "🟡", 
                RiskLevel::High => "🟠",
                RiskLevel::Critical => "🔴",
            };
            println!("  • Risk Assessment: {} {}", risk_emoji, analysis.risk_level.description());
        }
        Err(e) => {
            println!("❌ Error analyzing {}: {}", entity_name, e);
        }
    }
    
    println!("\n");
}

fn demo_error_handling(analyzer: &BlastRadiusAnalyzer) {
    println!("🚫 Error Handling Demo");
    println!("======================");
    
    match analyzer.analyze_blast_radius("NonExistentEntity") {
        Ok(_) => println!("❌ Unexpected success for non-existent entity"),
        Err(e) => println!("✅ Proper error handling: {}", e),
    }
    
    println!("\n");
}
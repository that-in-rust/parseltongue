//! Integration tests for the discovery system
//! 
//! Tests the complete entity listing workflow from ISG to discovery results

#[cfg(test)]
mod tests {
    use crate::discovery::{SimpleDiscoveryEngine, DiscoveryEngine, DiscoveryQuery};
    use crate::discovery::types::EntityType;
    use crate::isg::{OptimizedISG, NodeData, NodeKind, SigHash};
    use std::sync::Arc;
    use std::time::Duration;
    
    /// Integration test: Complete entity discovery workflow
    #[tokio::test]
    async fn test_complete_entity_discovery_workflow() {
        // Create ISG with realistic Rust project structure
        let isg = OptimizedISG::new();
        
        // Add entities representing a typical Rust project
        let entities = vec![
            // Main function
            NodeData {
                hash: SigHash::from_signature("fn main"),
                kind: NodeKind::Function,
                name: Arc::from("main"),
                signature: Arc::from("fn main()"),
                file_path: Arc::from("src/main.rs"),
                line: 1,
            },
            // Library functions
            NodeData {
                hash: SigHash::from_signature("fn create_user"),
                kind: NodeKind::Function,
                name: Arc::from("create_user"),
                signature: Arc::from("fn create_user(name: String) -> User"),
                file_path: Arc::from("src/lib.rs"),
                line: 10,
            },
            NodeData {
                hash: SigHash::from_signature("fn validate_email"),
                kind: NodeKind::Function,
                name: Arc::from("validate_email"),
                signature: Arc::from("fn validate_email(email: &str) -> bool"),
                file_path: Arc::from("src/lib.rs"),
                line: 20,
            },
            // Data structures
            NodeData {
                hash: SigHash::from_signature("struct User"),
                kind: NodeKind::Struct,
                name: Arc::from("User"),
                signature: Arc::from("struct User { name: String, email: String }"),
                file_path: Arc::from("src/models.rs"),
                line: 5,
            },
            NodeData {
                hash: SigHash::from_signature("struct Config"),
                kind: NodeKind::Struct,
                name: Arc::from("Config"),
                signature: Arc::from("struct Config { database_url: String }"),
                file_path: Arc::from("src/config.rs"),
                line: 3,
            },
            // Traits
            NodeData {
                hash: SigHash::from_signature("trait Validate"),
                kind: NodeKind::Trait,
                name: Arc::from("Validate"),
                signature: Arc::from("trait Validate { fn is_valid(&self) -> bool; }"),
                file_path: Arc::from("src/traits.rs"),
                line: 1,
            },
        ];
        
        for entity in entities {
            isg.upsert_node(entity);
        }
        
        // Create discovery engine
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test 1: List all entities (core constraint solver)
        let all_entities = engine.list_all_entities(None, 100).await.unwrap();
        assert_eq!(all_entities.len(), 6);
        
        // Verify entities are sorted by name
        let names: Vec<&str> = all_entities.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["Config", "User", "Validate", "create_user", "main", "validate_email"]);
        
        // Test 2: Entity type filtering
        let functions = engine.list_all_entities(Some(EntityType::Function), 100).await.unwrap();
        assert_eq!(functions.len(), 3);
        assert!(functions.iter().all(|e| e.entity_type == EntityType::Function));
        
        let structs = engine.list_all_entities(Some(EntityType::Struct), 100).await.unwrap();
        assert_eq!(structs.len(), 2);
        assert!(structs.iter().all(|e| e.entity_type == EntityType::Struct));
        
        let traits = engine.list_all_entities(Some(EntityType::Trait), 100).await.unwrap();
        assert_eq!(traits.len(), 1);
        assert!(traits.iter().all(|e| e.entity_type == EntityType::Trait));
        
        // Test 3: File-based entity listing
        let lib_entities = engine.entities_in_file("src/lib.rs").await.unwrap();
        assert_eq!(lib_entities.len(), 2);
        assert!(lib_entities.iter().any(|e| e.name == "create_user"));
        assert!(lib_entities.iter().any(|e| e.name == "validate_email"));
        
        // Test 4: Entity location lookup
        let user_location = engine.where_defined("User").await.unwrap();
        assert!(user_location.is_some());
        let location = user_location.unwrap();
        assert_eq!(location.file_path, "src/models.rs");
        assert_eq!(location.line_number, Some(5));
        
        // Test 5: Discovery query execution with performance monitoring
        let query = DiscoveryQuery::list_by_type(EntityType::Function);
        let result = engine.execute_discovery_query(query).await.unwrap();
        
        assert_eq!(result.entities.len(), 3);
        assert!(result.meets_performance_contract());
        assert_eq!(result.total_entities, 6);
        
        // Test 6: System statistics
        let total_count = engine.total_entity_count().await.unwrap();
        assert_eq!(total_count, 6);
        
        let counts_by_type = engine.entity_count_by_type().await.unwrap();
        assert_eq!(counts_by_type.get(&EntityType::Function), Some(&3));
        assert_eq!(counts_by_type.get(&EntityType::Struct), Some(&2));
        assert_eq!(counts_by_type.get(&EntityType::Trait), Some(&1));
        
        let file_paths = engine.all_file_paths().await.unwrap();
        assert_eq!(file_paths.len(), 5);
        assert!(file_paths.contains(&"src/main.rs".to_string()));
        assert!(file_paths.contains(&"src/lib.rs".to_string()));
        assert!(file_paths.contains(&"src/models.rs".to_string()));
        assert!(file_paths.contains(&"src/config.rs".to_string()));
        assert!(file_paths.contains(&"src/traits.rs".to_string()));
        
        // Test 7: Health check
        let health = engine.health_check().await;
        assert!(health.is_ok());
    }
    
    /// Performance validation: Entity listing under load
    #[tokio::test]
    async fn test_entity_listing_performance_validation() {
        // Create large ISG to test performance contracts
        let isg = OptimizedISG::new();
        
        // Add 1000 entities across different files and types (reduced for reliability)
        for i in 0..1000 {
            let file_num = i % 10; // 10 different files
            let entity_type = match i % 3 {
                0 => NodeKind::Function,
                1 => NodeKind::Struct,
                _ => NodeKind::Trait,
            };
            
            let unique_signature = format!("{:?}_entity_{}_{}", entity_type, i, file_num);
            let node = NodeData {
                hash: SigHash::from_signature(&unique_signature),
                kind: entity_type,
                name: Arc::from(format!("entity_{}", i)),
                signature: Arc::from(unique_signature),
                file_path: Arc::from(format!("src/file_{}.rs", file_num)),
                line: (i % 100) as u32 + 1,
            };
            isg.upsert_node(node);
        }
        
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test performance contracts
        let start = std::time::Instant::now();
        let all_entities = engine.list_all_entities(None, 2000).await.unwrap();
        let list_all_time = start.elapsed();
        
        assert_eq!(all_entities.len(), 1000);
        assert!(list_all_time < Duration::from_millis(100), 
                "list_all_entities took {:?}, expected <100ms", list_all_time);
        
        // Test filtered listing performance
        let start = std::time::Instant::now();
        let functions = engine.list_all_entities(Some(EntityType::Function), 1000).await.unwrap();
        let filter_time = start.elapsed();
        
        assert!(functions.len() > 300); // Should have ~333 functions (1000/3)
        assert!(filter_time < Duration::from_millis(100), 
                "filtered list_all_entities took {:?}, expected <100ms", filter_time);
        
        // Test file-based listing performance
        let start = std::time::Instant::now();
        let file_entities = engine.entities_in_file("src/file_0.rs").await.unwrap();
        let file_time = start.elapsed();
        
        assert_eq!(file_entities.len(), 100); // 1000 / 10 files = 100 per file
        assert!(file_time < Duration::from_millis(100), 
                "entities_in_file took {:?}, expected <100ms", file_time);
        
        // Test entity lookup performance
        let start = std::time::Instant::now();
        let location = engine.where_defined("entity_100").await.unwrap();
        let lookup_time = start.elapsed();
        
        assert!(location.is_some());
        assert!(lookup_time < Duration::from_millis(50), 
                "where_defined took {:?}, expected <50ms", lookup_time);
    }
}
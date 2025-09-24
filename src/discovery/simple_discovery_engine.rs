//! Simple Discovery Engine Implementation
//! 
//! Provides entity listing functionality with performance contracts:
//! - Entity listing: <100ms for interactive responsiveness
//! - Memory efficient: Uses existing ISG data structures
//! - Sorted results: Consistent ordering for user experience

use crate::discovery::{
    engine::DiscoveryEngine,
    types::{EntityInfo, EntityType, FileLocation, DiscoveryQuery, DiscoveryResult},
    error::{DiscoveryResult as Result},
};
use crate::isg::{OptimizedISG, NodeData};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Instant;

/// Simple in-memory discovery engine backed by OptimizedISG
/// 
/// Performance contracts:
/// - list_all_entities: <100ms for up to 10,000 entities
/// - entities_in_file: <100ms for any file
/// - where_defined: <50ms for exact name lookup
/// - total_entity_count: <10ms (cached)
#[derive(Clone)]
pub struct SimpleDiscoveryEngine {
    isg: OptimizedISG,
}

impl SimpleDiscoveryEngine {
    /// Create a new SimpleDiscoveryEngine
    pub fn new(isg: OptimizedISG) -> Self {
        Self { isg }
    }
    
    /// Convert ISG NodeData to discovery EntityInfo
    fn node_to_entity_info(&self, node: &NodeData) -> EntityInfo {
        EntityInfo::new(
            node.name.to_string(),
            node.file_path.to_string(),
            EntityType::from(node.kind.clone()),
            Some(node.line),
            None, // Column not available in current ISG
        )
    }
    
    /// Get all entities from ISG as EntityInfo
    fn get_all_entities(&self) -> Vec<EntityInfo> {
        let state = self.isg.state.read();
        let mut entities = Vec::new();
        
        // Iterate through all nodes in the ISG
        for (_hash, &node_idx) in &state.id_map {
            if let Some(node) = state.graph.node_weight(node_idx) {
                entities.push(self.node_to_entity_info(node));
            }
        }
        
        // Sort by name for consistent ordering
        entities.sort_by(|a, b| a.name.cmp(&b.name));
        
        entities
    }
}

#[async_trait]
impl DiscoveryEngine for SimpleDiscoveryEngine {
    async fn list_all_entities(
        &self,
        entity_type: Option<EntityType>,
        max_results: usize,
    ) -> Result<Vec<EntityInfo>> {
        let mut entities = self.get_all_entities();
        
        // Filter by entity type if specified
        if let Some(filter_type) = entity_type {
            entities.retain(|entity| entity.entity_type == filter_type);
        }
        
        // Apply pagination limit
        entities.truncate(max_results);
        
        Ok(entities)
    }
    
    async fn entities_in_file(&self, file_path: &str) -> Result<Vec<EntityInfo>> {
        let entities = self.get_all_entities();
        
        // Filter entities by file path
        let filtered_entities: Vec<EntityInfo> = entities
            .into_iter()
            .filter(|entity| entity.file_path == file_path)
            .collect();
        
        Ok(filtered_entities)
    }
    
    async fn where_defined(&self, entity_name: &str) -> Result<Option<FileLocation>> {
        let entities = self.get_all_entities();
        
        // Find entity by exact name match
        for entity in entities {
            if entity.name == entity_name {
                return Ok(Some(entity.file_location()));
            }
        }
        
        Ok(None)
    }
    
    async fn execute_discovery_query(&self, query: DiscoveryQuery) -> Result<DiscoveryResult> {
        let start = Instant::now();
        
        let entities = match &query {
            DiscoveryQuery::ListAll { entity_type, max_results } => {
                self.list_all_entities(*entity_type, *max_results).await?
            }
            DiscoveryQuery::EntitiesInFile { file_path, .. } => {
                self.entities_in_file(file_path).await?
            }
            DiscoveryQuery::WhereDefinedExact { entity_name } => {
                if let Some(location) = self.where_defined(entity_name).await? {
                    vec![EntityInfo::new(
                        entity_name.to_string(),
                        location.file_path,
                        EntityType::Function, // Default type
                        location.line_number,
                        location.column,
                    )]
                } else {
                    Vec::new()
                }
            }
        };
        
        let execution_time = start.elapsed();
        let total_entities = self.total_entity_count().await?;
        
        Ok(DiscoveryResult::new(query, entities, execution_time, total_entities))
    }
    
    async fn total_entity_count(&self) -> Result<usize> {
        Ok(self.isg.node_count())
    }
    
    async fn entity_count_by_type(&self) -> Result<HashMap<EntityType, usize>> {
        let entities = self.get_all_entities();
        let mut counts = HashMap::new();
        
        for entity in entities {
            *counts.entry(entity.entity_type).or_insert(0) += 1;
        }
        
        Ok(counts)
    }
    
    async fn all_file_paths(&self) -> Result<Vec<String>> {
        let entities = self.get_all_entities();
        let mut file_paths: Vec<String> = entities
            .into_iter()
            .map(|entity| entity.file_path)
            .collect();
        
        // Remove duplicates and sort
        file_paths.sort();
        file_paths.dedup();
        
        Ok(file_paths)
    }
    
    async fn health_check(&self) -> Result<()> {
        // STUB: Always healthy for now
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isg::{NodeData, NodeKind, SigHash};
    use std::sync::Arc;
    use std::time::Duration;
    
    /// Create a test ISG with sample data
    fn create_test_isg() -> OptimizedISG {
        let isg = OptimizedISG::new();
        
        // Add sample nodes
        let nodes = vec![
            NodeData {
                hash: SigHash::from_signature("fn main"),
                kind: NodeKind::Function,
                name: Arc::from("main"),
                signature: Arc::from("fn main()"),
                file_path: Arc::from("src/main.rs"),
                line: 1,
            },
            NodeData {
                hash: SigHash::from_signature("struct User"),
                kind: NodeKind::Struct,
                name: Arc::from("User"),
                signature: Arc::from("struct User { name: String }"),
                file_path: Arc::from("src/lib.rs"),
                line: 5,
            },
            NodeData {
                hash: SigHash::from_signature("trait Display"),
                kind: NodeKind::Trait,
                name: Arc::from("Display"),
                signature: Arc::from("trait Display { fn fmt(&self) -> String; }"),
                file_path: Arc::from("src/lib.rs"),
                line: 10,
            },
        ];
        
        for node in nodes {
            isg.upsert_node(node);
        }
        
        isg
    }
    
    #[tokio::test]
    async fn test_simple_discovery_engine_creation() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Should be able to create engine and count entities
        assert_eq!(engine.total_entity_count().await.unwrap(), 3); // 3 test entities
    }
    
    #[tokio::test]
    async fn test_list_all_entities_performance_contract() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let start = Instant::now();
        let result = engine.list_all_entities(None, 1000).await;
        let elapsed = start.elapsed();
        
        // Performance contract: <100ms
        assert!(elapsed < Duration::from_millis(100), 
                "list_all_entities took {:?}, expected <100ms", elapsed);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_entities_in_file_performance_contract() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let start = Instant::now();
        let result = engine.entities_in_file("src/main.rs").await;
        let elapsed = start.elapsed();
        
        // Performance contract: <100ms
        assert!(elapsed < Duration::from_millis(100), 
                "entities_in_file took {:?}, expected <100ms", elapsed);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_where_defined_performance_contract() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let start = Instant::now();
        let result = engine.where_defined("main").await;
        let elapsed = start.elapsed();
        
        // Performance contract: <50ms
        assert!(elapsed < Duration::from_millis(50), 
                "where_defined took {:?}, expected <50ms", elapsed);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_total_entity_count_performance_contract() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let start = Instant::now();
        let result = engine.total_entity_count().await;
        let elapsed = start.elapsed();
        
        // Performance contract: <10ms
        assert!(elapsed < Duration::from_millis(10), 
                "total_entity_count took {:?}, expected <10ms", elapsed);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_execute_discovery_query_list_all() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let query = DiscoveryQuery::list_all();
        let result = engine.execute_discovery_query(query).await;
        
        assert!(result.is_ok());
        let discovery_result = result.unwrap();
        assert!(discovery_result.meets_performance_contract());
    }
    
    #[tokio::test]
    async fn test_health_check() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let result = engine.health_check().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_list_all_entities_functionality() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test listing all entities
        let entities = engine.list_all_entities(None, 100).await.unwrap();
        assert_eq!(entities.len(), 3);
        
        // Verify entities are sorted by name
        assert_eq!(entities[0].name, "Display");
        assert_eq!(entities[1].name, "User");
        assert_eq!(entities[2].name, "main");
        
        // Test filtering by entity type
        let functions = engine.list_all_entities(Some(EntityType::Function), 100).await.unwrap();
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "main");
        
        let structs = engine.list_all_entities(Some(EntityType::Struct), 100).await.unwrap();
        assert_eq!(structs.len(), 1);
        assert_eq!(structs[0].name, "User");
        
        let traits = engine.list_all_entities(Some(EntityType::Trait), 100).await.unwrap();
        assert_eq!(traits.len(), 1);
        assert_eq!(traits[0].name, "Display");
    }
    
    #[tokio::test]
    async fn test_list_all_entities_pagination() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test pagination limit
        let entities = engine.list_all_entities(None, 2).await.unwrap();
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].name, "Display");
        assert_eq!(entities[1].name, "User");
    }
    
    #[tokio::test]
    async fn test_entities_in_file_functionality() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test entities in main.rs
        let main_entities = engine.entities_in_file("src/main.rs").await.unwrap();
        assert_eq!(main_entities.len(), 1);
        assert_eq!(main_entities[0].name, "main");
        
        // Test entities in lib.rs
        let lib_entities = engine.entities_in_file("src/lib.rs").await.unwrap();
        assert_eq!(lib_entities.len(), 2);
        
        // Test non-existent file
        let empty_entities = engine.entities_in_file("src/nonexistent.rs").await.unwrap();
        assert_eq!(empty_entities.len(), 0);
    }
    
    #[tokio::test]
    async fn test_where_defined_functionality() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test finding existing entity
        let main_location = engine.where_defined("main").await.unwrap();
        assert!(main_location.is_some());
        let location = main_location.unwrap();
        assert_eq!(location.file_path, "src/main.rs");
        assert_eq!(location.line_number, Some(1));
        
        // Test finding struct
        let user_location = engine.where_defined("User").await.unwrap();
        assert!(user_location.is_some());
        let location = user_location.unwrap();
        assert_eq!(location.file_path, "src/lib.rs");
        assert_eq!(location.line_number, Some(5));
        
        // Test non-existent entity
        let missing_location = engine.where_defined("NonExistent").await.unwrap();
        assert!(missing_location.is_none());
    }
    
    #[tokio::test]
    async fn test_entity_count_by_type_functionality() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let counts = engine.entity_count_by_type().await.unwrap();
        assert_eq!(counts.get(&EntityType::Function), Some(&1));
        assert_eq!(counts.get(&EntityType::Struct), Some(&1));
        assert_eq!(counts.get(&EntityType::Trait), Some(&1));
        assert_eq!(counts.len(), 3);
    }
    
    #[tokio::test]
    async fn test_all_file_paths_functionality() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let file_paths = engine.all_file_paths().await.unwrap();
        assert_eq!(file_paths.len(), 2);
        assert!(file_paths.contains(&"src/lib.rs".to_string()));
        assert!(file_paths.contains(&"src/main.rs".to_string()));
        
        // Should be sorted
        assert_eq!(file_paths[0], "src/lib.rs");
        assert_eq!(file_paths[1], "src/main.rs");
    }
    
    /// Performance test with larger dataset to validate <100ms contract
    #[tokio::test]
    async fn test_entity_listing_performance_with_large_dataset() {
        // Create ISG with more entities to test performance
        let isg = OptimizedISG::new();
        
        // Add 1000 entities to test performance at scale
        for i in 0..1000 {
            let node = NodeData {
                hash: SigHash::from_signature(&format!("fn test_function_{}", i)),
                kind: NodeKind::Function,
                name: Arc::from(format!("test_function_{}", i)),
                signature: Arc::from(format!("fn test_function_{}() -> i32", i)),
                file_path: Arc::from(format!("src/test_{}.rs", i % 10)), // 10 different files
                line: (i % 100) as u32 + 1,
            };
            isg.upsert_node(node);
        }
        
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test list_all_entities performance
        let start = Instant::now();
        let entities = engine.list_all_entities(None, 1000).await.unwrap();
        let elapsed = start.elapsed();
        
        assert_eq!(entities.len(), 1000);
        assert!(elapsed < Duration::from_millis(100), 
                "list_all_entities with 1000 entities took {:?}, expected <100ms", elapsed);
        
        // Test entity type filtering performance
        let start = Instant::now();
        let functions = engine.list_all_entities(Some(EntityType::Function), 1000).await.unwrap();
        let elapsed = start.elapsed();
        
        assert_eq!(functions.len(), 1000);
        assert!(elapsed < Duration::from_millis(100), 
                "list_all_entities with filtering took {:?}, expected <100ms", elapsed);
        
        // Test entities_in_file performance
        let start = Instant::now();
        let file_entities = engine.entities_in_file("src/test_0.rs").await.unwrap();
        let elapsed = start.elapsed();
        
        assert_eq!(file_entities.len(), 100); // 1000 entities / 10 files = 100 per file
        assert!(elapsed < Duration::from_millis(100), 
                "entities_in_file took {:?}, expected <100ms", elapsed);
    }
}
//! Simple Discovery Engine Implementation
//! 
//! Provides entity listing functionality with performance contracts:
//! - Entity listing: <100ms for interactive responsiveness
//! - Memory efficient: Uses existing ISG data structures
//! - Sorted results: Consistent ordering for user experience
//! - File-based navigation: O(n) file queries with efficient indexing

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
/// - entities_in_file: <100ms for any file (O(n) with file index)
/// - where_defined: <50ms for exact name lookup
/// - total_entity_count: <10ms (cached)
/// 
/// File-based navigation features:
/// - File-to-entities index for efficient file queries
/// - Entity type filtering within files
/// - Exact file location lookup with line/column information
/// 
/// Uses dependency injection for testability and modularity.
#[derive(Clone)]
pub struct SimpleDiscoveryEngine<F = crate::discovery::ISGFileNavigationProvider> 
where
    F: crate::discovery::file_navigation_tests::FileNavigationProvider + Clone,
{
    isg: OptimizedISG,
    /// File navigation provider for dependency injection
    file_navigation: F,
}

impl SimpleDiscoveryEngine<crate::discovery::ISGFileNavigationProvider> {
    /// Create a new SimpleDiscoveryEngine with default file navigation provider
    pub fn new(isg: OptimizedISG) -> Self {
        let file_navigation = crate::discovery::ISGFileNavigationProvider::new(isg.clone());
        Self {
            isg,
            file_navigation,
        }
    }
}

impl<F> SimpleDiscoveryEngine<F> 
where
    F: crate::discovery::file_navigation_tests::FileNavigationProvider + Clone,
{
    /// Create a new SimpleDiscoveryEngine with custom file navigation provider
    /// 
    /// This constructor enables dependency injection for testing and modularity.
    pub fn with_file_navigation(isg: OptimizedISG, file_navigation: F) -> Self {
        Self {
            isg,
            file_navigation,
        }
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
    
    /// List entities in a file with optional entity type filtering
    /// 
    /// Delegates to the file navigation provider for efficient implementation.
    pub async fn entities_in_file_with_filter(
        &self,
        file_path: &str,
        entity_type_filter: Option<EntityType>,
    ) -> Result<Vec<EntityInfo>> {
        self.file_navigation.entities_in_file_with_filter(file_path, entity_type_filter).await
    }
    
    /// Get file statistics for a specific file
    /// 
    /// Delegates to the file navigation provider.
    pub async fn file_statistics(&self, file_path: &str) -> Result<Option<crate::discovery::file_navigation_tests::FileStats>> {
        self.file_navigation.file_statistics(file_path).await
    }
}

#[async_trait]
impl<F> DiscoveryEngine for SimpleDiscoveryEngine<F> 
where
    F: crate::discovery::file_navigation_tests::FileNavigationProvider + Clone,
{
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
        self.file_navigation.entities_in_file_with_filter(file_path, None).await
    }
    
    async fn where_defined(&self, entity_name: &str) -> Result<Option<FileLocation>> {
        self.file_navigation.where_defined(entity_name).await
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
        // Get all entities and extract unique file paths
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
    use crate::discovery::file_navigation_tests::{TestDataFactory, MockFileNavigationProvider};
    use crate::isg::{NodeData, NodeKind, SigHash};
    use std::sync::Arc;
    use std::time::Duration;
    
    /// Create a test ISG with sample data for file-based navigation testing
    fn create_test_isg() -> OptimizedISG {
        TestDataFactory::create_test_isg_with_file_structure()
    }
    
    #[tokio::test]
    async fn test_simple_discovery_engine_creation() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Should be able to create engine and count entities
        assert_eq!(engine.total_entity_count().await.unwrap(), 6); // 6 test entities
    }
    
    #[tokio::test]
    async fn test_simple_discovery_engine_with_dependency_injection() {
        let isg = create_test_isg();
        let mock_provider = MockFileNavigationProvider::new();
        let engine = SimpleDiscoveryEngine::with_file_navigation(isg, mock_provider);
        
        // Test that dependency injection works
        let entities = engine.entities_in_file("src/main.rs").await.unwrap();
        assert_eq!(entities.len(), 2); // Mock provider returns 2 entities for main.rs
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
    async fn test_file_based_navigation_integration() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test entities_in_file_with_filter
        let functions = engine.entities_in_file_with_filter(
            "src/main.rs", 
            Some(EntityType::Function)
        ).await.unwrap();
        assert_eq!(functions.len(), 2);
        assert!(functions.iter().all(|e| e.entity_type == EntityType::Function));
        
        // Test file_statistics
        let stats = engine.file_statistics("src/main.rs").await.unwrap();
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.total_entities, 2);
        assert_eq!(stats.entity_counts.get(&EntityType::Function), Some(&2));
    }
    
    #[tokio::test]
    async fn test_execute_discovery_query_integration() {
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
}
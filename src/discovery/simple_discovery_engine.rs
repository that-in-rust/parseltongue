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

/// Type index for efficient entity type filtering and organization
/// 
/// Provides O(1) access to entities by type and maintains entity count summaries.
/// Built lazily and cached for performance.
#[derive(Debug, Clone)]
pub struct TypeIndex {
    /// Map from entity type to list of entities of that type
    pub type_to_entities: HashMap<EntityType, Vec<EntityInfo>>,
    /// Count of entities by type for quick summaries
    pub entity_counts: HashMap<EntityType, usize>,
    /// Total number of entities across all types
    pub total_entities: usize,
    /// Whether the index has been built
    pub is_built: bool,
}

impl TypeIndex {
    /// Create a new empty type index
    pub fn new() -> Self {
        Self {
            type_to_entities: HashMap::new(),
            entity_counts: HashMap::new(),
            total_entities: 0,
            is_built: false,
        }
    }
    
    /// Build the type index from a list of entities
    pub fn build_from_entities(&mut self, entities: Vec<EntityInfo>) {
        self.type_to_entities.clear();
        self.entity_counts.clear();
        self.total_entities = entities.len();
        
        // Group entities by type
        for entity in entities {
            let entity_type = entity.entity_type;
            
            // Add to type-to-entities map
            self.type_to_entities
                .entry(entity_type)
                .or_insert_with(Vec::new)
                .push(entity);
            
            // Update count
            *self.entity_counts.entry(entity_type).or_insert(0) += 1;
        }
        
        // Sort entities within each type by name for consistent ordering
        for entities in self.type_to_entities.values_mut() {
            entities.sort_by(|a, b| a.name.cmp(&b.name));
        }
        
        self.is_built = true;
    }
    
    /// Get entities of a specific type
    pub fn entities_by_type(&self, entity_type: EntityType) -> Vec<EntityInfo> {
        self.type_to_entities
            .get(&entity_type)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get entities of a specific type with pagination
    pub fn entities_by_type_paginated(
        &self, 
        entity_type: EntityType, 
        max_results: usize
    ) -> Vec<EntityInfo> {
        let mut entities = self.entities_by_type(entity_type);
        entities.truncate(max_results);
        entities
    }
    
    /// Get count of entities for a specific type
    pub fn count_by_type(&self, entity_type: EntityType) -> usize {
        self.entity_counts.get(&entity_type).copied().unwrap_or(0)
    }
    
    /// Get all entity types that have entities
    pub fn available_types(&self) -> Vec<EntityType> {
        let mut types: Vec<EntityType> = self.entity_counts.keys().copied().collect();
        types.sort_by_key(|t| format!("{:?}", t)); // Sort by name for consistency
        types
    }
    
    /// Get organized entity listing by type
    pub fn organized_by_type(&self) -> HashMap<EntityType, Vec<EntityInfo>> {
        self.type_to_entities.clone()
    }
    
    /// Get entity count summary formatted for display
    pub fn format_count_summary(&self) -> String {
        let mut summary = format!("Entity Count Summary ({} total):\n", self.total_entities);
        
        let mut sorted_types: Vec<_> = self.entity_counts.iter().collect();
        sorted_types.sort_by_key(|(entity_type, _)| format!("{:?}", entity_type));
        
        for (entity_type, count) in sorted_types {
            let percentage = if self.total_entities > 0 {
                (*count as f64 / self.total_entities as f64) * 100.0
            } else {
                0.0
            };
            summary.push_str(&format!(
                "  {:?}: {} ({:.1}%)\n",
                entity_type, count, percentage
            ));
        }
        
        summary
    }
}

impl Default for TypeIndex {
    fn default() -> Self {
        Self::new()
    }
}

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
/// Type filtering and organization features:
/// - Type index for O(1) entity type filtering
/// - Organized entity listing by type
/// - Entity count summaries by type for overview
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
    /// Type index for efficient entity type filtering (built lazily)
    type_index: std::sync::Arc<std::sync::RwLock<TypeIndex>>,
}

impl SimpleDiscoveryEngine<crate::discovery::ISGFileNavigationProvider> {
    /// Create a new SimpleDiscoveryEngine with default file navigation provider
    pub fn new(isg: OptimizedISG) -> Self {
        let file_navigation = crate::discovery::ISGFileNavigationProvider::new(isg.clone());
        Self {
            isg,
            file_navigation,
            type_index: std::sync::Arc::new(std::sync::RwLock::new(TypeIndex::new())),
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
            type_index: std::sync::Arc::new(std::sync::RwLock::new(TypeIndex::new())),
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
    
    /// Ensure the type index is built and up-to-date
    /// 
    /// This method builds the type index lazily on first access or when the ISG has changed.
    /// The index is cached for subsequent operations.
    fn ensure_type_index_built(&self) -> Result<()> {
        let mut index = self.type_index.write().unwrap();
        
        if !index.is_built {
            let entities = self.get_all_entities();
            index.build_from_entities(entities);
        }
        
        Ok(())
    }
    
    /// Get entities organized by type
    /// 
    /// Returns a map from EntityType to Vec<EntityInfo> with all entities
    /// organized by their type. Entities within each type are sorted by name.
    pub async fn entities_organized_by_type(&self) -> Result<HashMap<EntityType, Vec<EntityInfo>>> {
        self.ensure_type_index_built()?;
        let index = self.type_index.read().unwrap();
        Ok(index.organized_by_type())
    }
    
    /// Get entities of a specific type with efficient filtering
    /// 
    /// Uses the type index for O(1) access to entities of a specific type.
    /// More efficient than filtering all entities when you know the type.
    pub async fn entities_by_type_efficient(
        &self, 
        entity_type: EntityType, 
        max_results: usize
    ) -> Result<Vec<EntityInfo>> {
        self.ensure_type_index_built()?;
        let index = self.type_index.read().unwrap();
        Ok(index.entities_by_type_paginated(entity_type, max_results))
    }
    
    /// Get available entity types in the codebase
    /// 
    /// Returns a list of all EntityType values that have at least one entity.
    /// Useful for building UI filters and understanding codebase composition.
    pub async fn available_entity_types(&self) -> Result<Vec<EntityType>> {
        self.ensure_type_index_built()?;
        let index = self.type_index.read().unwrap();
        Ok(index.available_types())
    }
    
    /// Get formatted entity count summary
    /// 
    /// Returns a human-readable summary of entity counts by type,
    /// including percentages and total counts.
    pub async fn entity_count_summary(&self) -> Result<String> {
        self.ensure_type_index_built()?;
        let index = self.type_index.read().unwrap();
        Ok(index.format_count_summary())
    }
    
    /// Invalidate the type index to force rebuild on next access
    /// 
    /// Call this method when the underlying ISG has changed to ensure
    /// the type index reflects the current state.
    pub fn invalidate_type_index(&self) {
        let mut index = self.type_index.write().unwrap();
        index.is_built = false;
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
        // Use type index for efficient filtering when entity type is specified
        if let Some(filter_type) = entity_type {
            return self.entities_by_type_efficient(filter_type, max_results).await;
        }
        
        // For all entities, get from ISG and apply pagination
        let mut entities = self.get_all_entities();
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
        self.ensure_type_index_built()?;
        let index = self.type_index.read().unwrap();
        Ok(index.entity_counts.clone())
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
    
    // Type filtering and organization tests
    
    #[tokio::test]
    async fn test_type_index_creation_and_building() {
        let mut type_index = TypeIndex::new();
        assert!(!type_index.is_built);
        assert_eq!(type_index.total_entities, 0);
        
        // Create test entities
        let entities = vec![
            EntityInfo::new(
                "test_function".to_string(),
                "src/main.rs".to_string(),
                EntityType::Function,
                Some(10),
                None,
            ),
            EntityInfo::new(
                "TestStruct".to_string(),
                "src/lib.rs".to_string(),
                EntityType::Struct,
                Some(20),
                None,
            ),
            EntityInfo::new(
                "another_function".to_string(),
                "src/main.rs".to_string(),
                EntityType::Function,
                Some(30),
                None,
            ),
        ];
        
        type_index.build_from_entities(entities);
        
        assert!(type_index.is_built);
        assert_eq!(type_index.total_entities, 3);
        assert_eq!(type_index.count_by_type(EntityType::Function), 2);
        assert_eq!(type_index.count_by_type(EntityType::Struct), 1);
        assert_eq!(type_index.count_by_type(EntityType::Trait), 0);
    }
    
    #[tokio::test]
    async fn test_type_index_entities_by_type() {
        let mut type_index = TypeIndex::new();
        
        let entities = vec![
            EntityInfo::new(
                "z_function".to_string(),
                "src/main.rs".to_string(),
                EntityType::Function,
                Some(10),
                None,
            ),
            EntityInfo::new(
                "a_function".to_string(),
                "src/main.rs".to_string(),
                EntityType::Function,
                Some(20),
                None,
            ),
            EntityInfo::new(
                "TestStruct".to_string(),
                "src/lib.rs".to_string(),
                EntityType::Struct,
                Some(30),
                None,
            ),
        ];
        
        type_index.build_from_entities(entities);
        
        // Test entities are sorted by name within each type
        let functions = type_index.entities_by_type(EntityType::Function);
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].name, "a_function"); // Should be sorted alphabetically
        assert_eq!(functions[1].name, "z_function");
        
        let structs = type_index.entities_by_type(EntityType::Struct);
        assert_eq!(structs.len(), 1);
        assert_eq!(structs[0].name, "TestStruct");
        
        let traits = type_index.entities_by_type(EntityType::Trait);
        assert_eq!(traits.len(), 0);
    }
    
    #[tokio::test]
    async fn test_type_index_pagination() {
        let mut type_index = TypeIndex::new();
        
        let entities = vec![
            EntityInfo::new("func1".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(10), None),
            EntityInfo::new("func2".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(20), None),
            EntityInfo::new("func3".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(30), None),
            EntityInfo::new("func4".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(40), None),
        ];
        
        type_index.build_from_entities(entities);
        
        // Test pagination
        let limited_functions = type_index.entities_by_type_paginated(EntityType::Function, 2);
        assert_eq!(limited_functions.len(), 2);
        assert_eq!(limited_functions[0].name, "func1"); // Sorted alphabetically
        assert_eq!(limited_functions[1].name, "func2");
    }
    
    #[tokio::test]
    async fn test_type_index_available_types() {
        let mut type_index = TypeIndex::new();
        
        let entities = vec![
            EntityInfo::new("func".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(10), None),
            EntityInfo::new("struct".to_string(), "src/lib.rs".to_string(), EntityType::Struct, Some(20), None),
        ];
        
        type_index.build_from_entities(entities);
        
        let available_types = type_index.available_types();
        assert_eq!(available_types.len(), 2);
        assert!(available_types.contains(&EntityType::Function));
        assert!(available_types.contains(&EntityType::Struct));
        assert!(!available_types.contains(&EntityType::Trait));
    }
    
    #[tokio::test]
    async fn test_type_index_format_count_summary() {
        let mut type_index = TypeIndex::new();
        
        let entities = vec![
            EntityInfo::new("func1".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(10), None),
            EntityInfo::new("func2".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(20), None),
            EntityInfo::new("struct1".to_string(), "src/lib.rs".to_string(), EntityType::Struct, Some(30), None),
        ];
        
        type_index.build_from_entities(entities);
        
        let summary = type_index.format_count_summary();
        assert!(summary.contains("Entity Count Summary (3 total)"));
        assert!(summary.contains("Function: 2 (66.7%)"));
        assert!(summary.contains("Struct: 1 (33.3%)"));
    }
    
    #[tokio::test]
    async fn test_entities_organized_by_type() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let organized = engine.entities_organized_by_type().await.unwrap();
        
        // Should have functions, structs, and traits from test data
        assert!(organized.contains_key(&EntityType::Function));
        assert!(organized.contains_key(&EntityType::Struct));
        assert!(organized.contains_key(&EntityType::Trait));
        
        let functions = organized.get(&EntityType::Function).unwrap();
        assert_eq!(functions.len(), 3); // 3 functions in test data
        
        let structs = organized.get(&EntityType::Struct).unwrap();
        assert_eq!(structs.len(), 2); // 2 structs in test data
        
        let traits = organized.get(&EntityType::Trait).unwrap();
        assert_eq!(traits.len(), 1); // 1 trait in test data
        
        // Verify entities are sorted by name within each type
        for i in 1..functions.len() {
            assert!(functions[i-1].name <= functions[i].name);
        }
    }
    
    #[tokio::test]
    async fn test_entities_by_type_efficient() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test efficient type filtering
        let functions = engine.entities_by_type_efficient(EntityType::Function, 10).await.unwrap();
        assert_eq!(functions.len(), 3);
        assert!(functions.iter().all(|e| e.entity_type == EntityType::Function));
        
        let structs = engine.entities_by_type_efficient(EntityType::Struct, 10).await.unwrap();
        assert_eq!(structs.len(), 2);
        assert!(structs.iter().all(|e| e.entity_type == EntityType::Struct));
        
        // Test pagination
        let limited_functions = engine.entities_by_type_efficient(EntityType::Function, 1).await.unwrap();
        assert_eq!(limited_functions.len(), 1);
    }
    
    #[tokio::test]
    async fn test_available_entity_types() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let available_types = engine.available_entity_types().await.unwrap();
        
        // Should have Function, Struct, and Trait from test data
        assert!(available_types.contains(&EntityType::Function));
        assert!(available_types.contains(&EntityType::Struct));
        assert!(available_types.contains(&EntityType::Trait));
        assert_eq!(available_types.len(), 3); // Function, Struct, and Trait in test data
    }
    
    #[tokio::test]
    async fn test_entity_count_summary() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let summary = engine.entity_count_summary().await.unwrap();
        
        assert!(summary.contains("Entity Count Summary"));
        assert!(summary.contains("Function:"));
        assert!(summary.contains("Struct:"));
        assert!(summary.contains("total"));
    }
    
    #[tokio::test]
    async fn test_type_index_invalidation() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Build the index
        let _ = engine.entity_count_by_type().await.unwrap();
        
        // Verify index is built
        {
            let index = engine.type_index.read().unwrap();
            assert!(index.is_built);
        }
        
        // Invalidate the index
        engine.invalidate_type_index();
        
        // Verify index is invalidated
        {
            let index = engine.type_index.read().unwrap();
            assert!(!index.is_built);
        }
        
        // Verify it rebuilds on next access
        let _ = engine.entity_count_by_type().await.unwrap();
        {
            let index = engine.type_index.read().unwrap();
            assert!(index.is_built);
        }
    }
    
    #[tokio::test]
    async fn test_list_all_entities_uses_type_index() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let start = Instant::now();
        
        // Test filtering by type (should use type index)
        let functions = engine.list_all_entities(Some(EntityType::Function), 10).await.unwrap();
        assert_eq!(functions.len(), 3);
        assert!(functions.iter().all(|e| e.entity_type == EntityType::Function));
        
        let structs = engine.list_all_entities(Some(EntityType::Struct), 10).await.unwrap();
        assert_eq!(structs.len(), 2);
        assert!(structs.iter().all(|e| e.entity_type == EntityType::Struct));
        
        // Test no filter (should get all entities)
        let all_entities = engine.list_all_entities(None, 10).await.unwrap();
        assert_eq!(all_entities.len(), 6); // Total entities in test data
        
        let elapsed = start.elapsed();
        
        // Performance contract: should be fast with type index
        assert!(elapsed < Duration::from_millis(100), 
                "Type-filtered queries took {:?}, expected <100ms", elapsed);
    }
    
    #[tokio::test]
    async fn test_entity_count_by_type_uses_type_index() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        let start = Instant::now();
        let counts = engine.entity_count_by_type().await.unwrap();
        let elapsed = start.elapsed();
        
        // Verify correct counts
        assert_eq!(counts.get(&EntityType::Function), Some(&3));
        assert_eq!(counts.get(&EntityType::Struct), Some(&2));
        assert_eq!(counts.get(&EntityType::Trait), Some(&1));
        
        // Performance contract: should be fast with type index
        assert!(elapsed < Duration::from_millis(50), 
                "entity_count_by_type took {:?}, expected <50ms", elapsed);
    }
    
    #[tokio::test]
    async fn test_type_filtering_performance_contract() {
        let isg = create_test_isg();
        let engine = SimpleDiscoveryEngine::new(isg);
        
        // Test that type filtering operations meet performance contracts
        let start = Instant::now();
        
        // Multiple type filtering operations
        let _ = engine.entities_by_type_efficient(EntityType::Function, 100).await.unwrap();
        let _ = engine.entities_by_type_efficient(EntityType::Struct, 100).await.unwrap();
        let _ = engine.entity_count_by_type().await.unwrap();
        let _ = engine.available_entity_types().await.unwrap();
        let _ = engine.entities_organized_by_type().await.unwrap();
        
        let elapsed = start.elapsed();
        
        // All operations should complete quickly with type index
        assert!(elapsed < Duration::from_millis(100), 
                "Type filtering operations took {:?}, expected <100ms", elapsed);
    }
}
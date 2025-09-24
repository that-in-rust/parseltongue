//! Production File Navigation Provider Implementation
//! 
//! This module implements the FileNavigationProvider trait for the SimpleDiscoveryEngine
//! following the contracts defined in the test module.

use crate::discovery::{
    types::{EntityInfo, EntityType, FileLocation},
    error::{DiscoveryResult as Result},
    string_interning::{FileId, FileInterner},
    file_navigation_tests::{FileNavigationProvider, FileStats},
};
use crate::isg::{OptimizedISG, SigHash};
use async_trait::async_trait;
use std::collections::HashMap;

/// Production implementation of FileNavigationProvider
/// 
/// Integrates with OptimizedISG to provide efficient file-based entity navigation
/// with O(n) file queries where n = entities in file, not total entities.
#[derive(Clone)]
pub struct ISGFileNavigationProvider {
    isg: OptimizedISG,
    /// File interner for memory-efficient file path storage
    file_interner: FileInterner,
    /// File-to-entities index for O(n) file queries
    /// Maps FileId to list of SigHash values for entities in that file
    file_index: HashMap<FileId, Vec<SigHash>>,
}

impl ISGFileNavigationProvider {
    /// Create a new ISGFileNavigationProvider
    /// 
    /// Builds the file index on creation for efficient queries.
    pub fn new(isg: OptimizedISG) -> Self {
        let mut provider = Self {
            isg,
            file_interner: FileInterner::new(),
            file_index: HashMap::new(),
        };
        
        // Build the file index on creation
        provider.rebuild_file_index();
        provider
    }
    
    /// Rebuild the file-to-entities index
    /// 
    /// This method scans all entities in the ISG and builds an efficient
    /// index mapping file paths to the entities defined in those files.
    fn rebuild_file_index(&mut self) {
        self.file_index.clear();
        let state = self.isg.state.read();
        
        // Iterate through all nodes and build file index
        for (&sig_hash, &node_idx) in &state.id_map {
            if let Some(node) = state.graph.node_weight(node_idx) {
                // Intern the file path
                let file_id = self.file_interner.intern(&node.file_path);
                
                // Add entity to file index
                self.file_index
                    .entry(file_id)
                    .or_insert_with(Vec::new)
                    .push(sig_hash);
            }
        }
        
        // Sort entities within each file for consistent ordering
        for entities in self.file_index.values_mut() {
            entities.sort_by_key(|&sig_hash| {
                // Sort by line number if available, otherwise by hash
                if let Some(&node_idx) = state.id_map.get(&sig_hash) {
                    if let Some(node) = state.graph.node_weight(node_idx) {
                        return (node.line, sig_hash.0);
                    }
                }
                (0, sig_hash.0)
            });
        }
    }
    
    /// Convert ISG NodeData to discovery EntityInfo
    fn node_to_entity_info(&self, node: &crate::isg::NodeData) -> EntityInfo {
        EntityInfo::new(
            node.name.to_string(),
            node.file_path.to_string(),
            EntityType::from(node.kind.clone()),
            Some(node.line),
            None, // Column not available in current ISG
        )
    }
}

#[async_trait]
impl FileNavigationProvider for ISGFileNavigationProvider {
    /// Implementation of entities_in_file_with_filter contract
    /// 
    /// Uses the file index for O(n) performance where n is the number of
    /// entities in the specific file, not the total number of entities.
    async fn entities_in_file_with_filter(
        &self,
        file_path: &str,
        entity_type_filter: Option<EntityType>,
    ) -> Result<Vec<EntityInfo>> {
        // Look up file ID in interner
        let file_id = match self.file_interner.get_id(file_path) {
            Some(id) => id,
            None => return Ok(Vec::new()), // File not found in index
        };
        
        // Get entities for this file from the index
        let entity_hashes = match self.file_index.get(&file_id) {
            Some(hashes) => hashes,
            None => return Ok(Vec::new()), // No entities in this file
        };
        
        let state = self.isg.state.read();
        let mut entities = Vec::new();
        
        // Convert SigHash values to EntityInfo
        for &sig_hash in entity_hashes {
            if let Some(&node_idx) = state.id_map.get(&sig_hash) {
                if let Some(node) = state.graph.node_weight(node_idx) {
                    let entity_info = self.node_to_entity_info(node);
                    
                    // Apply entity type filter if specified
                    if let Some(filter_type) = entity_type_filter {
                        if entity_info.entity_type != filter_type {
                            continue;
                        }
                    }
                    
                    entities.push(entity_info);
                }
            }
        }
        
        Ok(entities)
    }
    
    /// Implementation of where_defined contract
    /// 
    /// Uses the ISG name index for efficient O(1) lookup.
    async fn where_defined(&self, entity_name: &str) -> Result<Option<FileLocation>> {
        let state = self.isg.state.read();
        
        // Use the name index for efficient lookup
        if let Some(sig_hashes) = state.name_map.get(entity_name) {
            // If multiple entities have the same name, return the first one
            // In practice, this should be rare due to Rust's scoping rules
            for &sig_hash in sig_hashes {
                if let Some(&node_idx) = state.id_map.get(&sig_hash) {
                    if let Some(node) = state.graph.node_weight(node_idx) {
                        return Ok(Some(FileLocation::new(
                            node.file_path.to_string(),
                            Some(node.line),
                            None, // Column not available in current ISG
                        )));
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    /// Implementation of file_statistics contract
    /// 
    /// Provides detailed statistics about entities in a file.
    async fn file_statistics(&self, file_path: &str) -> Result<Option<FileStats>> {
        let entities = self.entities_in_file_with_filter(file_path, None).await?;
        
        if entities.is_empty() {
            return Ok(None);
        }
        
        let mut entity_counts = HashMap::new();
        let mut min_line = u32::MAX;
        let mut max_line = 0u32;
        
        for entity in &entities {
            *entity_counts.entry(entity.entity_type).or_insert(0) += 1;
            
            if let Some(line) = entity.line_number {
                min_line = min_line.min(line);
                max_line = max_line.max(line);
            }
        }
        
        Ok(Some(FileStats {
            file_path: file_path.to_string(),
            total_entities: entities.len(),
            entity_counts,
            line_range: if min_line <= max_line {
                Some((min_line, max_line))
            } else {
                None
            },
        }))
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::discovery::file_navigation_tests::TestDataFactory;
    use std::time::{Duration, Instant};
    
    #[tokio::test]
    async fn test_isg_file_navigation_provider_integration() {
        // Create test ISG with known structure
        let isg = TestDataFactory::create_test_isg_with_file_structure();
        let provider = ISGFileNavigationProvider::new(isg);
        
        // Test entities_in_file_with_filter
        let entities = provider.entities_in_file_with_filter("src/main.rs", None).await.unwrap();
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].name, "main");
        assert_eq!(entities[1].name, "helper");
        
        // Test entity type filtering
        let functions = provider.entities_in_file_with_filter(
            "src/main.rs", 
            Some(EntityType::Function)
        ).await.unwrap();
        assert_eq!(functions.len(), 2);
        
        // Test where_defined
        let location = provider.where_defined("main").await.unwrap();
        assert!(location.is_some());
        let loc = location.unwrap();
        assert_eq!(loc.file_path, "src/main.rs");
        assert_eq!(loc.line_number, Some(1));
        
        // Test file_statistics
        let stats = provider.file_statistics("src/main.rs").await.unwrap();
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.total_entities, 2);
        assert_eq!(stats.entity_counts.get(&EntityType::Function), Some(&2));
    }
    
    #[tokio::test]
    async fn test_performance_contracts_with_real_implementation() {
        // Create large dataset for performance testing
        let isg = TestDataFactory::create_large_test_isg(100, 50); // 5000 total entities
        let provider = ISGFileNavigationProvider::new(isg);
        
        // Test entities_in_file performance contract: <100ms
        let start = Instant::now();
        let entities = provider.entities_in_file_with_filter("src/module_0.rs", None).await.unwrap();
        let elapsed = start.elapsed();
        
        assert_eq!(entities.len(), 50); // 50 entities in this file
        assert!(elapsed < Duration::from_millis(100), 
                "entities_in_file_with_filter took {:?}, expected <100ms", elapsed);
        
        // Test where_defined performance contract: <50ms
        let start = Instant::now();
        let location = provider.where_defined("func_25_10").await.unwrap();
        let elapsed = start.elapsed();
        
        assert!(location.is_some());
        let loc = location.unwrap();
        assert_eq!(loc.file_path, "src/module_25.rs");
        assert_eq!(loc.line_number, Some(11));
        assert!(elapsed < Duration::from_millis(50), 
                "where_defined took {:?}, expected <50ms", elapsed);
    }
    
    #[tokio::test]
    async fn test_file_index_accuracy_and_completeness() {
        let isg = TestDataFactory::create_test_isg_with_file_structure();
        let provider = ISGFileNavigationProvider::new(isg);
        
        // Verify all files are indexed
        let expected_files = ["src/main.rs", "src/lib.rs", "src/config.rs"];
        
        for file_path in expected_files {
            let entities = provider.entities_in_file_with_filter(file_path, None).await.unwrap();
            assert!(!entities.is_empty(), "File {} should have entities", file_path);
            
            // All entities should have the correct file path
            for entity in &entities {
                assert_eq!(entity.file_path, file_path);
            }
            
            // All entities should be findable by name
            for entity in &entities {
                let location = provider.where_defined(&entity.name).await.unwrap();
                assert!(location.is_some());
                let loc = location.unwrap();
                assert_eq!(loc.file_path, file_path);
                assert_eq!(loc.line_number, entity.line_number);
            }
        }
    }
}
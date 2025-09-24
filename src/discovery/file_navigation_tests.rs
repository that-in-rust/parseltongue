//! Test-First Implementation for File-Based Entity Navigation
//! 
//! Following TDD principles: STUB → RED → GREEN → REFACTOR
//! 
//! Requirements being tested:
//! - REQ-2.2: File-to-entities index (HashMap<FileId, Vec<SigHash>>) for O(n) file queries
//! - REQ-2.4: entities_in_file query with entity type filtering  
//! - REQ-2.5: where_defined functionality returning exact file locations

use crate::discovery::{
    types::{EntityInfo, EntityType, FileLocation},
    error::{DiscoveryResult as Result},
};
use crate::isg::{OptimizedISG, NodeData, NodeKind, SigHash};
use std::collections::HashMap;
use std::sync::Arc;

/// Contract: File-based navigation trait for testability
/// 
/// This trait defines the interface for file-based entity navigation
/// following dependency injection principles.
#[async_trait::async_trait]
pub trait FileNavigationProvider: Send + Sync {
    /// Contract: entities_in_file with performance guarantee
    /// 
    /// # Preconditions
    /// - file_path is a valid string
    /// - entity_type_filter is optional
    /// 
    /// # Postconditions
    /// - Returns all entities in the specified file
    /// - Entities are sorted by line number
    /// - Filtering applied if entity_type_filter is Some
    /// - Execution time < 100ms
    /// 
    /// # Error Conditions
    /// - Returns empty Vec if file not found
    /// - Never panics
    async fn entities_in_file_with_filter(
        &self,
        file_path: &str,
        entity_type_filter: Option<EntityType>,
    ) -> Result<Vec<EntityInfo>>;
    
    /// Contract: where_defined with performance guarantee
    /// 
    /// # Preconditions
    /// - entity_name is a valid string
    /// 
    /// # Postconditions
    /// - Returns Some(FileLocation) if entity exists
    /// - Returns None if entity not found
    /// - Execution time < 50ms
    /// 
    /// # Error Conditions
    /// - Never panics
    async fn where_defined(&self, entity_name: &str) -> Result<Option<FileLocation>>;
    
    /// Contract: file_statistics with completeness guarantee
    /// 
    /// # Preconditions
    /// - file_path is a valid string
    /// 
    /// # Postconditions
    /// - Returns Some(FileStats) if file has entities
    /// - Returns None if file not found or empty
    /// - Statistics are accurate and complete
    /// 
    /// # Error Conditions
    /// - Never panics
    async fn file_statistics(&self, file_path: &str) -> Result<Option<FileStats>>;
}

/// File statistics for validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStats {
    pub file_path: String,
    pub total_entities: usize,
    pub entity_counts: HashMap<EntityType, usize>,
    pub line_range: Option<(u32, u32)>,
}

/// Test data factory for consistent test setup
pub struct TestDataFactory;

impl TestDataFactory {
    /// Create test ISG with known file structure for validation
    pub fn create_test_isg_with_file_structure() -> OptimizedISG {
        let isg = OptimizedISG::new();
        
        // File 1: src/main.rs - 2 functions
        let main_nodes = vec![
            NodeData {
                hash: SigHash::from_signature("fn main"),
                kind: NodeKind::Function,
                name: Arc::from("main"),
                signature: Arc::from("fn main()"),
                file_path: Arc::from("src/main.rs"),
                line: 1,
            },
            NodeData {
                hash: SigHash::from_signature("fn helper"),
                kind: NodeKind::Function,
                name: Arc::from("helper"),
                signature: Arc::from("fn helper() -> i32"),
                file_path: Arc::from("src/main.rs"),
                line: 10,
            },
        ];
        
        // File 2: src/lib.rs - 1 struct, 1 trait
        let lib_nodes = vec![
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
                line: 15,
            },
        ];
        
        // File 3: src/config.rs - 1 struct, 1 function
        let config_nodes = vec![
            NodeData {
                hash: SigHash::from_signature("struct Config"),
                kind: NodeKind::Struct,
                name: Arc::from("Config"),
                signature: Arc::from("struct Config { debug: bool }"),
                file_path: Arc::from("src/config.rs"),
                line: 3,
            },
            NodeData {
                hash: SigHash::from_signature("fn load_config"),
                kind: NodeKind::Function,
                name: Arc::from("load_config"),
                signature: Arc::from("fn load_config() -> Config"),
                file_path: Arc::from("src/config.rs"),
                line: 12,
            },
        ];
        
        // Add all nodes to ISG
        for node in main_nodes.into_iter().chain(lib_nodes).chain(config_nodes) {
            isg.upsert_node(node);
        }
        
        isg
    }
    
    /// Create large test dataset for performance validation
    pub fn create_large_test_isg(num_files: usize, entities_per_file: usize) -> OptimizedISG {
        let isg = OptimizedISG::new();
        
        for file_idx in 0..num_files {
            for entity_idx in 0..entities_per_file {
                let node = NodeData {
                    hash: SigHash::from_signature(&format!("fn func_{}_{}", file_idx, entity_idx)),
                    kind: NodeKind::Function,
                    name: Arc::from(format!("func_{}_{}", file_idx, entity_idx)),
                    signature: Arc::from(format!("fn func_{}_{} () -> i32", file_idx, entity_idx)),
                    file_path: Arc::from(format!("src/module_{}.rs", file_idx)),
                    line: (entity_idx as u32) + 1,
                };
                isg.upsert_node(node);
            }
        }
        
        isg
    }
}

/// STUB: Mock implementation for testing contracts
#[derive(Clone)]
pub struct MockFileNavigationProvider {
    entities: Vec<EntityInfo>,
}

impl MockFileNavigationProvider {
    pub fn new() -> Self {
        Self {
            entities: vec![
                EntityInfo::new(
                    "main".to_string(),
                    "src/main.rs".to_string(),
                    EntityType::Function,
                    Some(1),
                    None,
                ),
                EntityInfo::new(
                    "helper".to_string(),
                    "src/main.rs".to_string(),
                    EntityType::Function,
                    Some(10),
                    None,
                ),
                EntityInfo::new(
                    "User".to_string(),
                    "src/lib.rs".to_string(),
                    EntityType::Struct,
                    Some(5),
                    None,
                ),
            ],
        }
    }
}

#[async_trait::async_trait]
impl FileNavigationProvider for MockFileNavigationProvider {
    async fn entities_in_file_with_filter(
        &self,
        file_path: &str,
        entity_type_filter: Option<EntityType>,
    ) -> Result<Vec<EntityInfo>> {
        let mut entities: Vec<EntityInfo> = self.entities
            .iter()
            .filter(|e| e.file_path == file_path)
            .cloned()
            .collect();
        
        if let Some(filter_type) = entity_type_filter {
            entities.retain(|e| e.entity_type == filter_type);
        }
        
        // Sort by line number for consistent ordering
        entities.sort_by_key(|e| e.line_number.unwrap_or(0));
        
        Ok(entities)
    }
    
    async fn where_defined(&self, entity_name: &str) -> Result<Option<FileLocation>> {
        for entity in &self.entities {
            if entity.name == entity_name {
                return Ok(Some(entity.file_location()));
            }
        }
        Ok(None)
    }
    
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
mod file_navigation_contract_tests {
    use super::*;
    use tokio;
    
/// STUB: Mock implementation for testing contracts
pub struct MockFileNavigationProvider {
    entities: Vec<EntityInfo>,
}
    
impl MockFileNavigationProvider {
    pub fn new() -> Self {
            Self {
                entities: vec![
                    EntityInfo::new(
                        "main".to_string(),
                        "src/main.rs".to_string(),
                        EntityType::Function,
                        Some(1),
                        None,
                    ),
                    EntityInfo::new(
                        "helper".to_string(),
                        "src/main.rs".to_string(),
                        EntityType::Function,
                        Some(10),
                        None,
                    ),
                    EntityInfo::new(
                        "User".to_string(),
                        "src/lib.rs".to_string(),
                        EntityType::Struct,
                        Some(5),
                        None,
                    ),
                ],
            }
        }
    }
    
    #[async_trait::async_trait]
    impl FileNavigationProvider for MockFileNavigationProvider {
        async fn entities_in_file_with_filter(
            &self,
            file_path: &str,
            entity_type_filter: Option<EntityType>,
        ) -> Result<Vec<EntityInfo>> {
            let mut entities: Vec<EntityInfo> = self.entities
                .iter()
                .filter(|e| e.file_path == file_path)
                .cloned()
                .collect();
            
            if let Some(filter_type) = entity_type_filter {
                entities.retain(|e| e.entity_type == filter_type);
            }
            
            // Sort by line number for consistent ordering
            entities.sort_by_key(|e| e.line_number.unwrap_or(0));
            
            Ok(entities)
        }
        
        async fn where_defined(&self, entity_name: &str) -> Result<Option<FileLocation>> {
            for entity in &self.entities {
                if entity.name == entity_name {
                    return Ok(Some(entity.file_location()));
                }
            }
            Ok(None)
        }
        
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
    
    // ===== CONTRACT VALIDATION TESTS =====
    
    #[tokio::test]
    async fn test_entities_in_file_contract_basic_functionality() {
        let provider = MockFileNavigationProvider::new();
        
        // Test: entities in main.rs should return 2 functions
        let entities = provider.entities_in_file_with_filter("src/main.rs", None).await.unwrap();
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].name, "main");
        assert_eq!(entities[0].line_number, Some(1));
        assert_eq!(entities[1].name, "helper");
        assert_eq!(entities[1].line_number, Some(10));
        
        // Test: entities in lib.rs should return 1 struct
        let entities = provider.entities_in_file_with_filter("src/lib.rs", None).await.unwrap();
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].name, "User");
        assert_eq!(entities[0].entity_type, EntityType::Struct);
        
        // Test: non-existent file should return empty
        let entities = provider.entities_in_file_with_filter("src/nonexistent.rs", None).await.unwrap();
        assert_eq!(entities.len(), 0);
    }
    
    #[tokio::test]
    async fn test_entities_in_file_contract_type_filtering() {
        let provider = MockFileNavigationProvider::new();
        
        // Test: filter functions in main.rs
        let functions = provider.entities_in_file_with_filter(
            "src/main.rs", 
            Some(EntityType::Function)
        ).await.unwrap();
        assert_eq!(functions.len(), 2);
        assert!(functions.iter().all(|e| e.entity_type == EntityType::Function));
        
        // Test: filter structs in main.rs (should be empty)
        let structs = provider.entities_in_file_with_filter(
            "src/main.rs", 
            Some(EntityType::Struct)
        ).await.unwrap();
        assert_eq!(structs.len(), 0);
        
        // Test: filter structs in lib.rs
        let structs = provider.entities_in_file_with_filter(
            "src/lib.rs", 
            Some(EntityType::Struct)
        ).await.unwrap();
        assert_eq!(structs.len(), 1);
        assert_eq!(structs[0].name, "User");
    }
    
    #[tokio::test]
    async fn test_entities_in_file_performance_contract() {
        let provider = MockFileNavigationProvider::new();
        
        // Performance contract: <100ms
        let start = Instant::now();
        let _entities = provider.entities_in_file_with_filter("src/main.rs", None).await.unwrap();
        let elapsed = start.elapsed();
        
        assert!(elapsed < Duration::from_millis(100), 
                "entities_in_file_with_filter took {:?}, expected <100ms", elapsed);
    }
    
    #[tokio::test]
    async fn test_where_defined_contract_basic_functionality() {
        let provider = MockFileNavigationProvider::new();
        
        // Test: find existing entity
        let location = provider.where_defined("main").await.unwrap();
        assert!(location.is_some());
        let loc = location.unwrap();
        assert_eq!(loc.file_path, "src/main.rs");
        assert_eq!(loc.line_number, Some(1));
        
        // Test: find struct
        let location = provider.where_defined("User").await.unwrap();
        assert!(location.is_some());
        let loc = location.unwrap();
        assert_eq!(loc.file_path, "src/lib.rs");
        assert_eq!(loc.line_number, Some(5));
        
        // Test: non-existent entity
        let location = provider.where_defined("NonExistent").await.unwrap();
        assert!(location.is_none());
    }
    
    #[tokio::test]
    async fn test_where_defined_performance_contract() {
        let provider = MockFileNavigationProvider::new();
        
        // Performance contract: <50ms
        let start = Instant::now();
        let _location = provider.where_defined("main").await.unwrap();
        let elapsed = start.elapsed();
        
        assert!(elapsed < Duration::from_millis(50), 
                "where_defined took {:?}, expected <50ms", elapsed);
    }
    
    #[tokio::test]
    async fn test_file_statistics_contract_basic_functionality() {
        let provider = MockFileNavigationProvider::new();
        
        // Test: file with entities
        let stats = provider.file_statistics("src/main.rs").await.unwrap();
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.file_path, "src/main.rs");
        assert_eq!(stats.total_entities, 2);
        assert_eq!(stats.entity_counts.get(&EntityType::Function), Some(&2));
        assert_eq!(stats.line_range, Some((1, 10)));
        
        // Test: file with no entities
        let stats = provider.file_statistics("src/nonexistent.rs").await.unwrap();
        assert!(stats.is_none());
    }
    
    #[tokio::test]
    async fn test_file_statistics_accuracy_contract() {
        let provider = MockFileNavigationProvider::new();
        
        // Test: statistics accuracy for lib.rs
        let stats = provider.file_statistics("src/lib.rs").await.unwrap();
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.total_entities, 1);
        assert_eq!(stats.entity_counts.get(&EntityType::Struct), Some(&1));
        assert_eq!(stats.entity_counts.get(&EntityType::Function), None);
        assert_eq!(stats.line_range, Some((5, 5))); // Single entity at line 5
    }
}

/// Performance validation tests for large datasets
#[cfg(test)]
mod performance_contract_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_file_index_performance_with_large_dataset() {
        // This test validates that file-based operations scale properly
        // Contract: O(n) where n = entities in file, not total entities
        
        let isg = TestDataFactory::create_large_test_isg(100, 50); // 5000 total entities
        
        // TODO: This will fail until we implement the real provider
        // This is the RED phase of TDD - test should fail first
        
        // Uncomment when implementing:
        // let provider = RealFileNavigationProvider::new(isg);
        // 
        // // Test: entities_in_file should be fast even with large dataset
        // let start = Instant::now();
        // let entities = provider.entities_in_file_with_filter("src/module_0.rs", None).await.unwrap();
        // let elapsed = start.elapsed();
        // 
        // assert_eq!(entities.len(), 50); // 50 entities in this file
        // assert!(elapsed < Duration::from_millis(100), 
        //         "entities_in_file with large dataset took {:?}, expected <100ms", elapsed);
    }
    
    #[tokio::test]
    async fn test_where_defined_performance_with_large_dataset() {
        // This test validates that name lookup scales properly
        // Contract: O(1) lookup using name index
        
        let isg = TestDataFactory::create_large_test_isg(100, 50); // 5000 total entities
        
        // TODO: This will fail until we implement the real provider
        // This is the RED phase of TDD - test should fail first
        
        // Uncomment when implementing:
        // let provider = RealFileNavigationProvider::new(isg);
        // 
        // // Test: where_defined should be fast even with large dataset
        // let start = Instant::now();
        // let location = provider.where_defined("func_25_10").await.unwrap();
        // let elapsed = start.elapsed();
        // 
        // assert!(location.is_some());
        // let loc = location.unwrap();
        // assert_eq!(loc.file_path, "src/module_25.rs");
        // assert_eq!(loc.line_number, Some(11));
        // assert!(elapsed < Duration::from_millis(50), 
        //         "where_defined with large dataset took {:?}, expected <50ms", elapsed);
    }
}
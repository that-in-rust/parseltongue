//! Discovery engine trait and core interfaces
//! 
//! Defines the DiscoveryEngine trait that provides the main interface for
//! entity discovery operations. This trait abstracts the discovery functionality
//! to enable different implementations (in-memory, database-backed, etc.).

use crate::discovery::{
    types::{EntityInfo, FileLocation, DiscoveryQuery, DiscoveryResult, EntityType},
    error::{DiscoveryResult as Result},
};
use async_trait::async_trait;

/// Core trait for entity discovery operations
/// 
/// Provides the main interface for discovering entities in a codebase.
/// Designed to be simple and focused on solving the core constraint:
/// entity discoverability.
/// 
/// # Performance Contracts
/// - Discovery queries: <100ms for interactive responsiveness
/// - Entity listing: <30 seconds for complete codebase exploration
/// - Memory usage: <20% increase from baseline ISG
/// 
/// # Thread Safety
/// All methods are async and should be thread-safe. Implementations
/// should handle concurrent access appropriately.
#[async_trait]
pub trait DiscoveryEngine: Send + Sync {
    /// List all entities (the core constraint solver)
    /// 
    /// This is the primary method that solves the entity discovery bottleneck.
    /// Users can browse all available entities without needing to guess names.
    /// 
    /// # Arguments
    /// * `entity_type` - Optional filter by entity type (Function, Struct, etc.)
    /// * `max_results` - Maximum number of results to return (for pagination)
    /// 
    /// # Performance Contract
    /// Must complete in <100ms for interactive responsiveness.
    /// 
    /// # Example
    /// ```rust,ignore
    /// let entities = engine.list_all_entities(Some(EntityType::Function), 50).await?;
    /// for entity in entities {
    ///     println!("{}: {}", entity.name, entity.file_path);
    /// }
    /// ```
    async fn list_all_entities(
        &self,
        entity_type: Option<EntityType>,
        max_results: usize,
    ) -> Result<Vec<EntityInfo>>;
    
    /// List all entities in a specific file
    /// 
    /// Enables file-centric navigation by showing all entities defined
    /// in a particular file. Essential for understanding file contents
    /// and navigating within files.
    /// 
    /// # Arguments
    /// * `file_path` - Path to the file to search
    /// 
    /// # Performance Contract
    /// Must complete in <100ms for interactive responsiveness.
    /// 
    /// # Example
    /// ```rust,ignore
    /// let entities = engine.entities_in_file("src/main.rs").await?;
    /// for entity in entities {
    ///     println!("{}:{} - {}", entity.file_path, entity.line_number.unwrap_or(0), entity.name);
    /// }
    /// ```
    async fn entities_in_file(
        &self,
        file_path: &str,
    ) -> Result<Vec<EntityInfo>>;
    
    /// Find exact file location for an entity (once they know the name)
    /// 
    /// Provides precise navigation to entity definitions. This is used
    /// after entity discovery to jump directly to the code.
    /// 
    /// # Arguments
    /// * `entity_name` - Exact name of the entity to locate
    /// 
    /// # Returns
    /// * `Some(FileLocation)` if entity is found
    /// * `None` if entity doesn't exist
    /// 
    /// # Performance Contract
    /// Must complete in <50ms for immediate navigation.
    /// 
    /// # Example
    /// ```rust,ignore
    /// if let Some(location) = engine.where_defined("MyStruct").await? {
    ///     println!("Found at: {}", location.format_for_editor());
    /// }
    /// ```
    async fn where_defined(
        &self,
        entity_name: &str,
    ) -> Result<Option<FileLocation>>;
    
    /// Execute a discovery query
    /// 
    /// Generic method for executing any type of discovery query.
    /// Provides a unified interface for all discovery operations
    /// with consistent result formatting and performance monitoring.
    /// 
    /// # Arguments
    /// * `query` - The discovery query to execute
    /// 
    /// # Returns
    /// * `DiscoveryResult` with entities, timing, and metadata
    /// 
    /// # Performance Contract
    /// Must complete within the performance contract for the specific query type.
    /// 
    /// # Example
    /// ```rust,ignore
    /// let query = DiscoveryQuery::list_by_type(EntityType::Function);
    /// let result = engine.execute_discovery_query(query).await?;
    /// 
    /// println!("Found {} functions in {}ms", 
    ///          result.result_count(), 
    ///          result.execution_time_ms());
    /// ```
    async fn execute_discovery_query(
        &self,
        query: DiscoveryQuery,
    ) -> Result<DiscoveryResult>;
    
    /// Get total number of entities in the system
    /// 
    /// Provides context for pagination and system size understanding.
    /// Useful for progress indicators and capacity planning.
    /// 
    /// # Performance Contract
    /// Must complete in <10ms (should be cached/precomputed).
    async fn total_entity_count(&self) -> Result<usize>;
    
    /// Get entity count by type
    /// 
    /// Provides breakdown of entities by type for overview and filtering.
    /// Helps users understand the composition of the codebase.
    /// 
    /// # Returns
    /// * Map from EntityType to count
    /// 
    /// # Performance Contract
    /// Must complete in <50ms (should be cached/precomputed).
    async fn entity_count_by_type(&self) -> Result<std::collections::HashMap<EntityType, usize>>;
    
    /// Get all unique file paths in the system
    /// 
    /// Enables file-based navigation and provides overview of codebase structure.
    /// Useful for file browsers and navigation interfaces.
    /// 
    /// # Performance Contract
    /// Must complete in <100ms for interactive file browsing.
    async fn all_file_paths(&self) -> Result<Vec<String>>;
    
    /// Health check for the discovery engine
    /// 
    /// Verifies that the discovery engine is functioning correctly
    /// and meets performance contracts. Used for monitoring and debugging.
    /// 
    /// # Returns
    /// * `Ok(())` if engine is healthy
    /// * `Err(DiscoveryError)` with details if there are issues
    async fn health_check(&self) -> Result<()>;
}

/// Extension trait for additional discovery operations
/// 
/// Provides convenience methods built on top of the core DiscoveryEngine trait.
/// These methods offer common patterns and combinations of basic operations.
#[async_trait]
pub trait DiscoveryEngineExt: DiscoveryEngine {
    /// Find entities by name pattern (exact match for now)
    /// 
    /// Convenience method for finding entities that match a specific name.
    /// Currently implements exact matching - fuzzy matching may be added later.
    async fn find_entities_by_name(&self, name: &str) -> Result<Vec<EntityInfo>> {
        let all_entities = self.list_all_entities(None, 10000).await?;
        let matching_entities = all_entities
            .into_iter()
            .filter(|entity| entity.name == name)
            .collect();
        Ok(matching_entities)
    }
    
    /// Get entities in multiple files
    /// 
    /// Convenience method for getting entities from multiple files at once.
    /// More efficient than calling entities_in_file multiple times.
    async fn entities_in_files(&self, file_paths: &[String]) -> Result<Vec<EntityInfo>> {
        let mut all_entities = Vec::new();
        
        for file_path in file_paths {
            let entities = self.entities_in_file(file_path).await?;
            all_entities.extend(entities);
        }
        
        Ok(all_entities)
    }
    
    /// Get summary statistics about the codebase
    /// 
    /// Provides high-level overview of the codebase composition.
    /// Useful for onboarding and architectural understanding.
    async fn codebase_summary(&self) -> Result<CodebaseSummary> {
        let total_entities = self.total_entity_count().await?;
        let entity_counts = self.entity_count_by_type().await?;
        let file_paths = self.all_file_paths().await?;
        
        Ok(CodebaseSummary {
            total_entities,
            entity_counts,
            total_files: file_paths.len(),
            file_paths,
        })
    }
}

/// Implement the extension trait for all types that implement DiscoveryEngine
impl<T: DiscoveryEngine> DiscoveryEngineExt for T {}

/// Summary statistics about a codebase
/// 
/// Provides high-level metrics about the codebase structure and composition.
/// Used for onboarding, documentation, and architectural analysis.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CodebaseSummary {
    /// Total number of entities across all types
    pub total_entities: usize,
    /// Count of entities by type
    pub entity_counts: std::collections::HashMap<EntityType, usize>,
    /// Total number of files containing entities
    pub total_files: usize,
    /// List of all file paths (for navigation)
    pub file_paths: Vec<String>,
}

impl CodebaseSummary {
    /// Get the most common entity type
    pub fn most_common_entity_type(&self) -> Option<EntityType> {
        self.entity_counts
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&entity_type, _)| entity_type)
    }
    
    /// Get the percentage of entities of a specific type
    pub fn entity_type_percentage(&self, entity_type: EntityType) -> f64 {
        if self.total_entities == 0 {
            return 0.0;
        }
        
        let count = self.entity_counts.get(&entity_type).copied().unwrap_or(0);
        (count as f64 / self.total_entities as f64) * 100.0
    }
    
    /// Get average entities per file
    pub fn average_entities_per_file(&self) -> f64 {
        if self.total_files == 0 {
            return 0.0;
        }
        
        self.total_entities as f64 / self.total_files as f64
    }
    
    /// Format as a human-readable summary
    pub fn format_summary(&self) -> String {
        let mut summary = format!(
            "Codebase Summary:\n  {} entities across {} files\n  Average: {:.1} entities per file\n\nEntity Types:\n",
            self.total_entities,
            self.total_files,
            self.average_entities_per_file()
        );
        
        let mut sorted_types: Vec<_> = self.entity_counts.iter().collect();
        sorted_types.sort_by_key(|(_, &count)| std::cmp::Reverse(count));
        
        for (entity_type, count) in sorted_types {
            let percentage = self.entity_type_percentage(*entity_type);
            summary.push_str(&format!(
                "  {:?}: {} ({:.1}%)\n",
                entity_type, count, percentage
            ));
        }
        
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_codebase_summary_creation() {
        let mut entity_counts = HashMap::new();
        entity_counts.insert(EntityType::Function, 100);
        entity_counts.insert(EntityType::Struct, 50);
        entity_counts.insert(EntityType::Trait, 25);
        
        let summary = CodebaseSummary {
            total_entities: 175,
            entity_counts,
            total_files: 25,
            file_paths: vec!["src/main.rs".to_string(), "src/lib.rs".to_string()],
        };
        
        assert_eq!(summary.total_entities, 175);
        assert_eq!(summary.total_files, 25);
        assert_eq!(summary.average_entities_per_file(), 7.0);
        assert_eq!(summary.most_common_entity_type(), Some(EntityType::Function));
        assert!((summary.entity_type_percentage(EntityType::Function) - (100.0 * 100.0 / 175.0)).abs() < 0.001);
    }
    
    #[test]
    fn test_codebase_summary_edge_cases() {
        let summary = CodebaseSummary {
            total_entities: 0,
            entity_counts: HashMap::new(),
            total_files: 0,
            file_paths: vec![],
        };
        
        assert_eq!(summary.average_entities_per_file(), 0.0);
        assert_eq!(summary.most_common_entity_type(), None);
        assert_eq!(summary.entity_type_percentage(EntityType::Function), 0.0);
    }
    
    #[test]
    fn test_codebase_summary_formatting() {
        let mut entity_counts = HashMap::new();
        entity_counts.insert(EntityType::Function, 10);
        entity_counts.insert(EntityType::Struct, 5);
        
        let summary = CodebaseSummary {
            total_entities: 15,
            entity_counts,
            total_files: 3,
            file_paths: vec!["src/main.rs".to_string()],
        };
        
        let formatted = summary.format_summary();
        assert!(formatted.contains("15 entities"));
        assert!(formatted.contains("3 files"));
        assert!(formatted.contains("Function: 10"));
        assert!(formatted.contains("Struct: 5"));
    }
}
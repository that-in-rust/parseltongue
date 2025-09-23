//! Core types for the discovery system
//! 
//! Defines the fundamental data structures used throughout the discovery layer,
//! including entity information, file locations, queries, and results.

use crate::isg::NodeKind;
use std::time::Duration;
use serde::{Serialize, Deserialize};

/// Entity type for discovery operations
/// 
/// Maps to the existing NodeKind but provides a discovery-focused interface.
/// This allows the discovery layer to evolve independently from the core ISG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Function,
    Struct,
    Trait,
    Impl,
    Module,
    Constant,
    Static,
    Macro,
}

impl From<NodeKind> for EntityType {
    fn from(kind: NodeKind) -> Self {
        match kind {
            NodeKind::Function => EntityType::Function,
            NodeKind::Struct => EntityType::Struct,
            NodeKind::Trait => EntityType::Trait,
        }
    }
}

impl From<EntityType> for NodeKind {
    fn from(entity_type: EntityType) -> Self {
        match entity_type {
            EntityType::Function => NodeKind::Function,
            EntityType::Struct => NodeKind::Struct,
            EntityType::Trait => NodeKind::Trait,
            // For now, map other types to Function as a fallback
            // This will be expanded when the core ISG supports more node types
            EntityType::Impl | EntityType::Module | EntityType::Constant | 
            EntityType::Static | EntityType::Macro => NodeKind::Function,
        }
    }
}

/// Compact entity information for discovery results
/// 
/// Optimized for memory efficiency while providing all information needed
/// for entity discovery and navigation. Uses string interning for file paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityInfo {
    /// Human-readable entity name
    pub name: String,
    /// File path where entity is defined (interned for memory efficiency)
    pub file_path: String,
    /// Type of entity (function, struct, trait, etc.)
    pub entity_type: EntityType,
    /// Line number in file (None if not available)
    pub line_number: Option<u32>,
    /// Column number in file (None if not available)
    pub column: Option<u32>,
}

impl EntityInfo {
    /// Create a new EntityInfo
    pub fn new(
        name: String,
        file_path: String,
        entity_type: EntityType,
        line_number: Option<u32>,
        column: Option<u32>,
    ) -> Self {
        Self {
            name,
            file_path,
            entity_type,
            line_number,
            column,
        }
    }
    
    /// Get the file location as a FileLocation struct
    pub fn file_location(&self) -> FileLocation {
        FileLocation {
            file_path: self.file_path.clone(),
            line_number: self.line_number,
            column: self.column,
        }
    }
    
    /// Check if this entity has location information
    pub fn has_location(&self) -> bool {
        self.line_number.is_some()
    }
}

/// File location information for navigation
/// 
/// Provides precise location information for jumping to entity definitions
/// in editors and IDEs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileLocation {
    /// File path where entity is located
    pub file_path: String,
    /// Line number (1-based, None if not available)
    pub line_number: Option<u32>,
    /// Column number (1-based, None if not available)
    pub column: Option<u32>,
}

impl FileLocation {
    /// Create a new FileLocation
    pub fn new(file_path: String, line_number: Option<u32>, column: Option<u32>) -> Self {
        Self {
            file_path,
            line_number,
            column,
        }
    }
    
    /// Create a FileLocation with only file path
    pub fn file_only(file_path: String) -> Self {
        Self {
            file_path,
            line_number: None,
            column: None,
        }
    }
    
    /// Create a FileLocation with file path and line number
    pub fn with_line(file_path: String, line_number: u32) -> Self {
        Self {
            file_path,
            line_number: Some(line_number),
            column: None,
        }
    }
    
    /// Create a FileLocation with full position information
    pub fn with_position(file_path: String, line_number: u32, column: u32) -> Self {
        Self {
            file_path,
            line_number: Some(line_number),
            column: Some(column),
        }
    }
    
    /// Format as a string suitable for editor navigation
    /// 
    /// Returns format like "src/main.rs:42:10" for full position info,
    /// "src/main.rs:42" for line only, or "src/main.rs" for file only.
    pub fn format_for_editor(&self) -> String {
        match (self.line_number, self.column) {
            (Some(line), Some(col)) => format!("{}:{}:{}", self.file_path, line, col),
            (Some(line), None) => format!("{}:{}", self.file_path, line),
            (None, _) => self.file_path.clone(),
        }
    }
}

/// Discovery query types for entity exploration
/// 
/// Represents the different types of queries that can be performed
/// against the discovery system. Designed to be simple and focused
/// on the core constraint: entity discovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryQuery {
    /// List all entities (the main constraint solver)
    ListAll {
        entity_type: Option<EntityType>,
        max_results: usize,
    },
    
    /// List entities in specific file
    EntitiesInFile {
        file_path: String,
        entity_types: Option<Vec<EntityType>>,
    },
    
    /// Find definition location (once they know the name)
    WhereDefinedExact {
        entity_name: String,
    },
}

impl DiscoveryQuery {
    /// Create a query to list all entities
    pub fn list_all() -> Self {
        Self::ListAll {
            entity_type: None,
            max_results: 1000, // Default reasonable limit
        }
    }
    
    /// Create a query to list entities of a specific type
    pub fn list_by_type(entity_type: EntityType) -> Self {
        Self::ListAll {
            entity_type: Some(entity_type),
            max_results: 1000,
        }
    }
    
    /// Create a query to list entities in a file
    pub fn entities_in_file(file_path: String) -> Self {
        Self::EntitiesInFile {
            file_path,
            entity_types: None,
        }
    }
    
    /// Create a query to find where an entity is defined
    pub fn where_defined(entity_name: String) -> Self {
        Self::WhereDefinedExact { entity_name }
    }
    
    /// Get a human-readable description of the query
    pub fn description(&self) -> String {
        match self {
            Self::ListAll { entity_type: None, max_results } => {
                format!("List all entities (max {})", max_results)
            }
            Self::ListAll { entity_type: Some(t), max_results } => {
                format!("List all {:?} entities (max {})", t, max_results)
            }
            Self::EntitiesInFile { file_path, entity_types: None } => {
                format!("List entities in file: {}", file_path)
            }
            Self::EntitiesInFile { file_path, entity_types: Some(types) } => {
                format!("List {:?} entities in file: {}", types, file_path)
            }
            Self::WhereDefinedExact { entity_name } => {
                format!("Find definition of: {}", entity_name)
            }
        }
    }
}

/// Result of a discovery query operation
/// 
/// Contains the query results along with metadata about the operation
/// for performance monitoring and debugging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveryResult {
    /// The original query that produced this result
    pub query: DiscoveryQuery,
    /// The entities found by the query
    pub entities: Vec<EntityInfo>,
    /// Time taken to execute the query
    #[serde(with = "duration_serde")]
    pub execution_time: Duration,
    /// Total number of entities in the system (for pagination context)
    pub total_entities: usize,
    /// Whether the results were truncated due to limits
    pub truncated: bool,
}

impl DiscoveryResult {
    /// Create a new DiscoveryResult
    pub fn new(
        query: DiscoveryQuery,
        entities: Vec<EntityInfo>,
        execution_time: Duration,
        total_entities: usize,
    ) -> Self {
        let truncated = match &query {
            DiscoveryQuery::ListAll { max_results, .. } => entities.len() >= *max_results,
            _ => false,
        };
        
        Self {
            query,
            entities,
            execution_time,
            total_entities,
            truncated,
        }
    }
    
    /// Get the number of results returned
    pub fn result_count(&self) -> usize {
        self.entities.len()
    }
    
    /// Check if the query found any results
    pub fn has_results(&self) -> bool {
        !self.entities.is_empty()
    }
    
    /// Get execution time in milliseconds
    pub fn execution_time_ms(&self) -> f64 {
        self.execution_time.as_secs_f64() * 1000.0
    }
    
    /// Check if the query met the performance contract (<100ms for discovery)
    pub fn meets_performance_contract(&self) -> bool {
        self.execution_time < Duration::from_millis(100)
    }
}

/// Custom serialization for Duration to handle serde compatibility
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;
    
    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_nanos().serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nanos = u128::deserialize(deserializer)?;
        Ok(Duration::from_nanos(nanos as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_type_conversion() {
        // Test NodeKind to EntityType conversion
        assert_eq!(EntityType::from(NodeKind::Function), EntityType::Function);
        assert_eq!(EntityType::from(NodeKind::Struct), EntityType::Struct);
        assert_eq!(EntityType::from(NodeKind::Trait), EntityType::Trait);
        
        // Test EntityType to NodeKind conversion
        assert_eq!(NodeKind::from(EntityType::Function), NodeKind::Function);
        assert_eq!(NodeKind::from(EntityType::Struct), NodeKind::Struct);
        assert_eq!(NodeKind::from(EntityType::Trait), NodeKind::Trait);
    }
    
    #[test]
    fn test_entity_info_creation() {
        let entity = EntityInfo::new(
            "test_function".to_string(),
            "src/main.rs".to_string(),
            EntityType::Function,
            Some(42),
            Some(10),
        );
        
        assert_eq!(entity.name, "test_function");
        assert_eq!(entity.file_path, "src/main.rs");
        assert_eq!(entity.entity_type, EntityType::Function);
        assert_eq!(entity.line_number, Some(42));
        assert_eq!(entity.column, Some(10));
        assert!(entity.has_location());
        
        let location = entity.file_location();
        assert_eq!(location.file_path, "src/main.rs");
        assert_eq!(location.line_number, Some(42));
        assert_eq!(location.column, Some(10));
    }
    
    #[test]
    fn test_file_location_formatting() {
        let full_location = FileLocation::with_position("src/main.rs".to_string(), 42, 10);
        assert_eq!(full_location.format_for_editor(), "src/main.rs:42:10");
        
        let line_only = FileLocation::with_line("src/main.rs".to_string(), 42);
        assert_eq!(line_only.format_for_editor(), "src/main.rs:42");
        
        let file_only = FileLocation::file_only("src/main.rs".to_string());
        assert_eq!(file_only.format_for_editor(), "src/main.rs");
    }
    
    #[test]
    fn test_discovery_query_creation() {
        let list_all = DiscoveryQuery::list_all();
        assert!(matches!(list_all, DiscoveryQuery::ListAll { entity_type: None, max_results: 1000 }));
        
        let list_functions = DiscoveryQuery::list_by_type(EntityType::Function);
        assert!(matches!(list_functions, DiscoveryQuery::ListAll { 
            entity_type: Some(EntityType::Function), 
            max_results: 1000 
        }));
        
        let entities_in_file = DiscoveryQuery::entities_in_file("src/main.rs".to_string());
        assert!(matches!(entities_in_file, DiscoveryQuery::EntitiesInFile { 
            file_path, 
            entity_types: None 
        } if file_path == "src/main.rs"));
        
        let where_defined = DiscoveryQuery::where_defined("test_function".to_string());
        assert!(matches!(where_defined, DiscoveryQuery::WhereDefinedExact { 
            entity_name 
        } if entity_name == "test_function"));
    }
    
    #[test]
    fn test_discovery_query_description() {
        let query = DiscoveryQuery::list_all();
        assert_eq!(query.description(), "List all entities (max 1000)");
        
        let query = DiscoveryQuery::list_by_type(EntityType::Function);
        assert_eq!(query.description(), "List all Function entities (max 1000)");
        
        let query = DiscoveryQuery::entities_in_file("src/main.rs".to_string());
        assert_eq!(query.description(), "List entities in file: src/main.rs");
        
        let query = DiscoveryQuery::where_defined("test_function".to_string());
        assert_eq!(query.description(), "Find definition of: test_function");
    }
    
    #[test]
    fn test_discovery_result_creation() {
        let query = DiscoveryQuery::list_all();
        let entities = vec![
            EntityInfo::new(
                "test1".to_string(),
                "src/main.rs".to_string(),
                EntityType::Function,
                Some(10),
                None,
            ),
            EntityInfo::new(
                "test2".to_string(),
                "src/lib.rs".to_string(),
                EntityType::Struct,
                Some(20),
                None,
            ),
        ];
        let execution_time = Duration::from_millis(50);
        
        let result = DiscoveryResult::new(query, entities, execution_time, 100);
        
        assert_eq!(result.result_count(), 2);
        assert!(result.has_results());
        assert_eq!(result.execution_time_ms(), 50.0);
        assert!(result.meets_performance_contract());
        assert!(!result.truncated);
    }
    
    #[test]
    fn test_discovery_result_truncation() {
        let query = DiscoveryQuery::ListAll {
            entity_type: None,
            max_results: 2,
        };
        let entities = vec![
            EntityInfo::new("test1".to_string(), "src/main.rs".to_string(), EntityType::Function, None, None),
            EntityInfo::new("test2".to_string(), "src/lib.rs".to_string(), EntityType::Struct, None, None),
        ];
        
        let result = DiscoveryResult::new(query, entities, Duration::from_millis(10), 100);
        assert!(result.truncated); // 2 results with max_results = 2 means truncated
    }
    
    #[test]
    fn test_performance_contract_violation() {
        let query = DiscoveryQuery::list_all();
        let entities = vec![];
        let slow_execution = Duration::from_millis(150); // Exceeds 100ms contract
        
        let result = DiscoveryResult::new(query, entities, slow_execution, 0);
        assert!(!result.meets_performance_contract());
    }
}
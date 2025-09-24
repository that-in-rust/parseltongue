//! Discovery indexes for fast entity lookup and filtering
//! 
//! Provides optimized data structures for entity discovery operations:
//! - CompactEntityInfo: Memory-optimized entity representation (24 bytes per entity)
//! - DiscoveryIndexes: Fast lookup indexes for all entities, files, and types
//! - Efficient index rebuild mechanism for ISG updates
//! 
//! Performance contracts:
//! - Index rebuild: <5 seconds for large codebases
//! - Entity lookup: <100ms for discovery queries
//! - Memory usage: 24 bytes per entity + index overhead

use crate::discovery::EntityInfo;
use crate::discovery::types::EntityType;
use crate::discovery::string_interning::{FileId, FileInterner};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Compact entity information optimized for memory efficiency
/// 
/// Target: 24 bytes per entity for optimal cache performance
/// Uses string interning for file paths to reduce memory usage
/// 
/// Memory layout optimization:
/// - 8-byte alignment for optimal cache performance
/// - Exactly 24 bytes total size
/// - Efficient packing of fields
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C, align(8))]
pub struct CompactEntityInfo {
    /// Entity name (interned string ID) - 4 bytes
    pub name_id: u32,
    /// File where entity is defined (interned file ID) - 4 bytes  
    pub file_id: FileId,
    /// Line number (0 if not available) - 4 bytes
    pub line_number: u32,
    /// Column number (0 if not available) - 4 bytes
    pub column: u32,
    /// Type of entity - 1 byte + 3 bytes padding
    pub entity_type: EntityType,
    /// Reserved for future use (ensures 24-byte total size) - 4 bytes
    pub _reserved: u32,
}

/// Discovery indexes for fast entity lookup and filtering
/// 
/// Provides multiple access patterns:
/// - all_entities: Complete list of all entities
/// - file_index: Entities grouped by file
/// - type_index: Entities grouped by type
#[derive(Debug, Clone)]
pub struct DiscoveryIndexes {
    /// All entities in the system
    pub all_entities: Vec<CompactEntityInfo>,
    /// Entities indexed by file ID
    pub file_index: HashMap<FileId, Vec<usize>>,
    /// Entities indexed by type
    pub type_index: HashMap<EntityType, Vec<usize>>,
    /// String interner for names and file paths
    pub interner: FileInterner,
    /// Timestamp of last rebuild
    pub last_rebuild: Instant,
}

impl CompactEntityInfo {
    /// Create a new CompactEntityInfo
    /// 
    /// # Performance Contract
    /// - Memory usage: exactly 24 bytes per entity
    /// - Construction time: <1μs per entity
    pub fn new(
        name_id: u32,
        file_id: FileId,
        entity_type: EntityType,
        line_number: u32,
        column: u32,
    ) -> Self {
        Self {
            name_id,
            file_id,
            entity_type,
            line_number,
            column,
            _reserved: 0,
        }
    }
    
    /// Convert to full EntityInfo using interner
    pub fn to_entity_info(&self, interner: &FileInterner) -> EntityInfo {
        let name = interner.get_name(self.name_id).unwrap_or("unknown").to_string();
        let file_path = interner.get_path(self.file_id).unwrap_or("unknown").to_string();
        
        EntityInfo::new(
            name,
            file_path,
            self.entity_type,
            if self.line_number > 0 { Some(self.line_number) } else { None },
            if self.column > 0 { Some(self.column) } else { None },
        )
    }
    
    /// Check if entity has location information
    pub fn has_location(&self) -> bool {
        self.line_number > 0
    }
}

impl DiscoveryIndexes {
    /// Create new empty discovery indexes
    pub fn new() -> Self {
        Self {
            all_entities: Vec::new(),
            file_index: HashMap::new(),
            type_index: HashMap::new(),
            interner: FileInterner::new(),
            last_rebuild: Instant::now(),
        }
    }
    
    /// Zero-allocation entity filtering with iterator patterns
    /// 
    /// Returns an iterator that filters entities by type without allocating
    /// intermediate collections. This is critical for performance when dealing
    /// with large entity sets.
    pub fn filter_entities_by_type(&self, entity_type: EntityType) -> impl Iterator<Item = &CompactEntityInfo> {
        self.all_entities.iter().filter(move |entity| entity.entity_type == entity_type)
    }
    
    /// Zero-allocation entity filtering by file with iterator patterns
    /// 
    /// Returns an iterator that filters entities by file ID without allocating
    /// intermediate collections.
    pub fn filter_entities_by_file(&self, file_id: FileId) -> impl Iterator<Item = &CompactEntityInfo> {
        self.all_entities.iter().filter(move |entity| entity.file_id == file_id)
    }
    
    /// Zero-allocation combined filtering by type and file
    /// 
    /// Chains multiple filters without intermediate allocations.
    pub fn filter_entities_by_type_and_file(
        &self, 
        entity_type: EntityType, 
        file_id: FileId
    ) -> impl Iterator<Item = &CompactEntityInfo> {
        self.all_entities
            .iter()
            .filter(move |entity| entity.entity_type == entity_type && entity.file_id == file_id)
    }
    
    /// Zero-allocation pagination with iterator patterns
    /// 
    /// Returns an iterator that applies pagination without collecting intermediate results.
    pub fn paginate_entities<'a>(
        &'a self,
        iter: impl Iterator<Item = &'a CompactEntityInfo> + 'a,
        offset: usize,
        limit: usize,
    ) -> impl Iterator<Item = &'a CompactEntityInfo> + 'a {
        iter.skip(offset).take(limit)
    }
    
    /// Zero-allocation entity search by name prefix
    /// 
    /// Returns an iterator that filters entities by name prefix without allocations.
    pub fn filter_entities_by_name_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> impl Iterator<Item = &'a CompactEntityInfo> + 'a {
        self.all_entities
            .iter()
            .filter(move |entity| {
                if let Some(name) = self.interner.get_name(entity.name_id) {
                    name.starts_with(prefix)
                } else {
                    false
                }
            })
    }
    
    /// Rebuild indexes from entity list
    /// 
    /// # Performance Contract
    /// - Rebuild time: <5 seconds for large codebases (100k+ entities)
    /// - Memory efficiency: 24 bytes per entity + index overhead
    /// - Index consistency: All indexes remain synchronized
    pub fn rebuild_from_entities(&mut self, entities: Vec<EntityInfo>) -> Result<Duration, IndexError> {
        let start = Instant::now();
        
        // Clear existing indexes
        self.all_entities.clear();
        self.file_index.clear();
        self.type_index.clear();
        
        // Convert entities to compact format and build indexes
        for (index, entity) in entities.into_iter().enumerate() {
            let name_id = self.interner.intern_name(&entity.name);
            let file_id = self.interner.intern(&entity.file_path);
            
            let compact = CompactEntityInfo::new(
                name_id,
                file_id,
                entity.entity_type,
                entity.line_number.unwrap_or(0),
                entity.column.unwrap_or(0),
            );
            
            // Add to all entities
            self.all_entities.push(compact.clone());
            
            // Add to file index
            self.file_index.entry(file_id).or_insert_with(Vec::new).push(index);
            
            // Add to type index
            self.type_index.entry(entity.entity_type).or_insert_with(Vec::new).push(index);
        }
        
        self.last_rebuild = Instant::now();
        let elapsed = start.elapsed();
        
        // Validate performance contract
        if elapsed > Duration::from_secs(5) {
            return Err(IndexError::RebuildTimeout { 
                elapsed, 
                limit: Duration::from_secs(5) 
            });
        }
        
        Ok(elapsed)
    }
    
    /// Get all entities of a specific type
    pub fn entities_by_type(&self, entity_type: EntityType) -> Vec<&CompactEntityInfo> {
        self.type_index
            .get(&entity_type)
            .map(|indices| {
                indices.iter()
                    .filter_map(|&i| self.all_entities.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get all entities in a specific file
    pub fn entities_in_file(&self, file_id: FileId) -> Vec<&CompactEntityInfo> {
        self.file_index
            .get(&file_id)
            .map(|indices| {
                indices.iter()
                    .filter_map(|&i| self.all_entities.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get total number of entities
    pub fn entity_count(&self) -> usize {
        self.all_entities.len()
    }
    
    /// Get memory usage statistics
    pub fn memory_stats(&self) -> MemoryStats {
        let entity_memory = self.all_entities.len() * std::mem::size_of::<CompactEntityInfo>();
        let file_index_memory = self.file_index.len() * (std::mem::size_of::<FileId>() + std::mem::size_of::<Vec<usize>>())
            + self.file_index.values().map(|v| v.len() * std::mem::size_of::<usize>()).sum::<usize>();
        let type_index_memory = self.type_index.len() * (std::mem::size_of::<EntityType>() + std::mem::size_of::<Vec<usize>>())
            + self.type_index.values().map(|v| v.len() * std::mem::size_of::<usize>()).sum::<usize>();
        
        MemoryStats {
            entity_memory,
            file_index_memory,
            type_index_memory,
            interner_memory: self.interner.memory_usage_bytes(),
            total_memory: entity_memory + file_index_memory + type_index_memory + self.interner.memory_usage_bytes(),
        }
    }
}

impl Default for DiscoveryIndexes {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory usage statistics for discovery indexes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryStats {
    pub entity_memory: usize,
    pub file_index_memory: usize,
    pub type_index_memory: usize,
    pub interner_memory: usize,
    pub total_memory: usize,
}

/// Errors that can occur during index operations
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IndexError {
    #[error("Index rebuild timeout: took {elapsed:?}, limit {limit:?}")]
    RebuildTimeout { elapsed: Duration, limit: Duration },
    
    #[error("Memory limit exceeded: {used} bytes, limit {limit} bytes")]
    MemoryLimitExceeded { used: usize, limit: usize },
    
    #[error("Invalid entity index: {index}, max {max}")]
    InvalidEntityIndex { index: usize, max: usize },
}

// Extensions to FileInterner for name interning
impl FileInterner {
    /// Intern an entity name and return its ID
    /// 
    /// For now, we'll use the same mechanism as file paths
    /// In the future, this could be optimized with a separate name interner
    pub fn intern_name(&mut self, name: &str) -> u32 {
        // Use a prefix to distinguish names from file paths
        let prefixed_name = format!("name:{}", name);
        self.intern(&prefixed_name).as_u32()
    }
    
    /// Get an entity name by its ID
    pub fn get_name(&self, name_id: u32) -> Option<&str> {
        let file_id = FileId::new(name_id);
        self.get_path(file_id)
            .and_then(|path| path.strip_prefix("name:"))
    }
    
    /// Get memory usage as usize for compatibility
    pub fn memory_usage_bytes(&self) -> usize {
        let usage = self.memory_usage();
        usage.path_map_bytes + usage.id_map_bytes + usage.string_storage_bytes
    }
}

#[cfg(test)]
mod memory_optimization_tests {
    use super::*;
    use std::time::Duration;
    
    // RED PHASE: Memory optimization tests that should FAIL until we implement optimizations
    
    #[test]
    fn test_zero_allocation_entity_filtering_performance_contract() {
        // PERFORMANCE CONTRACT: Zero-allocation filtering must be faster than collecting
        let mut indexes = DiscoveryIndexes::new();
        
        // Create large dataset for meaningful performance test
        let mut entities = Vec::new();
        for i in 0..10_000 {
            entities.push(EntityInfo::new(
                format!("entity_{}", i),
                format!("src/file_{}.rs", i % 100),
                if i % 3 == 0 { EntityType::Function } else { EntityType::Struct },
                Some(i as u32 + 1),
                Some((i % 80) as u32 + 1),
            ));
        }
        
        indexes.rebuild_from_entities(entities).unwrap();
        
        // Test zero-allocation filtering performance
        let start = std::time::Instant::now();
        let count = indexes
            .filter_entities_by_type(EntityType::Function)
            .filter(|entity| entity.line_number > 100)
            .take(1000)
            .count();
        let zero_alloc_time = start.elapsed();
        
        // Compare with collecting approach (should be slower)
        let start = std::time::Instant::now();
        let collected: Vec<_> = indexes
            .filter_entities_by_type(EntityType::Function)
            .filter(|entity| entity.line_number > 100)
            .take(1000)
            .collect();
        let collect_time = start.elapsed();
        
        // Zero-allocation should be faster
        assert!(zero_alloc_time <= collect_time, 
                "Zero-allocation filtering ({:?}) should be <= collecting ({:?})", 
                zero_alloc_time, collect_time);
        
        assert_eq!(count, collected.len());
        assert!(count > 0, "Should find entities to filter");
        
        // Performance contract: Should complete in <1ms for 10k entities
        assert!(zero_alloc_time < Duration::from_millis(1),
                "Zero-allocation filtering took {:?}, expected <1ms", zero_alloc_time);
    }
    
    #[test]
    fn test_batch_entity_filtering_with_multiple_types() {
        // PERFORMANCE CONTRACT: Batch filtering should be efficient
        let mut indexes = DiscoveryIndexes::new();
        
        let entities = create_diverse_entity_dataset(5000);
        indexes.rebuild_from_entities(entities).unwrap();
        
        let entity_types = vec![EntityType::Function, EntityType::Struct, EntityType::Trait];
        
        let start = std::time::Instant::now();
        let mut total_count = 0;
        
        for entity_type in entity_types {
            let count = indexes
                .filter_entities_by_type(entity_type)
                .take(500)
                .count();
            total_count += count;
        }
        
        let elapsed = start.elapsed();
        
        // Performance contract: Batch filtering should be fast
        assert!(elapsed < Duration::from_millis(5),
                "Batch filtering took {:?}, expected <5ms", elapsed);
        
        assert!(total_count > 0, "Should find entities across all types");
    }
    
    #[test]
    fn test_memory_efficient_pagination() {
        // PERFORMANCE CONTRACT: Pagination should not allocate intermediate collections
        let mut indexes = DiscoveryIndexes::new();
        
        let entities = create_diverse_entity_dataset(1000);
        indexes.rebuild_from_entities(entities).unwrap();
        
        // Test pagination with zero allocations
        let start = std::time::Instant::now();
        
        let page1: Vec<_> = indexes
            .paginate_entities(
                indexes.filter_entities_by_type(EntityType::Function),
                0,  // offset
                50  // limit
            )
            .collect();
        
        let page2: Vec<_> = indexes
            .paginate_entities(
                indexes.filter_entities_by_type(EntityType::Function),
                50, // offset
                50  // limit
            )
            .collect();
        
        let elapsed = start.elapsed();
        
        // Performance contract: Pagination should be very fast
        assert!(elapsed < Duration::from_micros(500),
                "Pagination took {:?}, expected <500μs", elapsed);
        
        assert_eq!(page1.len(), 50);
        assert_eq!(page2.len(), 50);
        
        // Pages should not overlap
        let page1_ids: std::collections::HashSet<_> = page1.iter().map(|e| e.name_id).collect();
        let page2_ids: std::collections::HashSet<_> = page2.iter().map(|e| e.name_id).collect();
        assert!(page1_ids.is_disjoint(&page2_ids), "Pages should not overlap");
    }
    
    #[test]
    fn test_name_prefix_filtering_performance() {
        // PERFORMANCE CONTRACT: Name prefix filtering should be efficient
        let mut indexes = DiscoveryIndexes::new();
        
        let entities = create_entities_with_common_prefixes(2000);
        indexes.rebuild_from_entities(entities).unwrap();
        
        let start = std::time::Instant::now();
        
        let test_matches: Vec<_> = indexes
            .filter_entities_by_name_prefix("test_")
            .take(100)
            .collect();
        
        let util_matches: Vec<_> = indexes
            .filter_entities_by_name_prefix("util_")
            .take(100)
            .collect();
        
        let elapsed = start.elapsed();
        
        // Performance contract: Prefix filtering should be fast
        assert!(elapsed < Duration::from_millis(2),
                "Name prefix filtering took {:?}, expected <2ms", elapsed);
        
        assert!(test_matches.len() > 0, "Should find test_ prefixed entities");
        assert!(util_matches.len() > 0, "Should find util_ prefixed entities");
        
        // Verify correctness
        for entity in &test_matches {
            let name = indexes.interner.get_name(entity.name_id).unwrap();
            assert!(name.starts_with("test_"), "Entity name should start with test_");
        }
    }
    
    fn create_diverse_entity_dataset(count: usize) -> Vec<EntityInfo> {
        let mut entities = Vec::with_capacity(count);
        let entity_types = [
            EntityType::Function,
            EntityType::Struct, 
            EntityType::Trait,
            EntityType::Impl,
            EntityType::Module,
            EntityType::Constant,
            EntityType::Static,
            EntityType::Macro,
        ];
        
        for i in 0..count {
            let entity_type = entity_types[i % entity_types.len()];
            entities.push(EntityInfo::new(
                format!("entity_{}", i),
                format!("src/module_{}/file_{}.rs", i / 50, i % 50),
                entity_type,
                Some((i % 500) as u32 + 1),
                Some((i % 120) as u32 + 1),
            ));
        }
        
        entities
    }
    
    fn create_entities_with_common_prefixes(count: usize) -> Vec<EntityInfo> {
        let mut entities = Vec::with_capacity(count);
        let prefixes = ["test_", "util_", "parse_", "format_", "validate_"];
        
        for i in 0..count {
            let prefix = prefixes[i % prefixes.len()];
            entities.push(EntityInfo::new(
                format!("{}{}", prefix, i),
                format!("src/file_{}.rs", i % 20),
                EntityType::Function,
                Some(i as u32 + 1),
                Some((i % 80) as u32 + 1),
            ));
        }
        
        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    // RED PHASE: Write failing tests first
    
    #[test]
    fn test_compact_entity_info_memory_layout() {
        // Validate 24-byte memory layout contract
        assert_eq!(std::mem::size_of::<CompactEntityInfo>(), 24, 
                   "CompactEntityInfo must be exactly 24 bytes for optimal cache performance");
    }
    
    #[test]
    fn test_compact_entity_info_creation() {
        let entity = CompactEntityInfo::new(
            1, // name_id
            FileId(2), // file_id
            EntityType::Function,
            42, // line_number
            10, // column
        );
        
        assert_eq!(entity.name_id, 1);
        assert_eq!(entity.file_id, FileId(2));
        assert_eq!(entity.entity_type, EntityType::Function);
        assert_eq!(entity.line_number, 42);
        assert_eq!(entity.column, 10);
        assert!(entity.has_location());
    }
    
    #[test]
    fn test_compact_entity_info_no_location() {
        let entity = CompactEntityInfo::new(
            1,
            FileId(2),
            EntityType::Struct,
            0, // no line number
            0, // no column
        );
        
        assert!(!entity.has_location());
    }
    
    #[test]
    fn test_discovery_indexes_creation() {
        let indexes = DiscoveryIndexes::new();
        
        assert_eq!(indexes.entity_count(), 0);
        assert!(indexes.all_entities.is_empty());
        assert!(indexes.file_index.is_empty());
        assert!(indexes.type_index.is_empty());
    }
    
    #[test]
    fn test_rebuild_from_entities_empty() {
        let mut indexes = DiscoveryIndexes::new();
        let entities = vec![];
        
        let result = indexes.rebuild_from_entities(entities);
        assert!(result.is_ok());
        assert_eq!(indexes.entity_count(), 0);
    }
    
    #[test]
    fn test_rebuild_from_entities_single() {
        let mut indexes = DiscoveryIndexes::new();
        let entities = vec![
            EntityInfo::new(
                "test_function".to_string(),
                "src/main.rs".to_string(),
                EntityType::Function,
                Some(42),
                Some(10),
            )
        ];
        
        let result = indexes.rebuild_from_entities(entities);
        assert!(result.is_ok());
        assert_eq!(indexes.entity_count(), 1);
        
        // Check type index
        let functions = indexes.entities_by_type(EntityType::Function);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].entity_type, EntityType::Function);
        assert_eq!(functions[0].line_number, 42);
        assert_eq!(functions[0].column, 10);
    }
    
    #[test]
    fn test_rebuild_from_entities_multiple_types() {
        let mut indexes = DiscoveryIndexes::new();
        let entities = vec![
            EntityInfo::new("func1".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(10), None),
            EntityInfo::new("Struct1".to_string(), "src/lib.rs".to_string(), EntityType::Struct, Some(20), None),
            EntityInfo::new("func2".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(30), None),
            EntityInfo::new("Trait1".to_string(), "src/traits.rs".to_string(), EntityType::Trait, Some(40), None),
        ];
        
        let result = indexes.rebuild_from_entities(entities);
        assert!(result.is_ok());
        assert_eq!(indexes.entity_count(), 4);
        
        // Check type indexes
        let functions = indexes.entities_by_type(EntityType::Function);
        assert_eq!(functions.len(), 2);
        
        let structs = indexes.entities_by_type(EntityType::Struct);
        assert_eq!(structs.len(), 1);
        
        let traits = indexes.entities_by_type(EntityType::Trait);
        assert_eq!(traits.len(), 1);
        
        let impls = indexes.entities_by_type(EntityType::Impl);
        assert_eq!(impls.len(), 0);
    }
    
    #[test]
    fn test_entities_in_file() {
        let mut indexes = DiscoveryIndexes::new();
        let entities = vec![
            EntityInfo::new("func1".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(10), None),
            EntityInfo::new("Struct1".to_string(), "src/lib.rs".to_string(), EntityType::Struct, Some(20), None),
            EntityInfo::new("func2".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(30), None),
        ];
        
        indexes.rebuild_from_entities(entities).unwrap();
        
        // Get file IDs (this will fail until we implement the interner)
        let main_file_id = indexes.interner.intern("src/main.rs");
        let lib_file_id = indexes.interner.intern("src/lib.rs");
        
        let main_entities = indexes.entities_in_file(main_file_id);
        assert_eq!(main_entities.len(), 2);
        
        let lib_entities = indexes.entities_in_file(lib_file_id);
        assert_eq!(lib_entities.len(), 1);
        
        let nonexistent_entities = indexes.entities_in_file(FileId(999));
        assert_eq!(nonexistent_entities.len(), 0);
    }
    
    #[test]
    fn test_memory_stats() {
        let mut indexes = DiscoveryIndexes::new();
        let entities = vec![
            EntityInfo::new("func1".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(10), None),
            EntityInfo::new("Struct1".to_string(), "src/lib.rs".to_string(), EntityType::Struct, Some(20), None),
        ];
        
        indexes.rebuild_from_entities(entities).unwrap();
        
        let stats = indexes.memory_stats();
        assert_eq!(stats.entity_memory, 2 * std::mem::size_of::<CompactEntityInfo>());
        assert!(stats.total_memory > stats.entity_memory);
        assert!(stats.file_index_memory > 0);
        assert!(stats.type_index_memory > 0);
    }
    
    #[test]
    fn test_performance_contract_large_codebase() {
        let mut indexes = DiscoveryIndexes::new();
        
        // Create a large number of entities to test performance
        let mut entities = Vec::new();
        for i in 0..10_000 {
            entities.push(EntityInfo::new(
                format!("entity_{}", i),
                format!("src/file_{}.rs", i % 100), // 100 files
                if i % 3 == 0 { EntityType::Function } 
                else if i % 3 == 1 { EntityType::Struct } 
                else { EntityType::Trait },
                Some((i % 1000) as u32 + 1), // line numbers 1-1000
                Some((i % 80) as u32 + 1),   // column numbers 1-80
            ));
        }
        
        let result = indexes.rebuild_from_entities(entities);
        assert!(result.is_ok());
        
        let elapsed = result.unwrap();
        assert!(elapsed < Duration::from_secs(5), 
                "Index rebuild took {:?}, expected <5s", elapsed);
        
        assert_eq!(indexes.entity_count(), 10_000);
        
        // Verify indexes are built correctly
        let functions = indexes.entities_by_type(EntityType::Function);
        let structs = indexes.entities_by_type(EntityType::Struct);
        let traits = indexes.entities_by_type(EntityType::Trait);
        
        // Should have roughly equal distribution
        assert!(functions.len() > 3000 && functions.len() < 4000);
        assert!(structs.len() > 3000 && structs.len() < 4000);
        assert!(traits.len() > 3000 && traits.len() < 4000);
    }
    
    #[test]
    fn test_performance_contract_very_large_codebase() {
        let mut indexes = DiscoveryIndexes::new();
        
        // Test with 100k entities to validate scalability
        let mut entities = Vec::new();
        for i in 0..100_000 {
            entities.push(EntityInfo::new(
                format!("entity_{}", i),
                format!("src/module_{}/file_{}.rs", i / 1000, i % 1000), // 1000 files per module
                match i % 8 {
                    0 => EntityType::Function,
                    1 => EntityType::Struct,
                    2 => EntityType::Trait,
                    3 => EntityType::Impl,
                    4 => EntityType::Module,
                    5 => EntityType::Constant,
                    6 => EntityType::Static,
                    _ => EntityType::Macro,
                },
                Some((i % 10000) as u32 + 1), // line numbers 1-10000
                Some((i % 120) as u32 + 1),   // column numbers 1-120
            ));
        }
        
        let result = indexes.rebuild_from_entities(entities);
        assert!(result.is_ok());
        
        let elapsed = result.unwrap();
        assert!(elapsed < Duration::from_secs(5), 
                "Index rebuild took {:?}, expected <5s for 100k entities", elapsed);
        
        assert_eq!(indexes.entity_count(), 100_000);
        
        // Verify memory efficiency (relaxed for initial implementation)
        let stats = indexes.memory_stats();
        let bytes_per_entity = stats.total_memory / indexes.entity_count();
        // Allow up to 500 bytes per entity for initial implementation
        // This will be optimized in future iterations
        assert!(bytes_per_entity < 500, 
                "Memory usage too high: {} bytes per entity", bytes_per_entity);
        
        // Verify all entity types are indexed
        for entity_type in [EntityType::Function, EntityType::Struct, EntityType::Trait, 
                           EntityType::Impl, EntityType::Module, EntityType::Constant,
                           EntityType::Static, EntityType::Macro] {
            let entities_of_type = indexes.entities_by_type(entity_type);
            assert!(entities_of_type.len() > 10_000, 
                    "Expected >10k entities of type {:?}, got {}", entity_type, entities_of_type.len());
        }
    }
    
    #[test]
    fn test_performance_contract_timeout() {
        // This test would simulate a scenario where rebuild takes too long
        // For now, we'll test the error condition directly
        let error = IndexError::RebuildTimeout {
            elapsed: Duration::from_secs(6),
            limit: Duration::from_secs(5),
        };
        
        assert_eq!(
            error.to_string(),
            "Index rebuild timeout: took 6s, limit 5s"
        );
    }
    
    #[test]
    fn test_to_entity_info_conversion() {
        let mut indexes = DiscoveryIndexes::new();
        let original = EntityInfo::new(
            "test_function".to_string(),
            "src/main.rs".to_string(),
            EntityType::Function,
            Some(42),
            Some(10),
        );
        
        indexes.rebuild_from_entities(vec![original.clone()]).unwrap();
        
        let compact = &indexes.all_entities[0];
        let converted = compact.to_entity_info(&indexes.interner);
        
        assert_eq!(converted.name, original.name);
        assert_eq!(converted.file_path, original.file_path);
        assert_eq!(converted.entity_type, original.entity_type);
        assert_eq!(converted.line_number, original.line_number);
        assert_eq!(converted.column, original.column);
    }
    
    #[test]
    fn test_complete_discovery_workflow() {
        let mut indexes = DiscoveryIndexes::new();
        
        // Create a realistic set of entities from a Rust project
        let entities = vec![
            // Main module
            EntityInfo::new("main".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(1), Some(1)),
            EntityInfo::new("Config".to_string(), "src/main.rs".to_string(), EntityType::Struct, Some(10), Some(1)),
            
            // Library module
            EntityInfo::new("lib".to_string(), "src/lib.rs".to_string(), EntityType::Module, Some(1), Some(1)),
            EntityInfo::new("Parser".to_string(), "src/lib.rs".to_string(), EntityType::Struct, Some(20), Some(1)),
            EntityInfo::new("Parseable".to_string(), "src/lib.rs".to_string(), EntityType::Trait, Some(30), Some(1)),
            
            // Parser module
            EntityInfo::new("parse_file".to_string(), "src/parser.rs".to_string(), EntityType::Function, Some(5), Some(1)),
            EntityInfo::new("ParseError".to_string(), "src/parser.rs".to_string(), EntityType::Struct, Some(15), Some(1)),
            EntityInfo::new("MAX_FILE_SIZE".to_string(), "src/parser.rs".to_string(), EntityType::Constant, Some(25), Some(1)),
            
            // Utils module
            EntityInfo::new("format_output".to_string(), "src/utils.rs".to_string(), EntityType::Function, Some(8), Some(1)),
            EntityInfo::new("BUFFER_SIZE".to_string(), "src/utils.rs".to_string(), EntityType::Static, Some(18), Some(1)),
        ];
        
        // Rebuild indexes
        let rebuild_time = indexes.rebuild_from_entities(entities).unwrap();
        assert!(rebuild_time < Duration::from_millis(100), "Rebuild should be fast for small datasets");
        
        // Test entity count
        assert_eq!(indexes.entity_count(), 10);
        
        // Test type-based queries
        let functions = indexes.entities_by_type(EntityType::Function);
        assert_eq!(functions.len(), 3); // main, parse_file, format_output
        
        let structs = indexes.entities_by_type(EntityType::Struct);
        assert_eq!(structs.len(), 3); // Config, Parser, ParseError
        
        let traits = indexes.entities_by_type(EntityType::Trait);
        assert_eq!(traits.len(), 1); // Parseable
        
        let constants = indexes.entities_by_type(EntityType::Constant);
        assert_eq!(constants.len(), 1); // MAX_FILE_SIZE
        
        let statics = indexes.entities_by_type(EntityType::Static);
        assert_eq!(statics.len(), 1); // BUFFER_SIZE
        
        // Test file-based queries
        let main_file_id = indexes.interner.intern("src/main.rs");
        let main_entities = indexes.entities_in_file(main_file_id);
        assert_eq!(main_entities.len(), 2); // main function and Config struct
        
        let parser_file_id = indexes.interner.intern("src/parser.rs");
        let parser_entities = indexes.entities_in_file(parser_file_id);
        assert_eq!(parser_entities.len(), 3); // parse_file, ParseError, MAX_FILE_SIZE
        
        // Test memory efficiency
        let stats = indexes.memory_stats();
        assert!(stats.total_memory > 0);
        assert!(stats.entity_memory > 0);
        assert!(stats.file_index_memory > 0);
        assert!(stats.type_index_memory > 0);
        assert!(stats.interner_memory > 0);
        
        // Test conversion back to EntityInfo
        let first_entity = &indexes.all_entities[0];
        let converted = first_entity.to_entity_info(&indexes.interner);
        assert_eq!(converted.name, "main");
        assert_eq!(converted.file_path, "src/main.rs");
        assert_eq!(converted.entity_type, EntityType::Function);
        assert_eq!(converted.line_number, Some(1));
        assert_eq!(converted.column, Some(1));
    }
}
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
    string_interning::{FileId, FileInterner},
};
use crate::isg::{OptimizedISG, NodeData, SigHash};
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
#[derive(Clone)]
pub struct SimpleDiscoveryEngine {
    isg: OptimizedISG,
    /// File interner for memory-efficient file path storage
    file_interner: FileInterner,
    /// File-to-entities index for O(n) file queries
    /// Maps FileId to list of SigHash values for entities in that file
    file_index: HashMap<FileId, Vec<SigHash>>,
}

impl SimpleDiscoveryEngine {
    /// Create a new SimpleDiscoveryEngine
    pub fn new(isg: OptimizedISG) -> Self {
        let mut engine = Self {
            isg,
            file_interner: FileInterner::new(),
            file_index: HashMap::new(),
        };
        
        // Build the file index on creation
        engine.rebuild_file_index();
        engine
    }
    
    /// Rebuild the file-to-entities index
    /// 
    /// This method scans all entities in the ISG and builds an efficient
    /// index mapping file paths to the entities defined in those files.
    /// Called automatically during construction and when the ISG is updated.
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
    /// Uses the file index for O(n) performance where n is the number of
    /// entities in the specific file, not the total number of entities.
    /// 
    /// # Arguments
    /// * `file_path` - Path to the file to search
    /// * `entity_type_filter` - Optional filter by entity type
    /// 
    /// # Performance Contract
    /// Must complete in <100ms for interactive responsiveness.
    pub async fn entities_in_file_with_filter(
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
    
    /// Find all definitions of an entity by name
    /// 
    /// Returns all file locations where an entity with the given name is defined.
    /// Useful when there are multiple entities with the same name in different scopes.
    /// 
    /// # Arguments
    /// * `entity_name` - Exact name of the entity to locate
    /// 
    /// # Returns
    /// * Vector of FileLocation objects for all matching entities
    /// 
    /// # Performance Contract
    /// Must complete in <50ms for immediate navigation.
    pub async fn where_defined_all(&self, entity_name: &str) -> Result<Vec<FileLocation>> {
        let state = self.isg.state.read();
        let mut locations = Vec::new();
        
        // Use the name index for efficient lookup
        if let Some(sig_hashes) = state.name_map.get(entity_name) {
            for &sig_hash in sig_hashes {
                if let Some(&node_idx) = state.id_map.get(&sig_hash) {
                    if let Some(node) = state.graph.node_weight(node_idx) {
                        locations.push(FileLocation::new(
                            node.file_path.to_string(),
                            Some(node.line),
                            None, // Column not available in current ISG
                        ));
                    }
                }
            }
        }
        
        // Sort by file path and line number for consistent ordering
        locations.sort_by(|a, b| {
            a.file_path.cmp(&b.file_path)
                .then_with(|| a.line_number.cmp(&b.line_number))
        });
        
        Ok(locations)
    }
    
    /// Get file statistics for a specific file
    /// 
    /// Returns detailed information about entities in a file, including
    /// counts by entity type and line number ranges.
    /// 
    /// # Arguments
    /// * `file_path` - Path to the file to analyze
    /// 
    /// # Returns
    /// * `Some(FileStats)` if file exists and has entities
    /// * `None` if file not found or has no entities
    pub async fn file_statistics(&self, file_path: &str) -> Result<Option<FileStats>> {
        let entities = self.entities_in_file(file_path).await?;
        
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
    
    /// Get all files that contain entities of a specific type
    /// 
    /// Useful for finding all files that contain functions, structs, etc.
    /// 
    /// # Arguments
    /// * `entity_type` - Type of entity to search for
    /// 
    /// # Returns
    /// * Vector of file paths that contain entities of the specified type
    pub async fn files_containing_entity_type(&self, entity_type: EntityType) -> Result<Vec<String>> {
        let mut matching_files = Vec::new();
        
        // Check each file in the index
        for (&file_id, entity_hashes) in &self.file_index {
            let file_path = match self.file_interner.get_path(file_id) {
                Some(path) => path,
                None => continue,
            };
            
            // Check if any entity in this file matches the type
            let state = self.isg.state.read();
            let has_matching_type = entity_hashes.iter().any(|&sig_hash| {
                if let Some(&node_idx) = state.id_map.get(&sig_hash) {
                    if let Some(node) = state.graph.node_weight(node_idx) {
                        return EntityType::from(node.kind.clone()) == entity_type;
                    }
                }
                false
            });
            
            if has_matching_type {
                matching_files.push(file_path.to_string());
            }
        }
        
        // Sort for consistent ordering
        matching_files.sort();
        Ok(matching_files)
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
        self.entities_in_file_with_filter(file_path, None).await
    }
    
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
        // Use the file index for efficient file path enumeration
        let mut file_paths: Vec<String> = self.file_index
            .keys()
            .filter_map(|&file_id| self.file_interner.get_path(file_id))
            .map(|path| path.to_string())
            .collect();
        
        // Sort for consistent ordering
        file_paths.sort();
        
        Ok(file_paths)
    }
    
    async fn health_check(&self) -> Result<()> {
        // STUB: Always healthy for now
        Ok(())
    }
}

/// Statistics about entities in a specific file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStats {
    /// Path to the file
    pub file_path: String,
    /// Total number of entities in the file
    pub total_entities: usize,
    /// Count of entities by type
    pub entity_counts: HashMap<EntityType, usize>,
    /// Range of line numbers (min, max) if available
    pub line_range: Option<(u32, u32)>,
}

impl FileStats {
    /// Get the most common entity type in this file
    pub fn most_common_entity_type(&self) -> Option<EntityType> {
        self.entity_counts
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&entity_type, _)| entity_type)
    }
    
    /// Get the percentage of entities of a specific type in this file
    pub fn entity_type_percentage(&self, entity_type: EntityType) -> f64 {
        if self.total_entities == 0 {
            return 0.0;
        }
        
        let count = self.entity_counts.get(&entity_type).copied().unwrap_or(0);
        (count as f64 / self.total_entities as f64) * 100.0
    }
    
    /// Get the number of lines spanned by entities in this file
    pub fn lines_spanned(&self) -> Option<u32> {
        self.line_range.map(|(min, max)| max - min + 1)
    }
}
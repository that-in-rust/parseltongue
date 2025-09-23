//! String interning system for memory-efficient file path storage
//! 
//! Provides FileId and FileInterner for deduplicating file paths across the system.
//! This is critical for memory efficiency when dealing with large codebases where
//! many entities share the same file paths.

use fxhash::FxHashMap;
use std::sync::Arc;

/// Interned file path identifier for memory efficiency
/// 
/// Uses u32 to minimize memory footprint while supporting up to 4B unique file paths.
/// In practice, even large codebases rarely exceed 100K files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FileId(pub u32);

impl FileId {
    /// Create a new FileId from a raw u32 value
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    
    /// Get the raw u32 value
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

/// Thread-safe string interner for file paths
/// 
/// Provides memory-efficient storage by deduplicating identical file paths.
/// Uses Arc<str> for zero-copy sharing across threads.
/// 
/// # Performance Characteristics
/// - Insertion: O(1) average case with FxHashMap
/// - Lookup: O(1) average case
/// - Memory: Single allocation per unique path
/// 
/// # Thread Safety
/// Individual operations are not synchronized - use external synchronization 
/// if concurrent modification is needed.
#[derive(Debug, Clone)]
pub struct FileInterner {
    /// Map from file path to FileId for deduplication
    path_to_id: FxHashMap<Arc<str>, FileId>,
    /// Map from FileId to file path for reverse lookup
    id_to_path: FxHashMap<FileId, Arc<str>>,
    /// Next available FileId
    next_id: u32,
}

impl Default for FileInterner {
    fn default() -> Self {
        Self::new()
    }
}

impl FileInterner {
    /// Create a new empty FileInterner
    pub fn new() -> Self {
        Self {
            path_to_id: FxHashMap::default(),
            id_to_path: FxHashMap::default(),
            next_id: 0,
        }
    }
    
    /// Create a FileInterner with pre-allocated capacity
    /// 
    /// Use this when you know the approximate number of unique file paths
    /// to avoid hash map reallocations during bulk insertion.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            path_to_id: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            id_to_path: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            next_id: 0,
        }
    }
    
    /// Intern a file path and return its FileId
    /// 
    /// If the path is already interned, returns the existing FileId.
    /// Otherwise, creates a new FileId and stores the mapping.
    /// 
    /// # Thread Safety
    /// This method is NOT thread-safe. Use external synchronization for
    /// concurrent access.
    /// 
    /// # Example
    /// ```rust
    /// use parseltongue::discovery::FileInterner;
    /// 
    /// let mut interner = FileInterner::new();
    /// let id1 = interner.intern("src/main.rs");
    /// let id2 = interner.intern("src/main.rs");
    /// assert_eq!(id1, id2); // Same path returns same ID
    /// ```
    pub fn intern(&mut self, path: &str) -> FileId {
        let path_arc: Arc<str> = Arc::from(path);
        
        // Check if already interned
        if let Some(&existing_id) = self.path_to_id.get(&path_arc) {
            return existing_id;
        }
        
        // Create new FileId
        let id = FileId(self.next_id);
        self.next_id += 1;
        
        // Store mappings
        self.path_to_id.insert(path_arc.clone(), id);
        self.id_to_path.insert(id, path_arc);
        
        id
    }
    
    /// Get the file path for a FileId
    /// 
    /// Returns None if the FileId is not found in the interner.
    /// 
    /// # Example
    /// ```rust
    /// use parseltongue::discovery::FileInterner;
    /// 
    /// let mut interner = FileInterner::new();
    /// let id = interner.intern("src/main.rs");
    /// assert_eq!(interner.get_path(id), Some("src/main.rs"));
    /// ```
    pub fn get_path(&self, id: FileId) -> Option<&str> {
        self.id_to_path.get(&id).map(|arc| arc.as_ref())
    }
    
    /// Get the FileId for a file path
    /// 
    /// Returns None if the path has not been interned.
    /// 
    /// # Example
    /// ```rust
    /// use parseltongue::discovery::FileInterner;
    /// 
    /// let mut interner = FileInterner::new();
    /// let id = interner.intern("src/main.rs");
    /// assert_eq!(interner.get_id("src/main.rs"), Some(id));
    /// assert_eq!(interner.get_id("not/interned.rs"), None);
    /// ```
    pub fn get_id(&self, path: &str) -> Option<FileId> {
        let path_arc: Arc<str> = Arc::from(path);
        self.path_to_id.get(&path_arc).copied()
    }
    
    /// Get the number of interned file paths
    pub fn len(&self) -> usize {
        self.path_to_id.len()
    }
    
    /// Check if the interner is empty
    pub fn is_empty(&self) -> bool {
        self.path_to_id.is_empty()
    }
    
    /// Get all interned file paths
    /// 
    /// Returns an iterator over all file paths in the interner.
    /// Useful for debugging and analysis.
    pub fn paths(&self) -> impl Iterator<Item = &str> {
        self.id_to_path.values().map(|arc| arc.as_ref())
    }
    
    /// Get all FileIds
    /// 
    /// Returns an iterator over all FileIds in the interner.
    pub fn ids(&self) -> impl Iterator<Item = FileId> + '_ {
        self.id_to_path.keys().copied()
    }
    
    /// Get memory usage statistics
    /// 
    /// Returns approximate memory usage in bytes for the interner.
    /// Useful for performance monitoring and optimization.
    pub fn memory_usage(&self) -> MemoryUsage {
        let path_map_size = self.path_to_id.len() * (std::mem::size_of::<Arc<str>>() + std::mem::size_of::<FileId>());
        let id_map_size = self.id_to_path.len() * (std::mem::size_of::<FileId>() + std::mem::size_of::<Arc<str>>());
        
        // Estimate string storage (approximate)
        let string_storage: usize = self.id_to_path.values()
            .map(|arc| arc.len())
            .sum();
        
        MemoryUsage {
            path_map_bytes: path_map_size,
            id_map_bytes: id_map_size,
            string_storage_bytes: string_storage,
            total_entries: self.len(),
        }
    }
}

/// Memory usage statistics for FileInterner
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryUsage {
    /// Bytes used by path-to-id mapping
    pub path_map_bytes: usize,
    /// Bytes used by id-to-path mapping
    pub id_map_bytes: usize,
    /// Bytes used by string storage
    pub string_storage_bytes: usize,
    /// Total number of interned entries
    pub total_entries: usize,
}

impl MemoryUsage {
    /// Get total memory usage in bytes
    pub fn total_bytes(&self) -> usize {
        self.path_map_bytes + self.id_map_bytes + self.string_storage_bytes
    }
    
    /// Get average bytes per entry
    pub fn bytes_per_entry(&self) -> f64 {
        if self.total_entries == 0 {
            0.0
        } else {
            self.total_bytes() as f64 / self.total_entries as f64
        }
    }
}

#[cfg(test)]
mod string_interning_performance_tests;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_id_creation() {
        let id = FileId::new(42);
        assert_eq!(id.as_u32(), 42);
    }
    
    #[test]
    fn test_file_interner_basic_operations() {
        let mut interner = FileInterner::new();
        
        // Test interning
        let id1 = interner.intern("src/main.rs");
        let id2 = interner.intern("src/lib.rs");
        let id3 = interner.intern("src/main.rs"); // Duplicate
        
        // Same path should return same ID
        assert_eq!(id1, id3);
        assert_ne!(id1, id2);
        
        // Test reverse lookup
        assert_eq!(interner.get_path(id1), Some("src/main.rs"));
        assert_eq!(interner.get_path(id2), Some("src/lib.rs"));
        
        // Test forward lookup
        assert_eq!(interner.get_id("src/main.rs"), Some(id1));
        assert_eq!(interner.get_id("src/lib.rs"), Some(id2));
        assert_eq!(interner.get_id("not/found.rs"), None);
        
        // Test size
        assert_eq!(interner.len(), 2); // Only 2 unique paths
        assert!(!interner.is_empty());
    }
    
    #[test]
    fn test_file_interner_with_capacity() {
        let interner = FileInterner::with_capacity(100);
        assert_eq!(interner.len(), 0);
        assert!(interner.is_empty());
    }
    
    #[test]
    fn test_file_interner_iterators() {
        let mut interner = FileInterner::new();
        
        interner.intern("src/main.rs");
        interner.intern("src/lib.rs");
        interner.intern("tests/test.rs");
        
        let paths: Vec<&str> = interner.paths().collect();
        assert_eq!(paths.len(), 3);
        assert!(paths.contains(&"src/main.rs"));
        assert!(paths.contains(&"src/lib.rs"));
        assert!(paths.contains(&"tests/test.rs"));
        
        let ids: Vec<FileId> = interner.ids().collect();
        assert_eq!(ids.len(), 3);
    }
    
    #[test]
    fn test_memory_usage_calculation() {
        let mut interner = FileInterner::new();
        
        interner.intern("src/main.rs");
        interner.intern("src/lib.rs");
        
        let usage = interner.memory_usage();
        assert_eq!(usage.total_entries, 2);
        assert!(usage.total_bytes() > 0);
        assert!(usage.bytes_per_entry() > 0.0);
    }
    
    #[test]
    fn test_empty_interner_memory_usage() {
        let interner = FileInterner::new();
        let usage = interner.memory_usage();
        
        assert_eq!(usage.total_entries, 0);
        assert_eq!(usage.bytes_per_entry(), 0.0);
    }
}
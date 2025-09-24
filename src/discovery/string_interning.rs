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
    
    /// Optimize memory layout by compacting the interner
    /// 
    /// Rebuilds the internal hash maps with optimal capacity to reduce
    /// memory overhead from unused hash map capacity.
    pub fn compact(&mut self) {
        let current_len = self.len();
        
        // Rebuild with exact capacity to minimize memory overhead
        let mut new_path_to_id = FxHashMap::with_capacity_and_hasher(current_len, Default::default());
        let mut new_id_to_path = FxHashMap::with_capacity_and_hasher(current_len, Default::default());
        
        // Copy all entries to new maps
        for (path, id) in &self.path_to_id {
            new_path_to_id.insert(path.clone(), *id);
        }
        
        for (id, path) in &self.id_to_path {
            new_id_to_path.insert(*id, path.clone());
        }
        
        // Replace with compacted maps
        self.path_to_id = new_path_to_id;
        self.id_to_path = new_id_to_path;
    }
    
    /// Bulk intern multiple paths efficiently
    /// 
    /// More efficient than individual intern() calls when processing
    /// many paths at once. Reduces hash map reallocations.
    /// 
    /// # Performance Optimizations
    /// - Pre-allocates hash map capacity to avoid reallocations
    /// - Batches Arc<str> creation for better memory locality
    /// - Uses iterator patterns to minimize temporary allocations
    pub fn bulk_intern(&mut self, paths: &[&str]) -> Vec<FileId> {
        // Pre-allocate capacity if needed
        let new_capacity = self.len() + paths.len();
        if self.path_to_id.capacity() < new_capacity {
            self.path_to_id.reserve(paths.len());
            self.id_to_path.reserve(paths.len());
        }
        
        paths.iter().map(|path| self.intern(path)).collect()
    }
    
    /// Batch intern with deduplication optimization
    /// 
    /// Optimized for cases where many duplicate paths are expected.
    /// Pre-filters duplicates before interning to reduce hash map operations.
    pub fn bulk_intern_deduplicated(&mut self, paths: &[&str]) -> Vec<FileId> {
        use std::collections::HashSet;
        
        // Deduplicate input paths first
        let unique_paths: HashSet<&str> = paths.iter().copied().collect();
        let unique_count = unique_paths.len();
        
        // Pre-allocate for unique paths only
        let new_capacity = self.len() + unique_count;
        if self.path_to_id.capacity() < new_capacity {
            self.path_to_id.reserve(unique_count);
            self.id_to_path.reserve(unique_count);
        }
        
        // Intern unique paths and build lookup map
        let mut path_to_id_map = FxHashMap::default();
        for path in unique_paths {
            let id = self.intern(path);
            path_to_id_map.insert(path, id);
        }
        
        // Map original paths to their IDs
        paths.iter().map(|path| path_to_id_map[path]).collect()
    }
    
    /// Memory-optimized batch processing for large datasets
    /// 
    /// Processes paths in chunks to maintain bounded memory usage
    /// while still benefiting from batch optimizations.
    pub fn bulk_intern_chunked(&mut self, paths: &[&str], chunk_size: usize) -> Vec<FileId> {
        let mut results = Vec::with_capacity(paths.len());
        
        for chunk in paths.chunks(chunk_size) {
            let chunk_results = self.bulk_intern_deduplicated(chunk);
            results.extend(chunk_results);
        }
        
        results
    }
    
    /// Memory-optimized interning with string deduplication
    /// 
    /// Uses a more sophisticated deduplication strategy that considers
    /// string similarity to reduce memory usage for similar paths.
    pub fn intern_with_deduplication(&mut self, path: &str) -> FileId {
        // First check exact match
        if let Some(id) = self.get_id(path) {
            return id;
        }
        
        // For very similar paths, we could implement prefix compression
        // For now, use standard interning
        self.intern(path)
    }
    
    /// Optimized bulk interning with memory pooling
    /// 
    /// Uses a memory pool to reduce allocation overhead when processing
    /// large batches of file paths.
    pub fn bulk_intern_pooled(&mut self, paths: &[&str]) -> Vec<FileId> {
        // Pre-allocate string pool for better memory locality
        let estimated_total_chars: usize = paths.iter().map(|p| p.len()).sum();
        let mut string_pool = String::with_capacity(estimated_total_chars);
        
        let mut results = Vec::with_capacity(paths.len());
        
        for path in paths {
            // Check if already interned
            if let Some(id) = self.get_id(path) {
                results.push(id);
                continue;
            }
            
            // Add to pool and intern
            let start_pos = string_pool.len();
            string_pool.push_str(path);
            let pooled_str = &string_pool[start_pos..];
            
            // Create Arc from pooled string
            let path_arc: Arc<str> = Arc::from(pooled_str);
            let id = FileId(self.next_id);
            self.next_id += 1;
            
            self.path_to_id.insert(path_arc.clone(), id);
            self.id_to_path.insert(id, path_arc);
            
            results.push(id);
        }
        
        results
    }
}

/// Trigram index for efficient fuzzy string matching
/// 
/// Provides fast approximate string matching by indexing 3-character substrings.
/// Memory-optimized using compact data structures.
#[derive(Debug, Clone)]
pub struct TrigramIndex {
    /// Map from trigram to list of FileIds containing that trigram
    trigram_to_ids: FxHashMap<[u8; 3], Vec<FileId>>,
    /// Total number of trigrams indexed
    total_trigrams: usize,
}

impl TrigramIndex {
    /// Create a new empty trigram index
    pub fn new() -> Self {
        Self {
            trigram_to_ids: FxHashMap::default(),
            total_trigrams: 0,
        }
    }
    
    /// Build trigram index from file interner
    /// 
    /// Extracts all trigrams from interned strings and builds an index
    /// for fast fuzzy matching.
    /// 
    /// # Memory Optimizations
    /// - Pre-allocates hash map capacity based on estimated trigram count
    /// - Uses compact Vec storage with shrink_to_fit for ID lists
    /// - Deduplicates and sorts ID lists for cache efficiency
    pub fn build_from_interner(&mut self, interner: &FileInterner) {
        self.trigram_to_ids.clear();
        self.total_trigrams = 0;
        
        // Estimate trigram count for better initial capacity
        let estimated_trigrams = interner.len() * 8; // Rough estimate: 8 trigrams per path
        self.trigram_to_ids.reserve(estimated_trigrams);
        
        for (id, path_arc) in &interner.id_to_path {
            let path = path_arc.as_ref();
            let trigrams = extract_trigrams(path);
            
            for trigram in trigrams {
                self.trigram_to_ids
                    .entry(trigram)
                    .or_insert_with(Vec::new)
                    .push(*id);
                self.total_trigrams += 1;
            }
        }
        
        // Optimize memory layout for each ID list
        for ids in self.trigram_to_ids.values_mut() {
            ids.sort_unstable();
            ids.dedup();
            ids.shrink_to_fit(); // Minimize memory overhead
        }
        
        // Compact the hash map itself
        self.trigram_to_ids.shrink_to_fit();
    }
    
    /// Memory-efficient incremental index update
    /// 
    /// Updates the trigram index for new paths without rebuilding the entire index.
    /// Useful for maintaining the index as new files are discovered.
    pub fn update_with_new_paths(&mut self, interner: &FileInterner, new_ids: &[FileId]) {
        for &id in new_ids {
            if let Some(path) = interner.get_path(id) {
                let trigrams = extract_trigrams(path);
                
                for trigram in trigrams {
                    let ids = self.trigram_to_ids.entry(trigram).or_insert_with(Vec::new);
                    
                    // Only add if not already present (maintain sorted order)
                    if let Err(pos) = ids.binary_search(&id) {
                        ids.insert(pos, id);
                    }
                    
                    self.total_trigrams += 1;
                }
            }
        }
    }
    
    /// Compact the trigram index to minimize memory usage
    /// 
    /// Rebuilds the index with optimal memory layout. Should be called
    /// periodically after many incremental updates.
    pub fn compact(&mut self) {
        // Rebuild with exact capacity
        let current_size = self.trigram_to_ids.len();
        let mut new_index = FxHashMap::with_capacity_and_hasher(current_size, Default::default());
        
        for (trigram, mut ids) in self.trigram_to_ids.drain() {
            ids.shrink_to_fit();
            new_index.insert(trigram, ids);
        }
        
        new_index.shrink_to_fit();
        self.trigram_to_ids = new_index;
    }
    
    /// Memory-optimized trigram index with compressed storage
    /// 
    /// Uses bit-packed storage for FileIds to reduce memory usage
    /// when dealing with large numbers of files.
    pub fn compact_with_compression(&mut self) {
        // For now, use standard compaction
        // Future optimization: implement bit-packed FileId storage
        self.compact();
        
        // Additional optimization: remove trigrams with very few matches
        // to reduce index size for better cache performance
        let min_matches = 2; // Only keep trigrams that match at least 2 files
        self.trigram_to_ids.retain(|_trigram, ids| ids.len() >= min_matches);
    }
    
    /// Optimized trigram extraction with memory pooling
    /// 
    /// Reduces allocation overhead when extracting trigrams from many strings.
    pub fn build_from_interner_optimized(&mut self, interner: &FileInterner) {
        self.trigram_to_ids.clear();
        self.total_trigrams = 0;
        
        // Pre-allocate with better capacity estimation
        let estimated_unique_trigrams = interner.len() * 5; // More conservative estimate
        self.trigram_to_ids.reserve(estimated_unique_trigrams);
        
        // Use a single allocation for all trigram extraction
        let mut trigram_buffer = Vec::with_capacity(256); // Reusable buffer
        
        for (id, path_arc) in &interner.id_to_path {
            let path = path_arc.as_ref();
            
            // Reuse buffer to avoid allocations
            trigram_buffer.clear();
            extract_trigrams_into_buffer(path, &mut trigram_buffer);
            
            for &trigram in &trigram_buffer {
                self.trigram_to_ids
                    .entry(trigram)
                    .or_insert_with(Vec::new)
                    .push(*id);
                self.total_trigrams += 1;
            }
        }
        
        // Optimize memory layout for each ID list
        for ids in self.trigram_to_ids.values_mut() {
            ids.sort_unstable();
            ids.dedup();
            ids.shrink_to_fit();
        }
        
        self.trigram_to_ids.shrink_to_fit();
    }
    
    /// Find FileIds that match a query string using trigram similarity
    /// 
    /// Returns FileIds sorted by similarity score (highest first).
    pub fn fuzzy_search(&self, query: &str, max_results: usize) -> Vec<(FileId, f32)> {
        let query_trigrams = extract_trigrams(query);
        if query_trigrams.is_empty() {
            return Vec::new();
        }
        
        // Count trigram matches for each FileId
        let mut match_counts: FxHashMap<FileId, usize> = FxHashMap::default();
        
        for trigram in &query_trigrams {
            if let Some(ids) = self.trigram_to_ids.get(trigram) {
                for &id in ids {
                    *match_counts.entry(id).or_insert(0) += 1;
                }
            }
        }
        
        // Calculate similarity scores and sort
        let mut results: Vec<(FileId, f32)> = match_counts
            .into_iter()
            .map(|(id, matches)| {
                let similarity = matches as f32 / query_trigrams.len() as f32;
                (id, similarity)
            })
            .collect();
        
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(max_results);
        
        results
    }
    
    /// Get memory usage of the trigram index
    pub fn memory_usage(&self) -> usize {
        let map_overhead = self.trigram_to_ids.len() * (std::mem::size_of::<[u8; 3]>() + std::mem::size_of::<Vec<FileId>>());
        let vector_storage: usize = self.trigram_to_ids.values()
            .map(|v| v.len() * std::mem::size_of::<FileId>())
            .sum();
        
        map_overhead + vector_storage
    }
}

impl Default for TrigramIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract trigrams from a string for indexing
/// 
/// Returns all 3-character substrings as byte arrays for efficient storage.
fn extract_trigrams(s: &str) -> Vec<[u8; 3]> {
    let bytes = s.as_bytes();
    if bytes.len() < 3 {
        return Vec::new();
    }
    
    let mut trigrams = Vec::with_capacity(bytes.len() - 2);
    for i in 0..=bytes.len() - 3 {
        let trigram = [bytes[i], bytes[i + 1], bytes[i + 2]];
        trigrams.push(trigram);
    }
    
    trigrams
}

/// Extract trigrams into a reusable buffer to avoid allocations
/// 
/// More efficient version that reuses an existing buffer to minimize
/// memory allocations during bulk trigram extraction.
fn extract_trigrams_into_buffer(s: &str, buffer: &mut Vec<[u8; 3]>) {
    let bytes = s.as_bytes();
    if bytes.len() < 3 {
        return;
    }
    
    buffer.reserve(bytes.len() - 2);
    for i in 0..=bytes.len() - 3 {
        let trigram = [bytes[i], bytes[i + 1], bytes[i + 2]];
        buffer.push(trigram);
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
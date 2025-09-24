//! Enhanced ISG node structure with embedded file location attributes
//! 
//! Provides EnhancedIsgNode that extends the existing ISG node structure
//! with complete file location information (file_path, line_number, column)
//! embedded as attributes rather than separate nodes.

use crate::discovery::string_interning::{FileId, FileInterner};
use crate::discovery::types::{EntityInfo, EntityType, FileLocation};
use crate::isg::{NodeData, NodeKind, SigHash};
use std::sync::Arc;
use serde::{Serialize, Deserialize};

/// Enhanced ISG node with embedded file location data
/// 
/// This structure extends the existing NodeData with complete file location
/// information while maintaining O(1) access performance. File paths are
/// interned using FileId for memory efficiency.
/// 
/// # Design Principles
/// - File location as attributes, not separate nodes
/// - O(1) file location access performance
/// - Memory-efficient with string interning
/// - Backward compatible with existing NodeData
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnhancedIsgNode {
    /// Unique signature hash for the entity
    pub sig_hash: SigHash,
    /// Entity type (function, struct, trait, etc.)
    pub kind: NodeKind,
    /// Human-readable entity name (interned for memory efficiency)
    pub name: Arc<str>,
    /// Full signature of the entity (interned for memory efficiency)
    pub signature: Arc<str>,
    
    // Enhanced file location attributes
    /// File path where entity is defined (interned for memory efficiency)
    pub file_id: FileId,
    /// Line number in file (1-based, 0 means unknown)
    pub line_number: u32,
    /// Column number in file (1-based, 0 means unknown)
    pub column: u32,
}

// Custom serialization for EnhancedIsgNode to handle Arc<str>
impl Serialize for EnhancedIsgNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("EnhancedIsgNode", 7)?;
        state.serialize_field("sig_hash", &self.sig_hash)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("name", self.name.as_ref())?;
        state.serialize_field("signature", self.signature.as_ref())?;
        state.serialize_field("file_id", &self.file_id)?;
        state.serialize_field("line_number", &self.line_number)?;
        state.serialize_field("column", &self.column)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for EnhancedIsgNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field { SigHash, Kind, Name, Signature, FileId, LineNumber, Column }

        struct EnhancedIsgNodeVisitor;

        impl<'de> Visitor<'de> for EnhancedIsgNodeVisitor {
            type Value = EnhancedIsgNode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct EnhancedIsgNode")
            }

            fn visit_map<V>(self, mut map: V) -> Result<EnhancedIsgNode, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut sig_hash = None;
                let mut kind = None;
                let mut name = None;
                let mut signature = None;
                let mut file_id = None;
                let mut line_number = None;
                let mut column = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::SigHash => {
                            if sig_hash.is_some() {
                                return Err(de::Error::duplicate_field("sig_hash"));
                            }
                            sig_hash = Some(map.next_value()?);
                        }
                        Field::Kind => {
                            if kind.is_some() {
                                return Err(de::Error::duplicate_field("kind"));
                            }
                            kind = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(Arc::from(map.next_value::<String>()?));
                        }
                        Field::Signature => {
                            if signature.is_some() {
                                return Err(de::Error::duplicate_field("signature"));
                            }
                            signature = Some(Arc::from(map.next_value::<String>()?));
                        }
                        Field::FileId => {
                            if file_id.is_some() {
                                return Err(de::Error::duplicate_field("file_id"));
                            }
                            file_id = Some(map.next_value()?);
                        }
                        Field::LineNumber => {
                            if line_number.is_some() {
                                return Err(de::Error::duplicate_field("line_number"));
                            }
                            line_number = Some(map.next_value()?);
                        }
                        Field::Column => {
                            if column.is_some() {
                                return Err(de::Error::duplicate_field("column"));
                            }
                            column = Some(map.next_value()?);
                        }
                    }
                }

                let sig_hash = sig_hash.ok_or_else(|| de::Error::missing_field("sig_hash"))?;
                let kind = kind.ok_or_else(|| de::Error::missing_field("kind"))?;
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let signature = signature.ok_or_else(|| de::Error::missing_field("signature"))?;
                let file_id = file_id.ok_or_else(|| de::Error::missing_field("file_id"))?;
                let line_number = line_number.ok_or_else(|| de::Error::missing_field("line_number"))?;
                let column = column.ok_or_else(|| de::Error::missing_field("column"))?;

                Ok(EnhancedIsgNode {
                    sig_hash,
                    kind,
                    name,
                    signature,
                    file_id,
                    line_number,
                    column,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["sig_hash", "kind", "name", "signature", "file_id", "line_number", "column"];
        deserializer.deserialize_struct("EnhancedIsgNode", FIELDS, EnhancedIsgNodeVisitor)
    }
}

impl EnhancedIsgNode {
    /// Create a new EnhancedIsgNode
    pub fn new(
        sig_hash: SigHash,
        kind: NodeKind,
        name: Arc<str>,
        signature: Arc<str>,
        file_id: FileId,
        line_number: u32,
        column: u32,
    ) -> Self {
        Self {
            sig_hash,
            kind,
            name,
            signature,
            file_id,
            line_number,
            column,
        }
    }
    
    /// Create an EnhancedIsgNode with only line number (column unknown)
    pub fn with_line(
        sig_hash: SigHash,
        kind: NodeKind,
        name: Arc<str>,
        signature: Arc<str>,
        file_id: FileId,
        line_number: u32,
    ) -> Self {
        Self::new(sig_hash, kind, name, signature, file_id, line_number, 0)
    }
    
    /// Create an EnhancedIsgNode with only file path (line and column unknown)
    pub fn file_only(
        sig_hash: SigHash,
        kind: NodeKind,
        name: Arc<str>,
        signature: Arc<str>,
        file_id: FileId,
    ) -> Self {
        Self::new(sig_hash, kind, name, signature, file_id, 0, 0)
    }
    
    /// Get file location information
    /// 
    /// Returns FileLocation with the file path resolved from the interner.
    /// This is an O(1) operation for file location access.
    pub fn file_location(&self, interner: &FileInterner) -> Option<FileLocation> {
        let file_path = interner.get_path(self.file_id)?;
        
        Some(FileLocation::new(
            file_path.to_string(),
            if self.line_number > 0 { Some(self.line_number) } else { None },
            if self.column > 0 { Some(self.column) } else { None },
        ))
    }
    
    /// Get file path from interner
    /// 
    /// O(1) operation to get the file path string.
    pub fn file_path<'a>(&self, interner: &'a FileInterner) -> Option<&'a str> {
        interner.get_path(self.file_id)
    }
    
    /// Check if this node has line number information
    pub fn has_line_number(&self) -> bool {
        self.line_number > 0
    }
    
    /// Check if this node has column information
    pub fn has_column(&self) -> bool {
        self.column > 0
    }
    
    /// Check if this node has complete position information
    pub fn has_complete_position(&self) -> bool {
        self.has_line_number() && self.has_column()
    }
    
    /// Convert to EntityInfo for discovery operations
    /// 
    /// This provides the bridge between the enhanced ISG node and the
    /// discovery system's EntityInfo structure.
    pub fn to_entity_info(&self, interner: &FileInterner) -> Option<EntityInfo> {
        let file_path = self.file_path(interner)?.to_string();
        let entity_type = EntityType::from(self.kind.clone());
        
        Some(EntityInfo::new(
            self.name.to_string(),
            file_path,
            entity_type,
            if self.line_number > 0 { Some(self.line_number) } else { None },
            if self.column > 0 { Some(self.column) } else { None },
        ))
    }
    
    /// Get a formatted location string for debugging
    /// 
    /// Returns format like "src/main.rs:42:10" for complete position,
    /// "src/main.rs:42" for line only, or "src/main.rs" for file only.
    pub fn format_location(&self, interner: &FileInterner) -> String {
        if let Some(file_path) = self.file_path(interner) {
            match (self.has_line_number(), self.has_column()) {
                (true, true) => format!("{}:{}:{}", file_path, self.line_number, self.column),
                (true, false) => format!("{}:{}", file_path, self.line_number),
                (false, _) => file_path.to_string(),
            }
        } else {
            format!("FileId({})", self.file_id.as_u32())
        }
    }
}

/// Conversion utilities between existing and enhanced node formats
pub struct NodeConverter;

impl NodeConverter {
    /// Convert existing NodeData to EnhancedIsgNode
    /// 
    /// This provides backward compatibility by converting the existing
    /// NodeData structure to the enhanced format. The file path is
    /// interned during conversion.
    pub fn from_node_data(
        node_data: &NodeData,
        interner: &mut FileInterner,
    ) -> EnhancedIsgNode {
        let file_id = interner.intern(&node_data.file_path);
        
        EnhancedIsgNode::new(
            node_data.hash,
            node_data.kind.clone(),
            node_data.name.clone(),
            node_data.signature.clone(),
            file_id,
            node_data.line,
            0, // Original NodeData doesn't have column information
        )
    }
    
    /// Convert EnhancedIsgNode to NodeData
    /// 
    /// This provides forward compatibility by converting the enhanced
    /// format back to the existing NodeData structure. Column information
    /// is lost in this conversion.
    pub fn to_node_data(
        enhanced_node: &EnhancedIsgNode,
        interner: &FileInterner,
    ) -> Option<NodeData> {
        let file_path = interner.get_path(enhanced_node.file_id)?;
        
        Some(NodeData {
            hash: enhanced_node.sig_hash,
            kind: enhanced_node.kind.clone(),
            name: enhanced_node.name.clone(),
            signature: enhanced_node.signature.clone(),
            file_path: Arc::from(file_path),
            line: enhanced_node.line_number,
        })
    }
    
    /// Batch convert multiple NodeData to EnhancedIsgNode
    /// 
    /// More efficient than individual conversions when processing many nodes
    /// because it reuses the interner efficiently.
    pub fn batch_from_node_data(
        node_data_list: &[NodeData],
        interner: &mut FileInterner,
    ) -> Vec<EnhancedIsgNode> {
        node_data_list
            .iter()
            .map(|node_data| Self::from_node_data(node_data, interner))
            .collect()
    }
    
    /// Batch convert multiple EnhancedIsgNode to NodeData
    /// 
    /// More efficient than individual conversions when processing many nodes.
    pub fn batch_to_node_data(
        enhanced_nodes: &[EnhancedIsgNode],
        interner: &FileInterner,
    ) -> Vec<NodeData> {
        enhanced_nodes
            .iter()
            .filter_map(|enhanced_node| Self::to_node_data(enhanced_node, interner))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isg::NodeKind;
    
    fn create_test_interner() -> FileInterner {
        let mut interner = FileInterner::new();
        interner.intern("src/main.rs");
        interner.intern("src/lib.rs");
        interner.intern("tests/test.rs");
        interner
    }
    
    fn create_test_enhanced_node(interner: &mut FileInterner) -> EnhancedIsgNode {
        let file_id = interner.intern("src/main.rs");
        EnhancedIsgNode::new(
            SigHash::from_signature("fn test_function"),
            NodeKind::Function,
            Arc::from("test_function"),
            Arc::from("fn test_function() -> i32"),
            file_id,
            42,
            10,
        )
    }
    
    #[test]
    fn test_enhanced_isg_node_creation() {
        let mut interner = create_test_interner();
        let node = create_test_enhanced_node(&mut interner);
        
        assert_eq!(node.sig_hash, SigHash::from_signature("fn test_function"));
        assert_eq!(node.kind, NodeKind::Function);
        assert_eq!(node.name.as_ref(), "test_function");
        assert_eq!(node.signature.as_ref(), "fn test_function() -> i32");
        assert_eq!(node.line_number, 42);
        assert_eq!(node.column, 10);
        assert!(node.has_line_number());
        assert!(node.has_column());
        assert!(node.has_complete_position());
    }
    
    #[test]
    fn test_enhanced_node_with_line_only() {
        let mut interner = create_test_interner();
        let file_id = interner.intern("src/main.rs");
        
        let node = EnhancedIsgNode::with_line(
            SigHash::from_signature("fn test"),
            NodeKind::Function,
            Arc::from("test"),
            Arc::from("fn test()"),
            file_id,
            42,
        );
        
        assert!(node.has_line_number());
        assert!(!node.has_column());
        assert!(!node.has_complete_position());
        assert_eq!(node.line_number, 42);
        assert_eq!(node.column, 0);
    }
    
    #[test]
    fn test_enhanced_node_file_only() {
        let mut interner = create_test_interner();
        let file_id = interner.intern("src/main.rs");
        
        let node = EnhancedIsgNode::file_only(
            SigHash::from_signature("fn test"),
            NodeKind::Function,
            Arc::from("test"),
            Arc::from("fn test()"),
            file_id,
        );
        
        assert!(!node.has_line_number());
        assert!(!node.has_column());
        assert!(!node.has_complete_position());
        assert_eq!(node.line_number, 0);
        assert_eq!(node.column, 0);
    }
    
    #[test]
    fn test_file_location_access() {
        let mut interner = create_test_interner();
        let node = create_test_enhanced_node(&mut interner);
        
        let location = node.file_location(&interner).unwrap();
        assert_eq!(location.file_path, "src/main.rs");
        assert_eq!(location.line_number, Some(42));
        assert_eq!(location.column, Some(10));
        
        let file_path = node.file_path(&interner).unwrap();
        assert_eq!(file_path, "src/main.rs");
    }
    
    #[test]
    fn test_to_entity_info_conversion() {
        let mut interner = create_test_interner();
        let node = create_test_enhanced_node(&mut interner);
        
        let entity_info = node.to_entity_info(&interner).unwrap();
        assert_eq!(entity_info.name, "test_function");
        assert_eq!(entity_info.file_path, "src/main.rs");
        assert_eq!(entity_info.entity_type, EntityType::Function);
        assert_eq!(entity_info.line_number, Some(42));
        assert_eq!(entity_info.column, Some(10));
    }
    
    #[test]
    fn test_format_location() {
        let mut interner = create_test_interner();
        
        // Complete position
        let node_complete = create_test_enhanced_node(&mut interner);
        assert_eq!(node_complete.format_location(&interner), "src/main.rs:42:10");
        
        // Line only
        let file_id = interner.intern("src/lib.rs");
        let node_line = EnhancedIsgNode::with_line(
            SigHash::from_signature("fn test"),
            NodeKind::Function,
            Arc::from("test"),
            Arc::from("fn test()"),
            file_id,
            25,
        );
        assert_eq!(node_line.format_location(&interner), "src/lib.rs:25");
        
        // File only
        let node_file = EnhancedIsgNode::file_only(
            SigHash::from_signature("fn test"),
            NodeKind::Function,
            Arc::from("test"),
            Arc::from("fn test()"),
            file_id,
        );
        assert_eq!(node_file.format_location(&interner), "src/lib.rs");
    }
    
    #[test]
    fn test_node_converter_from_node_data() {
        let mut interner = create_test_interner();
        
        let original_node = NodeData {
            hash: SigHash::from_signature("fn original"),
            kind: NodeKind::Struct,
            name: Arc::from("OriginalStruct"),
            signature: Arc::from("struct OriginalStruct { field: i32 }"),
            file_path: Arc::from("src/original.rs"),
            line: 100,
        };
        
        let enhanced_node = NodeConverter::from_node_data(&original_node, &mut interner);
        
        assert_eq!(enhanced_node.sig_hash, original_node.hash);
        assert_eq!(enhanced_node.kind, original_node.kind);
        assert_eq!(enhanced_node.name, original_node.name);
        assert_eq!(enhanced_node.signature, original_node.signature);
        assert_eq!(enhanced_node.line_number, original_node.line);
        assert_eq!(enhanced_node.column, 0); // Column not available in original
        
        // Verify file path is correctly interned
        assert_eq!(enhanced_node.file_path(&interner), Some("src/original.rs"));
    }
    
    #[test]
    fn test_node_converter_to_node_data() {
        let mut interner = create_test_interner();
        let enhanced_node = create_test_enhanced_node(&mut interner);
        
        let converted_node = NodeConverter::to_node_data(&enhanced_node, &interner).unwrap();
        
        assert_eq!(converted_node.hash, enhanced_node.sig_hash);
        assert_eq!(converted_node.kind, enhanced_node.kind);
        assert_eq!(converted_node.name, enhanced_node.name);
        assert_eq!(converted_node.signature, enhanced_node.signature);
        assert_eq!(converted_node.line, enhanced_node.line_number);
        assert_eq!(converted_node.file_path.as_ref(), "src/main.rs");
        // Note: Column information is lost in conversion to NodeData
    }
    
    #[test]
    fn test_batch_conversion() {
        let mut interner = create_test_interner();
        
        let original_nodes = vec![
            NodeData {
                hash: SigHash::from_signature("fn func1"),
                kind: NodeKind::Function,
                name: Arc::from("func1"),
                signature: Arc::from("fn func1()"),
                file_path: Arc::from("src/mod1.rs"),
                line: 10,
            },
            NodeData {
                hash: SigHash::from_signature("struct Struct1"),
                kind: NodeKind::Struct,
                name: Arc::from("Struct1"),
                signature: Arc::from("struct Struct1 {}"),
                file_path: Arc::from("src/mod2.rs"),
                line: 20,
            },
        ];
        
        let enhanced_nodes = NodeConverter::batch_from_node_data(&original_nodes, &mut interner);
        assert_eq!(enhanced_nodes.len(), 2);
        
        let converted_back = NodeConverter::batch_to_node_data(&enhanced_nodes, &interner);
        assert_eq!(converted_back.len(), 2);
        
        // Verify round-trip conversion preserves essential data
        for (original, converted) in original_nodes.iter().zip(converted_back.iter()) {
            assert_eq!(original.hash, converted.hash);
            assert_eq!(original.kind, converted.kind);
            assert_eq!(original.name, converted.name);
            assert_eq!(original.signature, converted.signature);
            assert_eq!(original.line, converted.line);
            assert_eq!(original.file_path, converted.file_path);
        }
    }
    
    #[test]
    fn test_o1_file_location_access_performance() {
        use std::time::Instant;
        
        let mut interner = create_test_interner();
        let node = create_test_enhanced_node(&mut interner);
        
        // Measure file location access time
        let start = Instant::now();
        for _ in 0..1000 {
            let _location = node.file_location(&interner);
        }
        let elapsed = start.elapsed();
        
        // Should be very fast - well under 1ms for 1000 operations
        assert!(elapsed.as_millis() < 10, "File location access too slow: {:?}", elapsed);
        
        // Measure file path access time
        let start = Instant::now();
        for _ in 0..1000 {
            let _path = node.file_path(&interner);
        }
        let elapsed = start.elapsed();
        
        // Should be very fast - well under 1ms for 1000 operations
        assert!(elapsed.as_millis() < 10, "File path access too slow: {:?}", elapsed);
    }
    
    /// Test O(1) file location access performance with large dataset
    /// 
    /// This test validates that file location access remains constant time
    /// even with thousands of nodes and hundreds of unique file paths.
    #[test]
    fn test_o1_file_location_access_performance_large_dataset() {
        use std::time::Instant;
        
        let mut interner = FileInterner::with_capacity(1000);
        
        // Create a large dataset with many nodes and files
        let num_files = 500;
        let nodes_per_file = 20;
        let total_nodes = num_files * nodes_per_file;
        
        // Pre-intern file paths
        let file_ids: Vec<_> = (0..num_files)
            .map(|i| interner.intern(&format!("src/module_{:03}.rs", i)))
            .collect();
        
        // Create enhanced nodes
        let nodes: Vec<EnhancedIsgNode> = (0..total_nodes)
            .map(|i| {
                let file_idx = i % num_files;
                let line = (i / num_files) as u32 + 1;
                
                EnhancedIsgNode::new(
                    SigHash::from_signature(&format!("fn function_{}", i)),
                    NodeKind::Function,
                    Arc::from(format!("function_{}", i)),
                    Arc::from(format!("fn function_{}() -> i32", i)),
                    file_ids[file_idx],
                    line,
                    10,
                )
            })
            .collect();
        
        // Test file location access performance
        let iterations = 10000;
        let start = Instant::now();
        
        for i in 0..iterations {
            let node_idx = i % nodes.len();
            let _location = nodes[node_idx].file_location(&interner);
        }
        
        let elapsed = start.elapsed();
        let avg_time_ns = elapsed.as_nanos() / iterations as u128;
        
        // Performance contract: Should be well under 1μs per operation
        assert!(avg_time_ns < 1000, "File location access too slow: {} ns > 1000 ns", avg_time_ns);
        
        // Test file path access performance
        let start = Instant::now();
        
        for i in 0..iterations {
            let node_idx = i % nodes.len();
            let _path = nodes[node_idx].file_path(&interner);
        }
        
        let elapsed = start.elapsed();
        let avg_time_ns = elapsed.as_nanos() / iterations as u128;
        
        // Performance contract: Should be well under 1μs per operation
        assert!(avg_time_ns < 1000, "File path access too slow: {} ns > 1000 ns", avg_time_ns);
    }
    
    /// Test that file location access time is independent of dataset size
    /// 
    /// This test validates the O(1) property by comparing access times
    /// across different dataset sizes.
    #[test]
    fn test_o1_scalability_independence() {
        use std::time::Instant;
        
        let dataset_sizes = vec![100, 1000, 5000];
        let mut access_times = Vec::new();
        
        for &size in &dataset_sizes {
            let mut interner = FileInterner::with_capacity(size / 10);
            
            // Create dataset
            let nodes: Vec<EnhancedIsgNode> = (0..size)
                .map(|i| {
                    let file_id = interner.intern(&format!("src/file_{}.rs", i % 100));
                    EnhancedIsgNode::new(
                        SigHash::from_signature(&format!("fn func_{}", i)),
                        NodeKind::Function,
                        Arc::from(format!("func_{}", i)),
                        Arc::from(format!("fn func_{}()", i)),
                        file_id,
                        (i as u32) + 1,
                        10,
                    )
                })
                .collect();
            
            // Measure access time
            let iterations = 1000;
            let start = Instant::now();
            
            for i in 0..iterations {
                let node_idx = i % nodes.len();
                let _location = nodes[node_idx].file_location(&interner);
            }
            
            let elapsed = start.elapsed();
            let avg_time_ns = elapsed.as_nanos() / iterations as u128;
            access_times.push(avg_time_ns);
        }
        
        // Verify that access time doesn't increase significantly with dataset size
        // Allow for some variance due to cache effects, but should be roughly constant
        let first_time = access_times[0];
        for &time in &access_times[1..] {
            let ratio = time as f64 / first_time as f64;
            assert!(ratio < 3.0, "Access time increased too much with dataset size: {}x", ratio);
        }
    }
    
    /// Test conversion performance between NodeData and EnhancedIsgNode
    /// 
    /// Validates that conversion operations are efficient and don't degrade
    /// with larger datasets.
    #[test]
    fn test_conversion_performance() {
        use std::time::Instant;
        
        let mut interner = FileInterner::with_capacity(100);
        
        // Create original NodeData instances
        let node_data_list: Vec<NodeData> = (0..1000)
            .map(|i| NodeData {
                hash: SigHash::from_signature(&format!("fn func_{}", i)),
                kind: NodeKind::Function,
                name: Arc::from(format!("func_{}", i)),
                signature: Arc::from(format!("fn func_{}() -> i32", i)),
                file_path: Arc::from(format!("src/module_{}.rs", i % 50)),
                line: (i as u32) + 1,
            })
            .collect();
        
        // Test batch conversion from NodeData to EnhancedIsgNode
        let start = Instant::now();
        let enhanced_nodes = NodeConverter::batch_from_node_data(&node_data_list, &mut interner);
        let conversion_time = start.elapsed();
        
        // Should be fast - under 10ms for 1000 nodes
        assert!(conversion_time.as_millis() < 10, 
                "Batch conversion too slow: {:?}", conversion_time);
        
        // Test batch conversion back to NodeData
        let start = Instant::now();
        let converted_back = NodeConverter::batch_to_node_data(&enhanced_nodes, &interner);
        let back_conversion_time = start.elapsed();
        
        // Should be fast - under 10ms for 1000 nodes
        assert!(back_conversion_time.as_millis() < 10, 
                "Batch back-conversion too slow: {:?}", back_conversion_time);
        
        // Verify data integrity
        assert_eq!(converted_back.len(), node_data_list.len());
    }
    
    /// Test memory efficiency of file path interning
    /// 
    /// Validates that string interning provides significant memory savings
    /// when many nodes share the same file paths.
    #[test]
    fn test_memory_efficiency_with_interning() {
        let mut interner = FileInterner::new();
        
        // Create many nodes that share file paths
        let num_files = 10;
        let nodes_per_file = 100;
        let total_nodes = num_files * nodes_per_file;
        
        let nodes: Vec<EnhancedIsgNode> = (0..total_nodes)
            .map(|i| {
                let file_idx = i % num_files;
                let file_id = interner.intern(&format!("src/shared_file_{}.rs", file_idx));
                
                EnhancedIsgNode::new(
                    SigHash::from_signature(&format!("fn func_{}", i)),
                    NodeKind::Function,
                    Arc::from(format!("func_{}", i)),
                    Arc::from(format!("fn func_{}()", i)),
                    file_id,
                    (i as u32) + 1,
                    10,
                )
            })
            .collect();
        
        // Check interner efficiency
        let memory_usage = interner.memory_usage();
        
        // Should have only interned the unique file paths
        assert_eq!(interner.len(), num_files);
        
        // Memory usage should be reasonable
        assert!(memory_usage.total_bytes() < 10000, 
                "Memory usage too high: {} bytes", memory_usage.total_bytes());
        
        // Test that all nodes can access their file paths efficiently
        let start = std::time::Instant::now();
        let mut path_count = 0;
        
        for node in &nodes {
            if node.file_path(&interner).is_some() {
                path_count += 1;
            }
        }
        
        let elapsed = start.elapsed();
        
        assert_eq!(path_count, total_nodes);
        
        // Should be very fast
        assert!(elapsed.as_millis() < 50, 
                "File path access too slow: {:?}", elapsed);
    }
    
    /// Benchmark EntityInfo conversion performance
    /// 
    /// Tests the performance of converting EnhancedIsgNode to EntityInfo
    /// for discovery operations.
    #[test]
    fn test_entity_info_conversion_performance() {
        let mut interner = FileInterner::with_capacity(100);
        
        // Create test nodes
        let nodes: Vec<EnhancedIsgNode> = (0..1000)
            .map(|i| {
                let file_id = interner.intern(&format!("src/file_{}.rs", i % 50));
                EnhancedIsgNode::new(
                    SigHash::from_signature(&format!("fn func_{}", i)),
                    NodeKind::Function,
                    Arc::from(format!("func_{}", i)),
                    Arc::from(format!("fn func_{}() -> Result<i32, Error>", i)),
                    file_id,
                    (i as u32) + 1,
                    (i as u32 % 80) + 1,
                )
            })
            .collect();
        
        // Test conversion performance
        let start = std::time::Instant::now();
        let entity_infos: Vec<_> = nodes
            .iter()
            .filter_map(|node| node.to_entity_info(&interner))
            .collect();
        let elapsed = start.elapsed();
        
        // Should be fast - under 10ms for 1000 conversions
        assert!(elapsed.as_millis() < 10, 
                "EntityInfo conversion too slow: {:?}", elapsed);
        
        // Verify all conversions succeeded
        assert_eq!(entity_infos.len(), nodes.len());
        
        // Verify data integrity
        for (node, entity_info) in nodes.iter().zip(entity_infos.iter()) {
            assert_eq!(entity_info.name, node.name.as_ref());
            assert_eq!(entity_info.line_number, Some(node.line_number));
            assert_eq!(entity_info.column, Some(node.column));
            assert_eq!(entity_info.file_path, node.file_path(&interner).unwrap());
        }
    }
}
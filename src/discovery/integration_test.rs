//! Integration test for EnhancedIsgNode with discovery system
//! 
//! This test demonstrates that the enhanced ISG node structure integrates
//! correctly with the discovery system and provides the expected O(1)
//! file location access performance.

#[cfg(test)]
mod integration_tests {
    use crate::discovery::enhanced_isg_node::{EnhancedIsgNode, NodeConverter};
    use crate::discovery::string_interning::FileInterner;
    use crate::discovery::types::{EntityInfo, EntityType};
    use crate::isg::{NodeData, NodeKind, SigHash};
    use std::sync::Arc;
    use std::time::Instant;

    /// Integration test: Convert existing NodeData to EnhancedIsgNode and verify discovery functionality
    #[test]
    fn test_enhanced_node_discovery_integration() {
        let mut interner = FileInterner::new();
        
        // Create sample NodeData (simulating existing ISG data)
        let original_nodes = vec![
            NodeData {
                hash: SigHash::from_signature("fn main"),
                kind: NodeKind::Function,
                name: Arc::from("main"),
                signature: Arc::from("fn main()"),
                file_path: Arc::from("src/main.rs"),
                line: 1,
            },
            NodeData {
                hash: SigHash::from_signature("struct User"),
                kind: NodeKind::Struct,
                name: Arc::from("User"),
                signature: Arc::from("struct User { name: String, age: u32 }"),
                file_path: Arc::from("src/models/user.rs"),
                line: 10,
            },
            NodeData {
                hash: SigHash::from_signature("trait Display"),
                kind: NodeKind::Trait,
                name: Arc::from("Display"),
                signature: Arc::from("trait Display { fn fmt(&self) -> String; }"),
                file_path: Arc::from("src/traits/display.rs"),
                line: 5,
            },
        ];
        
        // Convert to enhanced nodes
        let enhanced_nodes = NodeConverter::batch_from_node_data(&original_nodes, &mut interner);
        
        // Verify conversion succeeded
        assert_eq!(enhanced_nodes.len(), 3);
        
        // Test O(1) file location access
        let start = Instant::now();
        for node in &enhanced_nodes {
            let location = node.file_location(&interner);
            assert!(location.is_some());
            
            let file_path = node.file_path(&interner);
            assert!(file_path.is_some());
        }
        let elapsed = start.elapsed();
        
        // Should be very fast
        assert!(elapsed.as_micros() < 100, "File location access too slow: {:?}", elapsed);
        
        // Test EntityInfo conversion for discovery
        let entity_infos: Vec<EntityInfo> = enhanced_nodes
            .iter()
            .filter_map(|node| node.to_entity_info(&interner))
            .collect();
        
        assert_eq!(entity_infos.len(), 3);
        
        // Verify entity info data
        let main_entity = entity_infos.iter().find(|e| e.name == "main").unwrap();
        assert_eq!(main_entity.file_path, "src/main.rs");
        assert_eq!(main_entity.line_number, Some(1));
        assert_eq!(main_entity.entity_type, EntityType::Function);
        
        let user_entity = entity_infos.iter().find(|e| e.name == "User").unwrap();
        assert_eq!(user_entity.file_path, "src/models/user.rs");
        assert_eq!(user_entity.line_number, Some(10));
        assert_eq!(user_entity.entity_type, EntityType::Struct);
        
        let display_entity = entity_infos.iter().find(|e| e.name == "Display").unwrap();
        assert_eq!(display_entity.file_path, "src/traits/display.rs");
        assert_eq!(display_entity.line_number, Some(5));
        assert_eq!(display_entity.entity_type, EntityType::Trait);
    }
    
    /// Test that enhanced nodes can be converted back to NodeData without data loss
    #[test]
    fn test_round_trip_conversion() {
        let mut interner = FileInterner::new();
        
        // Create original NodeData
        let original = NodeData {
            hash: SigHash::from_signature("fn test_function"),
            kind: NodeKind::Function,
            name: Arc::from("test_function"),
            signature: Arc::from("fn test_function(x: i32) -> String"),
            file_path: Arc::from("src/test.rs"),
            line: 42,
        };
        
        // Convert to enhanced node
        let enhanced = NodeConverter::from_node_data(&original, &mut interner);
        
        // Verify enhanced node has additional column capability
        assert_eq!(enhanced.column, 0); // Column not available in original
        
        // Convert back to NodeData
        let converted_back = NodeConverter::to_node_data(&enhanced, &interner).unwrap();
        
        // Verify essential data is preserved
        assert_eq!(converted_back.hash, original.hash);
        assert_eq!(converted_back.kind, original.kind);
        assert_eq!(converted_back.name, original.name);
        assert_eq!(converted_back.signature, original.signature);
        assert_eq!(converted_back.file_path, original.file_path);
        assert_eq!(converted_back.line, original.line);
    }
    
    /// Test enhanced node with complete position information (line + column)
    #[test]
    fn test_enhanced_node_with_complete_position() {
        let mut interner = FileInterner::new();
        let file_id = interner.intern("src/complete.rs");
        
        // Create enhanced node with complete position
        let enhanced_node = EnhancedIsgNode::new(
            SigHash::from_signature("fn complete_function"),
            NodeKind::Function,
            Arc::from("complete_function"),
            Arc::from("fn complete_function() -> Result<(), Error>"),
            file_id,
            25,  // line
            15,  // column
        );
        
        // Verify complete position information
        assert!(enhanced_node.has_complete_position());
        assert_eq!(enhanced_node.line_number, 25);
        assert_eq!(enhanced_node.column, 15);
        
        // Test formatted location
        let formatted = enhanced_node.format_location(&interner);
        assert_eq!(formatted, "src/complete.rs:25:15");
        
        // Test EntityInfo conversion includes column
        let entity_info = enhanced_node.to_entity_info(&interner).unwrap();
        assert_eq!(entity_info.column, Some(15));
        
        // Test file location includes column
        let file_location = enhanced_node.file_location(&interner).unwrap();
        assert_eq!(file_location.column, Some(15));
    }
    
    /// Performance test: Verify O(1) access scales with large datasets
    #[test]
    fn test_large_scale_o1_performance() {
        let mut interner = FileInterner::with_capacity(1000);
        
        // Create a large dataset
        let num_nodes = 10000;
        let num_files = 100;
        
        let enhanced_nodes: Vec<EnhancedIsgNode> = (0..num_nodes)
            .map(|i| {
                let file_idx = i % num_files;
                let file_id = interner.intern(&format!("src/module_{:03}.rs", file_idx));
                
                EnhancedIsgNode::new(
                    SigHash::from_signature(&format!("fn function_{}", i)),
                    NodeKind::Function,
                    Arc::from(format!("function_{}", i)),
                    Arc::from(format!("fn function_{}() -> i32", i)),
                    file_id,
                    (i as u32 % 1000) + 1,
                    (i as u32 % 80) + 1,
                )
            })
            .collect();
        
        // Test file location access performance
        let iterations = 10000;
        let start = Instant::now();
        
        for i in 0..iterations {
            let node_idx = i % enhanced_nodes.len();
            let _location = enhanced_nodes[node_idx].file_location(&interner);
        }
        
        let elapsed = start.elapsed();
        let avg_time_ns = elapsed.as_nanos() / iterations as u128;
        
        // Performance contract: Should be well under 1μs per operation
        assert!(avg_time_ns < 1000, "File location access too slow: {} ns > 1000 ns", avg_time_ns);
        
        // Test EntityInfo conversion performance
        let start = Instant::now();
        let entity_infos: Vec<_> = enhanced_nodes
            .iter()
            .take(1000)  // Test with subset for reasonable test time
            .filter_map(|node| node.to_entity_info(&interner))
            .collect();
        let elapsed = start.elapsed();
        
        assert_eq!(entity_infos.len(), 1000);
        
        // Should be fast - under 10ms for 1000 conversions
        assert!(elapsed.as_millis() < 10, 
                "EntityInfo conversion too slow: {:?}", elapsed);
    }
}
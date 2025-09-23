//! Performance tests for EnhancedIsgNode file location access
//! 
//! Validates the O(1) file location access performance contract specified
//! in the requirements. These tests ensure that file location operations
//! remain fast regardless of the number of nodes or files in the system.

#[cfg(test)]
mod performance_tests {
    use super::super::enhanced_isg_node::{EnhancedIsgNode, NodeConverter};
    use super::super::string_interning::FileInterner;
    use crate::isg::{NodeData, NodeKind, SigHash};
    use std::sync::Arc;
    use std::time::Instant;

    /// Test O(1) file location access performance with large dataset
    /// 
    /// This test validates that file location access remains constant time
    /// even with thousands of nodes and hundreds of unique file paths.
    #[test]
    fn test_o1_file_location_access_performance_large_dataset() {
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
        
        println!("Created {} nodes across {} files", total_nodes, num_files);
        
        // Test file location access performance
        let iterations = 10000;
        let start = Instant::now();
        
        for i in 0..iterations {
            let node_idx = i % nodes.len();
            let _location = nodes[node_idx].file_location(&interner);
        }
        
        let elapsed = start.elapsed();
        let avg_time_ns = elapsed.as_nanos() / iterations as u128;
        
        println!("File location access: {} iterations in {:?}", iterations, elapsed);
        println!("Average time per access: {} ns", avg_time_ns);
        
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
        
        println!("File path access: {} iterations in {:?}", iterations, elapsed);
        println!("Average time per access: {} ns", avg_time_ns);
        
        // Performance contract: Should be well under 1μs per operation
        assert!(avg_time_ns < 1000, "File path access too slow: {} ns > 1000 ns", avg_time_ns);
    }
    
    /// Test that file location access time is independent of dataset size
    /// 
    /// This test validates the O(1) property by comparing access times
    /// across different dataset sizes.
    #[test]
    fn test_o1_scalability_independence() {
        let dataset_sizes = vec![100, 1000, 10000];
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
            
            println!("Dataset size {}: {} ns per access", size, avg_time_ns);
        }
        
        // Verify that access time doesn't increase significantly with dataset size
        // Allow for some variance due to cache effects, but should be roughly constant
        let first_time = access_times[0];
        for &time in &access_times[1..] {
            let ratio = time as f64 / first_time as f64;
            assert!(ratio < 3.0, "Access time increased too much with dataset size: {}x", ratio);
        }
        
        println!("O(1) scalability test passed - access times remain roughly constant");
    }
    
    /// Test conversion performance between NodeData and EnhancedIsgNode
    /// 
    /// Validates that conversion operations are efficient and don't degrade
    /// with larger datasets.
    #[test]
    fn test_conversion_performance() {
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
        
        println!("Batch conversion to enhanced: {} nodes in {:?}", 
                 enhanced_nodes.len(), conversion_time);
        
        // Should be fast - under 10ms for 1000 nodes
        assert!(conversion_time.as_millis() < 10, 
                "Batch conversion too slow: {:?}", conversion_time);
        
        // Test batch conversion back to NodeData
        let start = Instant::now();
        let converted_back = NodeConverter::batch_to_node_data(&enhanced_nodes, &interner);
        let back_conversion_time = start.elapsed();
        
        println!("Batch conversion back: {} nodes in {:?}", 
                 converted_back.len(), back_conversion_time);
        
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
        println!("Memory usage: {} total entries, {} bytes total", 
                 memory_usage.total_entries, memory_usage.total_bytes());
        println!("Average bytes per entry: {:.2}", memory_usage.bytes_per_entry());
        
        // Should have only interned the unique file paths
        assert_eq!(interner.len(), num_files);
        
        // Memory usage should be reasonable
        assert!(memory_usage.total_bytes() < 10000, 
                "Memory usage too high: {} bytes", memory_usage.total_bytes());
        
        // Test that all nodes can access their file paths efficiently
        let start = Instant::now();
        let mut path_count = 0;
        
        for node in &nodes {
            if node.file_path(&interner).is_some() {
                path_count += 1;
            }
        }
        
        let elapsed = start.elapsed();
        
        assert_eq!(path_count, total_nodes);
        println!("Accessed {} file paths in {:?}", path_count, elapsed);
        
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
        let start = Instant::now();
        let entity_infos: Vec<_> = nodes
            .iter()
            .filter_map(|node| node.to_entity_info(&interner))
            .collect();
        let elapsed = start.elapsed();
        
        println!("Converted {} nodes to EntityInfo in {:?}", 
                 entity_infos.len(), elapsed);
        
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
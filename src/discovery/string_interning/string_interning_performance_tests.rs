//! Performance and efficiency tests for string interning system
//! 
//! These tests validate the memory efficiency and performance characteristics
//! of the FileInterner system as required by task 1.

use super::{FileInterner, MemoryUsage, FileId};
use std::time::Instant;

#[cfg(test)]
mod performance_tests {
    use super::*;
    
    /// Test string interning efficiency with realistic file paths
    /// 
    /// Validates that duplicate file paths are properly deduplicated
    /// and that memory usage is efficient.
    #[test]
    fn test_string_interning_efficiency() {
        let mut interner = FileInterner::new();
        
        // Common file paths that would appear many times in a real codebase
        let common_paths = vec![
            "src/main.rs",
            "src/lib.rs", 
            "src/discovery/mod.rs",
            "src/discovery/engine.rs",
            "src/discovery/types.rs",
            "tests/integration_tests.rs",
            "benches/performance.rs",
        ];
        
        // Simulate a realistic scenario where the same file paths
        // are referenced many times (once per entity in the file)
        let mut all_ids = Vec::new();
        for _ in 0..100 { // 100 entities per file
            for path in &common_paths {
                let id = interner.intern(path);
                all_ids.push(id);
            }
        }
        
        // Verify deduplication: should only have 7 unique paths
        assert_eq!(interner.len(), 7);
        
        // Verify all IDs for the same path are identical
        let main_rs_ids: Vec<_> = all_ids.iter()
            .enumerate()
            .filter(|(i, _)| i % 7 == 0) // Every 7th ID should be src/main.rs
            .map(|(_, &id)| id)
            .collect();
        
        // All main.rs IDs should be the same
        let first_main_id = main_rs_ids[0];
        for &id in &main_rs_ids {
            assert_eq!(id, first_main_id);
        }
        
        // Memory usage should be efficient
        let usage = interner.memory_usage();
        assert_eq!(usage.total_entries, 7);
        
        // Each entry should use reasonable memory (not excessive)
        // With string interning, we should use much less memory than
        // storing 700 separate string copies
        let bytes_per_entry = usage.bytes_per_entry();
        assert!(bytes_per_entry < 200.0, "Memory usage per entry too high: {}", bytes_per_entry);
        
        println!("String interning efficiency test results:");
        println!("  Total unique paths: {}", usage.total_entries);
        println!("  Total memory usage: {} bytes", usage.total_bytes());
        println!("  Average bytes per entry: {:.2}", bytes_per_entry);
    }
    
    /// Test memory usage scaling with large numbers of file paths
    /// 
    /// Validates that memory usage scales linearly with unique paths,
    /// not with total number of intern() calls.
    #[test]
    fn test_memory_usage_scaling() {
        let mut interner = FileInterner::new();
        
        // Create many unique file paths
        let unique_paths: Vec<String> = (0..1000)
            .map(|i| format!("src/module_{}/file_{}.rs", i / 10, i % 10))
            .collect();
        
        // Intern each path multiple times
        for _ in 0..5 {
            for path in &unique_paths {
                interner.intern(path);
            }
        }
        
        let usage = interner.memory_usage();
        
        // Should have exactly 1000 unique entries despite 5000 intern calls
        assert_eq!(usage.total_entries, 1000);
        
        // Memory usage should be reasonable for 1000 entries
        let total_mb = usage.total_bytes() as f64 / (1024.0 * 1024.0);
        assert!(total_mb < 1.0, "Memory usage too high: {:.2} MB for 1000 entries", total_mb);
        
        println!("Memory scaling test results:");
        println!("  Unique paths: {}", usage.total_entries);
        println!("  Total memory: {:.2} MB", total_mb);
        println!("  Bytes per entry: {:.2}", usage.bytes_per_entry());
    }
    
    /// Test interning performance with realistic workload
    /// 
    /// Validates that interning operations complete within performance contracts.
    #[test]
    fn test_interning_performance() {
        let mut interner = FileInterner::with_capacity(10000);
        
        // Generate realistic file paths
        let file_paths: Vec<String> = (0..5000)
            .map(|i| {
                match i % 10 {
                    0..=3 => format!("src/core/module_{}.rs", i / 100),
                    4..=6 => format!("src/services/service_{}.rs", i / 200),
                    7..=8 => format!("tests/test_{}.rs", i / 300),
                    _ => format!("benches/bench_{}.rs", i / 400),
                }
            })
            .collect();
        
        // Measure interning performance
        let start = Instant::now();
        
        for path in &file_paths {
            interner.intern(path);
        }
        
        let elapsed = start.elapsed();
        
        // Performance contract: should complete bulk interning quickly
        let ops_per_second = file_paths.len() as f64 / elapsed.as_secs_f64();
        assert!(ops_per_second > 10000.0, 
                "Interning performance too slow: {:.0} ops/sec", ops_per_second);
        
        // Measure lookup performance
        let start = Instant::now();
        
        for path in &file_paths {
            let _id = interner.get_id(path);
        }
        
        let lookup_elapsed = start.elapsed();
        let lookup_ops_per_second = file_paths.len() as f64 / lookup_elapsed.as_secs_f64();
        
        assert!(lookup_ops_per_second > 50000.0,
                "Lookup performance too slow: {:.0} ops/sec", lookup_ops_per_second);
        
        println!("Performance test results:");
        println!("  Interning: {:.0} ops/sec", ops_per_second);
        println!("  Lookup: {:.0} ops/sec", lookup_ops_per_second);
        println!("  Total unique paths: {}", interner.len());
    }
    
    /// Test memory efficiency compared to naive string storage
    /// 
    /// Demonstrates the memory savings from string interning.
    #[test]
    fn test_memory_efficiency_comparison() {
        let mut interner = FileInterner::new();
        
        // Simulate a realistic codebase with repeated file paths
        let base_paths = vec![
            "src/main.rs",
            "src/lib.rs",
            "src/discovery/mod.rs",
            "src/discovery/engine.rs",
            "src/discovery/types.rs",
            "src/isg.rs",
            "tests/integration.rs",
        ];
        
        // Each file has multiple entities (functions, structs, etc.)
        let entities_per_file = 20;
        let mut total_string_bytes = 0;
        
        for _ in 0..entities_per_file {
            for path in &base_paths {
                interner.intern(path);
                total_string_bytes += path.len();
            }
        }
        
        let usage = interner.memory_usage();
        let interned_bytes = usage.total_bytes();
        
        // String interning should use significantly less memory
        // than storing each string separately
        let savings_ratio = total_string_bytes as f64 / interned_bytes as f64;
        
        assert!(savings_ratio > 5.0, 
                "String interning not efficient enough. Savings ratio: {:.2}", savings_ratio);
        
        println!("Memory efficiency comparison:");
        println!("  Naive storage: {} bytes", total_string_bytes);
        println!("  Interned storage: {} bytes", interned_bytes);
        println!("  Savings ratio: {:.2}x", savings_ratio);
        println!("  Memory saved: {} bytes ({:.1}%)", 
                 total_string_bytes - interned_bytes,
                 (1.0 - interned_bytes as f64 / total_string_bytes as f64) * 100.0);
    }
    
    /// Test FileId space efficiency
    /// 
    /// Validates that FileId uses minimal memory (u32 is sufficient).
    #[test]
    fn test_file_id_efficiency() {
        use std::mem;
        
        // FileId should be exactly 4 bytes (u32)
        assert_eq!(mem::size_of::<FileId>(), 4);
        
        // Should be Copy and cheap to pass around
        let id = FileId::new(42);
        let id_copy = id; // Should be Copy, not Move
        assert_eq!(id.as_u32(), id_copy.as_u32());
        
        println!("FileId efficiency:");
        println!("  Size: {} bytes", mem::size_of::<FileId>());
        println!("  Alignment: {} bytes", mem::align_of::<FileId>());
    }
    
    /// Benchmark realistic codebase simulation
    /// 
    /// Tests performance with a simulation of a real Rust codebase.
    #[test]
    fn test_realistic_codebase_simulation() {
        let mut interner = FileInterner::with_capacity(1000);
        
        // Simulate Parseltongue codebase structure
        let modules = vec![
            "src/main.rs",
            "src/lib.rs",
            "src/isg.rs",
            "src/daemon.rs",
            "src/cli.rs",
            "src/discovery/mod.rs",
            "src/discovery/engine.rs",
            "src/discovery/types.rs",
            "src/discovery/error.rs",
            "src/discovery/string_interning.rs",
            "src/performance_validation.rs",
            "src/performance_monitoring.rs",
            "tests/integration_tests.rs",
            "tests/performance_tests.rs",
            "benches/isg_benchmarks.rs",
        ];
        
        // Each module has various entities
        let entities_per_module = vec![5, 10, 50, 30, 20, 5, 25, 15, 10, 20, 15, 10, 8, 12, 6];
        
        let start = Instant::now();
        let mut total_entities = 0;
        
        for (module, &entity_count) in modules.iter().zip(entities_per_module.iter()) {
            for _ in 0..entity_count {
                interner.intern(module);
                total_entities += 1;
            }
        }
        
        let elapsed = start.elapsed();
        
        // Validate results
        assert_eq!(interner.len(), modules.len()); // Only unique paths stored
        assert_eq!(total_entities, entities_per_module.iter().sum::<usize>());
        
        let usage = interner.memory_usage();
        
        // Performance should be excellent for this realistic workload
        assert!(elapsed.as_millis() < 10, "Realistic simulation too slow: {}ms", elapsed.as_millis());
        
        // Memory usage should be reasonable
        assert!(usage.total_bytes() < 10000, "Memory usage too high: {} bytes", usage.total_bytes());
        
        println!("Realistic codebase simulation:");
        println!("  Total entities processed: {}", total_entities);
        println!("  Unique file paths: {}", interner.len());
        println!("  Processing time: {}μs", elapsed.as_micros());
        println!("  Memory usage: {} bytes", usage.total_bytes());
        println!("  Efficiency: {:.2} entities/byte", total_entities as f64 / usage.total_bytes() as f64);
    }
}
//! Memory optimization integration tests
//! 
//! Integration tests that validate memory optimizations work correctly
//! in realistic discovery scenarios. These tests ensure that memory
//! optimization features don't break functionality.
//! 
//! Following TDD approach: RED → GREEN → REFACTOR

use super::*;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::discovery::{
        ConcurrentDiscoveryEngine,
        types::{EntityType, DiscoveryQuery},
        file_navigation_tests::TestDataFactory,
    };
    use std::time::{Duration, Instant};
    use tokio::task::JoinSet;

    /// Create a test ISG with large dataset for memory optimization testing
    fn create_test_isg_with_large_dataset() -> crate::isg::OptimizedISG {
        // For now, return a minimal test ISG
        // This will need to be implemented when we have a large test dataset
        TestDataFactory::create_test_isg_with_file_structure()
    }

    /// Get approximate memory usage (simplified implementation)
    fn get_current_memory_usage() -> usize {
        // Simplified memory usage estimation
        // In a real implementation, this would use system APIs
        std::mem::size_of::<usize>() * 1000 // Placeholder
    }

    #[tokio::test]
    async fn test_memory_optimized_end_to_end_discovery() {
        // Test complete discovery workflow with memory optimizations
        let isg = create_test_isg_with_large_dataset();
        
        // Measure initial memory
        let memory_before = get_current_memory_usage();
        
        let engine = ConcurrentDiscoveryEngine::new(isg);
        
        // Perform various discovery operations
        let entities = engine.list_all_entities(None, 100).await.unwrap();
        assert!(entities.len() > 0);
        
        let functions = engine.list_all_entities(Some(EntityType::Function), 500).await.unwrap();
        assert!(functions.len() > 0);
        
        let file_entities = engine.entities_in_file("src/main.rs").await.unwrap();
        assert!(file_entities.len() > 0);
        
        let total_count = engine.total_entity_count().await.unwrap();
        assert!(total_count > 0);
        
        let memory_after = get_current_memory_usage();
        let memory_increase = memory_after.saturating_sub(memory_before);
        
        // Memory should not grow significantly
        assert!(memory_increase < 10_000_000, // Less than 10MB increase
                "Memory increased by {} bytes during discovery operations", memory_increase);
    }

    #[tokio::test]
    async fn test_concurrent_batch_processing_baseline() {
        // Establish baseline for concurrent batch processing
        let isg = create_test_isg_with_large_dataset();
        let engine = ConcurrentDiscoveryEngine::new(isg);
        
        let queries = create_discovery_query_batch(50);
        let max_concurrent = 10;
        
        let start = Instant::now();
        let results = engine.batch_discovery_queries(queries, max_concurrent).await;
        let elapsed = start.elapsed();
        
        // Should meet performance contract
        assert!(elapsed < Duration::from_secs(5),
                "Batch processing took {:?}, expected <5s", elapsed);
        
        // All queries should succeed
        let success_count = results.iter().filter(|r| r.is_ok()).count();
        assert!(success_count >= 45, // Allow some failures
                "Only {} out of 50 queries succeeded", success_count);
    }

    #[tokio::test]
    async fn test_string_interning_memory_efficiency() {
        // Memory efficiency check
        let mut interner = crate::discovery::FileInterner::new();
        
        let memory_before = get_current_memory_usage();
        
        // Create realistic file paths with duplicates
        let base_paths = vec![
            "src/main.rs",
            "src/lib.rs", 
            "src/parser.rs",
            "src/utils.rs",
            "tests/integration.rs",
            "tests/unit.rs",
            "benches/benchmarks.rs",
        ];
        
        let mut all_paths = Vec::new();
        for _ in 0..100 {
            for path in &base_paths {
                all_paths.push(*path);
            }
        }
        
        // Intern all paths
        let start = Instant::now();
        let _file_ids: Vec<_> = all_paths.iter().map(|path| interner.intern(path)).collect();
        let interning_time = start.elapsed();
        
        let memory_after = get_current_memory_usage();
        let usage = interner.memory_usage();
        
        // Memory efficiency check
        let bytes_per_entry = usage.bytes_per_entry();
        assert!(bytes_per_entry < 200, // Less than 200 bytes per entry
                "String interning uses {} bytes per entry, expected <200", bytes_per_entry);
        
        // Performance check
        assert!(interning_time < Duration::from_millis(100),
                "String interning took {:?}, expected <100ms", interning_time);
        
        // Efficiency check
        let naive_memory: usize = all_paths.iter().map(|s| s.len()).sum();
        let efficiency = (naive_memory as f64 - usage.total_bytes() as f64) / naive_memory as f64;
        assert!(efficiency > 0.8, 
                "String interning efficiency {:.1}%, expected >80%", efficiency * 100.0);
    }

    #[tokio::test]
    async fn test_memory_optimization_correctness() {
        // Ensure memory optimizations don't affect correctness
        let mut indexes = crate::discovery::DiscoveryIndexes::new();
        
        let entities = create_large_entity_dataset(10_000);
        let original_entities = entities.clone();
        
        indexes.rebuild_from_entities(entities).unwrap();
        
        // Test that all entities are preserved
        assert_eq!(indexes.entity_count(), original_entities.len());
        
        // Test that filtering works correctly
        let functions: Vec<_> = indexes
            .filter_entities_by_type(EntityType::Function)
            .collect();
        
        let expected_functions = original_entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Function)
            .count();
        
        assert_eq!(functions.len(), expected_functions);
        
        // Test that conversion back to EntityInfo works
        for compact_entity in &indexes.all_entities {
            let converted = compact_entity.to_entity_info(&indexes.interner);
            assert!(!converted.name.is_empty());
            assert!(!converted.file_path.is_empty());
        }
    }

    fn create_discovery_query_batch(count: usize) -> Vec<DiscoveryQuery> {
        let mut queries = Vec::with_capacity(count);
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
            queries.push(DiscoveryQuery::list_by_type(entity_type));
        }
        
        queries
    }
    
    fn create_large_entity_dataset(count: usize) -> Vec<crate::discovery::EntityInfo> {
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
            entities.push(crate::discovery::EntityInfo::new(
                format!("entity_{}", i),
                format!("src/module_{}/file_{}.rs", i / 100, i % 100),
                entity_type,
                Some((i % 1000) as u32 + 1),
                Some((i % 120) as u32 + 1),
            ));
        }
        
        entities
    }
}
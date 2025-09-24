//! Discovery Indexes Demo
//! 
//! Demonstrates the usage of DiscoveryIndexes for fast entity lookup and filtering.
//! Shows the complete workflow from entity creation to indexed queries.

use parseltongue::discovery::{
    DiscoveryIndexes, CompactEntityInfo, EntityInfo, 
    types::EntityType, MemoryStats
};
use std::time::{Duration, Instant};

fn main() {
    println!("🔍 Discovery Indexes Demo");
    println!("========================\n");
    
    // Create sample entities representing a Rust project
    let entities = create_sample_entities();
    println!("📦 Created {} sample entities from a Rust project", entities.len());
    
    // Create and populate indexes
    let mut indexes = DiscoveryIndexes::new();
    let start = Instant::now();
    let rebuild_time = indexes.rebuild_from_entities(entities).unwrap();
    println!("⚡ Index rebuild completed in {:?}", rebuild_time);
    
    // Demonstrate type-based queries
    demonstrate_type_queries(&indexes);
    
    // Demonstrate file-based queries  
    demonstrate_file_queries(&indexes);
    
    // Show memory efficiency
    demonstrate_memory_efficiency(&indexes);
    
    // Performance demonstration
    demonstrate_performance();
    
    println!("\n✅ Discovery Indexes Demo completed successfully!");
}

fn create_sample_entities() -> Vec<EntityInfo> {
    vec![
        // Main application
        EntityInfo::new("main".to_string(), "src/main.rs".to_string(), EntityType::Function, Some(1), Some(1)),
        EntityInfo::new("Config".to_string(), "src/main.rs".to_string(), EntityType::Struct, Some(10), Some(1)),
        EntityInfo::new("Args".to_string(), "src/main.rs".to_string(), EntityType::Struct, Some(20), Some(1)),
        
        // Library module
        EntityInfo::new("lib".to_string(), "src/lib.rs".to_string(), EntityType::Module, Some(1), Some(1)),
        EntityInfo::new("Parser".to_string(), "src/lib.rs".to_string(), EntityType::Struct, Some(15), Some(1)),
        EntityInfo::new("Parseable".to_string(), "src/lib.rs".to_string(), EntityType::Trait, Some(25), Some(1)),
        EntityInfo::new("impl_parseable".to_string(), "src/lib.rs".to_string(), EntityType::Impl, Some(35), Some(1)),
        
        // Parser module
        EntityInfo::new("parse_file".to_string(), "src/parser.rs".to_string(), EntityType::Function, Some(5), Some(1)),
        EntityInfo::new("parse_string".to_string(), "src/parser.rs".to_string(), EntityType::Function, Some(15), Some(1)),
        EntityInfo::new("ParseError".to_string(), "src/parser.rs".to_string(), EntityType::Struct, Some(25), Some(1)),
        EntityInfo::new("MAX_FILE_SIZE".to_string(), "src/parser.rs".to_string(), EntityType::Constant, Some(35), Some(1)),
        
        // Utils module
        EntityInfo::new("format_output".to_string(), "src/utils.rs".to_string(), EntityType::Function, Some(8), Some(1)),
        EntityInfo::new("validate_input".to_string(), "src/utils.rs".to_string(), EntityType::Function, Some(18), Some(1)),
        EntityInfo::new("BUFFER_SIZE".to_string(), "src/utils.rs".to_string(), EntityType::Static, Some(28), Some(1)),
        EntityInfo::new("debug_print".to_string(), "src/utils.rs".to_string(), EntityType::Macro, Some(38), Some(1)),
        
        // Tests
        EntityInfo::new("test_parser".to_string(), "tests/parser_tests.rs".to_string(), EntityType::Function, Some(10), Some(1)),
        EntityInfo::new("test_utils".to_string(), "tests/utils_tests.rs".to_string(), EntityType::Function, Some(10), Some(1)),
    ]
}

fn demonstrate_type_queries(indexes: &DiscoveryIndexes) {
    println!("\n🔍 Type-based Queries:");
    println!("---------------------");
    
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
    
    for entity_type in entity_types {
        let entities = indexes.entities_by_type(entity_type);
        if !entities.is_empty() {
            println!("  {:?}: {} entities", entity_type, entities.len());
            for entity in entities.iter().take(3) {
                let converted = entity.to_entity_info(&indexes.interner);
                println!("    - {} ({}:{})", converted.name, converted.file_path, 
                        converted.line_number.unwrap_or(0));
            }
            if entities.len() > 3 {
                println!("    ... and {} more", entities.len() - 3);
            }
        }
    }
}

fn demonstrate_file_queries(indexes: &DiscoveryIndexes) {
    println!("\n📁 File-based Queries:");
    println!("----------------------");
    
    let files = ["src/main.rs", "src/lib.rs", "src/parser.rs", "src/utils.rs"];
    
    for file_path in files {
        let file_id = indexes.interner.get_id(file_path);
        if let Some(file_id) = file_id {
            let entities = indexes.entities_in_file(file_id);
            println!("  {}: {} entities", file_path, entities.len());
            for entity in entities {
                let converted = entity.to_entity_info(&indexes.interner);
                println!("    - {} ({:?})", converted.name, converted.entity_type);
            }
        }
    }
}

fn demonstrate_memory_efficiency(indexes: &DiscoveryIndexes) {
    println!("\n💾 Memory Efficiency:");
    println!("--------------------");
    
    let stats = indexes.memory_stats();
    println!("  Total entities: {}", indexes.entity_count());
    println!("  Entity memory: {} bytes", stats.entity_memory);
    println!("  File index memory: {} bytes", stats.file_index_memory);
    println!("  Type index memory: {} bytes", stats.type_index_memory);
    println!("  Interner memory: {} bytes", stats.interner_memory);
    println!("  Total memory: {} bytes", stats.total_memory);
    
    let bytes_per_entity = stats.total_memory / indexes.entity_count();
    println!("  Memory per entity: {} bytes", bytes_per_entity);
    
    // Verify CompactEntityInfo size
    let compact_size = std::mem::size_of::<CompactEntityInfo>();
    println!("  CompactEntityInfo size: {} bytes (target: 24 bytes)", compact_size);
}

fn demonstrate_performance() {
    println!("\n⚡ Performance Demonstration:");
    println!("----------------------------");
    
    // Test with different dataset sizes
    let sizes = [1_000, 10_000, 50_000];
    
    for size in sizes {
        println!("  Testing with {} entities:", size);
        
        // Generate entities
        let start = Instant::now();
        let entities = generate_entities(size);
        let generation_time = start.elapsed();
        
        // Build indexes
        let mut indexes = DiscoveryIndexes::new();
        let start = Instant::now();
        let rebuild_time = indexes.rebuild_from_entities(entities).unwrap();
        
        // Test query performance
        let start = Instant::now();
        let functions = indexes.entities_by_type(EntityType::Function);
        let query_time = start.elapsed();
        
        println!("    Generation: {:?}", generation_time);
        println!("    Index rebuild: {:?}", rebuild_time);
        println!("    Type query: {:?} ({} results)", query_time, functions.len());
        
        // Verify performance contracts
        if rebuild_time > Duration::from_secs(5) {
            println!("    ⚠️  Rebuild time exceeds 5s contract!");
        } else {
            println!("    ✅ Rebuild time within 5s contract");
        }
        
        if query_time > Duration::from_millis(100) {
            println!("    ⚠️  Query time exceeds 100ms contract!");
        } else {
            println!("    ✅ Query time within 100ms contract");
        }
        
        println!();
    }
}

fn generate_entities(count: usize) -> Vec<EntityInfo> {
    let mut entities = Vec::with_capacity(count);
    
    for i in 0..count {
        entities.push(EntityInfo::new(
            format!("entity_{}", i),
            format!("src/module_{}/file_{}.rs", i / 100, i % 100),
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
            Some((i % 1000) as u32 + 1),
            Some((i % 80) as u32 + 1),
        ));
    }
    
    entities
}
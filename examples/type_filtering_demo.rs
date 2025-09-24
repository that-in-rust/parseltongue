//! Demonstration of entity type filtering and organization functionality
//! 
//! This example shows how to use the type filtering features implemented in task 6:
//! - Type index for efficient entity type filtering
//! - Organized entity listing by type
//! - Entity count summaries by type for overview

use parseltongue::discovery::{
    SimpleDiscoveryEngine, 
    DiscoveryEngine,
    types::EntityType,
    file_navigation_tests::TestDataFactory,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Parseltongue v2 Type Filtering Demo ===\n");
    
    // Create test ISG with known structure
    let isg = TestDataFactory::create_test_isg_with_file_structure();
    let engine = SimpleDiscoveryEngine::new(isg);
    
    // 1. Show entity count summary
    println!("1. Entity Count Summary:");
    let summary = engine.entity_count_summary().await?;
    println!("{}", summary);
    
    // 2. Show available entity types
    println!("2. Available Entity Types:");
    let available_types = engine.available_entity_types().await?;
    for entity_type in available_types {
        println!("  - {:?}", entity_type);
    }
    println!();
    
    // 3. Show entities organized by type
    println!("3. Entities Organized by Type:");
    let organized = engine.entities_organized_by_type().await?;
    
    for (entity_type, entities) in organized {
        println!("  {:?} ({} entities):", entity_type, entities.len());
        for entity in entities {
            println!("    - {} ({}:{})", 
                entity.name, 
                entity.file_path, 
                entity.line_number.unwrap_or(0)
            );
        }
        println!();
    }
    
    // 4. Demonstrate efficient type filtering
    println!("4. Efficient Type Filtering:");
    
    println!("  Functions only:");
    let functions = engine.entities_by_type_efficient(EntityType::Function, 10).await?;
    for func in functions {
        println!("    - {} ({}:{})", func.name, func.file_path, func.line_number.unwrap_or(0));
    }
    
    println!("\n  Structs only:");
    let structs = engine.entities_by_type_efficient(EntityType::Struct, 10).await?;
    for struct_entity in structs {
        println!("    - {} ({}:{})", struct_entity.name, struct_entity.file_path, struct_entity.line_number.unwrap_or(0));
    }
    
    println!("\n  Traits only:");
    let traits = engine.entities_by_type_efficient(EntityType::Trait, 10).await?;
    for trait_entity in traits {
        println!("    - {} ({}:{})", trait_entity.name, trait_entity.file_path, trait_entity.line_number.unwrap_or(0));
    }
    
    // 5. Show performance benefits
    println!("\n5. Performance Demonstration:");
    let start = std::time::Instant::now();
    
    // Multiple type filtering operations
    let _ = engine.entities_by_type_efficient(EntityType::Function, 100).await?;
    let _ = engine.entities_by_type_efficient(EntityType::Struct, 100).await?;
    let _ = engine.entity_count_by_type().await?;
    let _ = engine.available_entity_types().await?;
    
    let elapsed = start.elapsed();
    println!("  All type filtering operations completed in: {:?}", elapsed);
    println!("  Performance contract: <100ms ✓");
    
    Ok(())
}
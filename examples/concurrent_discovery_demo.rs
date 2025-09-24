//! Concurrent Discovery Engine Demo
//! 
//! Demonstrates the thread-safe concurrent discovery engine with performance validation.

use parseltongue::{
    discovery::{ConcurrentDiscoveryEngine, DiscoveryEngine, types::EntityType},
    isg::OptimizedISG,
};
use std::sync::Arc;
use std::time::Instant;
use tokio::task::JoinSet;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Concurrent Discovery Engine Demo");
    println!("=====================================\n");
    
    // Create a sample ISG for demonstration
    let isg = create_demo_isg();
    println!("📊 Created demo ISG with {} entities", isg.node_count());
    
    // Create concurrent discovery engine
    let engine = Arc::new(ConcurrentDiscoveryEngine::new(isg));
    println!("🚀 Created concurrent discovery engine\n");
    
    // Demo: Basic concurrent operations
    println!("Demo: Basic Concurrent Operations");
    println!("---------------------------------");
    demo_basic_concurrent_operations(Arc::clone(&engine)).await?;
    
    println!("\n✅ Concurrent discovery demo completed successfully!");
    Ok(())
}

/// Create a demo ISG with sample entities
fn create_demo_isg() -> OptimizedISG {
    use parseltongue::discovery::file_navigation_tests::TestDataFactory;
    TestDataFactory::create_test_isg_with_file_structure()
}

/// Demo basic concurrent operations
async fn demo_basic_concurrent_operations(
    engine: Arc<ConcurrentDiscoveryEngine>
) -> Result<()> {
    let mut join_set = JoinSet::new();
    let start = Instant::now();
    
    // Spawn concurrent readers
    for i in 0..5 {
        let engine_clone = Arc::clone(&engine);
        join_set.spawn(async move {
            let operation_start = Instant::now();
            
            match i % 3 {
                0 => {
                    let entities = engine_clone.list_all_entities(Some(EntityType::Function), 10).await?;
                    println!("  Thread {}: Found {} functions", i, entities.len());
                }
                1 => {
                    let entities = engine_clone.entities_in_file("src/main.rs").await?;
                    println!("  Thread {}: Found {} entities in main.rs", i, entities.len());
                }
                2 => {
                    let count = engine_clone.total_entity_count().await?;
                    println!("  Thread {}: Total entities: {}", i, count);
                }
                _ => unreachable!(),
            }
            
            let elapsed = operation_start.elapsed();
            Ok::<_, anyhow::Error>(elapsed)
        });
    }
    
    // Wait for all operations to complete
    let mut total_time = std::time::Duration::ZERO;
    while let Some(result) = join_set.join_next().await {
        let elapsed = result??;
        total_time += elapsed;
    }
    
    let overall_elapsed = start.elapsed();
    println!("  ✅ Completed 5 concurrent operations in {:?}", overall_elapsed);
    println!("  📊 Average operation time: {:?}", total_time / 5);
    
    Ok(())
}
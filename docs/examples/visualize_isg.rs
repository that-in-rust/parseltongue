// Simple program to demonstrate ISG visualization
use parseltongue::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== UNDERSTANDING THE INTERFACE SIGNATURE GRAPH ===\n");
    
    // Create a new daemon
    let mut daemon = ParseltongueAIM::new();
    
    // Ingest our example code
    println!("1. Ingesting example code...");
    let stats = daemon.ingest_code_dump(std::path::Path::new("example_dump.txt"))?;
    println!("   ✓ Processed {} files, created {} nodes\n", stats.files_processed, stats.nodes_created);
    
    // Show the raw ISG structure
    println!("2. Raw ISG Structure:");
    println!("{}\n", daemon.isg.debug_print());
    
    // Show what each query does
    println!("3. Query Examples:");
    
    // Find what implements Display
    if let Ok(trait_hash) = daemon.find_entity_by_name("Display") {
        if let Ok(implementors) = daemon.isg.find_implementors(trait_hash) {
            println!("   What implements Display trait:");
            for imp in implementors {
                println!("     - {} ({})", imp.name, imp.signature);
            }
        }
    }
    
    // Show blast radius from main function
    if let Ok(main_hash) = daemon.find_entity_by_name("main") {
        if let Ok(radius) = daemon.isg.calculate_blast_radius(main_hash) {
            println!("   Blast radius from main() function:");
            println!("     - {} entities would be affected by changes to main()", radius.len());
        }
    }
    
    // Generate context for User struct
    if let Ok(user_hash) = daemon.find_entity_by_name("User") {
        let dependencies = daemon.get_dependencies(user_hash);
        let callers = daemon.get_callers(user_hash);
        
        println!("   User struct relationships:");
        println!("     - Dependencies: {} entities", dependencies.len());
        println!("     - Callers: {} entities", callers.len());
        
        for dep in dependencies {
            println!("       → depends on: {}", dep.name);
        }
        for caller in callers {
            println!("       ← used by: {}", caller.name);
        }
    }
    
    println!("\n4. Graphviz Visualization:");
    println!("   Copy this to a file called 'graph.dot' and run:");
    println!("   dot -Tpng graph.dot -o graph.png\n");
    println!("{}", daemon.isg.export_dot());
    
    Ok(())
}
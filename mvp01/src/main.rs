#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debugging: Print the path being included
    println!("Including file from: {}", concat!(env!("OUT_DIR"), "/summary.rs"));

    // Your main logic here
    println!("Hello, world!");
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/summary.rs"));

use std::env;
use std::path::PathBuf;
use prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["src/summary.proto"], &["src/"])?;
    Ok(())
}

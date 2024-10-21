use std::io::Result;
use std::path::PathBuf;
use prost_build;

fn main() -> Result<()> {
    let proto_file = "proto/summary.proto";
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    prost_build::compile_protos(&[proto_file], &["."])?;
    
    println!("cargo:rerun-if-changed={}", proto_file);
    
    Ok(())
}

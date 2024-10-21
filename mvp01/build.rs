use std::env;
use std::path::PathBuf;
use prost_build;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = PathBuf::from("/home/amuldotexe/Desktop/GitHub202410/parseltongue/mvp01/src/summary.proto");
    
    if !proto_file.exists() {
        return Err(format!("{} does not exist", proto_file.display()).into());
    }

    println!("cargo:rerun-if-changed={}", proto_file.display());

    prost_build::compile_protos(&[proto_file], &["/home/amuldotexe/Desktop/GitHub202410/parseltongue/mvp01/src/"])?;
    Ok(())
}

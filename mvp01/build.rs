use std::io::Result;

fn main() -> Result<()> {
    let proto_file = "proto/summary.proto";

    prost_build::compile_protos(&[proto_file], &["."])?;

    println!("cargo:rerun-if-changed={}", proto_file);

    Ok(())
}

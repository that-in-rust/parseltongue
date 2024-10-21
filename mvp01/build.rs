use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    tonic_build::compile_protos("proto/summary.proto")?;
    println!("cargo:rerun-if-changed=proto/summary.proto");
    Ok(())
}

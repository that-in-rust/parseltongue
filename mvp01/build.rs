use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::compile_protos("proto/summary.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    println!("cargo:rerun-if-changed=proto/summary.proto");
}

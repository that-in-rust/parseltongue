use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/summary.proto");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    #[cfg(feature = "proto")]
    {
        if !is_protoc_installed() {
            eprintln!("Error: 'protoc' is not installed or not found in PATH.");
            eprintln!("Please install Protocol Buffers compiler.");
            std::process::exit(1);
        }

        match tonic_build::compile_protos("proto/summary.proto") {
            Ok(_) => println!("Successfully compiled proto files"),
            Err(e) => {
                eprintln!("Failed to compile proto files: {}", e);
                // Generate stub file if compilation fails
                generate_stub_file(&out_dir)?;
            }
        }
    }

    #[cfg(not(feature = "proto"))]
    {
        // Generate stub file when proto feature is not enabled
        generate_stub_file(&out_dir)?;
    }

    Ok(())
}

#[cfg(feature = "proto")]
fn is_protoc_installed() -> bool {
    match std::process::Command::new("protoc").arg("--version").output() {
        Ok(output) => {
            println!("protoc version: {}", String::from_utf8_lossy(&output.stdout));
            true
        }
        Err(e) => {
            eprintln!("Failed to run protoc: {}", e);
            false
        }
    }
}

fn generate_stub_file(out_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let stub_path = out_dir.join("summary.rs");
    std::fs::write(
        &stub_path,
        "// This is a stub file generated when proto compilation is not available.\n\
         // Implement your fallback structures here if needed.\n",
    )?;
    println!("Generated stub file at: {}", stub_path.display());
    Ok(())
}

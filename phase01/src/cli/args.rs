//! CLI Argument Handling
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Argument Processing
//! - ArgumentProcessor (processes arguments)
//! - Validator         (validates arguments)
//! 
//! Level 3: Argument Types
//! - InputArgs        (input file arguments)
//! - OutputArgs       (output configuration)
//! 
//! Level 2: Argument Implementation
//! - ArgParser        (argument parsing)
//! - ArgValidator     (validation logic)
//! 
//! Level 1 (Base): Core Argument Types
//! - Args            (argument structure)
//! - ArgError        (argument errors)
//! - ValidationRule  (validation rules)

use std::path::{Path, PathBuf};
use clap::Parser;
use crate::core::{error::Result, types::*};

// ===== Level 1: Core Argument Types =====
// Design Choice: Using clap for argument parsing

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Input ZIP file path
    #[clap(short = 'i', long = "input-zip", value_parser = validate_input_path)]
    pub input_path: PathBuf,

    /// Output directory path
    #[clap(short = 'o', long = "output-dir", value_parser = validate_output_path)]
    pub output_dir: PathBuf,

    /// Enable verbose output
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,
}

// ===== Level 2: Argument Implementation =====
// Design Choice: Using validation functions

/// Validates input path
fn validate_input_path(path: impl AsRef<Path>) -> std::result::Result<PathBuf, String> {
    let path = path.as_ref();
    
    // Check if path exists
    if !path.exists() {
        return Err(format!("Input file does not exist: {}", path.display()));
    }

    // Check if it's a file
    if !path.is_file() {
        return Err(format!("Input path is not a file: {}", path.display()));
    }

    // Check file extension
    if let Some(ext) = path.extension() {
        if ext != "zip" {
            return Err(format!(
                "Input file must have .zip extension, got: {}",
                path.display()
            ));
        }
    } else {
        return Err(format!("Input file has no extension: {}", path.display()));
    }

    // Check if file is readable
    match std::fs::metadata(path) {
        Ok(meta) => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                let mode = meta.mode();
                if mode & 0o444 == 0 {
                    return Err(format!("Input file is not readable: {}", path.display()));
                }
            }
        }
        Err(e) => {
            return Err(format!(
                "Failed to read metadata for input file {}: {}",
                path.display(), e
            ));
        }
    }

    Ok(path.to_path_buf())
}

/// Validates output path
fn validate_output_path(path: impl AsRef<Path>) -> std::result::Result<PathBuf, String> {
    let path = path.as_ref();
    
    // If path exists, check if it's a directory
    if path.exists() {
        if !path.is_dir() {
            return Err(format!(
                "Output path exists but is not a directory: {}", 
                path.display()
            ));
        }

        // Check if directory is writable
        match std::fs::metadata(path) {
            Ok(meta) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    let mode = meta.mode();
                    if mode & 0o222 == 0 {
                        return Err(format!(
                            "Output directory is not writable: {}", 
                            path.display()
                        ));
                    }
                }
            }
            Err(e) => {
                return Err(format!(
                    "Failed to read metadata for output directory {}: {}", 
                    path.display(), e
                ));
            }
        }
    } else {
        // Try to create the directory
        if let Err(e) = std::fs::create_dir_all(path) {
            return Err(format!(
                "Failed to create output directory {}: {}", 
                path.display(), e
            ));
        }
    }

    // Verify path is absolute
    if !path.is_absolute() {
        return Err(format!(
            "Output directory must be an absolute path: {}", 
            path.display()
        ));
    }

    Ok(path.to_path_buf())
}

impl Args {
    /// Ensures all paths are canonicalized
    pub fn canonicalize_paths(&mut self) -> Result<()> {
        self.input_path = self.input_path.canonicalize().map_err(|e| 
            Error::InvalidPath(format!(
                "Failed to canonicalize input path {}: {}", 
                self.input_path.display(), e
            ))
        )?;

        self.output_dir = self.output_dir.canonicalize().map_err(|e| 
            Error::InvalidPath(format!(
                "Failed to canonicalize output path {}: {}", 
                self.output_dir.display(), e
            ))
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;

    #[test]
    fn test_input_path_validation() {
        let temp_dir = TempDir::new().unwrap();
        
        // Test non-existent file
        let bad_path = temp_dir.path().join("nonexistent.zip");
        assert!(validate_input_path(&bad_path).is_err());

        // Test wrong extension
        let wrong_ext = temp_dir.path().join("test.txt");
        File::create(&wrong_ext).unwrap();
        assert!(validate_input_path(&wrong_ext).is_err());

        // Test valid ZIP file
        let good_path = temp_dir.path().join("test.zip");
        File::create(&good_path).unwrap();
        assert!(validate_input_path(&good_path).is_ok());
    }

    #[test]
    fn test_output_path_validation() {
        let temp_dir = TempDir::new().unwrap();
        
        // Test file as output
        let file_path = temp_dir.path().join("file");
        File::create(&file_path).unwrap();
        assert!(validate_output_path(&file_path).is_err());

        // Test valid directory
        assert!(validate_output_path(temp_dir.path()).is_ok());

        // Test non-absolute path
        assert!(validate_output_path("relative/path").is_err());
    }

    #[test]
    fn test_path_canonicalization() {
        let temp_dir = TempDir::new().unwrap();
        let zip_path = temp_dir.path().join("test.zip");
        File::create(&zip_path).unwrap();

        let mut args = Args {
            input_path: zip_path,
            output_dir: temp_dir.path().to_path_buf(),
            verbose: false,
        };

        assert!(args.canonicalize_paths().is_ok());
        assert!(args.input_path.is_absolute());
        assert!(args.output_dir.is_absolute());
    }
}

//! File discovery and filtering functionality
//! Following TDD-first principle - tests first, implementation second

use crate::error::{ToolError, ToolResult};
use ignore::WalkBuilder;
use std::path::PathBuf;

/// Discovers files in a directory tree
#[derive(Debug, Clone)]
pub struct FileDiscovery {
    root_path: PathBuf,
    follow_symlinks: bool,
    max_depth: Option<usize>,
}

impl FileDiscovery {
    /// Create a new file discovery instance
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            follow_symlinks: false,
            max_depth: None,
        }
    }

    /// Set whether to follow symbolic links
    pub fn follow_symlinks(mut self, follow: bool) -> Self {
        self.follow_symlinks = follow;
        self
    }

    /// Set maximum depth for directory traversal
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Discover all files in the directory tree
    pub async fn discover_all(&self) -> ToolResult<Vec<PathBuf>> {
        let mut files = Vec::new();

        let mut walk_builder = WalkBuilder::new(&self.root_path);

        // Apply max_depth constraint if specified
        if let Some(max_depth) = self.max_depth {
            walk_builder.max_depth(Some(max_depth));
        }

        // Apply follow_symlinks constraint
        if self.follow_symlinks {
            walk_builder.follow_links(true);
        }

        let walk = walk_builder.build();
        for entry in walk {
            let entry = entry
                .map_err(|e| ToolError::file_discovery(format!("Walking directory: {}", e)))?;

            if let Some(file_type) = entry.file_type() {
                if file_type.is_file() {
                    files.push(entry.into_path());
                }
            }
        }

        Ok(files)
    }

    /// Get the root path
    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }
}

/// Filters files to find Rust source files
#[derive(Debug, Clone)]
pub struct RustFileFilter {
    include_tests: bool,
    include_examples: bool,
    include_benches: bool,
}

impl RustFileFilter {
    /// Create a new Rust file filter
    pub fn new() -> Self {
        Self {
            include_tests: true,
            include_examples: true,
            include_benches: true,
        }
    }

    /// Set whether to include test files
    pub fn include_tests(mut self, include: bool) -> Self {
        self.include_tests = include;
        self
    }

    /// Set whether to include example files
    pub fn include_examples(mut self, include: bool) -> Self {
        self.include_examples = include;
        self
    }

    /// Set whether to include benchmark files
    pub fn include_benches(mut self, include: bool) -> Self {
        self.include_benches = include;
        self
    }

    /// Filter a list of files to only Rust source files
    pub fn filter_files(&self, files: &[PathBuf]) -> ToolResult<Vec<PathBuf>> {
        let mut rust_files = Vec::new();

        for file_path in files {
            if self.is_rust_file(file_path)? {
                rust_files.push(file_path.clone());
            }
        }

        Ok(rust_files)
    }

    /// Check if a file is a Rust source file
    pub fn is_rust_file(&self, path: &std::path::Path) -> ToolResult<bool> {
        if let Some(extension) = path.extension() {
            if extension == "rs" {
                return Ok(self.should_include_path(path));
            }
        }
        Ok(false)
    }

    /// Check if a path should be included based on configuration
    fn should_include_path(&self, path: &std::path::Path) -> bool {
        let path_str = path.to_string_lossy();

        // Always include main source files
        if !path_str.contains("tests/")
            && !path_str.contains("examples/")
            && !path_str.contains("benches/")
        {
            return true;
        }

        // Include based on configuration
        if path_str.contains("tests/") && self.include_tests {
            return true;
        }
        if path_str.contains("examples/") && self.include_examples {
            return true;
        }
        if path_str.contains("benches/") && self.include_benches {
            return true;
        }

        false
    }
}

impl Default for RustFileFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_discovers_files_in_directory() {
        // RED: This test should fail because FileDiscovery::discover_all is not implemented
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "fn test() {}").unwrap();

        let discovery = FileDiscovery::new(temp_dir.path().to_path_buf());
        let files = discovery.discover_all().await.unwrap();

        assert!(!files.is_empty(), "Should discover at least one file");
        assert!(files.contains(&test_file), "Should discover the test file");
    }

    #[tokio::test]
    async fn test_file_discovery_respects_max_depth() {
        // RED: This test should fail because max_depth constraint is not implemented
        let temp_dir = TempDir::new().unwrap();

        // Create nested structure
        let deep_dir = temp_dir.path().join("deep").join("deeper");
        fs::create_dir_all(&deep_dir).unwrap();
        let shallow_file = temp_dir.path().join("shallow.rs");
        let deep_file = deep_dir.join("deep.rs");
        fs::write(&shallow_file, "fn shallow() {}").unwrap();
        fs::write(&deep_file, "fn deep() {}").unwrap();

        let discovery = FileDiscovery::new(temp_dir.path().to_path_buf()).max_depth(1);
        let files = discovery.discover_all().await.unwrap();

        assert!(
            files.contains(&shallow_file),
            "Should discover shallow file"
        );
        assert!(
            !files.contains(&deep_file),
            "Should not discover deep file with max_depth=1"
        );
    }

    #[test]
    fn test_rust_file_filter_identifies_rust_files() {
        // RED: This test should fail because RustFileFilter::is_rust_file is not implemented
        let filter = RustFileFilter::new();

        let rust_file = PathBuf::from("src/main.rs");
        let py_file = PathBuf::from("script.py");

        assert!(
            filter.is_rust_file(&rust_file).unwrap(),
            "Should identify .rs files"
        );
        assert!(
            !filter.is_rust_file(&py_file).unwrap(),
            "Should reject non-.rs files"
        );
    }

    #[test]
    fn test_rust_file_filter_filters_files() {
        // RED: This test should fail because RustFileFilter::filter_files is not implemented
        let filter = RustFileFilter::new().include_tests(false);

        let files = vec![
            PathBuf::from("src/main.rs"),
            PathBuf::from("tests/integration_test.rs"),
            PathBuf::from("examples/example.rs"),
            PathBuf::from("script.py"),
        ];

        let filtered = filter.filter_files(&files).unwrap();

        assert_eq!(filtered.len(), 2, "Should filter to 2 Rust files");
        assert!(
            filtered.contains(&PathBuf::from("src/main.rs")),
            "Should include main.rs"
        );
        assert!(
            filtered.contains(&PathBuf::from("examples/example.rs")),
            "Should include example.rs"
        );
        assert!(
            !filtered.contains(&PathBuf::from("tests/integration_test.rs")),
            "Should exclude test files"
        );
        assert!(
            !filtered.contains(&PathBuf::from("script.py")),
            "Should exclude non-Rust files"
        );
    }

    #[test]
    fn test_rust_file_filter_includes_tests_when_configured() {
        // RED: This test should fail because test inclusion logic is not implemented
        let filter = RustFileFilter::new().include_tests(true);

        let files = vec![
            PathBuf::from("src/main.rs"),
            PathBuf::from("tests/integration_test.rs"),
        ];

        let filtered = filter.filter_files(&files).unwrap();

        assert_eq!(
            filtered.len(),
            2,
            "Should include both files when tests are enabled"
        );
        assert!(
            filtered.contains(&PathBuf::from("src/main.rs")),
            "Should include main.rs"
        );
        assert!(
            filtered.contains(&PathBuf::from("tests/integration_test.rs")),
            "Should include test files"
        );
    }
}

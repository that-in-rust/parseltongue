//! File streaming implementation for folder-to-cozoDB processing.

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tokio::fs;
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use async_trait::async_trait;

use parseltongue_core::entities::CodeEntity;
use crate::errors::*;
use crate::isgl1_generator::*;
use crate::StreamerConfig;

/// File streamer interface
#[async_trait::async_trait]
pub trait FileStreamer: Send + Sync {
    /// Stream all files from the configured directory to database
    async fn stream_directory(&self) -> Result<StreamResult>;

    /// Stream a single file to database
    async fn stream_file(&self, file_path: &Path) -> Result<FileResult>;

    /// Get current streaming statistics
    fn get_stats(&self) -> StreamStats;
}

/// Streaming operation results
#[derive(Debug, Clone)]
pub struct StreamResult {
    pub total_files: usize,
    pub processed_files: usize,
    pub entities_created: usize,
    pub errors: Vec<String>,
    pub duration: std::time::Duration,
}

/// Single file processing result
#[derive(Debug, Clone)]
pub struct FileResult {
    pub file_path: String,
    pub entities_created: usize,
    pub success: bool,
    pub error: Option<String>,
}

/// Streaming statistics
#[derive(Debug, Clone, Default)]
pub struct StreamStats {
    pub files_processed: usize,
    pub entities_created: usize,
    pub errors_encountered: usize,
}

/// File streamer implementation
pub struct FileStreamerImpl {
    config: StreamerConfig,
    key_generator: Arc<dyn Isgl1KeyGenerator>,
    stats: std::sync::Mutex<StreamStats>,
}

impl FileStreamerImpl {
    /// Create new file streamer
    pub fn new(
        config: StreamerConfig,
        key_generator: Arc<dyn Isgl1KeyGenerator>,
    ) -> Self {
        Self {
            config,
            key_generator,
            stats: std::sync::Mutex::new(StreamStats::default()),
        }
    }

    /// Check if file should be processed based on patterns
    fn should_process_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy();

        // Check exclude patterns
        for pattern in &self.config.exclude_patterns {
            if self.matches_pattern(&path_str, pattern) {
                return false;
            }
        }

        // Check include patterns
        for pattern in &self.config.include_patterns {
            if self.matches_pattern(&path_str, pattern) {
                return true;
            }
        }

        false
    }

    /// Simple glob pattern matching
    fn matches_pattern(&self, path: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            // Convert glob pattern to regex-like matching
            let regex_pattern = pattern
                .replace('.', "\\.")
                .replace('*', ".*")
                .replace('?', ".");

            path.contains(&pattern.replace('*', "")) || path == pattern
        } else {
            path.contains(pattern)
        }
    }

    /// Read file content with size limit
    async fn read_file_content(&self, file_path: &Path) -> Result<String> {
        let metadata = fs::metadata(file_path).await.map_err(|e| {
            StreamerError::FileSystemError {
                path: file_path.to_string_lossy().to_string(),
                source: e,
            }
        })?;

        if metadata.len() as usize > self.config.max_file_size {
            return Err(StreamerError::FileTooLarge {
                path: file_path.to_string_lossy().to_string(),
                size: metadata.len() as usize,
                limit: self.config.max_file_size,
            });
        }

        let mut content = String::new();
        let mut file = fs::File::open(file_path).await.map_err(|e| {
            StreamerError::FileSystemError {
                path: file_path.to_string_lossy().to_string(),
                source: e,
            }
        })?;

        file.read_to_string(&mut content).await.map_err(|e| {
            StreamerError::FileSystemError {
                path: file_path.to_string_lossy().to_string(),
                source: e,
            }
        })?;

        Ok(content)
    }

    /// Update streaming statistics
    fn update_stats(&self, entities_created: usize, had_error: bool) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.files_processed += 1;
            stats.entities_created += entities_created;
            if had_error {
                stats.errors_encountered += 1;
            }
        }
    }
}

#[async_trait::async_trait]
impl FileStreamer for FileStreamerImpl {
    async fn stream_directory(&self) -> Result<StreamResult> {
        let start_time = Instant::now();
        let mut total_files = 0;
        let mut processed_files = 0;
        let mut entities_created = 0;
        let mut errors = Vec::new();

        println!(
            "{}",
            style("Starting directory streaming...").blue().bold()
        );

        // Setup progress bar
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap()
        );
        pb.set_message("Scanning files...");

        // Walk through directory
        for entry in WalkDir::new(&self.config.root_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() && self.should_process_file(path) {
                total_files += 1;
                pb.set_message(format!("Processing: {}", path.display()));

                match self.stream_file(path).await {
                    Ok(result) => {
                        processed_files += 1;
                        entities_created += result.entities_created;
                    }
                    Err(e) => {
                        let error_msg = format!("{}: {}", path.display(), e);
                        errors.push(error_msg.clone());
                        pb.println(format!("{} {}", style("⚠").yellow().for_stderr(), error_msg));
                        self.update_stats(0, true);
                    }
                }
            }
        }

        pb.finish_with_message("Directory streaming completed");

        let duration = start_time.elapsed();

        // Print summary
        println!("\n{}", style("Streaming Summary:").green().bold());
        println!("Total files found: {}", total_files);
        println!("Files processed: {}", processed_files);
        println!("Entities created: {}", entities_created);
        println!("Errors encountered: {}", errors.len());
        println!("Duration: {:?}", duration);

        Ok(StreamResult {
            total_files,
            processed_files,
            entities_created,
            errors,
            duration,
        })
    }

    async fn stream_file(&self, file_path: &Path) -> Result<FileResult> {
        let file_path_str = file_path.to_string_lossy().to_string();

        // Read file content
        let content = self.read_file_content(file_path).await?;

        // Parse code entities
        let parsed_entities = self.key_generator.parse_source(&content, file_path)?;

        let mut entities_created = 0;
        let mut errors: Vec<String> = Vec::new();

        // Process each parsed entity
        for parsed_entity in parsed_entities {
            // Generate ISGL1 key
            let isgl1_key = self.key_generator.generate_key(&parsed_entity)?;

            // Store in database
            // TODO: Implement actual database storage with CodeEntity
            // For now, just count the entity
            entities_created += 1;
        }

        self.update_stats(entities_created, false);

        Ok(FileResult {
            file_path: file_path_str,
            entities_created,
            success: true,
            error: None,
        })
    }

    fn get_stats(&self) -> StreamStats {
        self.stats.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
    }
}


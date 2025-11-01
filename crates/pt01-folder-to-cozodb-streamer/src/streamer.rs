//! File streaming implementation for folder-to-cozoDB processing.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tokio::fs;
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use parseltongue_core::entities::*;
use parseltongue_core::storage::CozoDbStorage;
use crate::errors::*;
use crate::isgl1_generator::*;
use crate::lsp_client::*;
use crate::StreamerConfig;

// Import LSP metadata types from parseltongue-core
use parseltongue_core::entities::{LspMetadata, TypeInformation, UsageAnalysis};

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
    lsp_client: Arc<dyn RustAnalyzerClient>,
    db: Arc<CozoDbStorage>,
    stats: std::sync::Mutex<StreamStats>,
}

impl FileStreamerImpl {
    /// Create new file streamer with database connection
    pub async fn new(
        config: StreamerConfig,
        key_generator: Arc<dyn Isgl1KeyGenerator>,
    ) -> Result<Self> {
        // Initialize database connection
        let db = CozoDbStorage::new(&config.db_path)
            .await
            .map_err(|e| StreamerError::StorageError {
                details: format!("Failed to create database: {}", e),
            })?;

        // Create schema
        db.create_schema()
            .await
            .map_err(|e| StreamerError::StorageError {
                details: format!("Failed to create schema: {}", e),
            })?;

        // Initialize LSP client (graceful degradation if unavailable)
        let lsp_client = RustAnalyzerClientImpl::new().await;

        Ok(Self {
            config,
            key_generator,
            lsp_client: Arc::new(lsp_client),
            db: Arc::new(db),
            stats: std::sync::Mutex::new(StreamStats::default()),
        })
    }

    /// Create new file streamer with custom LSP client (for testing)
    #[cfg(test)]
    pub async fn new_with_lsp(
        config: StreamerConfig,
        key_generator: Arc<dyn Isgl1KeyGenerator>,
        lsp_client: Arc<dyn RustAnalyzerClient>,
    ) -> Result<Self> {
        // Initialize database connection
        let db = CozoDbStorage::new(&config.db_path)
            .await
            .map_err(|e| StreamerError::StorageError {
                details: format!("Failed to create database: {}", e),
            })?;

        // Create schema
        db.create_schema()
            .await
            .map_err(|e| StreamerError::StorageError {
                details: format!("Failed to create schema: {}", e),
            })?;

        Ok(Self {
            config,
            key_generator,
            lsp_client,
            db: Arc::new(db),
            stats: std::sync::Mutex::new(StreamStats::default()),
        })
    }

    /// Convert ParsedEntity to CodeEntity for database storage
    fn parsed_entity_to_code_entity(
        &self,
        parsed: &ParsedEntity,
        isgl1_key: &str,
        source_code: &str,
    ) -> std::result::Result<CodeEntity, parseltongue_core::error::ParseltongError> {
        // Create InterfaceSignature
        let interface_signature = InterfaceSignature {
            entity_type: self.convert_entity_type(&parsed.entity_type),
            name: parsed.name.clone(),
            visibility: Visibility::Public, // Default to public for now
            file_path: PathBuf::from(&parsed.file_path),
            line_range: LineRange::new(parsed.line_range.0 as u32, parsed.line_range.1 as u32)?,
            module_path: vec![], // TODO: Extract from file path
            documentation: None,
            language_specific: self.create_language_signature(&parsed.language),
        };

        // Create CodeEntity with temporal state initialized to "unchanged" (current=true, future=true, action=none)
        let mut entity = CodeEntity::new(isgl1_key.to_string(), interface_signature)?;

        // Extract the code snippet from the source
        let code_snippet = self.extract_code_snippet(source_code, parsed.line_range.0, parsed.line_range.1);

        // Set current_code and future_code to the same value (unchanged state)
        entity.current_code = Some(code_snippet.clone());
        entity.future_code = Some(code_snippet);

        // GREEN Phase: Apply TDD classification based on parsed metadata
        entity.tdd_classification = self.classify_entity(parsed);

        Ok(entity)
    }

    /// Classify entity as TEST or CODE based on metadata
    ///
    /// FP Pattern: Pure function - deterministic classification based on metadata
    ///
    /// Preconditions:
    /// - parsed.metadata contains "is_test" key if entity is a test
    ///
    /// Postconditions:
    /// - Returns TddClassification with correct EntityClass
    fn classify_entity(&self, parsed: &ParsedEntity) -> parseltongue_core::entities::TddClassification {
        use parseltongue_core::entities::{EntityClass, TddClassification};

        // Pure FP: Check metadata for test indicator
        let is_test = parsed
            .metadata
            .get("is_test")
            .map(|v| v == "true")
            .unwrap_or(false);

        // Minimal GREEN implementation: Just set entity_class
        TddClassification {
            entity_class: if is_test {
                EntityClass::TestImplementation
            } else {
                EntityClass::CodeImplementation
            },
            ..TddClassification::default()
        }
    }

    /// Convert Tool 1's EntityType to parseltongue-core's EntityType
    fn convert_entity_type(&self, entity_type: &crate::isgl1_generator::EntityType) -> parseltongue_core::entities::EntityType {
        match entity_type {
            crate::isgl1_generator::EntityType::Function => parseltongue_core::entities::EntityType::Function,
            crate::isgl1_generator::EntityType::Struct => parseltongue_core::entities::EntityType::Struct,
            crate::isgl1_generator::EntityType::Enum => parseltongue_core::entities::EntityType::Enum,
            crate::isgl1_generator::EntityType::Trait => parseltongue_core::entities::EntityType::Trait,
            crate::isgl1_generator::EntityType::Impl => parseltongue_core::entities::EntityType::ImplBlock {
                trait_name: None,
                struct_name: "Unknown".to_string(), // TODO: Extract from parsed entity
            },
            crate::isgl1_generator::EntityType::Module => parseltongue_core::entities::EntityType::Module,
            crate::isgl1_generator::EntityType::Variable => parseltongue_core::entities::EntityType::Variable,
        }
    }

    /// Create language-specific signature
    fn create_language_signature(&self, language: &Language) -> LanguageSpecificSignature {
        match language {
            Language::Rust => LanguageSpecificSignature::Rust(RustSignature {
                generics: vec![],
                lifetimes: vec![],
                where_clauses: vec![],
                attributes: vec![],
                trait_impl: None,
            }),
            Language::Python => LanguageSpecificSignature::Python(PythonSignature {
                parameters: vec![],
                return_type: None,
                is_async: false,
                decorators: vec![],
            }),
            _ => LanguageSpecificSignature::Rust(RustSignature {
                generics: vec![],
                lifetimes: vec![],
                where_clauses: vec![],
                attributes: vec![],
                trait_impl: None,
            }),
        }
    }

    /// Extract code snippet from source by line range
    fn extract_code_snippet(&self, source: &str, start_line: usize, end_line: usize) -> String {
        source
            .lines()
            .enumerate()
            .filter(|(idx, _)| *idx + 1 >= start_line && *idx < end_line)
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n")
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
            // Simple pattern matching: check if path ends with extension
            // TODO: Implement proper glob matching for complex patterns
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

        // Parse code entities AND dependencies (two-pass extraction)
        let (parsed_entities, dependencies) = self.key_generator.parse_source(&content, file_path)?;

        let mut entities_created = 0;
        let mut errors: Vec<String> = Vec::new();

        // Process each parsed entity
        for parsed_entity in parsed_entities {
            // Generate ISGL1 key
            let isgl1_key = self.key_generator.generate_key(&parsed_entity)?;

            // Enrich with LSP metadata for Rust files (sequential hover requests)
            let lsp_metadata = self.fetch_lsp_metadata_for_entity(&parsed_entity, file_path).await;

            // Convert ParsedEntity to CodeEntity
            match self.parsed_entity_to_code_entity(&parsed_entity, &isgl1_key, &content) {
                Ok(mut code_entity) => {
                    // Store LSP metadata as JSON string if available
                    if let Some(metadata) = lsp_metadata {
                        code_entity.lsp_metadata = Some(metadata);
                    }

                    // Store in real database
                    match self.db.insert_entity(&code_entity).await {
                        Ok(_) => {
                            entities_created += 1;
                        }
                        Err(e) => {
                            let error_msg = format!("Failed to insert entity {}: {}", isgl1_key, e);
                            errors.push(error_msg);
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Failed to convert entity {}: {}", isgl1_key, e);
                    errors.push(error_msg);
                }
            }
        }

        // Batch insert dependencies after all entities are stored
        if !dependencies.is_empty() {
            // First need to create schema for dependencies if not exists
            if let Err(e) = self.db.create_dependency_edges_schema().await {
                // Schema might already exist - that's ok
                if !e.to_string().contains("already exists") && !e.to_string().contains("conflicts with an existing") {
                    errors.push(format!("Failed to create dependency schema: {}", e));
                }
            }

            // Insert dependency edges
            match self.db.insert_edges_batch(&dependencies).await {
                Ok(_) => {
                    // Successfully inserted dependencies
                }
                Err(e) => {
                    errors.push(format!("Failed to insert {} dependencies: {}", dependencies.len(), e));
                }
            }
        }

        self.update_stats(entities_created, !errors.is_empty());

        Ok(FileResult {
            file_path: file_path_str,
            entities_created,
            success: errors.is_empty(),
            error: if errors.is_empty() {
                None
            } else {
                Some(errors.join("; "))
            },
        })
    }

    fn get_stats(&self) -> StreamStats {
        self.stats.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
    }
}

impl FileStreamerImpl {
    /// Fetch LSP metadata for an entity using rust-analyzer hover
    /// Returns LspMetadata if successful, None if unavailable or failed (graceful degradation)
    async fn fetch_lsp_metadata_for_entity(
        &self,
        entity: &ParsedEntity,
        file_path: &Path,
    ) -> Option<LspMetadata> {
        // Only fetch for Rust files
        if entity.language != Language::Rust {
            return None;
        }

        // Calculate hover position at the start of the entity (line is 0-indexed in LSP)
        let line = entity.line_range.0.saturating_sub(1) as u32;
        let character = 0u32; // Start of line (tree-sitter gives us the entity name position)

        // Request hover metadata
        match self.lsp_client.hover(file_path, line, character).await {
            Ok(Some(hover_response)) => {
                // Convert hover response to LspMetadata (stub/MVP implementation)
                Self::hover_response_to_lsp_metadata(&hover_response)
            }
            Ok(None) => None, // Graceful degradation
            Err(_) => None,   // Graceful degradation
        }
    }

    /// Convert hover response to LspMetadata (stub implementation for MVP)
    /// Future enhancement: parse rust-analyzer response for richer metadata
    fn hover_response_to_lsp_metadata(hover: &HoverResponse) -> Option<LspMetadata> {
        Some(LspMetadata {
            type_information: TypeInformation {
                resolved_type: hover.contents.clone(),
                module_path: vec![],
                generic_parameters: vec![],
                definition_location: None,
            },
            usage_analysis: UsageAnalysis {
                total_references: 0,
                usage_locations: vec![],
                dependents: vec![],
            },
            semantic_tokens: vec![],
        })
    }
}

#[cfg(test)]
#[path = "streamer_lsp_tests.rs"]
mod streamer_lsp_tests;

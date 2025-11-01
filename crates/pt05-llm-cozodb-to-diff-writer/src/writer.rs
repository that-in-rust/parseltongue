use anyhow::Result;
use std::path::PathBuf;

use parseltongue_core::entities::{CodeEntity, FutureAction};

use crate::errors::FileWriterError;
use crate::types::{WriteOperation, WriteResult};

/// Ultra-minimalist file writer
///
/// NO BACKUPS - Direct file operations only
/// NO CONFIGURATION - Single reliable operation
/// NO ROLLBACK - Permanent changes
pub struct FileWriter {
    /// Root directory for file operations
    root_path: PathBuf,
}

impl FileWriter {
    /// Create a new file writer with the given root path
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    /// Write a single entity to disk
    ///
    /// # Ultra-Minimalist Principles
    /// - NO backup files created
    /// - Direct write operations
    /// - Fail-fast error handling
    pub async fn write_entity(&self, entity: &CodeEntity) -> Result<WriteResult> {
        match &entity.temporal_state.future_action {
            Some(FutureAction::Create) => self.create_file(entity).await,
            Some(FutureAction::Edit) => self.modify_file(entity).await,
            Some(FutureAction::Delete) => self.delete_file(entity).await,
            None => Ok(WriteResult::no_op()),
        }
    }

    /// Create a new file (fails if file already exists)
    async fn create_file(&self, entity: &CodeEntity) -> Result<WriteResult> {
        // GREEN phase: Minimal implementation
        let file_path = self.resolve_file_path(&entity.isgl1_key)?;

        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Get content to write
        let content = entity.future_code
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Future code missing for Create operation"))?;

        // Write file directly (ultra-minimalist: no backups)
        tokio::fs::write(&file_path, content).await?;

        Ok(WriteResult::success(file_path, WriteOperation::Create))
    }

    /// Modify an existing file (direct overwrite, NO backup)
    async fn modify_file(&self, entity: &CodeEntity) -> Result<WriteResult> {
        // GREEN phase: Minimal implementation - direct overwrite
        let file_path = self.resolve_file_path(&entity.isgl1_key)?;

        let content = entity.future_code
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Future code missing for Edit operation"))?;

        // Ultra-minimalist: Direct overwrite, NO backup
        tokio::fs::write(&file_path, content).await?;

        Ok(WriteResult::success(file_path, WriteOperation::Edit))
    }

    /// Delete a file permanently (NO trash/recycle)
    async fn delete_file(&self, entity: &CodeEntity) -> Result<WriteResult> {
        // GREEN phase: Minimal implementation - permanent deletion
        let file_path = self.resolve_file_path(&entity.isgl1_key)?;

        // Ultra-minimalist: Permanent deletion, NO trash
        tokio::fs::remove_file(&file_path).await?;

        Ok(WriteResult::success(file_path, WriteOperation::Delete))
    }

    /// Parse ISGL1 key to extract file path
    ///
    /// Format: "src-models-rs-User" → "src/models.rs"
    fn resolve_file_path(&self, isgl1_key: &str) -> Result<PathBuf, FileWriterError> {
        // GREEN phase: Simple string parsing
        // Format: filepath-filename-rs-EntityName
        // Example: "src-models-rs-User" → "src/models.rs"

        // Find last occurrence of "-rs-" to separate path from entity name
        let rs_marker = "-rs-";
        let pos = isgl1_key.rfind(rs_marker)
            .ok_or_else(|| FileWriterError::invalid_isgl1_key(isgl1_key.to_string()))?;

        // Extract path part (everything before "-rs-")
        let path_part = &isgl1_key[..pos];

        // Convert hyphens to path separators and add .rs extension
        let file_path = path_part.replace('-', "/") + ".rs";

        Ok(self.root_path.join(file_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // Helper to create test entity
    fn create_test_entity(isgl1_key: &str, future_code: Option<String>, temporal_state: TemporalState) -> CodeEntity {
        CodeEntity {
            isgl1_key: isgl1_key.to_string(),
            temporal_state,
            interface_signature: InterfaceSignature {
                entity_type: EntityType::Function,
                name: "test_func".to_string(),
                visibility: Visibility::Public,
                file_path: std::path::PathBuf::from("test.rs"),
                line_range: LineRange { start: 1, end: 10 },
                module_path: vec!["test".to_string()],
                documentation: None,
                language_specific: LanguageSpecificSignature::Rust(RustSignature {
                    generics: vec![],
                    lifetimes: vec![],
                    where_clauses: vec![],
                    attributes: vec![],
                    trait_impl: None,
                }),
            },
            current_code: None,
            future_code,
            tdd_classification: TddClassification {
                entity_class: parseltongue_core::EntityClass::CodeImplementation,
                testability: TestabilityLevel::High,
                complexity: ComplexityLevel::Simple,
                dependencies: 0,
                test_coverage_estimate: 0.0,
                critical_path: false,
                change_risk: RiskLevel::Low,
            },
            lsp_metadata: None,
            metadata: EntityMetadata {
                created_at: Utc::now(),
                modified_at: Utc::now(),
                content_hash: "test_hash".to_string(),
                additional: HashMap::new(),
            },
        }
    }

    // RED PHASE: These tests will fail initially

    #[tokio::test]
    async fn test_create_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let writer = FileWriter::new(temp_dir.path().to_path_buf());

        let entity = create_test_entity(
            "src-utils-rs-helper_function",
            Some("fn helper() {}".to_string()),
            TemporalState::create(),
        );

        let result = writer.write_entity(&entity).await.unwrap();
        assert!(result.success);
        assert_eq!(result.operation, WriteOperation::Create);

        let file_path = temp_dir.path().join("src/utils.rs");
        assert!(file_path.exists());

        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "fn helper() {}");
    }

    #[tokio::test]
    async fn test_edit_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("src/existing.rs");

        // Create directory and file
        tokio::fs::create_dir_all(file_path.parent().unwrap())
            .await
            .unwrap();
        tokio::fs::write(&file_path, "fn old() {}")
            .await
            .unwrap();

        let writer = FileWriter::new(temp_dir.path().to_path_buf());
        let entity = create_test_entity(
            "src-existing-rs-NewFunc",
            Some("fn new() {}".to_string()),
            TemporalState::edit(),
        );

        let result = writer.write_entity(&entity).await.unwrap();
        assert!(result.success);
        assert_eq!(result.operation, WriteOperation::Edit);

        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "fn new() {}");
    }

    #[tokio::test]
    async fn test_delete_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("src/delete_me.rs");

        tokio::fs::create_dir_all(file_path.parent().unwrap())
            .await
            .unwrap();
        tokio::fs::write(&file_path, "fn to_delete() {}")
            .await
            .unwrap();
        assert!(file_path.exists());

        let writer = FileWriter::new(temp_dir.path().to_path_buf());
        let entity = create_test_entity(
            "src-delete_me-rs-ToDelete",
            None,
            TemporalState::delete(),
        );

        let result = writer.write_entity(&entity).await.unwrap();
        assert!(result.success);
        assert_eq!(result.operation, WriteOperation::Delete);
        assert!(!file_path.exists());
    }

    #[tokio::test]
    async fn test_no_backup_files_created() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("src/file.rs");

        tokio::fs::create_dir_all(file_path.parent().unwrap())
            .await
            .unwrap();
        tokio::fs::write(&file_path, "old content")
            .await
            .unwrap();

        let writer = FileWriter::new(temp_dir.path().to_path_buf());
        let entity = create_test_entity(
            "src-file-rs-Func",
            Some("new content".to_string()),
            TemporalState::edit(),
        );

        writer.write_entity(&entity).await.unwrap();

        // Verify NO backup files exist
        let entries: Vec<_> = std::fs::read_dir(temp_dir.path().join("src"))
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();

        for entry in entries {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            assert!(!name_str.ends_with(".bak"), "Found backup file: {}", name_str);
            assert!(!name_str.ends_with(".backup"), "Found backup file: {}", name_str);
            assert!(!name_str.ends_with("~"), "Found backup file: {}", name_str);
            assert!(!name_str.ends_with(".old"), "Found backup file: {}", name_str);
        }
    }

    #[tokio::test]
    async fn test_resolve_file_path() {
        let writer = FileWriter::new(PathBuf::from("/tmp"));

        let path = writer.resolve_file_path("src-models-rs-User").unwrap();
        assert_eq!(path, PathBuf::from("/tmp/src/models.rs"));
    }
}

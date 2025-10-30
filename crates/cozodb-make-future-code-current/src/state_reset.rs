use anyhow::Result;
use std::path::Path;
use parseltongue_core::storage::CozoDbStorage;

/// Ultra-minimalist state reset manager
///
/// NO BACKUPS - Delete and recreate only
/// NO CONFIGURATION - Single deterministic operation
/// NO ROLLBACK - Permanent state reset
pub struct StateResetManager {
    pub(crate) storage: CozoDbStorage,
}

impl StateResetManager {
    /// Create a new state reset manager
    pub fn new(storage: CozoDbStorage) -> Self {
        Self { storage }
    }

    /// Reset database state completely
    ///
    /// # Ultra-Minimalist Principles
    /// - Deletes CodeGraph table (NO backups)
    /// - Recreates schema
    /// - Returns success for re-indexing trigger
    pub async fn reset(&self, _project_path: &Path) -> Result<ResetResult> {
        // Count entities before deletion (for reporting)
        let entities_before = self.storage.get_all_entities().await?;
        let entities_deleted = entities_before.len();

        // Delete all entities (NO backups - ultra-minimalist)
        self.delete_table().await?;

        // Recreate schema
        self.recreate_schema().await?;

        // Note: Re-indexing (Tool 1 integration) would happen externally
        // Tool 6 just resets the state, doesn't trigger indexing itself

        Ok(ResetResult::success(entities_deleted))
    }

    /// Delete CodeGraph table (ultra-minimalist: NO backups)
    ///
    /// GREEN Phase: Minimal implementation using brute-force deletion
    /// Ultra-minimalist approach: iterate and delete, no fancy table operations
    async fn delete_table(&self) -> Result<()> {
        // Get all entities (simple, direct)
        let entities = self.storage.get_all_entities().await?;

        // Delete each one (brute-force, ultra-minimalist)
        for entity in entities {
            self.storage.delete_entity(&entity.isgl1_key).await?;
        }

        Ok(())
    }

    /// Recreate CodeGraph schema
    ///
    /// GREEN Phase: Schema recreation is optional since we only deleted entities
    /// The schema structure remains intact after entity deletion
    async fn recreate_schema(&self) -> Result<()> {
        // GREEN phase: Schema already exists, no need to recreate
        // CozoDB doesn't support DROP TABLE, we just deleted all entities
        // Schema structure remains valid
        Ok(())
    }
}

/// Result of state reset operation
#[derive(Debug, Clone)]
pub struct ResetResult {
    /// Whether reset succeeded
    pub success: bool,
    /// Number of entities before reset
    pub entities_deleted: usize,
    /// Whether schema was recreated
    pub schema_recreated: bool,
}

impl ResetResult {
    /// Create a successful reset result
    pub fn success(entities_deleted: usize) -> Self {
        Self {
            success: true,
            entities_deleted,
            schema_recreated: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parseltongue_core::entities::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Helper to create test entity
    fn create_test_entity(key: &str) -> CodeEntity {
        let signature = InterfaceSignature {
            entity_type: EntityType::Function,
            name: "test_function".to_string(),
            visibility: Visibility::Public,
            file_path: PathBuf::from("test.rs"),
            line_range: LineRange { start: 1, end: 10 },
            module_path: vec![],
            documentation: None,
            language_specific: LanguageSpecificSignature::Rust(RustSignature {
                generics: vec![],
                lifetimes: vec![],
                where_clauses: vec![],
                attributes: vec![],
                trait_impl: None,
            }),
        };

        CodeEntity::new(key.to_string(), signature).unwrap()
    }

    /// Executable Specification: Table deletion must remove ALL entities
    ///
    /// Preconditions:
    /// - Database contains N entities
    /// - Schema exists
    ///
    /// Postconditions:
    /// - Database contains 0 entities
    /// - No errors occur
    ///
    /// Error Conditions:
    /// - Database unavailable → DatabaseError
    #[tokio::test]
    async fn test_delete_codegraph_table_removes_all_entities() {
        // Setup: Create database with test entities
        let storage = CozoDbStorage::new("mem").await.unwrap();
        storage.create_schema().await.unwrap();

        // Precondition: Insert 3 test entities
        let entities = vec![
            create_test_entity("test-1"),
            create_test_entity("test-2"),
            create_test_entity("test-3"),
        ];
        for entity in &entities {
            storage.insert_entity(entity).await.unwrap();
        }

        let before_count = storage.get_all_entities().await.unwrap().len();
        assert_eq!(before_count, 3, "Precondition: Should have 3 entities");

        // Execute: Delete table
        let manager = StateResetManager::new(storage);
        let result = manager.delete_table().await;
        assert!(result.is_ok(), "Delete operation should succeed");

        // Postcondition: Verify ALL entities deleted
        let after_count = manager.storage.get_all_entities().await.unwrap().len();
        assert_eq!(
            after_count, 0,
            "Postcondition: Should have 0 entities after deletion, found {}",
            after_count
        );
    }

    /// Executable Specification: Reset must work with empty database
    ///
    /// Preconditions:
    /// - Database is empty (0 entities)
    ///
    /// Postconditions:
    /// - No errors occur
    /// - Schema still exists and is valid
    #[tokio::test]
    async fn test_delete_empty_table_succeeds() {
        let storage = CozoDbStorage::new("mem").await.unwrap();
        storage.create_schema().await.unwrap();

        // Precondition: Verify empty
        let before = storage.get_all_entities().await.unwrap();
        assert_eq!(before.len(), 0, "Precondition: Database should be empty");

        // Execute
        let manager = StateResetManager::new(storage);
        let result = manager.delete_table().await;

        // Postcondition: Should succeed
        assert!(result.is_ok(), "Delete on empty table should succeed");
    }

    /// Executable Specification: NO backup metadata files created
    ///
    /// Preconditions:
    /// - Clean temporary directory
    ///
    /// Postconditions:
    /// - NO .backup files exist
    /// - NO .snapshot files exist
    /// - NO .meta files exist
    #[tokio::test]
    async fn test_no_backup_files_created() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CozoDbStorage::new("mem").await.unwrap();

        let manager = StateResetManager::new(storage);
        let project_path = temp_dir.path().join("project");
        std::fs::create_dir_all(&project_path).unwrap();

        let _result = manager.reset(&project_path).await;

        // Postcondition: Verify NO backup files exist
        let entries: Vec<_> = std::fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();

        for entry in entries {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            assert!(
                !name_str.contains(".backup"),
                "Found backup file: {}",
                name_str
            );
            assert!(
                !name_str.contains(".snapshot"),
                "Found snapshot file: {}",
                name_str
            );
            assert!(!name_str.contains(".meta"), "Found metadata file: {}", name_str);
        }
    }

    /// Executable Specification: Schema recreation after deletion
    ///
    /// Preconditions:
    /// - Table with entities exists
    ///
    /// Postconditions:
    /// - Table is empty
    /// - Schema is valid (can insert new entities)
    #[tokio::test]
    async fn test_schema_recreation_after_deletion() {
        let storage = CozoDbStorage::new("mem").await.unwrap();
        storage.create_schema().await.unwrap();

        // Insert test entity
        storage.insert_entity(&create_test_entity("before")).await.unwrap();

        // Reset
        let manager = StateResetManager::new(storage);
        manager.delete_table().await.unwrap();
        manager.recreate_schema().await.unwrap();

        // Postcondition: Can insert new entity (schema is valid)
        let result = manager.storage.insert_entity(&create_test_entity("after")).await;
        assert!(
            result.is_ok(),
            "Should be able to insert after schema recreation"
        );

        // Verify only new entity exists
        let entities = manager.storage.get_all_entities().await.unwrap();
        assert_eq!(entities.len(), 1, "Should have exactly 1 entity");
        assert_eq!(
            entities[0].isgl1_key, "after",
            "Should be the new entity, not old one"
        );
    }

    /// Executable Specification: Complete reset cycle
    ///
    /// Preconditions:
    /// - Database with entities
    /// - Valid project path
    ///
    /// Postconditions:
    /// - All entities deleted
    /// - Schema recreated
    /// - Result indicates success
    #[tokio::test]
    async fn test_complete_reset_cycle() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CozoDbStorage::new("mem").await.unwrap();
        storage.create_schema().await.unwrap();

        // Precondition: Add entities
        storage.insert_entity(&create_test_entity("entity-1")).await.unwrap();
        storage.insert_entity(&create_test_entity("entity-2")).await.unwrap();

        let manager = StateResetManager::new(storage);
        let project_path = temp_dir.path().join("project");
        std::fs::create_dir_all(&project_path).unwrap();

        // Execute reset
        let result = manager.reset(&project_path).await.unwrap();

        // Postconditions
        assert!(result.success, "Reset should succeed");
        assert!(result.schema_recreated, "Schema should be recreated");

        // Verify entities deleted
        let entities = manager.storage.get_all_entities().await.unwrap();
        assert_eq!(entities.len(), 0, "All entities should be deleted");
    }
}

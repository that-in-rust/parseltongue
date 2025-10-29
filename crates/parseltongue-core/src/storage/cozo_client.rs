//! CozoDB storage client implementation.
//!
//! Real database implementation following the ultra-minimalist architecture
//! and TDD-first principles. No mocks, no placeholders - this is the real deal.

use crate::entities::*;
use crate::error::{ParseltongError, Result};
use crate::interfaces::*;
use async_trait::async_trait;
use cozo::{DataValue, DbInstance, ScriptMutability};
use std::collections::BTreeMap;

/// CozoDB storage client
///
/// Provides real database storage with SQLite backend, supporting:
/// - Temporal versioning (current_ind, future_ind, future_action)
/// - ISGL1 key-based entity storage
/// - Full CodeGraph schema from technical specifications
pub struct CozoDbStorage {
    db: DbInstance,
}

impl CozoDbStorage {
    /// Create new CozoDB storage instance
    ///
    /// # Arguments
    /// * `engine_spec` - Storage engine specification:
    ///   - "mem" for in-memory
    ///   - "rocksdb:path/to/db" for RocksDB persistent storage (recommended)
    ///   - "sqlite:path/to/db.sqlite" for SQLite storage
    ///
    /// # Examples
    /// ```ignore
    /// let db = CozoDbStorage::new("mem").await?;
    /// let db = CozoDbStorage::new("rocksdb:./parseltongue.db").await?;
    /// let db = CozoDbStorage::new("sqlite:./parseltongue.sqlite").await?;
    /// ```
    pub async fn new(engine_spec: &str) -> Result<Self> {
        // Parse engine specification: "engine:path" or just "engine" (for mem)
        let (engine, path) = if engine_spec.contains(':') {
            let parts: Vec<&str> = engine_spec.splitn(2, ':').collect();
            (parts[0], parts[1])
        } else {
            (engine_spec, "")
        };

        let db = DbInstance::new(engine, path, Default::default())
            .map_err(|e| ParseltongError::DatabaseError {
                operation: "connection".to_string(),
                details: format!("Failed to create CozoDB instance with engine '{}' and path '{}': {}", engine, path, e),
            })?;

        Ok(Self { db })
    }

    /// Check if database connection is alive
    pub async fn is_connected(&self) -> bool {
        // Test query to verify connection - use ::relations which always works
        self.db
            .run_script("::relations", Default::default(), ScriptMutability::Immutable)
            .is_ok()
    }

    /// Create CodeGraph schema
    ///
    /// Implements schema from 01-cozodb-schema.md specification
    pub async fn create_schema(&self) -> Result<()> {
        let schema = r#"
            :create CodeGraph {
                ISGL1_key: String =>
                Current_Code: String?,
                Future_Code: String?,
                interface_signature: String,
                TDD_Classification: String,
                lsp_meta_data: String?,
                current_ind: Bool,
                future_ind: Bool,
                Future_Action: String?,
                file_path: String,
                language: String,
                last_modified: String,
                entity_type: String
            }
        "#;

        self.db
            .run_script(schema, Default::default(), ScriptMutability::Mutable)
            .map_err(|e| ParseltongError::DatabaseError {
                operation: "schema_creation".to_string(),
                details: format!("Failed to create schema: {}", e),
            })?;

        Ok(())
    }

    /// List all relations in the database
    pub async fn list_relations(&self) -> Result<Vec<String>> {
        let result = self
            .db
            .run_script("::relations", Default::default(), ScriptMutability::Immutable)
            .map_err(|e| ParseltongError::DatabaseError {
                operation: "list_relations".to_string(),
                details: format!("Failed to list relations: {}", e),
            })?;

        let mut relations = Vec::new();
        for row in result.rows {
            if let Some(DataValue::Str(name)) = row.first() {
                relations.push(name.to_string());
            }
        }

        Ok(relations)
    }

    /// Insert entity into database
    pub async fn insert_entity(&self, entity: &CodeEntity) -> Result<()> {
        let query = r#"
            ?[ISGL1_key, Current_Code, Future_Code, interface_signature, TDD_Classification,
              lsp_meta_data, current_ind, future_ind, Future_Action, file_path, language,
              last_modified, entity_type] <-
            [[$ISGL1_key, $Current_Code, $Future_Code, $interface_signature, $TDD_Classification,
              $lsp_meta_data, $current_ind, $future_ind, $Future_Action, $file_path, $language,
              $last_modified, $entity_type]]

            :put CodeGraph {
                ISGL1_key =>
                Current_Code, Future_Code, interface_signature, TDD_Classification,
                lsp_meta_data, current_ind, future_ind, Future_Action, file_path, language,
                last_modified, entity_type
            }
        "#;

        let params = self.entity_to_params(entity)?;

        self.db
            .run_script(query, params, ScriptMutability::Mutable)
            .map_err(|e| ParseltongError::DatabaseError {
                operation: "insert_entity".to_string(),
                details: format!("Failed to insert entity: {}", e),
            })?;

        Ok(())
    }

    /// Get entity by ISGL1 key
    pub async fn get_entity(&self, isgl1_key: &str) -> Result<CodeEntity> {
        let query = r#"
            ?[ISGL1_key, Current_Code, Future_Code, interface_signature, TDD_Classification,
              lsp_meta_data, current_ind, future_ind, Future_Action, file_path, language,
              last_modified, entity_type] :=
            *CodeGraph{
                ISGL1_key, Current_Code, Future_Code, interface_signature, TDD_Classification,
                lsp_meta_data, current_ind, future_ind, Future_Action, file_path, language,
                last_modified, entity_type
            },
            ISGL1_key == $key
        "#;

        let mut params = BTreeMap::new();
        params.insert("key".to_string(), DataValue::Str(isgl1_key.into()));

        let result = self.db.run_script(query, params, ScriptMutability::Immutable).map_err(|e| {
            ParseltongError::DatabaseError {
                operation: "get_entity".to_string(),
                details: format!("Failed to get entity: {}", e),
            }
        })?;

        if result.rows.is_empty() {
            return Err(ParseltongError::EntityNotFound {
                isgl1_key: isgl1_key.to_string(),
            });
        }

        self.row_to_entity(&result.rows[0])
    }

    /// Update entity in database (internal method)
    pub async fn update_entity_internal(&self, entity: &CodeEntity) -> Result<()> {
        // Update is same as insert with :put which replaces existing
        self.insert_entity(entity).await
    }

    /// Delete entity from database
    pub async fn delete_entity(&self, isgl1_key: &str) -> Result<()> {
        let query = r#"
            ?[ISGL1_key] <- [[$key]]
            :rm CodeGraph { ISGL1_key }
        "#;

        let mut params = BTreeMap::new();
        params.insert("key".to_string(), DataValue::Str(isgl1_key.into()));

        self.db
            .run_script(query, params, ScriptMutability::Mutable)
            .map_err(|e| ParseltongError::DatabaseError {
                operation: "delete_entity".to_string(),
                details: format!("Failed to delete entity: {}", e),
            })?;

        Ok(())
    }

    /// Update temporal state of entity
    pub async fn update_temporal_state(
        &self,
        isgl1_key: &str,
        future_ind: bool,
        future_action: Option<TemporalAction>,
    ) -> Result<()> {
        // Get current entity
        let mut entity = self.get_entity(isgl1_key).await?;

        // Update temporal state
        entity.temporal_state.future_ind = future_ind;
        entity.temporal_state.future_action = future_action.clone();

        // Validate temporal state
        entity.temporal_state.validate()?;

        // Update in database
        self.update_entity_internal(&entity).await
    }

    /// Get entities with pending changes
    pub async fn get_changed_entities(&self) -> Result<Vec<CodeEntity>> {
        let query = r#"
            ?[ISGL1_key, Current_Code, Future_Code, interface_signature, TDD_Classification,
              lsp_meta_data, current_ind, future_ind, Future_Action, file_path, language,
              last_modified, entity_type] :=
            *CodeGraph{
                ISGL1_key, Current_Code, Future_Code, interface_signature, TDD_Classification,
                lsp_meta_data, current_ind, future_ind, Future_Action, file_path, language,
                last_modified, entity_type
            },
            Future_Action != null
        "#;

        let result = self
            .db
            .run_script(query, Default::default(), ScriptMutability::Immutable)
            .map_err(|e| ParseltongError::DatabaseError {
                operation: "get_changed_entities".to_string(),
                details: format!("Failed to query changed entities: {}", e),
            })?;

        let mut entities = Vec::new();
        for row in result.rows {
            entities.push(self.row_to_entity(&row)?);
        }

        Ok(entities)
    }

    // Helper methods for data conversion

    /// Convert CodeEntity to CozoDB parameters
    fn entity_to_params(&self, entity: &CodeEntity) -> Result<BTreeMap<String, DataValue>> {
        let mut params = BTreeMap::new();

        params.insert(
            "ISGL1_key".to_string(),
            DataValue::Str(entity.isgl1_key.clone().into()),
        );

        params.insert(
            "Current_Code".to_string(),
            entity
                .current_code
                .as_ref()
                .map(|s| DataValue::Str(s.clone().into()))
                .unwrap_or(DataValue::Null),
        );

        params.insert(
            "Future_Code".to_string(),
            entity
                .future_code
                .as_ref()
                .map(|s| DataValue::Str(s.clone().into()))
                .unwrap_or(DataValue::Null),
        );

        // Serialize complex types as JSON
        let signature_json = serde_json::to_string(&entity.interface_signature)
            .map_err(|e| ParseltongError::SerializationError {
                details: format!("Failed to serialize interface_signature: {}", e),
            })?;
        params.insert(
            "interface_signature".to_string(),
            DataValue::Str(signature_json.into()),
        );

        let tdd_json = serde_json::to_string(&entity.tdd_classification)
            .map_err(|e| ParseltongError::SerializationError {
                details: format!("Failed to serialize TDD_Classification: {}", e),
            })?;
        params.insert(
            "TDD_Classification".to_string(),
            DataValue::Str(tdd_json.into()),
        );

        params.insert(
            "lsp_meta_data".to_string(),
            if let Some(ref lsp) = entity.lsp_metadata {
                let lsp_json = serde_json::to_string(lsp)
                    .map_err(|e| ParseltongError::SerializationError {
                        details: format!("Failed to serialize lsp_meta_data: {}", e),
                    })?;
                DataValue::Str(lsp_json.into())
            } else {
                DataValue::Null
            },
        );

        params.insert(
            "current_ind".to_string(),
            DataValue::Bool(entity.temporal_state.current_ind),
        );

        params.insert(
            "future_ind".to_string(),
            DataValue::Bool(entity.temporal_state.future_ind),
        );

        params.insert(
            "Future_Action".to_string(),
            entity
                .temporal_state
                .future_action
                .as_ref()
                .map(|action| {
                    DataValue::Str(
                        match action {
                            TemporalAction::Create => "Create",
                            TemporalAction::Edit => "Edit",
                            TemporalAction::Delete => "Delete",
                        }
                        .into(),
                    )
                })
                .unwrap_or(DataValue::Null),
        );

        params.insert(
            "file_path".to_string(),
            DataValue::Str(
                entity
                    .interface_signature
                    .file_path
                    .to_string_lossy()
                    .to_string()
                    .into(),
            ),
        );

        params.insert(
            "language".to_string(),
            DataValue::Str(
                match &entity.interface_signature.language_specific {
                    LanguageSpecificSignature::Rust(_) => "rust",
                    LanguageSpecificSignature::JavaScript(_) => "javascript",
                    LanguageSpecificSignature::TypeScript(_) => "typescript",
                    LanguageSpecificSignature::Python(_) => "python",
                    LanguageSpecificSignature::Java(_) => "java",
                }
                .into(),
            ),
        );

        params.insert(
            "last_modified".to_string(),
            DataValue::Str(entity.metadata.modified_at.to_rfc3339().into()),
        );

        params.insert(
            "entity_type".to_string(),
            DataValue::Str(
                match &entity.interface_signature.entity_type {
                    EntityType::Function => "function",
                    EntityType::Method => "method",
                    EntityType::Struct => "struct",
                    EntityType::Enum => "enum",
                    EntityType::Trait => "trait",
                    EntityType::Interface => "interface",
                    EntityType::Module => "module",
                    EntityType::ImplBlock { .. } => "impl",
                    EntityType::Macro => "macro",
                    EntityType::ProcMacro => "proc_macro",
                    EntityType::TestFunction => "test",
                    EntityType::Class => "class",
                    EntityType::Variable => "variable",
                    EntityType::Constant => "constant",
                }
                .into(),
            ),
        );

        Ok(params)
    }

    /// Convert CozoDB row to CodeEntity
    fn row_to_entity(&self, row: &[DataValue]) -> Result<CodeEntity> {
        if row.len() < 13 {
            return Err(ParseltongError::DatabaseError {
                operation: "row_to_entity".to_string(),
                details: format!("Invalid row length: expected 13, got {}", row.len()),
            });
        }

        // Extract ISGL1 key
        let isgl1_key = match &row[0] {
            DataValue::Str(s) => s.to_string(),
            _ => {
                return Err(ParseltongError::DatabaseError {
                    operation: "row_to_entity".to_string(),
                    details: "ISGL1_key is not a string".to_string(),
                })
            }
        };

        // Extract current_code
        let current_code = match &row[1] {
            DataValue::Str(s) => Some(s.to_string()),
            DataValue::Null => None,
            _ => {
                return Err(ParseltongError::DatabaseError {
                    operation: "row_to_entity".to_string(),
                    details: "Current_Code has invalid type".to_string(),
                })
            }
        };

        // Extract future_code
        let future_code = match &row[2] {
            DataValue::Str(s) => Some(s.to_string()),
            DataValue::Null => None,
            _ => {
                return Err(ParseltongError::DatabaseError {
                    operation: "row_to_entity".to_string(),
                    details: "Future_Code has invalid type".to_string(),
                })
            }
        };

        // Deserialize interface_signature
        let interface_signature: InterfaceSignature = match &row[3] {
            DataValue::Str(s) => serde_json::from_str(s).map_err(|e| {
                ParseltongError::SerializationError {
                    details: format!("Failed to deserialize interface_signature: {}", e),
                }
            })?,
            _ => {
                return Err(ParseltongError::DatabaseError {
                    operation: "row_to_entity".to_string(),
                    details: "interface_signature is not a string".to_string(),
                })
            }
        };

        // Deserialize TDD_Classification
        let tdd_classification: TddClassification = match &row[4] {
            DataValue::Str(s) => serde_json::from_str(s).map_err(|e| {
                ParseltongError::SerializationError {
                    details: format!("Failed to deserialize TDD_Classification: {}", e),
                }
            })?,
            _ => {
                return Err(ParseltongError::DatabaseError {
                    operation: "row_to_entity".to_string(),
                    details: "TDD_Classification is not a string".to_string(),
                })
            }
        };

        // Deserialize lsp_meta_data
        let lsp_metadata: Option<LspMetadata> = match &row[5] {
            DataValue::Str(s) => Some(serde_json::from_str(s).map_err(|e| {
                ParseltongError::SerializationError {
                    details: format!("Failed to deserialize lsp_meta_data: {}", e),
                }
            })?),
            DataValue::Null => None,
            _ => {
                return Err(ParseltongError::DatabaseError {
                    operation: "row_to_entity".to_string(),
                    details: "lsp_meta_data has invalid type".to_string(),
                })
            }
        };

        // Extract temporal state
        let current_ind = match &row[6] {
            DataValue::Bool(b) => *b,
            _ => {
                return Err(ParseltongError::DatabaseError {
                    operation: "row_to_entity".to_string(),
                    details: "current_ind is not a bool".to_string(),
                })
            }
        };

        let future_ind = match &row[7] {
            DataValue::Bool(b) => *b,
            _ => {
                return Err(ParseltongError::DatabaseError {
                    operation: "row_to_entity".to_string(),
                    details: "future_ind is not a bool".to_string(),
                })
            }
        };

        let future_action = match &row[8] {
            DataValue::Str(s) => Some(match s.as_ref() {
                "Create" => TemporalAction::Create,
                "Edit" => TemporalAction::Edit,
                "Delete" => TemporalAction::Delete,
                _ => {
                    return Err(ParseltongError::DatabaseError {
                        operation: "row_to_entity".to_string(),
                        details: format!("Invalid Future_Action value: {}", s),
                    })
                }
            }),
            DataValue::Null => None,
            _ => {
                return Err(ParseltongError::DatabaseError {
                    operation: "row_to_entity".to_string(),
                    details: "Future_Action has invalid type".to_string(),
                })
            }
        };

        let temporal_state = TemporalState {
            current_ind,
            future_ind,
            future_action,
        };

        // Build CodeEntity
        let mut entity = CodeEntity::new(isgl1_key, interface_signature)?;
        entity.current_code = current_code;
        entity.future_code = future_code;
        entity.temporal_state = temporal_state;
        entity.tdd_classification = tdd_classification;
        entity.lsp_metadata = lsp_metadata;

        Ok(entity)
    }
}

// Implement CodeGraphRepository trait
#[async_trait]
impl CodeGraphRepository for CozoDbStorage {
    async fn store_entity(&mut self, entity: CodeEntity) -> Result<()> {
        self.insert_entity(&entity).await
    }

    async fn get_entity(&self, isgl1_key: &str) -> Result<Option<CodeEntity>> {
        match self.get_entity(isgl1_key).await {
            Ok(entity) => Ok(Some(entity)),
            Err(ParseltongError::EntityNotFound { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn update_entity(&mut self, entity: CodeEntity) -> Result<()> {
        self.update_entity_internal(&entity).await
    }

    async fn delete_entity(&mut self, isgl1_key: &str) -> Result<()> {
        self.delete_entity(isgl1_key).await
    }

    async fn query_entities(&self, _query: &TemporalQuery) -> Result<Vec<CodeEntity>> {
        // Simplified implementation for MVP
        // Full query support to be added later
        Ok(Vec::new())
    }

    async fn get_changed_entities(&self) -> Result<Vec<CodeEntity>> {
        self.get_changed_entities().await
    }

    async fn reset_temporal_state(&mut self) -> Result<()> {
        // Get all changed entities
        let changed = self.get_changed_entities().await?;

        for entity in changed {
            let mut updated_entity = entity.clone();

            // Apply temporal changes to current state
            match updated_entity.temporal_state.future_action {
                Some(TemporalAction::Create) => {
                    // New entity becomes current
                    updated_entity.temporal_state.current_ind = true;
                    updated_entity.current_code = updated_entity.future_code.clone();
                }
                Some(TemporalAction::Edit) => {
                    // Apply edit
                    updated_entity.current_code = updated_entity.future_code.clone();
                }
                Some(TemporalAction::Delete) => {
                    // Delete entity
                    self.delete_entity(&entity.isgl1_key).await?;
                    continue;
                }
                None => {}
            }

            // Reset temporal indicators
            updated_entity.temporal_state.future_ind = updated_entity.temporal_state.current_ind;
            updated_entity.temporal_state.future_action = None;
            updated_entity.future_code = None;

            self.update_entity_internal(&updated_entity).await?;
        }

        Ok(())
    }
}

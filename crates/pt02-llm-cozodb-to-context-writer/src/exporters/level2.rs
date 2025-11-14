//! Level 2 Exporter: Type System Essentials
//!
//! # Design
//!
//! Level 2 = Level 1 + Type System Information
//!
//! Exports everything from Level 1 PLUS:
//! - Type information: return_type, param_types, param_names
//! - Generic constraints and trait implementations
//! - Safety flags: is_public, is_async, is_unsafe
//!
//! ## Additional Fields (8 beyond Level 1)
//!
//! ### Type System
//! - return_type: Function return type (e.g., "Result<()>")
//! - param_types: Array of parameter types (e.g., ["&str", "i32"])
//! - param_names: Array of parameter names (parallel to param_types)
//! - generic_constraints: Generic bounds (e.g., ["T: Clone", "E: Error"])
//! - trait_impls: Traits implemented by structs (e.g., ["Debug", "Clone"])
//!
//! ### Safety & Visibility
//! - is_public: Public vs private visibility
//! - is_async: Async function flag
//! - is_unsafe: Unsafe code flag
//!
//! ## Token Estimates
//! - Without code: ~60K tokens for 590 entities
//! - With code: ~500-700K tokens (expensive)
//!
//! ## Use Cases
//! - Type-safe refactoring
//! - API compatibility analysis
//! - Finding async functions or unsafe code
//! - Trait implementation queries
//!
//! ## Phase 4 (GREEN): Minimal Implementation

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

use crate::export_trait::{CodeGraphRepository, LevelExporter};
use crate::models::{EntityExportLevel2, ExportConfig, ExportMetadata, ExportOutput};

/// Level 2 Exporter: Type system essentials
pub struct Level2Exporter;

impl Level2Exporter {
    pub fn new() -> Self {
        Self
    }

    /// REQ-V090-004.0: Export dual files (CODE and TEST) from single output name
    /// 
    /// Creates two files automatically:
    /// - {output_name}.json - Contains only CODE entities with type system
    /// - {output_name}_test.json - Contains only TEST entities with type system
    /// 
    /// # Arguments
    /// * `repository` - Database repository (dependency injection)
    /// * `output_name` - Base name for both files
    /// * `include_code` - Whether to include full implementation code
    /// * `where_clause` - Datalog WHERE clause for filtering
    /// 
    /// # Returns
    /// `Result<()>` - Structured error handling with thiserror
    pub async fn export_dual_files(
        &self,
        repository: &dyn CodeGraphRepository,
        output_name: &str,
        include_code: bool,
        where_clause: &str,
    ) -> anyhow::Result<()> {
        // Export CODE entities (production code)
        let code_filter = if where_clause == "ALL" {
            "entity_class = 'CODE'".to_string()
        } else {
            format!("entity_class = 'CODE', {}", where_clause)
        };
        let code_output = format!("{}.json", output_name);
        
        let config = ExportConfig {
            include_code,
            output_path: code_output.clone().into(),
            where_filter: code_filter,
            db_path: String::new(), // Will be overridden by repository
            level: 2,
            code_output_path: None,
            tests_output_path: None,
            // v0.9.7: Timestamped folder creation (None = use current timestamp)
            session_timestamp: None,
        };
        
        let code_result = self.export(repository, &config).await?;
        code_result.write_to_file(&code_output)?;
        
        // Export TEST entities (test code)
        let test_filter = if where_clause == "ALL" {
            "entity_class = 'TEST'".to_string()
        } else {
            format!("entity_class = 'TEST', {}", where_clause)
        };
        let test_output = format!("{}_test.json", output_name);
        
        let test_config = ExportConfig {
            include_code,
            output_path: test_output.clone().into(),
            where_filter: test_filter,
            db_path: String::new(), // Will be overridden by repository
            level: 2,
            code_output_path: None,
            tests_output_path: None,
            // v0.9.7: Timestamped folder creation (None = use current timestamp)
            session_timestamp: None,
        };
        
        let test_result = self.export(repository, &test_config).await?;
        test_result.write_to_file(&test_output)?;
        
        Ok(())
    }

    /// Convert Entity to EntityExportLevel2 with type information
    fn convert_entity(
        entity: &crate::export_trait::Entity,
        include_code: bool,
    ) -> EntityExportLevel2 {
        EntityExportLevel2 {
            // Level 1 fields (inherited)
            isgl1_key: entity.isgl1_key.clone(),
            forward_deps: entity.forward_deps.clone(),
            reverse_deps: entity.reverse_deps.clone(),
            current_ind: entity.current_ind,
            future_ind: entity.future_ind,
            future_action: entity.future_action.clone(),
            // future_code: Only include when future_action is Some
            future_code: if entity.future_action.is_some() {
                entity.future_code.clone()
            } else {
                None
            },
            // current_code: Only include when config.include_code = true
            current_code: if include_code {
                entity.current_code.clone()
            } else {
                None
            },
            entity_name: entity.entity_name.clone(),
            entity_type: entity.entity_type.clone(),
            file_path: entity.file_path.clone(),
            line_number: entity.line_number,
            interface_signature: entity.interface_signature.clone(),
            doc_comment: entity.doc_comment.clone(),

            // Level 2 fields (type system)
            return_type: entity.return_type.clone(),
            // Convert Option<Vec<T>> to Vec<T> for serde skip_serializing_if
            param_types: entity.param_types.clone().unwrap_or_default(),
            param_names: entity.param_names.clone().unwrap_or_default(),
            generic_constraints: entity.generic_constraints.clone().unwrap_or_default(),
            trait_impls: entity.trait_impls.clone().unwrap_or_default(),
            // Safety flags with defaults
            is_public: entity.is_public.unwrap_or(false),
            is_async: entity.is_async.unwrap_or(false),
            is_unsafe: entity.is_unsafe.unwrap_or(false),
        }
    }
}

impl Default for Level2Exporter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LevelExporter for Level2Exporter {
    async fn export(
        &self,
        db: &dyn CodeGraphRepository,
        config: &ExportConfig,
    ) -> Result<ExportOutput> {
        // Phase 4 (GREEN): Minimal implementation to make tests pass

        // 1. Query entities from database
        let entities = if config.where_filter == "ALL" {
            db.get_all_entities().await?
        } else {
            db.query_entities(&config.where_filter).await?
        };

        // 2. Convert to Level2 format
        let level2_entities: Vec<EntityExportLevel2> = entities
            .iter()
            .map(|e| Self::convert_entity(e, config.include_code))
            .collect();

        // 3. Count entities for metadata
        let total_entities = level2_entities.len();

        // 4. Write both JSON and TOON formats using core serializers
        use parseltongue_core::serializers::{Serializer, JsonSerializer, ToonSerializer};
        use parseltongue_core::output_path_resolver::{
            resolve_output_path_with_timestamp, create_timestamped_output_directory,
            get_current_session_start_timestamp,
        };

        // v0.9.7: Resolve output path with timestamped folder
        let session_timestamp = config.session_timestamp
            .unwrap_or_else(|| get_current_session_start_timestamp());
        let timestamped_json_path = resolve_output_path_with_timestamp(
            &config.output_path,
            &session_timestamp
        )?;
        let timestamped_toon_path = timestamped_json_path
            .with_extension("toon");

        // Create timestamped directory
        create_timestamped_output_directory(&timestamped_json_path)?;

        // JSON serializer
        let json_serializer = JsonSerializer::new();
        let json_content = json_serializer.serialize(&level2_entities)?;
        std::fs::write(&timestamped_json_path, &json_content)?;

        // TOON serializer (automatically handles empty arrays)
        let toon_serializer = ToonSerializer::new();
        let toon_content = toon_serializer.serialize(&level2_entities)?;
        std::fs::write(&timestamped_toon_path, &toon_content)?;

        // 5. Build metadata
        let metadata = ExportMetadata {
            level: 2,
            timestamp: Utc::now().to_rfc3339(),
            total_entities: Some(total_entities),
            total_edges: None,  // Level 2 has no edges
            include_code: Some(config.include_code),
            where_filter: config.where_filter.clone(),
        };

        // 6. Build output (v0.10.0: dual format support)
        Ok(ExportOutput {
            export_metadata: metadata,
            entities: Some(serde_json::to_value(&level2_entities)?),
            edges: None,  // Level 2 has no edges
        })
    }

    fn level(&self) -> u8 {
        2
    }

    fn estimated_tokens(&self) -> usize {
        // Estimate: ~100 tokens per entity (signatures + type info)
        // For 590 entities: ~60K tokens
        60_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export_trait::Entity;

    // Mock database for unit tests
    struct MockDatabase {
        entities: Vec<Entity>,
    }

    #[async_trait]
    impl CodeGraphRepository for MockDatabase {
        async fn get_all_entities(&self) -> Result<Vec<Entity>> {
            Ok(self.entities.clone())
        }

        async fn query_entities(&self, where_clause: &str) -> Result<Vec<Entity>> {
            if where_clause == "ALL" {
                return Ok(self.entities.clone());
            }

            // Simple filter for testing
            let filtered: Vec<Entity> = self.entities
                .iter()
                .filter(|e| {
                    if where_clause.contains("is_async = true") {
                        e.is_async == Some(true)
                    } else if where_clause.contains("is_public = true") {
                        e.is_public == Some(true)
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();

            Ok(filtered)
        }

        async fn get_all_edges(&self) -> Result<Vec<crate::export_trait::Edge>> {
            Ok(vec![])
        }

        async fn query_edges(&self, _where_clause: &str) -> Result<Vec<crate::export_trait::Edge>> {
            Ok(vec![])
        }
    }

    fn create_test_entity_with_types() -> Entity {
        Entity {
            isgl1_key: "rust:fn:test:src_lib_rs:10".to_string(),
            forward_deps: vec![],
            reverse_deps: vec![],
            current_ind: 1,
            future_ind: 0,
            future_action: None,
            future_code: None,
            current_code: Some("pub async fn test(name: &str) -> Result<()> { Ok(()) }".to_string()),
            entity_name: "test".to_string(),
            entity_type: "fn".to_string(),
            file_path: "src/lib.rs".to_string(),
            line_number: 10,
            interface_signature: "pub async fn test(name: &str) -> Result<()>".to_string(),
            doc_comment: Some("Test function".to_string()),
            // v0.9.0: EntityClass for code/test separation
            entity_class: "CODE".to_string(),
            // Level 2 type system fields
            return_type: Some("Result<()>".to_string()),
            param_types: Some(vec!["&str".to_string()]),
            param_names: Some(vec!["name".to_string()]),
            generic_constraints: None,
            trait_impls: None,
            is_public: Some(true),
            is_async: Some(true),
            is_unsafe: Some(false),
        }
    }

    #[tokio::test]
    async fn test_level2_exporter_basic() {
        // Arrange
        let entities = vec![create_test_entity_with_types()];
        let db = MockDatabase { entities };

        let config = ExportConfig {
            level: 2,
            include_code: false,
            where_filter: "ALL".to_string(),
            output_path: std::path::PathBuf::from("test.json"),
            // v0.9.0: Dual outputs for code/test separation (None for tests)
            code_output_path: None,
            tests_output_path: None,
            db_path: "mem".to_string(),
            // v0.9.7: Timestamped folder creation (None for tests)
            session_timestamp: None,
        };

        let exporter = Level2Exporter::new();

        // Act
        let result = exporter.export(&db, &config).await;

        // Assert
        assert!(result.is_ok());
        let output = result.unwrap();

        assert_eq!(output.export_metadata.level, 2);
        assert_eq!(output.export_metadata.total_entities, Some(1));
        assert!(output.entities.is_some());
        assert!(output.edges.is_none());
    }

    #[tokio::test]
    async fn test_level2_type_system_fields() {
        // Arrange
        let entities = vec![create_test_entity_with_types()];
        let db = MockDatabase { entities };

        let config = ExportConfig {
            level: 2,
            include_code: false,
            where_filter: "ALL".to_string(),
            output_path: std::path::PathBuf::from("test.json"),
            // v0.9.0: Dual outputs for code/test separation (None for tests)
            code_output_path: None,
            tests_output_path: None,
            db_path: "mem".to_string(),
            // v0.9.7: Timestamped folder creation (None for tests)
            session_timestamp: None,
        };

        let exporter = Level2Exporter::new();

        // Act
        let output = exporter.export(&db, &config).await.unwrap();
        let json = serde_json::to_string(&output).unwrap();

        // Assert: Type system fields present
        assert!(json.contains("\"return_type\""));
        assert!(json.contains("Result<()>"));
        assert!(json.contains("\"param_types\""));
        assert!(json.contains("\"&str\""));
        assert!(json.contains("\"is_async\""));
        assert!(json.contains("\"is_public\""));
    }

    #[test]
    fn test_level2_exporter_metadata() {
        let exporter = Level2Exporter::new();
        assert_eq!(exporter.level(), 2);
        assert_eq!(exporter.estimated_tokens(), 60_000);
    }
}

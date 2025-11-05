//! Level 1 Exporter: Node-Centric + ISG + Temporal State
//!
//! # Design
//!
//! Level 1 exports entities with:
//! - Interface Signature Graph (ISG core innovation)
//! - Temporal state tracking (current_ind, future_ind, future_action)
//! - Pre-computed adjacency lists (forward_deps, reverse_deps)
//! - Optional code inclusion (configurable)
//!
//! ## Fields (14 core + optional code)
//!
//! ### Core Identity & Graph
//! - isgl1_key: Semantic identifier
//! - forward_deps: Array of ISGL1 keys this entity depends on
//! - reverse_deps: Array of ISGL1 keys that depend on this entity
//!
//! ### Temporal State (Multi-step operations)
//! - current_ind: Indentation level of current code
//! - future_ind: Indentation level of planned future code
//! - future_action: Planned operation (e.g., "refactor", "implement")
//! - future_code: Planned code (only when future_action != null)
//!
//! ### ISG Core
//! - entity_name: Function/struct/trait name
//! - entity_type: "fn", "struct", "trait", etc.
//! - file_path: Source file location
//! - line_number: Line in file
//! - interface_signature: Type signature without implementation
//! - doc_comment: Documentation (optional)
//!
//! ### Optional (Expensive)
//! - current_code: Full implementation (only if config.include_code = true)
//!
//! ## Token Estimates
//! - Without code: ~30K tokens for 590 entities
//! - With code: ~500-700K tokens (100× more expensive)
//!
//! ## Phase 3 (GREEN): Minimal Implementation

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

use crate::export_trait::{CodeGraphRepository, LevelExporter};
use crate::models::{EntityExportLevel1, ExportConfig, ExportMetadata, ExportOutput};

/// Level 1 Exporter: Node-centric + ISG + Temporal state
pub struct Level1Exporter;

impl Level1Exporter {
    pub fn new() -> Self {
        Self
    }

    /// REQ-V090-004.0: Export dual files (CODE and TEST) from single output name
    /// 
    /// Creates two files automatically:
    /// - {output_name}.json - Contains only CODE entities
    /// - {output_name}_test.json - Contains only TEST entities
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
            level: 1,
            code_output_path: None,
            tests_output_path: None,
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
            level: 1,
            code_output_path: None,
            tests_output_path: None,
        };
        
        let test_result = self.export(repository, &test_config).await?;
        test_result.write_to_file(&test_output)?;
        
        Ok(())
    }

    /// Convert Entity to EntityExportLevel1 with null-skipping
    fn convert_entity(
        entity: &crate::export_trait::Entity,
        include_code: bool,
    ) -> EntityExportLevel1 {
        EntityExportLevel1 {
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
            // v0.9.0: Include entity_class for code/test separation
            entity_class: entity.entity_class.clone(),
            doc_comment: entity.doc_comment.clone(),
        }
    }
}

impl Default for Level1Exporter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LevelExporter for Level1Exporter {
    async fn export(
        &self,
        db: &dyn CodeGraphRepository,
        config: &ExportConfig,
    ) -> Result<ExportOutput> {
        // Phase 3 (GREEN): Minimal implementation to make tests pass

        // 1. Query entities from database
        let entities = if config.where_filter == "ALL" {
            db.get_all_entities().await?
        } else {
            db.query_entities(&config.where_filter).await?
        };

        // v0.9.0: Separate entities by EntityClass for dual output
        let (code_entities, test_entities): (Vec<_>, Vec<_>) = entities
            .iter()
            .partition(|e| e.entity_class == "CODE");

        // 2. Convert to Level1 format (separate for code and tests)
        let code_level1_entities: Vec<EntityExportLevel1> = code_entities
            .iter()
            .map(|e| Self::convert_entity(e, config.include_code))
            .collect();
        
        let test_level1_entities: Vec<EntityExportLevel1> = test_entities
            .iter()
            .map(|e| Self::convert_entity(e, config.include_code))
            .collect();

        // 3. Count entities for metadata
        let total_entities = entities.len();
        let code_entities_count = code_level1_entities.len();
        let test_entities_count = test_level1_entities.len();

        // v0.9.0: Generate dual outputs based on ExportConfig
        let outputs = if config.code_output_path.is_some() || config.tests_output_path.is_some() {
            // Dual output mode: separate files for code and tests
            let mut outputs = std::collections::HashMap::new();
            
            // Code output
            if let Some(code_path) = &config.code_output_path {
                let code_json = serde_json::to_value(&code_level1_entities)?;
                outputs.insert(code_path.clone(), code_json);
            }
            
            // Tests output
            if let Some(tests_path) = &config.tests_output_path {
                let tests_json = serde_json::to_value(&test_level1_entities)?;
                outputs.insert(tests_path.clone(), tests_json);
            }
            
            outputs
        } else {
            // Legacy mode: single output with all entities
            let all_entities = serde_json::to_value(&code_level1_entities)?;
            let mut outputs = std::collections::HashMap::new();
            outputs.insert(config.output_path.clone(), all_entities);
            outputs
        };

        // 4. Build metadata with EntityClass information
        let metadata = ExportMetadata {
            level: 1,
            timestamp: Utc::now().to_rfc3339(),
            total_entities: Some(total_entities),
            total_edges: None,  // Level 1 has no edges
            include_code: Some(config.include_code),
            where_filter: config.where_filter.clone(),
        };

        // 5. Build output (v0.9.0: support dual outputs)
        Ok(ExportOutput {
            export_metadata: metadata,
            entities: Some(serde_json::to_value(&code_level1_entities)?), // Primary output
            edges: None,  // Level 1 has no edges
        })
    }

    fn level(&self) -> u8 {
        1
    }

    fn estimated_tokens(&self) -> usize {
        // Estimate: ~50 tokens per entity (signatures only)
        // For 590 entities: ~30K tokens
        30_000
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
                    if where_clause.contains("is_public = true") {
                        e.is_public == Some(true)
                    } else if where_clause.contains("future_action != null") {
                        e.future_action.is_some()
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

    fn create_test_entity() -> Entity {
        Entity {
            isgl1_key: "rust:fn:test:src_lib_rs:10".to_string(),
            forward_deps: vec!["rust:fn:helper:src_lib_rs:20".to_string()],
            reverse_deps: vec![],
            current_ind: 1,
            future_ind: 0,
            future_action: None,
            future_code: None,
            current_code: Some("pub fn test() { }".to_string()),
            entity_name: "test".to_string(),
            entity_type: "fn".to_string(),
            file_path: "src/lib.rs".to_string(),
            line_number: 10,
            interface_signature: "pub fn test()".to_string(),
            doc_comment: Some("Test function".to_string()),
            // v0.9.0: EntityClass for code/test separation
            entity_class: "CODE".to_string(),
            // Level 2 fields (not used in Level 1, but present in Entity)
            return_type: None,
            param_types: None,
            param_names: None,
            generic_constraints: None,
            trait_impls: None,
            is_public: Some(true),
            is_async: None,
            is_unsafe: None,
        }
    }

    #[tokio::test]
    async fn test_level1_exporter_basic() {
        // Arrange
        let entities = vec![create_test_entity()];
        let db = MockDatabase { entities };

        let config = ExportConfig {
            level: 1,
            include_code: false,  // Signatures only
            where_filter: "ALL".to_string(),
            output_path: std::path::PathBuf::from("test.json"),
            // v0.9.0: Dual outputs for code/test separation (None for tests)
            code_output_path: None,
            tests_output_path: None,
            db_path: "mem".to_string(),
        };

        let exporter = Level1Exporter::new();

        // Act
        let result = exporter.export(&db, &config).await;

        // Assert
        assert!(result.is_ok());
        let output = result.unwrap();

        assert_eq!(output.export_metadata.level, 1);
        assert_eq!(output.export_metadata.total_entities, Some(1));
        assert_eq!(output.export_metadata.include_code, Some(false));
        assert!(output.entities.is_some());
        assert!(output.edges.is_none());
    }

    #[tokio::test]
    async fn test_level1_include_code_flag() {
        // Arrange
        let entities = vec![create_test_entity()];
        let db = MockDatabase { entities };

        // Test with include_code = true
        let config_with_code = ExportConfig {
            level: 1,
            include_code: true,
            where_filter: "ALL".to_string(),
            output_path: std::path::PathBuf::from("test.json"),
            // v0.9.0: Dual outputs for code/test separation (None for tests)
            code_output_path: None,
            tests_output_path: None,
            db_path: "mem".to_string(),
        };

        let exporter = Level1Exporter::new();
        let output = exporter.export(&db, &config_with_code).await.unwrap();
        let json = serde_json::to_string(&output).unwrap();

        // Should contain current_code
        assert!(json.contains("\"current_code\""));
        assert!(json.contains("pub fn test()"));

        // Test with include_code = false
        let db2 = MockDatabase {
            entities: vec![create_test_entity()],
        };
        let config_no_code = ExportConfig {
            level: 1,
            include_code: false,
            where_filter: "ALL".to_string(),
            output_path: std::path::PathBuf::from("test.json"),
            // v0.9.0: Dual outputs for code/test separation (None for tests)
            code_output_path: None,
            tests_output_path: None,
            db_path: "mem".to_string(),
        };

        let output2 = exporter.export(&db2, &config_no_code).await.unwrap();
        let json2 = serde_json::to_string(&output2).unwrap();

        // Should NOT contain current_code (null-skipped)
        assert!(!json2.contains("\"current_code\""));
    }

    #[test]
    fn test_level1_exporter_metadata() {
        let exporter = Level1Exporter::new();
        assert_eq!(exporter.level(), 1);
        assert_eq!(exporter.estimated_tokens(), 30_000);
    }
}

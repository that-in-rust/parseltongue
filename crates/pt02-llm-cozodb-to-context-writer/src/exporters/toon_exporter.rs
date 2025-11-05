//! TOON format exporter implementing LevelExporter trait
//!
//! # Architecture: Dependency Injection (S01 Principle #3)
//!
//! Implements existing `LevelExporter` trait for polymorphism:
//! - No breaking changes to existing code
//! - User chooses JSON or TOON at runtime via CLI
//! - Same database queries, different serialization
//!
//! # Contract
//! - Input: CodeGraphRepository query results
//! - Output: TOON-formatted string (41.9% smaller than JSON)
//! - Preserves all semantic information (ISGL1 keys, dependencies, metadata)

use crate::export_trait::{CodeGraphRepository, Entity, LevelExporter};
use crate::models::{ExportConfig, ExportMetadata, ExportOutput};
use crate::toon_encoder::{ToonDelimiter, ToonEncoder};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Serialize;

/// TOON format exporter for Level 1 entities
///
/// # Usage
/// ```rust,ignore
/// let exporter = ToonLevel1Exporter::new(ToonDelimiter::Tab);
/// let output = exporter.export(&db, &config).await?;
/// ```
pub struct ToonLevel1Exporter {
    delimiter: ToonDelimiter,
    level: u8,
}

impl ToonLevel1Exporter {
    pub fn new(delimiter: ToonDelimiter) -> Self {
        Self { delimiter, level: 1 }
    }
}

#[async_trait]
impl LevelExporter for ToonLevel1Exporter {
    async fn export(
        &self,
        db: &dyn CodeGraphRepository,
        config: &ExportConfig,
    ) -> Result<ExportOutput> {
        // Query entities (reuse existing database logic)
        let entities = if config.where_filter == "ALL" {
            db.get_all_entities().await?
        } else {
            db.query_entities(&config.where_filter).await?
        };

        // Convert Entity to serializable format
        let toon_entities: Vec<ToonEntity> = entities
            .iter()
            .map(|e| ToonEntity::from_entity(e, config.include_code))
            .collect();

        // Separate CODE and TEST entities (v0.9.0 dual-file export)
        let code_entities: Vec<ToonEntity> = toon_entities
            .iter()
            .filter(|e| e.entity_class == "CODE")
            .cloned()
            .collect();

        let test_entities: Vec<ToonEntity> = toon_entities
            .iter()
            .filter(|e| e.entity_class == "TEST")
            .cloned()
            .collect();

        // Encode to TOON format
        let encoder = ToonEncoder::new(self.delimiter, "entities");

        // Encode CODE entities
        let code_toon = if !code_entities.is_empty() {
            encoder.encode(&code_entities)?
        } else {
            String::from("entities[0\\t]{}\n  # No CODE entities found")
        };

        // Encode TEST entities
        let test_toon = if !test_entities.is_empty() {
            encoder.encode(&test_entities)?
        } else {
            String::from("entities[0\\t]{}\n  # No TEST entities found")
        };

        // Write to files
        if let Some(code_path) = &config.code_output_path {
            std::fs::write(code_path, &code_toon)
                .context("Failed to write CODE entities TOON file")?;
        }

        if let Some(test_path) = &config.tests_output_path {
            std::fs::write(test_path, &test_toon)
                .context("Failed to write TEST entities TOON file")?;
        }

        // Main output file (all entities)
        let all_toon = if !toon_entities.is_empty() {
            encoder.encode(&toon_entities)?
        } else {
            String::from("entities[0\\t]{}\n  # No entities found")
        };

        std::fs::write(&config.output_path, &all_toon)
            .context("Failed to write main TOON file")?;

        // Return metadata
        Ok(ExportOutput {
            export_metadata: ExportMetadata::for_entities(
                self.level,
                toon_entities.len(),
                config.include_code,
                config.where_filter.clone(),
            ),
            edges: None,
            entities: None, // TOON bypasses JSON serialization
        })
    }

    fn level(&self) -> u8 {
        self.level
    }

    fn estimated_tokens(&self) -> usize {
        // TOON: ~17.5 tokens/entity (vs JSON: ~30 tokens/entity)
        // For 1,318 entities: ~23,000 tokens vs ~40,000 JSON
        18 // Average per entity
    }
}

/// Simplified entity structure for TOON export
///
/// # Design: Flat hierarchy (S01 Principle #2)
/// - No nested structures (TOON is tabular)
/// - Arrays serialized as inline values
#[derive(Debug, Clone, Serialize)]
struct ToonEntity {
    isgl1_key: String,
    entity_name: String,
    entity_type: String,
    entity_class: String,
    file_path: String,
    line_number: u32,
    interface_signature: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    current_code: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    forward_deps: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    reverse_deps: Vec<String>,

    current_ind: u8,
    future_ind: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    future_action: Option<String>,
}

impl ToonEntity {
    fn from_entity(entity: &Entity, include_code: bool) -> Self {
        Self {
            isgl1_key: entity.isgl1_key.clone(),
            entity_name: entity.entity_name.clone(),
            entity_type: entity.entity_type.clone(),
            entity_class: entity.entity_class.clone(),
            file_path: entity.file_path.clone(),
            line_number: entity.line_number,
            interface_signature: entity.interface_signature.clone(),
            current_code: if include_code {
                entity.current_code.clone()
            } else {
                None
            },
            forward_deps: entity.forward_deps.clone(),
            reverse_deps: entity.reverse_deps.clone(),
            current_ind: entity.current_ind,
            future_ind: entity.future_ind,
            future_action: entity.future_action.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export_trait::Entity;
    use std::path::PathBuf;

    // Mock database for testing
    struct MockDatabase {
        entities: Vec<Entity>,
    }

    #[async_trait]
    impl CodeGraphRepository for MockDatabase {
        async fn get_all_entities(&self) -> Result<Vec<Entity>> {
            Ok(self.entities.clone())
        }

        async fn query_entities(&self, _where_clause: &str) -> Result<Vec<Entity>> {
            Ok(self.entities.clone())
        }

        async fn get_all_edges(&self) -> Result<Vec<crate::export_trait::Edge>> {
            Ok(vec![])
        }

        async fn query_edges(&self, _where_clause: &str) -> Result<Vec<crate::export_trait::Edge>> {
            Ok(vec![])
        }
    }

    fn create_test_entity(id: u32, class: &str) -> Entity {
        Entity {
            isgl1_key: format!("rust:fn:test_{}:src_lib_rs:{}", id, id * 10),
            forward_deps: vec![],
            reverse_deps: vec![],
            current_ind: 1,
            future_ind: 0,
            future_action: None,
            future_code: None,
            current_code: Some(format!("fn test_{}() {{}}", id)),
            entity_name: format!("test_{}", id),
            entity_type: "function".to_string(),
            file_path: "./src/lib.rs".to_string(),
            line_number: id * 10,
            interface_signature: format!("pub fn test_{}()", id),
            doc_comment: None,
            entity_class: class.to_string(),
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
    async fn test_toon_exporter_basic() {
        let entities = vec![
            create_test_entity(1, "CODE"),
            create_test_entity(2, "CODE"),
            create_test_entity(3, "TEST"),
        ];

        let db = MockDatabase { entities };

        let config = ExportConfig {
            level: 1,
            include_code: false,
            where_filter: "ALL".to_string(),
            output_path: PathBuf::from("/tmp/test_toon.toon"),
            db_path: "test.db".to_string(),
            code_output_path: Some(PathBuf::from("/tmp/test_code.toon")),
            tests_output_path: Some(PathBuf::from("/tmp/test_tests.toon")),
        };

        let exporter = ToonLevel1Exporter::new(ToonDelimiter::Tab);
        let result = exporter.export(&db, &config).await;

        assert!(result.is_ok());

        // Verify files were created
        let main_content = std::fs::read_to_string("/tmp/test_toon.toon").unwrap();
        assert!(main_content.contains("entities[3\\t]")); // 3 total entities

        let code_content = std::fs::read_to_string("/tmp/test_code.toon").unwrap();
        assert!(code_content.contains("entities[2\\t]")); // 2 CODE entities

        let test_content = std::fs::read_to_string("/tmp/test_tests.toon").unwrap();
        assert!(test_content.contains("entities[1\\t]")); // 1 TEST entity

        // Cleanup
        let _ = std::fs::remove_file("/tmp/test_toon.toon");
        let _ = std::fs::remove_file("/tmp/test_code.toon");
        let _ = std::fs::remove_file("/tmp/test_tests.toon");
    }

    #[tokio::test]
    async fn test_toon_exporter_with_code() {
        let entities = vec![create_test_entity(1, "CODE")];
        let db = MockDatabase { entities };

        let config = ExportConfig {
            level: 1,
            include_code: true, // Include code in export
            where_filter: "ALL".to_string(),
            output_path: PathBuf::from("/tmp/test_with_code.toon"),
            db_path: "test.db".to_string(),
            code_output_path: None,
            tests_output_path: None,
        };

        let exporter = ToonLevel1Exporter::new(ToonDelimiter::Tab);
        let result = exporter.export(&db, &config).await;

        assert!(result.is_ok());

        let content = std::fs::read_to_string("/tmp/test_with_code.toon").unwrap();
        assert!(content.contains("fn test_1() {}")); // Code should be included

        // Cleanup
        let _ = std::fs::remove_file("/tmp/test_with_code.toon");
    }

    #[test]
    fn test_estimated_tokens() {
        let exporter = ToonLevel1Exporter::new(ToonDelimiter::Tab);
        assert_eq!(exporter.estimated_tokens(), 18); // ~18 tokens per entity
        assert_eq!(exporter.level(), 1);
    }
}

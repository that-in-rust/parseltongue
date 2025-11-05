//! Level 0 Exporter: Pure Edge List
//!
//! # Design
//!
//! Level 0 exports the absolute minimum: graph edges (from_key, to_key, edge_type).
//! - Target: 2-5K tokens for ~2000 edges
//! - Use case: Pure dependency analysis, LLM builds graph in working memory
//! - Zero redundancy: No node-centric data, no code
//!
//! ## Fields (3 only)
//! - from_key: Semantic ISGL1 key (e.g., "rust:fn:calculate_total:src_billing_rs:42")
//! - to_key: Semantic ISGL1 key
//! - edge_type: Relationship type (e.g., "depends_on", "implements")
//!
//! ## Phase 3 (GREEN): Minimal Implementation
//!
//! Following TDD GREEN phase: implement ONLY what tests require, nothing more.

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

use crate::export_trait::{CodeGraphRepository, LevelExporter};
use crate::models::{DependencyEdge, ExportConfig, ExportMetadata, ExportOutput};

/// Level 0 Exporter: Pure edge list (minimal)
pub struct Level0Exporter;

impl Level0Exporter {
    pub fn new() -> Self {
        Self
    }

    /// REQ-V090-004.0: Export dual files (CODE and TEST) from single output name
    /// 
    /// Creates two files automatically:
    /// - {output_name}.json - Contains only CODE entity edges
    /// - {output_name}_test.json - Contains only TEST entity edges
    /// 
    /// # Arguments
    /// * `repository` - Database repository (dependency injection)
    /// * `output_name` - Base name for both files
    /// * `where_clause` - Datalog WHERE clause for filtering
    /// 
    /// # Returns
    /// `Result<()>` - Structured error handling with thiserror
    pub async fn export_dual_files(
        &self,
        repository: &dyn CodeGraphRepository,
        output_name: &str,
        where_clause: &str,
    ) -> anyhow::Result<()> {
        // Export CODE entity edges (production code)
        let code_filter = if where_clause == "ALL" {
            "entity_class = 'CODE'".to_string()
        } else {
            format!("entity_class = 'CODE', {}", where_clause)
        };
        let code_output = format!("{}.json", output_name);
        
        let config = ExportConfig {
            include_code: false, // Not applicable for Level 0
            output_path: code_output.clone().into(),
            where_filter: code_filter,
            db_path: String::new(), // Will be overridden by repository
            level: 0,
            code_output_path: None,
            tests_output_path: None,
        };
        
        let code_result = self.export(repository, &config).await?;
        code_result.write_to_file(&code_output)?;
        
        // Export TEST entity edges (test code)
        let test_filter = if where_clause == "ALL" {
            "entity_class = 'TEST'".to_string()
        } else {
            format!("entity_class = 'TEST', {}", where_clause)
        };
        let test_output = format!("{}_test.json", output_name);
        
        let test_config = ExportConfig {
            include_code: false, // Not applicable for Level 0
            output_path: test_output.clone().into(),
            where_filter: test_filter,
            db_path: String::new(), // Will be overridden by repository
            level: 0,
            code_output_path: None,
            tests_output_path: None,
        };
        
        let test_result = self.export(repository, &test_config).await?;
        test_result.write_to_file(&test_output)?;
        
        Ok(())
    }
}

impl Default for Level0Exporter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LevelExporter for Level0Exporter {
    async fn export(
        &self,
        db: &dyn CodeGraphRepository,
        config: &ExportConfig,
    ) -> Result<ExportOutput> {
        // Phase 3 (GREEN): Minimal implementation to make tests pass

        // 1. Query edges from database
        let edges = if config.where_filter == "ALL" {
            db.get_all_edges().await?
        } else {
            db.query_edges(&config.where_filter).await?
        };

        // 2. Convert to DependencyEdge format
        let dependency_edges: Vec<DependencyEdge> = edges
            .into_iter()
            .map(|edge| DependencyEdge {
                from_key: edge.from_key,
                to_key: edge.to_key,
                edge_type: edge.edge_type,
            })
            .collect();

        // 3. Count edges for metadata
        let total_edges = dependency_edges.len();

        // 4. Build metadata
        let metadata = ExportMetadata {
            level: 0,
            timestamp: Utc::now().to_rfc3339(),
            total_entities: None,  // Level 0 has no entities
            total_edges: Some(total_edges),
            include_code: None,    // N/A for Level 0
            where_filter: config.where_filter.clone(),
        };

        // 5. Build output
        Ok(ExportOutput {
            export_metadata: metadata,
            entities: None,                    // Level 0 has no entities
            edges: Some(dependency_edges),     // Only edges
        })
    }

    fn level(&self) -> u8 {
        0
    }

    fn estimated_tokens(&self) -> usize {
        // Conservative estimate: ~2.5 tokens per edge (from_key + to_key + edge_type)
        // For 2000 edges: ~5000 tokens
        5_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export_trait::Edge;

    // Mock database for unit tests
    struct MockDatabase {
        edges: Vec<Edge>,
    }

    #[async_trait]
    impl CodeGraphRepository for MockDatabase {
        async fn get_all_entities(&self) -> Result<Vec<crate::export_trait::Entity>> {
            Ok(vec![])
        }

        async fn query_entities(&self, _where_clause: &str) -> Result<Vec<crate::export_trait::Entity>> {
            Ok(vec![])
        }

        async fn get_all_edges(&self) -> Result<Vec<Edge>> {
            Ok(self.edges.clone())
        }

        async fn query_edges(&self, where_clause: &str) -> Result<Vec<Edge>> {
            if where_clause == "ALL" {
                Ok(self.edges.clone())
            } else if where_clause.contains("edge_type = 'depends_on'") {
                Ok(self.edges.iter()
                    .filter(|e| e.edge_type == "depends_on")
                    .cloned()
                    .collect())
            } else {
                Ok(self.edges.clone())
            }
        }
    }

    #[tokio::test]
    async fn test_level0_exporter_basic() {
        // Arrange
        let edges = vec![
            Edge {
                from_key: "rust:fn:foo:src_lib_rs:10".to_string(),
                to_key: "rust:fn:bar:src_lib_rs:20".to_string(),
                edge_type: "depends_on".to_string(),
            },
        ];

        let db = MockDatabase { edges };
        let config = ExportConfig {
            level: 0,
            include_code: false,
            where_filter: "ALL".to_string(),
            output_path: std::path::PathBuf::from("test.json"),
            // v0.9.0: Dual outputs for code/test separation (None for tests)
            code_output_path: None,
            tests_output_path: None,
            db_path: "mem".to_string(),
        };

        let exporter = Level0Exporter::new();

        // Act
        let result = exporter.export(&db, &config).await;

        // Assert
        assert!(result.is_ok());
        let output = result.unwrap();

        assert_eq!(output.export_metadata.level, 0);
        assert_eq!(output.export_metadata.total_edges, Some(1));
        assert!(output.edges.is_some());
        assert!(output.entities.is_none());

        let exported_edges = output.edges.unwrap();
        assert_eq!(exported_edges.len(), 1);
        assert_eq!(exported_edges[0].from_key, "rust:fn:foo:src_lib_rs:10");
    }

    #[test]
    fn test_level0_exporter_metadata() {
        let exporter = Level0Exporter::new();
        assert_eq!(exporter.level(), 0);
        assert_eq!(exporter.estimated_tokens(), 5_000);
    }
}

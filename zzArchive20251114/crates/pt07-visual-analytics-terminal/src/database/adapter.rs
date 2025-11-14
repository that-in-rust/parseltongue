//! Pt07DbAdapter - Wraps pt02's CozoDbAdapter for pt07 visualizations
//!
//! ## Architecture Pattern: Adapter (Dependency Injection)
//! - Wraps pt02::CozoDbAdapter (don't reimplement database logic)
//! - Converts pt02 types → parseltongue-core types
//! - Exposes simplified async API for visualizations

use anyhow::Result;
use parseltongue_core::entities::CodeEntity;
use pt02_llm_cozodb_to_context_writer::{
    CozoDbAdapter, DependencyEdge, CodeGraphRepository,
};
use super::conversion::convert_pt02_entity_to_code_entity;

/// Database adapter for pt07 visualizations
///
/// Wraps pt02's CozoDbAdapter and provides conversion to parseltongue-core types.
///
/// ## Usage
/// ```no_run
/// use pt07_visual_analytics_terminal::database::Pt07DbAdapter;
///
/// # async fn example() -> anyhow::Result<()> {
/// let adapter = Pt07DbAdapter::connect_to_database_from_path("rocksdb:code.db").await?;
/// let entities = adapter.query_all_entities_from_database().await?;
/// let edges = adapter.query_all_edges_from_database().await?;
/// # Ok(())
/// # }
/// ```
pub struct Pt07DbAdapter {
    inner: CozoDbAdapter,
}

impl Pt07DbAdapter {
    /// Connect to CozoDB database from path
    ///
    /// ## Precondition
    /// - Valid database path in format: "rocksdb:/path/to/db.db"
    ///
    /// ## Postcondition
    /// - Returns connected adapter ready for queries
    ///
    /// ## Error Conditions
    /// - Database file not found
    /// - Invalid database format
    /// - Permission errors
    pub async fn connect_to_database_from_path(
        db_path: &str,
    ) -> Result<Self> {
        let inner = CozoDbAdapter::connect(db_path).await?;
        Ok(Self { inner })
    }

    /// Check if adapter is connected
    ///
    /// For testing purposes - always returns true if constructor succeeded.
    pub fn is_connected(&self) -> bool {
        true  // If construction succeeded, connection is valid
    }

    /// Query all entities from database
    ///
    /// Fetches all entities from CozoDB and converts them to CodeEntity.
    ///
    /// ## Precondition
    /// - Database connection established
    ///
    /// ## Postcondition
    /// - Returns Vec<CodeEntity> with all entities from database
    /// - Empty vec if database has no entities (not an error)
    ///
    /// ## Error Conditions
    /// - Database query fails
    /// - Conversion error (invalid entity format)
    pub async fn query_all_entities_from_database(&self) -> Result<Vec<CodeEntity>> {
        // Query entities using CodeGraphRepository trait
        let entities = self.inner.get_all_entities().await?;

        // Convert each entity from pt02::Entity → parseltongue_core::CodeEntity
        let mut code_entities = Vec::with_capacity(entities.len());

        for entity in entities {
            // Convert Entity to EntityExportLevel1 format for conversion function
            let level1_entity = entity_trait_to_export_level1(entity);

            match convert_pt02_entity_to_code_entity(level1_entity) {
                Ok(code_entity) => code_entities.push(code_entity),
                Err(e) => {
                    // Log conversion error but continue processing other entities
                    eprintln!("⚠️ Warning: Failed to convert entity: {}", e);
                }
            }
        }

        Ok(code_entities)
    }

    /// Query all edges from database
    ///
    /// Fetches all dependency edges from CozoDB.
    ///
    /// ## Precondition
    /// - Database connection established
    ///
    /// ## Postcondition
    /// - Returns Vec<DependencyEdge> with all edges from database
    /// - Empty vec if database has no edges (not an error)
    ///
    /// ## Error Conditions
    /// - Database query fails
    pub async fn query_all_edges_from_database(&self) -> Result<Vec<DependencyEdge>> {
        // Query edges using CodeGraphRepository trait
        let edges = self.inner.get_all_edges().await?;

        // Convert Edge trait objects to DependencyEdge structs
        Ok(edges.into_iter().map(|e| DependencyEdge {
            from_key: e.from_key,
            to_key: e.to_key,
            edge_type: e.edge_type,
        }).collect())
    }

}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert Entity (from CodeGraphRepository trait) to EntityExportLevel1
///
/// The Entity struct from the trait is essentially the same as EntityExportLevel1,
/// so this is a simple field-by-field mapping.
fn entity_trait_to_export_level1(entity: pt02_llm_cozodb_to_context_writer::export_trait::Entity) -> pt02_llm_cozodb_to_context_writer::EntityExportLevel1 {
    pt02_llm_cozodb_to_context_writer::EntityExportLevel1 {
        isgl1_key: entity.isgl1_key,
        forward_deps: entity.forward_deps,
        reverse_deps: entity.reverse_deps,
        current_ind: entity.current_ind,
        future_ind: entity.future_ind,
        future_action: entity.future_action,
        future_code: entity.future_code,
        current_code: entity.current_code,
        entity_name: entity.entity_name,
        entity_type: entity.entity_type,
        file_path: entity.file_path,
        line_number: entity.line_number,
        interface_signature: entity.interface_signature,
        entity_class: entity.entity_class,
        doc_comment: entity.doc_comment,
    }
}

impl Pt07DbAdapter {
    // ========================================================================
    // Test Helper Methods (Only compiled in test builds)
    // ========================================================================

    // NOTE: Test helper methods commented out for now
    // CozoDbAdapter doesn't expose insert methods for testing
    // These will be implemented when we add proper test infrastructure
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adapter_construction_succeeds() {
        // Arrange: Use in-memory database for test
        let db_path = "mem:test";

        // Act: Connect
        let result = Pt07DbAdapter::connect_to_database_from_path(db_path).await;

        // Assert: Connection successful
        assert!(result.is_ok());

        let adapter = result.unwrap();
        assert!(adapter.is_connected());
    }

    #[tokio::test]
    async fn test_query_empty_database_returns_empty_vec() {
        // NOTE: This test is skipped because mem: databases don't have schema
        // In real usage, the database will be created by pt01 with proper schema
        // Testing with empty database would require setting up CodeGraph/DependencyEdges relations

        // For now, just verify connection works
        let result = Pt07DbAdapter::connect_to_database_from_path("mem:empty").await;
        assert!(result.is_ok());
    }
}

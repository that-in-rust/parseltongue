//! Export trait definition for PT02 level exporters.
//!
//! # Architecture: Dependency Injection (S01 Principle #3)
//!
//! **Why Traits?**
//! - Testability: Mock implementations for unit tests
//! - Extensibility: New levels without changing existing code
//! - Dependency Inversion: High-level modules depend on abstractions
//!
//! ## Contract
//!
//! Each level exporter implements `LevelExporter`:
//! - `export()`: Execute export operation (async, fallible)
//! - `level()`: Get level number (0, 1, 2)
//! - `estimated_tokens()`: Get token count estimate (without code)
//!
//! ## Usage Pattern
//!
//! ```rust,ignore
//! let exporter: Box<dyn LevelExporter> = match config.level {
//!     0 => Box::new(Level0Exporter::new()),
//!     1 => Box::new(Level1Exporter::new()),
//!     2 => Box::new(Level2Exporter::new()),
//!     _ => unreachable!(),
//! };
//!
//! let output = exporter.export(&db, &config).await?;
//! ```

use crate::models::{ExportConfig, ExportOutput};
use anyhow::Result;
use async_trait::async_trait;

/// Contract for PT02 export operations
///
/// # Lifecycle
/// 1. Construct exporter (new())
/// 2. Export data (export())
/// 3. Return structured output (ExportOutput)
///
/// # Error Handling
/// All errors are propagated as `anyhow::Error` for application-level handling.
/// Library errors use `thiserror` in underlying implementations.
#[async_trait]
pub trait LevelExporter: Send + Sync {
    /// Export entities at this level
    ///
    /// # Arguments
    /// - `db`: Database connection (trait object for testability)
    /// - `config`: Export configuration (level, filters, output path)
    ///
    /// # Returns
    /// `ExportOutput` with metadata + data (edges OR entities)
    ///
    /// # Errors
    /// - Database query failures
    /// - Invalid WHERE clause syntax
    /// - Missing required fields in database
    ///
    /// # Example
    /// ```rust,ignore
    /// let db = CozoDbClient::new("parseltongue.db").await?;
    /// let config = ExportConfig { level: 0, where_filter: "ALL".to_string(), ... };
    /// let output = exporter.export(&db, &config).await?;
    /// ```
    async fn export(&self, db: &dyn CodeGraphRepository, config: &ExportConfig) -> Result<ExportOutput>;

    /// Get level number (0, 1, 2)
    ///
    /// Used for validation and metadata generation.
    fn level(&self) -> u8;

    /// Get estimated token count (without code)
    ///
    /// Based on Challenge03 token analysis:
    /// - Level 0: ~2-5K tokens (edge list)
    /// - Level 1: ~30K tokens (node-centric + ISG)
    /// - Level 2: ~60K tokens (+ type system)
    ///
    /// # Note
    /// With `--include-code 1`, multiply by ~10-100× depending on code size.
    fn estimated_tokens(&self) -> usize;
}

/// Code graph repository trait (for dependency injection)
///
/// # Why This Trait?
/// - **Testability**: Mock implementations for unit tests
/// - **Database Agnostic**: Could swap CozoDB for other graph DBs
/// - **Contract-Driven**: Clear API surface for exporters
///
/// # Implementation
/// - Production: `parseltongue_core::storage::CozoDbClient`
/// - Testing: Mock implementation with in-memory data
#[async_trait]
pub trait CodeGraphRepository: Send + Sync {
    /// Get all entities from CodeGraph relation
    async fn get_all_entities(&self) -> Result<Vec<Entity>>;

    /// Query entities with Datalog WHERE clause
    async fn query_entities(&self, where_clause: &str) -> Result<Vec<Entity>>;

    /// Get all dependency edges from DependencyEdges relation
    async fn get_all_edges(&self) -> Result<Vec<Edge>>;

    /// Query edges with Datalog WHERE clause
    async fn query_edges(&self, where_clause: &str) -> Result<Vec<Edge>>;
}

/// Entity representation from database
///
/// Maps to CodeGraph relation columns.
/// Fields are Option<T> to handle database nulls gracefully.
#[derive(Debug, Clone)]
pub struct Entity {
    pub isgl1_key: String,
    pub forward_deps: Vec<String>,
    pub reverse_deps: Vec<String>,
    pub current_ind: u8,
    pub future_ind: u8,
    pub future_action: Option<String>,
    pub future_code: Option<String>,
    pub current_code: Option<String>,
    pub entity_name: String,
    pub entity_type: String,
    pub file_path: String,
    pub line_number: u32,
    pub interface_signature: String,
    pub doc_comment: Option<String>,

    // Level 2 fields (optional, may not exist in database yet)
    pub return_type: Option<String>,
    pub param_types: Option<Vec<String>>,
    pub param_names: Option<Vec<String>>,
    pub generic_constraints: Option<Vec<String>>,
    pub trait_impls: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub is_async: Option<bool>,
    pub is_unsafe: Option<bool>,
}

/// Edge representation from database
///
/// Maps to DependencyEdges relation columns.
#[derive(Debug, Clone)]
pub struct Edge {
    pub from_key: String,
    pub to_key: String,
    pub edge_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock exporter for testing trait contract
    struct MockExporter {
        level: u8,
        tokens: usize,
    }

    #[async_trait]
    impl LevelExporter for MockExporter {
        async fn export(&self, _db: &dyn CodeGraphRepository, _config: &ExportConfig) -> Result<ExportOutput> {
            todo!("Mock implementation")
        }

        fn level(&self) -> u8 {
            self.level
        }

        fn estimated_tokens(&self) -> usize {
            self.tokens
        }
    }

    #[test]
    fn test_mock_exporter_level() {
        let exporter = MockExporter { level: 0, tokens: 5000 };
        assert_eq!(exporter.level(), 0);
        assert_eq!(exporter.estimated_tokens(), 5000);
    }

    #[test]
    fn test_entity_debug() {
        let entity = Entity {
            isgl1_key: "rust:fn:test:src_lib_rs:10".to_string(),
            forward_deps: vec![],
            reverse_deps: vec![],
            current_ind: 1,
            future_ind: 0,
            future_action: None,
            future_code: None,
            current_code: None,
            entity_name: "test".to_string(),
            entity_type: "fn".to_string(),
            file_path: "src/lib.rs".to_string(),
            line_number: 10,
            interface_signature: "pub fn test()".to_string(),
            doc_comment: None,
            return_type: None,
            param_types: None,
            param_names: None,
            generic_constraints: None,
            trait_impls: None,
            is_public: Some(true),
            is_async: Some(false),
            is_unsafe: Some(false),
        };

        let debug_str = format!("{:?}", entity);
        assert!(debug_str.contains("rust:fn:test"));
    }
}

//! CozoDB adapter implementing CodeGraphRepository trait
//!
//! **Purpose**: Bridge between parseltongue-core's CozoDbStorage and PT02's export trait.
//!
//! # Architecture (S01 Principle #3: Dependency Injection)
//!
//! This adapter allows PT02 exporters to work with real CozoDB without tight coupling.
//!
//! ```text
//! PT02 Exporters → CodeGraphRepository trait → CozoDbAdapter → CozoDbStorage → CozoDB
//! ```

use crate::export_trait::{CodeGraphRepository, Edge, Entity};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use parseltongue_core::storage::CozoDbStorage;

/// CozoDB adapter for PT02 exports
///
/// Wraps `parseltongue_core::storage::CozoDbStorage` and implements
/// the `CodeGraphRepository` trait that PT02 exporters expect.
pub struct CozoDbAdapter {
    storage: CozoDbStorage,
}

impl CozoDbAdapter {
    /// Create new adapter from CozoDbStorage
    pub fn new(storage: CozoDbStorage) -> Self {
        Self { storage }
    }

    /// Create adapter by connecting to database
    pub async fn connect(db_path: &str) -> Result<Self> {
        let storage = CozoDbStorage::new(db_path)
            .await
            .map_err(|e| anyhow!("Failed to connect to CozoDB: {}", e))?;
        Ok(Self::new(storage))
    }
}

#[async_trait]
impl CodeGraphRepository for CozoDbAdapter {
    async fn get_all_entities(&self) -> Result<Vec<Entity>> {
        // Query all entities from CodeGraph
        let query = r#"
            ?[ISGL1_key, interface_signature, entity_type, file_path,
              Current_Code, Future_Code, current_ind, future_ind, Future_Action, entity_class] :=
            *CodeGraph{
                ISGL1_key,
                interface_signature,
                entity_type,
                file_path,
                Current_Code,
                Future_Code,
                current_ind,
                future_ind,
                Future_Action,
                entity_class
            }
        "#;

        let result = self.storage.raw_query(query).await
            .map_err(|e| anyhow!("Failed to query entities: {}", e))?;

        // Parse result into Entity structs
        let entities = parse_entities_from_query_result(&result)?;
        Ok(entities)
    }

    async fn query_entities(&self, where_clause: &str) -> Result<Vec<Entity>> {
        if where_clause == "ALL" {
            return self.get_all_entities().await;
        }

        // Build Datalog query with WHERE clause
        let query = format!(
            r#"
            ?[ISGL1_key, interface_signature, entity_type, file_path,
              Current_Code, Future_Code, current_ind, future_ind, Future_Action, entity_class] :=
            *CodeGraph{{
                ISGL1_key,
                interface_signature,
                entity_type,
                file_path,
                Current_Code,
                Future_Code,
                current_ind,
                future_ind,
                Future_Action,
                entity_class
            }},
            {}
            "#,
            where_clause
        );

        let result = self.storage.raw_query(&query).await
            .map_err(|e| anyhow!("Failed to query entities with WHERE clause: {}", e))?;

        let entities = parse_entities_from_query_result(&result)?;
        Ok(entities)
    }

    async fn get_all_edges(&self) -> Result<Vec<Edge>> {
        // Query all dependency edges
        let query = r#"
            ?[from_key, to_key, edge_type] :=
            *DependencyEdges{from_key, to_key, edge_type}
        "#;

        let result = self.storage.raw_query(query).await
            .map_err(|e| anyhow!("Failed to query edges: {}", e))?;

        let edges = parse_edges_from_query_result(&result)?;
        Ok(edges)
    }

    async fn query_edges(&self, where_clause: &str) -> Result<Vec<Edge>> {
        if where_clause == "ALL" {
            return self.get_all_edges().await;
        }

        // Build Datalog query with WHERE clause
        let query = format!(
            r#"
            ?[from_key, to_key, edge_type] :=
            *DependencyEdges{{from_key, to_key, edge_type}},
            {}
            "#,
            where_clause
        );

        let result = self.storage.raw_query(&query).await
            .map_err(|e| anyhow!("Failed to query edges with WHERE clause: {}", e))?;

        let edges = parse_edges_from_query_result(&result)?;
        Ok(edges)
    }
}

/// Parse entities from CozoDB query result
///
/// # CozoDB Result Format
/// Results come back as:
/// ```json
/// {
///   "headers": ["ISGL1_key", "interface_signature", ...],
///   "rows": [
///     ["rust:fn:main:src_main_rs:1", "pub fn main() {}", ...],
///     ...
///   ]
/// }
/// ```
fn parse_entities_from_query_result(result: &cozo::NamedRows) -> Result<Vec<Entity>> {
    let mut entities = Vec::new();

    for row in &result.rows {
        let entity = Entity {
            isgl1_key: extract_string(row, 0)?,
            interface_signature: extract_string(row, 1)?,
            entity_type: extract_string(row, 2)?,
            file_path: extract_string(row, 3)?,
            current_code: extract_optional_string(row, 4),
            future_code: extract_optional_string(row, 5),
            current_ind: extract_u8(row, 6)?,
            future_ind: extract_u8(row, 7)?,
            future_action: extract_optional_string(row, 8),

            // Parse entity name and line number from ISGL1 key
            entity_name: parse_entity_name_from_key(&extract_string(row, 0)?),
            line_number: parse_line_number_from_key(&extract_string(row, 0)?),

            // Default empty values for dependencies (will compute later if needed)
            forward_deps: Vec::new(),
            reverse_deps: Vec::new(),
            doc_comment: None,

            // v0.9.0: Extract entity_class from database (column 9)
            entity_class: extract_string(row, 9)?,

            // Level 2 fields (not in database yet)
            return_type: None,
            param_types: None,
            param_names: None,
            generic_constraints: None,
            trait_impls: None,
            is_public: None,
            is_async: None,
            is_unsafe: None,
        };

        entities.push(entity);
    }

    Ok(entities)
}

/// Parse edges from CozoDB query result
fn parse_edges_from_query_result(result: &cozo::NamedRows) -> Result<Vec<Edge>> {
    let mut edges = Vec::new();

    for row in &result.rows {
        let edge = Edge {
            from_key: extract_string(row, 0)?,
            to_key: extract_string(row, 1)?,
            edge_type: extract_string(row, 2)?,
        };

        edges.push(edge);
    }

    Ok(edges)
}

/// Extract string from DataValue at index
fn extract_string(row: &[cozo::DataValue], index: usize) -> Result<String> {
    match &row[index] {
        cozo::DataValue::Str(s) => Ok(s.to_string()),
        other => Err(anyhow!("Expected string at index {}, got {:?}", index, other)),
    }
}

/// Extract optional string from DataValue
fn extract_optional_string(row: &[cozo::DataValue], index: usize) -> Option<String> {
    match &row[index] {
        cozo::DataValue::Str(s) => Some(s.to_string()),
        cozo::DataValue::Null => None,
        _ => None,
    }
}

/// Extract u8 from DataValue (for current_ind/future_ind)
fn extract_u8(row: &[cozo::DataValue], index: usize) -> Result<u8> {
    match &row[index] {
        cozo::DataValue::Bool(b) => Ok(if *b { 1 } else { 0 }),
        other => Err(anyhow!("Expected bool at index {}, got {:?}", index, other)),
    }
}

/// Parse entity name from ISGL1 key
///
/// Format: `rust:fn:entity_name:file_path:line`
fn parse_entity_name_from_key(key: &str) -> String {
    let parts: Vec<&str> = key.split(':').collect();
    if parts.len() >= 3 {
        parts[2].to_string()
    } else {
        "unknown".to_string()
    }
}

/// Parse line number from ISGL1 key
///
/// Format: `rust:fn:entity_name:file_path:line`
fn parse_line_number_from_key(key: &str) -> u32 {
    let parts: Vec<&str> = key.split(':').collect();
    if parts.len() >= 5 {
        parts[4].parse().unwrap_or(0)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entity_name() {
        assert_eq!(
            parse_entity_name_from_key("rust:fn:main:src_main_rs:1"),
            "main"
        );
    }

    #[test]
    fn test_parse_line_number() {
        assert_eq!(
            parse_line_number_from_key("rust:fn:main:src_main_rs:42"),
            42
        );
    }
}

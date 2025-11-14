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
        let mut entities = parse_entities_from_query_result(&result)?;

        // v0.9.7: Populate forward_deps and reverse_deps from DependencyEdges
        populate_entity_dependencies(&mut entities, &self.storage).await?;

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

        let mut entities = parse_entities_from_query_result(&result)?;

        // v0.9.7: Populate forward_deps and reverse_deps from DependencyEdges
        populate_entity_dependencies(&mut entities, &self.storage).await?;

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

/// Normalize ISGL1 key to handle pt01 ingestion inconsistencies (v0.9.7)
///
/// # Normalization Rules
/// 1. Convert file paths: `./test.rs` → `__test_rs` (dots and slashes to underscores)
/// 2. Preserve format: `rust:fn:name:file_path:line-range`
///
/// # Example
/// ```
/// // Before: rust:fn:main:./test.rs:1-3
/// // After:  rust:fn:main:__test_rs:1-3
/// ```
fn normalize_isgl1_key(key: &str) -> String {
    let parts: Vec<&str> = key.split(':').collect();

    if parts.len() >= 4 {
        // Full ISGL1 format: lang:type:name:filepath:linerange
        let language = parts[0];
        let entity_type = parts[1];
        let name = parts[2];
        let file_path = parts[3];
        let line_range = parts.get(4).unwrap_or(&"");

        // Normalize file path: ./test.rs → __test_rs
        let normalized_path = if file_path.starts_with("./") {
            // ./test.rs → __test_rs (preserve leading __ to match entity keys)
            let without_dot_slash = &file_path[2..];  // Remove "./"
            let with_underscores = without_dot_slash.replace("/", "_").replace(".", "_");
            format!("__{}", with_underscores)  // Add __ prefix
        } else if file_path.starts_with("/") {
            // /path/test.rs → _path_test_rs
            file_path.replace("/", "_").replace(".", "_")
        } else {
            // test.rs → test_rs or already normalized
            file_path.replace("/", "_").replace(".", "_")
        };

        if line_range.is_empty() {
            format!("{}:{}:{}:{}", language, entity_type, name, normalized_path)
        } else {
            format!("{}:{}:{}:{}:{}", language, entity_type, name, normalized_path, line_range)
        }
    } else {
        // If format doesn't match, return as-is
        key.to_string()
    }
}

/// Populate forward_deps and reverse_deps for all entities (v0.9.7)
///
/// # Algorithm
/// 1. Query all edges from DependencyEdges relation
/// 2. Build forward_deps map (entity -> what it depends on)
/// 3. Build reverse_deps map (entity -> what depends on it)
/// 4. Update each entity with its dependency arrays
///
/// # Performance
/// - O(E) to query edges
/// - O(E) to build maps
/// - O(N) to populate entities
/// - Total: O(N + E) where N = entities, E = edges
async fn populate_entity_dependencies(
    entities: &mut [Entity],
    storage: &CozoDbStorage,
) -> Result<()> {
    use std::collections::HashMap;

    // Query all edges from DependencyEdges
    let edges_query = r#"
        ?[from_key, to_key, edge_type] :=
        *DependencyEdges{from_key, to_key, edge_type}
    "#;

    let edges_result = storage.raw_query(edges_query).await
        .map_err(|e| anyhow!("Failed to query edges for dependency population: {}", e))?;

    // Build dependency maps
    let mut forward_deps_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut reverse_deps_map: HashMap<String, Vec<String>> = HashMap::new();

    for row in &edges_result.rows {
        let from_key = extract_string(row, 0)?;
        let to_key = extract_string(row, 1)?;

        // v0.9.7: Normalize keys to handle pt01 inconsistencies
        // (./test.rs → __test_rs)
        let from_key_normalized = normalize_isgl1_key(&from_key);
        let to_key_normalized = normalize_isgl1_key(&to_key);

        // v0.9.7: WORKAROUND for pt01 bug - edges have unknown:0-0 as to_key
        // Resolve by matching function name against actual entities
        let final_to_key = if to_key_normalized.contains("unknown") {
            // Extract function name from to_key: rust:fn:NAME:unknown:0-0
            let to_parts: Vec<&str> = to_key_normalized.split(':').collect();
            if to_parts.len() >= 3 {
                let target_func_name = to_parts[2];

                // Find matching entity by name (heuristic match)
                let matched_entity = entities.iter().find(|e| {
                    let entity_parts: Vec<&str> = e.isgl1_key.split(':').collect();
                    entity_parts.len() >= 3 && entity_parts[2] == target_func_name
                });

                if let Some(entity) = matched_entity {
                    entity.isgl1_key.clone()
                } else {
                    // Skip edge if we can't resolve the target
                    continue;
                }
            } else {
                // Skip edge if format is invalid
                continue;
            }
        } else {
            to_key_normalized.clone()
        };

        // Forward deps: from_key depends on to_key
        forward_deps_map
            .entry(from_key_normalized.clone())
            .or_insert_with(Vec::new)
            .push(final_to_key.clone());

        // Reverse deps: to_key is depended upon by from_key
        reverse_deps_map
            .entry(final_to_key)
            .or_insert_with(Vec::new)
            .push(from_key_normalized);
    }

    // Populate each entity with its dependencies
    for entity in entities.iter_mut() {
        entity.forward_deps = forward_deps_map
            .get(&entity.isgl1_key)
            .cloned()
            .unwrap_or_default();

        entity.reverse_deps = reverse_deps_map
            .get(&entity.isgl1_key)
            .cloned()
            .unwrap_or_default();
    }

    Ok(())
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

            // v0.9.7: Dependencies populated by populate_entity_dependencies()
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

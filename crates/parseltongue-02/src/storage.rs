//! CozoDB storage functionality
//! Following TDD-first principle - tests first, implementation second

use crate::chunking::Chunk;
use crate::error::ToolResult;
use parseltongue_01::types::ISGL1Key;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;
use cozo::{DbInstance, NamedRows, DataValue, Json};
use miette::{IntoDiagnostic, Context, WrapErr};
use eyre::{Context as EyreContext, WrapErr as EyreWrapErr};

/// Result of ingesting data into CozoDB
#[derive(Debug, Clone)]
pub struct IngestionResult {
    pub chunks_ingested: usize,
    pub relationships_created: usize,
    pub ingestion_time_ms: u64,
    pub success: bool,
}

/// CozoDB connection and ingestion interface
#[derive(Clone)]
pub struct CozoDBConnection {
    db: Arc<DbInstance>,
    connection_id: Uuid,
}

impl std::fmt::Debug for CozoDBConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CozoDBConnection")
            .field("connection_id", &self.connection_id)
            .field("db", &"<Arc<DbInstance>>")
            .finish()
    }
}

impl CozoDBConnection {
    /// Create a new CozoDB connection
    pub fn new(db_path: Option<PathBuf>) -> ToolResult<Self> {
        let db_path = db_path.unwrap_or_else(|| PathBuf::from(":memory:"));
        let db = DbInstance::new("sqlite", &db_path, "{}")
            .map_err(|e| miette::miette!("Failed to create CozoDB instance: {}", e))?;

        Ok(Self {
            db: Arc::new(db),
            connection_id: Uuid::new_v4(),
        })
    }

    /// Initialize the database schema
    pub async fn initialize_schema(&self) -> ToolResult<()> {
        // Create chunks relation
        self.db.run_default(r#"
            :create chunks {
                id: String,
                content: String,
                start_line: Int,
                end_line: Int,
                chunk_type: String,
                metadata: Json,
                file_path: String,
                interface_name: String,
                parent_id: String?
            }
        "#).map_err(|e| miette::miette!("Failed to create chunks relation: {}", e))?;

        // Create relationships relation
        self.db.run_default(r#"
            :create relationships {
                source_id: String,
                target_id: String,
                relationship_type: String,
                metadata: Json
            }
        "#).map_err(|e| miette::miette!("Failed to create relationships relation: {}", e))?;

        // Create indexes for performance
        self.db.run_default(r#"
            :create index chunks_id_idx ON chunks(id);
            :create index chunks_interface_idx ON chunks(interface_name);
            :create index relationships_source_idx ON relationships(source_id);
            :create index relationships_target_idx ON relationships(target_id);
        "#).map_err(|e| miette::miette!("Failed to create indexes: {}", e))?;

        Ok(())
    }

    /// Ingest chunks into the database
    pub async fn ingest_chunks(
        &self,
        chunks: &[Chunk],
        keys: &[ISGL1Key],
    ) -> ToolResult<IngestionResult> {
        let start_time = std::time::Instant::now();

        if chunks.len() != keys.len() {
            return Err(miette::miette!("Number of chunks and keys must match").into_diagnostic());
        }

        // Import chunks using individual insert statements
        for (chunk, key) in chunks.iter().zip(keys.iter()) {
            let metadata_json = serde_json::to_value(&chunk.metadata)
                .map_err(|e| miette::miette!("Failed to serialize chunk metadata: {}", e))?;

            let insert_query = format!(r#"
                ?[id, content, start_line, end_line, chunk_type, metadata, file_path, interface_name, parent_id] :=
                    chunks["{}", "{}", {}, {}, "{}", "{}", "{}", "{}", "{}"]
            "#,
                chunk.id.to_string(),
                chunk.content.replace('"', "\\\""),
                chunk.start_line,
                chunk.end_line,
                format!("{:?}", chunk.chunk_type),
                serde_json::to_string(&metadata_json).unwrap_or_default(),
                key.filepath.to_string_lossy(),
                key.interface_name,
                chunk.metadata.parent_id.map(|id| id.to_string()).unwrap_or_default()
            );

            self.db.run_default(&insert_query)
                .map_err(|e| miette::miette!("Failed to insert chunk: {}", e))?;
        }

        let ingestion_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(IngestionResult {
            chunks_ingested: chunks.len(),
            relationships_created: 0,
            ingestion_time_ms,
            success: true,
        })
    }

    /// Query chunks by various criteria
    pub async fn query_chunks(&self, query: &str) -> ToolResult<Vec<Chunk>> {
        // Build a safe query that filters chunks
        let safe_query = format!(r#"
            ?[id, content, start_line, end_line, chunk_type, metadata, file_path, interface_name, parent_id] :=
                chunks[id, content, start_line, end_line, chunk_type, metadata, file_path, interface_name, parent_id],
                contains(content, "{}")
        "#, query.replace("\"", "\\\""));

        let result = self.db.run_default(&safe_query)
            .map_err(|e| miette::miette!("Failed to query chunks: {}", e))?;

        let mut chunks = Vec::new();
        for row in result.rows {
            if let (Some(id), Some(content), Some(start_line), Some(end_line),
                 Some(chunk_type), Some(metadata), Some(_file_path), Some(_interface_name), parent_id) =
                (row.get("id"), row.get("content"), row.get("start_line"), row.get("end_line"),
                 row.get("chunk_type"), row.get("metadata"), row.get("file_path"), row.get("interface_name"),
                 row.get("parent_id")) {

                let chunk_id = Uuid::parse_str(&id.to_string())
                    .map_err(|e| miette::miette!("Failed to parse chunk ID: {}", e))?;
                let start_line = start_line.get_int().map_err(|_| miette::miette!("Invalid start_line"))? as usize;
                let end_line = end_line.get_int().map_err(|_| miette::miette!("Invalid end_line"))? as usize;
                let content = content.to_string();
                let chunk_type_str = chunk_type.to_string();

                // Parse chunk type
                let chunk_type = if chunk_type_str.contains("Function") {
                    crate::chunking::ChunkType::Function
                } else if chunk_type_str.contains("Struct") {
                    crate::chunking::ChunkType::Struct
                } else if chunk_type_str.contains("Impl") {
                    crate::chunking::ChunkType::Impl
                } else if chunk_type_str.contains("Module") {
                    crate::chunking::ChunkType::Module
                } else {
                    crate::chunking::ChunkType::Other
                };

                let metadata_json = serde_json::to_value(metadata)
                    .map_err(|e| miette::miette!("Failed to serialize metadata: {}", e))?;
                let metadata: crate::chunking::ChunkMetadata = serde_json::from_value(metadata_json)
                    .map_err(|e| miette::miette!("Failed to deserialize metadata: {}", e))?;

                chunks.push(Chunk {
                    id: chunk_id,
                    content,
                    start_line,
                    end_line,
                    chunk_type,
                    metadata,
                });
            }
        }

        Ok(chunks)
    }

    /// Create relationships between chunks
    pub async fn create_relationships(
        &self,
        relationships: &[ChunkRelationship],
    ) -> ToolResult<()> {
        if relationships.is_empty() {
            return Ok(());
        }

        // Import relationships using individual insert statements
        for rel in relationships {
            let metadata_json = serde_json::to_value(&rel.metadata)
                .map_err(|e| miette::miette!("Failed to serialize relationship metadata: {}", e))?;

            let insert_query = format!(r#"
                ?[source_id, target_id, relationship_type, metadata] :=
                    relationships["{}", "{}", "{}", "{}"]
            "#,
                rel.source_id.to_string(),
                rel.target_id.to_string(),
                format!("{:?}", rel.relationship_type),
                serde_json::to_string(&metadata_json).unwrap_or_default()
            );

            self.db.run_default(&insert_query)
                .map_err(|e| miette::miette!("Failed to insert relationship: {}", e))?;
        }

        Ok(())
    }

    /// Get database statistics
    pub async fn get_stats(&self) -> ToolResult<DatabaseStats> {
        // Count chunks
        let chunk_count_result = self.db.run_default(r#"
            ?[count] := chunks[*]
        "#).map_err(|e| miette::miette!("Failed to count chunks: {}", e))?;

        let total_chunks = chunk_count_result.rows
            .first()
            .and_then(|row| row.get("count"))
            .and_then(|val| val.get_int().ok())
            .unwrap_or(0) as usize;

        // Count relationships
        let rel_count_result = self.db.run_default(r#"
            ?[count] := relationships[*]
        "#).map_err(|e| miette::miette!("Failed to count relationships: {}", e))?;

        let total_relationships = rel_count_result.rows
            .first()
            .and_then(|row| row.get("count"))
            .and_then(|val| val.get_int().ok())
            .unwrap_or(0) as usize;

        // Count unique files
        let file_count_result = self.db.run_default(r#"
            ?[count] := COUNT[DISTINCT file_path] FROM chunks[*]
        "#).map_err(|e| miette::miette!("Failed to count files: {}", e))?;

        let file_count = file_count_result.rows
            .first()
            .and_then(|row| row.get("count"))
            .and_then(|val| val.get_int().ok())
            .unwrap_or(0) as usize;

        Ok(DatabaseStats {
            total_chunks,
            total_relationships,
            file_count,
            last_updated: chrono::Utc::now(),
        })
    }

    /// Close the connection
    pub async fn close(&self) -> ToolResult<()> {
        // CozoDB handles cleanup automatically when DbInstance is dropped
        Ok(())
    }
}

/// Represents a relationship between chunks
#[derive(Debug, Clone)]
pub struct ChunkRelationship {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub relationship_type: RelationshipType,
    pub metadata: serde_json::Value,
}

/// Types of relationships between chunks
#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipType {
    Calls,
    Imports,
    Implements,
    Inherits,
    Contains,
    References,
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub total_chunks: usize,
    pub total_relationships: usize,
    pub file_count: usize,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for CozoDBConnection {
    fn default() -> Self {
        Self::new(None).expect("Failed to create default CozoDB connection")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_creation() {
        // RED: This test should fail because connection creation is not implemented
        let db_path = Some(PathBuf::from("/tmp/test.db"));
        let connection = CozoDBConnection::new(db_path);

        assert!(connection.db_path.is_some(), "Should store database path");
        assert_ne!(
            connection.connection_id,
            Uuid::default(),
            "Should have unique connection ID"
        );
    }

    #[tokio::test]
    async fn test_schema_initialization() {
        // RED: This test should fail because schema initialization is not implemented
        let connection = CozoDBConnection::default();
        let result = connection.initialize_schema().await;

        assert!(result.is_ok(), "Should initialize schema successfully");
    }

    #[tokio::test]
    async fn test_chunk_ingestion() {
        // RED: This test should fail because chunk ingestion is not implemented
        let connection = CozoDBConnection::default();

        let test_chunk = Chunk {
            id: Uuid::new_v4(),
            content: "fn test() {}".to_string(),
            start_line: 1,
            end_line: 1,
            chunk_type: crate::chunking::ChunkType::Function,
            metadata: crate::chunking::ChunkMetadata {
                parent_id: None,
                children_ids: vec![],
                dependencies: vec![],
                exports: vec![],
            },
        };

        let test_key = ISGL1Key::new(
            PathBuf::from("test.rs"),
            "test.rs".to_string(),
            "test_function".to_string(),
        );

        let result = connection.ingest_chunks(&[test_chunk], &[test_key]).await;
        assert!(result.is_ok(), "Should ingest chunks successfully");

        let ingestion_result = result.unwrap();
        assert_eq!(ingestion_result.chunks_ingested, 1, "Should ingest 1 chunk");
        assert!(ingestion_result.success, "Ingestion should be successful");
    }

    #[tokio::test]
    async fn test_chunk_querying() {
        // RED: This test should fail because querying is not implemented
        let connection = CozoDBConnection::default();

        // First ingest some test data
        let test_chunk = Chunk {
            id: Uuid::new_v4(),
            content: "fn query_test() {}".to_string(),
            start_line: 1,
            end_line: 1,
            chunk_type: crate::chunking::ChunkType::Function,
            metadata: crate::chunking::ChunkMetadata {
                parent_id: None,
                children_ids: vec![],
                dependencies: vec![],
                exports: vec![],
            },
        };

        let test_key = ISGL1Key::new(
            PathBuf::from("query_test.rs"),
            "query_test.rs".to_string(),
            "query_test_function".to_string(),
        );

        connection
            .ingest_chunks(&[test_chunk], &[test_key])
            .await
            .unwrap();

        // Now query for the chunk
        let query = "MATCH * WHERE content CONTAINS 'query_test'";
        let result = connection.query_chunks(query).await;
        assert!(result.is_ok(), "Should query chunks successfully");

        let chunks = result.unwrap();
        assert!(!chunks.is_empty(), "Should return matching chunks");
    }

    #[tokio::test]
    async fn test_relationship_creation() {
        // RED: This test should fail because relationship creation is not implemented
        let connection = CozoDBConnection::default();

        let relationship = ChunkRelationship {
            source_id: Uuid::new_v4(),
            target_id: Uuid::new_v4(),
            relationship_type: RelationshipType::Calls,
            metadata: serde_json::json!({"line": 10}),
        };

        let result = connection.create_relationships(&[relationship]).await;
        assert!(result.is_ok(), "Should create relationships successfully");
    }

    #[tokio::test]
    async fn test_database_stats() {
        // RED: This test should fail because stats retrieval is not implemented
        let connection = CozoDBConnection::default();
        let result = connection.get_stats().await;

        assert!(result.is_ok(), "Should retrieve database stats");

        let stats = result.unwrap();
        assert_eq!(stats.total_chunks, 0, "New database should have 0 chunks");
        assert_eq!(
            stats.total_relationships, 0,
            "New database should have 0 relationships"
        );
    }

    #[tokio::test]
    async fn test_connection_close() {
        // RED: This test should fail because connection close is not implemented
        let connection = CozoDBConnection::default();
        let result = connection.close().await;

        assert!(result.is_ok(), "Should close connection successfully");
    }

    #[test]
    fn test_relationship_type_equality() {
        assert_eq!(RelationshipType::Calls, RelationshipType::Calls);
        assert_ne!(RelationshipType::Calls, RelationshipType::Imports);
    }

    #[test]
    fn test_chunk_relationship_structure() {
        let relationship = ChunkRelationship {
            source_id: Uuid::new_v4(),
            target_id: Uuid::new_v4(),
            relationship_type: RelationshipType::Implements,
            metadata: serde_json::json!({"trait": "Debug"}),
        };

        assert_ne!(
            relationship.source_id, relationship.target_id,
            "Source and target should be different"
        );
        assert_eq!(
            relationship.relationship_type,
            RelationshipType::Implements,
            "Should store relationship type"
        );
        assert!(!relationship.metadata.is_null(), "Should store metadata");
    }
}

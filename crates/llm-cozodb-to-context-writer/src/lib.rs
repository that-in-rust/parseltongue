//! Parseltongue Tool 03: LLM-cozoDB-to-context-writer
//!
//! Ultra-minimalist context optimization tool that reads entity graphs from CozoDB,
//! generates optimized CodeGraphContext.json files using LLM reasoning, and writes
//! them for consumption by other tools. Following TDD-first principles with
//! executable specifications.

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(missing_docs)]

use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub mod cli;
pub mod context_optimizer;
pub mod errors;
pub mod llm_client;

// Re-export commonly used types
pub use errors::*;
pub use context_optimizer::{ContextOptimizerImpl, *};
pub use llm_client::{ContextLlmClientImpl, *};

/// Tool metadata and configuration
#[derive(Debug, Clone)]
pub struct ContextWriterConfig {
    /// Database connection string
    pub db_path: String,
    /// LLM API endpoint
    pub llm_endpoint: String,
    /// LLM API key
    pub llm_api_key: String,
    /// Model to use for LLM
    pub model: String,
    /// Maximum tokens per request
    pub max_tokens: usize,
    /// Temperature for LLM generation
    pub temperature: f32,
    /// Query to select entity graph for context generation
    pub entity_query: String,
    /// Maximum context size in tokens
    pub max_context_tokens: usize,
    /// Relevance threshold for entity inclusion
    pub relevance_threshold: f32,
    /// Output directory for context files
    pub output_dir: String,
}

impl Default for ContextWriterConfig {
    fn default() -> Self {
        Self {
            db_path: "parseltongue.db".to_string(),
            llm_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            llm_api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            model: "gpt-4".to_string(),
            max_tokens: 8192,
            temperature: 0.3,
            entity_query: "MATCH (e:Entity)-[r:RELATED_TO]->(n:Entity) RETURN e, r, n LIMIT 100".to_string(),
            max_context_tokens: 128000,
            relevance_threshold: 0.7,
            output_dir: "./contexts".to_string(),
        }
    }
}

/// Tool factory for dependency injection
pub struct ToolFactory;

impl ToolFactory {
    /// Create a new context optimizer instance (async factory method)
    pub async fn create_context_optimizer(config: ContextWriterConfig) -> Result<Arc<ContextOptimizerImpl>> {
        // Create storage instance
        let storage = parseltongue_core::storage::CozoDbStorage::new(&config.db_path)
            .await
            .map_err(|e| ContextWriterError::DatabaseError {
                reason: format!("Failed to create storage: {}", e)
            })?;

        let storage = Arc::new(storage);
        let llm_client = Arc::new(ContextLlmClientImpl::new(config.clone()));
        let optimizer = Arc::new(ContextOptimizerImpl::new(storage, config, llm_client));
        Ok(optimizer)
    }

    /// Create a new LLM client instance
    pub fn create_llm_client(config: ContextWriterConfig) -> Arc<ContextLlmClientImpl> {
        ContextLlmClientFactory::new(config)
    }
}

/// Minimal entity representation (PRD-compliant: excludes current_code/future_code)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MinimalEntity {
    /// ISGL1 primary key
    pub isgl1_key: String,
    /// Interface signature (formatted string)
    pub interface_signature: String,
    /// TDD classification (formatted string)
    pub tdd_classification: String,
    /// Optional LSP metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsp_metadata: Option<String>,
}

/// CodeGraphContext output format (PRD specification)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphContext {
    /// Minimal entities (ONLY: ISGL1, interface, TDD, LSP - NO code fields)
    pub entities: Vec<MinimalEntity>,
    /// Total number of entities
    pub entity_count: usize,
    /// Estimated token count
    pub token_count: usize,
    /// Generation timestamp
    pub generated_at: String,
}
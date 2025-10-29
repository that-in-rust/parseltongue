//! Parseltongue Tool 02: LLM-to-cozoDB-writer
//!
//! Ultra-minimalist LLM communication tool that reads ISGL1 keys from CozoDB,
//! generates code changes using LLM reasoning, and writes them back as temporal
//! changes. Following TDD-first principles with executable specifications.

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(missing_docs)]

use std::collections::HashMap;
use std::sync::Arc;

pub mod cli;
pub mod errors;
pub mod llm_client;
pub mod temporal_writer;

// Re-export commonly used types
pub use errors::*;
pub use llm_client::{LlmClientImpl, *};
pub use temporal_writer::{TemporalWriterImpl, *};

/// Tool metadata and configuration
#[derive(Debug, Clone)]
pub struct LlmWriterConfig {
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
    /// Query to select entities for processing
    pub query_filter: String,
    /// Batch size for processing
    pub batch_size: usize,
}

impl Default for LlmWriterConfig {
    fn default() -> Self {
        Self {
            db_path: "parseltongue.db".to_string(),
            llm_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            llm_api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            model: "gpt-4".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            query_filter: "SELECT * FROM CodeEntity WHERE temporal_state = 'current' LIMIT 10".to_string(),
            batch_size: 5,
        }
    }
}

/// Tool factory for dependency injection
pub struct ToolFactory;

impl ToolFactory {
    /// Create a new LLM writer instance
    pub fn create_llm_writer(config: LlmWriterConfig) -> Result<Arc<TemporalWriterImpl>> {
        let llm_client = Arc::new(LlmClientImpl::new(config.clone()));
        let writer = Arc::new(TemporalWriterImpl::new(config, llm_client));
        Ok(writer)
    }

    /// Create a new LLM client instance
    pub fn create_llm_client(config: LlmWriterConfig) -> Arc<LlmClientImpl> {
        LlmClientFactory::new(config)
    }
}
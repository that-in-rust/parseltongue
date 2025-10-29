//! LLM client for context generation and optimization.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::*;
use parseltongue_core::entities::{CodeEntity, EntityType, Language, EntityMetadata, TemporalAction, InterfaceSignature, Visibility, LineRange};

/// Context optimization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextOptimizationRequest {
    pub entities: Vec<CodeEntity>,
    pub relationships: Vec<EntityRelationship>,
    pub target_context_size: usize,
    pub focus_areas: Vec<String>,
    pub optimization_goals: Vec<OptimizationGoal>,
}

/// Entity relationship for context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRelationship {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
    pub strength: f32,
    pub context: Option<String>,
}

/// Relationship types between entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    DependsOn,
    Implements,
    Extends,
    Calls,
    Contains,
    References,
    SimilarTo,
    RelatedTo,
}

/// Context optimization goals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationGoal {
    MinimizeSize,
    MaximizeRelevance,
    PreserveConnectivity,
    FocusOnTypes,
    FocusOnFunctions,
    BalanceComplexity,
}

/// Generated context response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextOptimizationResponse {
    pub optimized_entities: Vec<OptimizedEntity>,
    pub pruning_summary: PruningSummary,
    pub context_metadata: ContextMetadata,
    pub confidence_score: f32,
}

/// Optimized entity with relevance scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedEntity {
    pub entity: CodeEntity,
    pub relevance_score: f32,
    pub inclusion_reason: InclusionReason,
    pub token_estimate: usize,
    pub dependencies: Vec<String>,
}

/// Reason for entity inclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InclusionReason {
    DirectReference,
    StructuralDependency,
    TypeInformation,
    ImplementationDetail,
    ContextBridge,
    HighRelevance,
}

/// Summary of pruning operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruningSummary {
    pub original_entities: usize,
    pub pruned_entities: usize,
    pub retained_entities: usize,
    pub tokens_saved: usize,
    pub pruning_ratio: f32,
}

/// Context metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMetadata {
    pub context_id: String,
    pub generated_at: String,
    pub model_used: String,
    pub optimization_applied: bool,
    pub token_count: usize,
    pub entity_count: usize,
    pub relationship_count: usize,
}

/// LLM client interface for context optimization
#[async_trait::async_trait]
pub trait ContextLlmClient: Send + Sync {
    /// Generate optimized context from entities and relationships
    async fn optimize_context(&self, request: ContextOptimizationRequest) -> Result<ContextOptimizationResponse>;

    /// Get client capabilities and limits
    fn capabilities(&self) -> ContextLlmCapabilities;

    /// Validate configuration
    fn validate_config(&self) -> Result<()>;
}

/// LLM capabilities for context optimization
#[derive(Debug, Clone)]
pub struct ContextLlmCapabilities {
    pub max_input_tokens: usize,
    pub max_output_tokens: usize,
    pub supported_models: Vec<String>,
    pub rate_limit_rpm: u32,
    pub supports_context_optimization: bool,
    pub supports_relationship_analysis: bool,
}

/// LLM request for context optimization
#[derive(Debug, Clone, Serialize)]
struct ContextLlmRequest {
    model: String,
    messages: Vec<ContextLlmMessage>,
    max_tokens: Option<usize>,
    temperature: f32,
    stream: bool,
    user: Option<String>,
}

/// LLM message for context optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContextLlmMessage {
    role: MessageRole,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageRole {
    System,
    User,
    Assistant,
}

/// LLM API response
#[derive(Debug, Clone, Deserialize)]
struct ContextLlmResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ContextChoice>,
    usage: ContextUsage,
}

#[derive(Debug, Clone, Deserialize)]
struct ContextChoice {
    index: usize,
    message: ContextLlmMessage,
    finish_reason: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ContextUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

/// Context LLM client implementation
pub struct ContextLlmClientImpl {
    config: crate::ContextWriterConfig,
    http_client: reqwest::Client,
    capabilities: ContextLlmCapabilities,
}

impl ContextLlmClientImpl {
    /// Create new context LLM client
    pub fn new(config: crate::ContextWriterConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        let capabilities = ContextLlmCapabilities {
            max_input_tokens: 128000,
            max_output_tokens: config.max_tokens,
            supported_models: vec!["gpt-4".to_string(), "gpt-3.5-turbo".to_string()],
            rate_limit_rpm: 60,
            supports_context_optimization: true,
            supports_relationship_analysis: true,
        };

        Self {
            config,
            http_client,
            capabilities,
        }
    }

    /// Create system prompt for context optimization
    fn create_system_prompt(&self) -> String {
        r#"You are an expert code context optimizer specializing in creating minimal, highly relevant CodeGraphContext.json files for LLM consumption.

Your task is to analyze the provided entity graph and optimize it for context usage following these principles:

1. **Token Efficiency**: Remove redundant entities while preserving connectivity
2. **Relevance Preservation**: Keep entities with high relevance scores (>0.7)
3. **Structural Integrity**: Maintain dependency relationships
4. **Type Safety**: Preserve type information and signatures
5. **Implementation Clarity**: Keep key implementation details

For each entity, provide:
- Relevance score (0.0-1.0)
- Inclusion reason (why this entity matters)
- Token estimate (approximate token cost)
- Dependencies (required related entities)

Output format: JSON with optimized_entities, pruning_summary, context_metadata, and confidence_score."#.to_string()
    }

    /// Create user prompt from context request
    fn create_user_prompt(&self, request: &ContextOptimizationRequest) -> String {
        let mut prompt = String::new();
        prompt.push_str("Optimize this code entity graph for LLM context consumption:\n\n");

        prompt.push_str(&format!("Target context size: {} tokens\n", request.target_context_size));
        prompt.push_str(&format!("Entities: {}\n", request.entities.len()));
        prompt.push_str(&format!("Relationships: {}\n", request.relationships.len()));
        prompt.push_str(&format!("Focus areas: {:?}\n", request.focus_areas));
        prompt.push_str(&format!("Optimization goals: {:?}\n\n", request.optimization_goals));

        prompt.push_str("Entities:\n");
        for (i, entity) in request.entities.iter().enumerate() {
            prompt.push_str(&format!(
                "{}. {} ({:?}) - {:?}\n   Code: {}\n\n",
                i + 1,
                entity.interface_signature.name,
                entity.interface_signature.entity_type,
                entity.interface_signature.language_specific,
                entity.current_code.as_deref().unwrap_or("No code")
            ));
        }

        prompt.push_str("Relationships:\n");
        for (i, rel) in request.relationships.iter().enumerate() {
            prompt.push_str(&format!(
                "{}. {} -> {} ({:?}) strength: {}\n",
                i + 1,
                rel.source_id,
                rel.target_id,
                rel.relationship_type,
                rel.strength
            ));
        }

        prompt.push_str("\nGenerate optimized JSON context with relevance scores and pruning summary.");
        prompt
    }

    /// Parse LLM response into context optimization response
    fn parse_response(&self, response: &str) -> Result<ContextOptimizationResponse> {
        // Try to extract JSON from response
        let json_start = response.find('{').ok_or_else(|| ContextWriterError::ContextGenerationError {
            entity: "response".to_string(),
            reason: "No JSON object found in response".to_string(),
        })?;

        let json_end = response.rfind('}').ok_or_else(|| ContextWriterError::ContextGenerationError {
            entity: "response".to_string(),
            reason: "Unclosed JSON object in response".to_string(),
        })?;

        let json_str = &response[json_start..=json_end];

        let optimized_response: ContextOptimizationResponse = serde_json::from_str(json_str)
            .map_err(|e| ContextWriterError::ContextGenerationError {
                entity: "response".to_string(),
                reason: format!("Invalid JSON: {}", e),
            })?;

        Ok(optimized_response)
    }
}

#[async_trait::async_trait]
impl ContextLlmClient for ContextLlmClientImpl {
    async fn optimize_context(&self, request: ContextOptimizationRequest) -> Result<ContextOptimizationResponse> {
        let llm_request = ContextLlmRequest {
            model: self.config.model.clone(),
            messages: vec![
                ContextLlmMessage {
                    role: MessageRole::System,
                    content: self.create_system_prompt(),
                },
                ContextLlmMessage {
                    role: MessageRole::User,
                    content: self.create_user_prompt(&request),
                },
            ],
            max_tokens: Some(self.config.max_tokens),
            temperature: self.config.temperature,
            stream: false,
            user: Some(Uuid::new_v4().to_string()),
        };

        let response = self.http_client
            .post(&self.config.llm_endpoint)
            .header("Authorization", format!("Bearer {}", self.config.llm_api_key))
            .header("Content-Type", "application/json")
            .json(&llm_request)
            .send()
            .await
            .map_err(|e| ContextWriterError::LlmApiError {
                status: 0,
                message: format!("Request failed: {}", e),
            })?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ContextWriterError::LlmApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let llm_response: ContextLlmResponse = response.json().await.map_err(|e| {
            ContextWriterError::ContextGenerationError {
                entity: "response".to_string(),
                reason: format!("Failed to parse JSON response: {}", e),
            }
        })?;

        if llm_response.choices.is_empty() {
            return Err(ContextWriterError::ContextGenerationError {
                entity: "response".to_string(),
                reason: "No choices in LLM response".to_string(),
            });
        }

        let content = &llm_response.choices[0].message.content;
        self.parse_response(content)
    }

    fn capabilities(&self) -> ContextLlmCapabilities {
        self.capabilities.clone()
    }

    fn validate_config(&self) -> Result<()> {
        if self.config.llm_api_key.is_empty() {
            return Err(ContextWriterError::ConfigurationError {
                field: "llm_api_key".to_string(),
                reason: "API key cannot be empty".to_string(),
            });
        }

        if self.config.llm_endpoint.is_empty() {
            return Err(ContextWriterError::ConfigurationError {
                field: "llm_endpoint".to_string(),
                reason: "Endpoint cannot be empty".to_string(),
            });
        }

        if self.config.model.is_empty() {
            return Err(ContextWriterError::ConfigurationError {
                field: "model".to_string(),
                reason: "Model cannot be empty".to_string(),
            });
        }

        Ok(())
    }
}

/// Factory for creating context LLM clients
pub struct ContextLlmClientFactory;

impl ContextLlmClientFactory {
    /// Create new context LLM client instance
    pub fn new(config: crate::ContextWriterConfig) -> Arc<ContextLlmClientImpl> {
        Arc::new(ContextLlmClientImpl::new(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompt_creation() {
        let config = crate::ContextWriterConfig::default();
        let client = ContextLlmClientImpl::new(config);
        let prompt = client.create_system_prompt();

        assert!(prompt.contains("code context optimizer"));
        assert!(prompt.contains("Token Efficiency"));
        assert!(prompt.contains("Relevance Preservation"));
    }

    #[test]
    fn test_config_validation() {
        let mut config = crate::ContextWriterConfig::default();
        config.llm_api_key = "test-key".to_string();

        let client = ContextLlmClientImpl::new(config);
        assert!(client.validate_config().is_ok());

        let invalid_config = crate::ContextWriterConfig::default();
        let invalid_client = ContextLlmClientImpl::new(invalid_config);
        assert!(invalid_client.validate_config().is_err());
    }

    #[test]
    fn test_context_request_creation() {
        let request = ContextOptimizationRequest {
            entities: vec![],
            relationships: vec![],
            target_context_size: 1000,
            focus_areas: vec!["types".to_string()],
            optimization_goals: vec![OptimizationGoal::MinimizeSize],
        };

        assert_eq!(request.target_context_size, 1000);
        assert_eq!(request.focus_areas.len(), 1);
    }
}
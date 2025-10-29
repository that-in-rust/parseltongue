//! LLM client implementation for communication with language models.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::*;
use parseltongue_core::entities::{CodeEntity, EntityType, Language, EntityMetadata, TemporalAction, InterfaceSignature, Visibility, LineRange};
use parseltongue_core::interfaces::TemporalChange;

/// LLM client interface
#[async_trait::async_trait]
pub trait LlmClient: Send + Sync {
    /// Generate code changes for given entities
    async fn generate_changes(&self, entities: &[CodeEntity]) -> Result<Vec<TemporalChange>>;

    /// Get client capabilities and limits
    fn capabilities(&self) -> LlmCapabilities;

    /// Validate configuration
    fn validate_config(&self) -> Result<()>;
}

/// LLM capabilities and limits
#[derive(Debug, Clone)]
pub struct LlmCapabilities {
    pub max_input_tokens: usize,
    pub max_output_tokens: usize,
    pub supported_models: Vec<String>,
    pub rate_limit_rpm: u32,
    pub supports_streaming: bool,
}

/// LLM request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

/// LLM API request
#[derive(Debug, Clone, Serialize)]
struct LlmRequest {
    model: String,
    messages: Vec<LlmMessage>,
    max_tokens: Option<usize>,
    temperature: f32,
    stream: bool,
    user: Option<String>,
}

/// LLM API response
#[derive(Debug, Clone, Deserialize)]
struct LlmResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Clone, Deserialize)]
struct Choice {
    index: usize,
    message: LlmMessage,
    finish_reason: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

/// Generated temporal change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedChange {
    pub isgl1_key: String,
    pub change_type: ChangeType,
    pub description: String,
    pub future_code: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    AddFunction,
    ModifyFunction,
    RemoveFunction,
    AddStruct,
    ModifyStruct,
    RemoveStruct,
    Refactor,
    Optimize,
}

/// LLM client implementation using OpenAI API
pub struct LlmClientImpl {
    config: crate::LlmWriterConfig,
    http_client: reqwest::Client,
    capabilities: LlmCapabilities,
}

impl LlmClientImpl {
    /// Create new LLM client
    pub fn new(config: crate::LlmWriterConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to create HTTP client");

        let capabilities = LlmCapabilities {
            max_input_tokens: 128000,
            max_output_tokens: config.max_tokens,
            supported_models: vec!["gpt-4".to_string(), "gpt-3.5-turbo".to_string()],
            rate_limit_rpm: 60,
            supports_streaming: false,
        };

        Self {
            config,
            http_client,
            capabilities,
        }
    }

    /// Create system prompt for code generation
    fn create_system_prompt(&self) -> String {
        r#"You are an expert Rust code optimizer and refactoring assistant. Your task is to analyze the given code entities and suggest specific improvements.

Follow these guidelines:
1. Only suggest changes that provide clear benefits (performance, readability, maintainability)
2. Generate concrete Rust code, not just descriptions
3. Maintain backward compatibility when possible
4. Focus on the ISGL1 key entities provided
5. Output changes in the specified JSON format

For each change, provide:
- The ISGL1 key being modified
- Type of change (add/modify/remove/optimize)
- Brief description of the improvement
- The complete new code implementation
- Confidence level (0.0-1.0)

Be conservative and prioritize correctness over aggressive optimization."#.to_string()
    }

    /// Create user prompt from entities
    fn create_user_prompt(&self, entities: &[CodeEntity]) -> String {
        let mut prompt = String::new();
        prompt.push_str("Analyze these Rust code entities and suggest improvements:\n\n");

        for (i, entity) in entities.iter().enumerate() {
            prompt.push_str(&format!(
                "{}. ISGL1: {}\n   Type: {:?}\n   Code: {}\n\n",
                i + 1,
                entity.isgl1_key,
                entity.interface_signature.entity_type,
                entity.current_code.as_deref().unwrap_or("No code available")
            ));
        }

        prompt.push_str("Generate specific code improvements in JSON format.");
        prompt
    }

    /// Parse LLM response into temporal changes
    fn parse_response(&self, response: &str) -> Result<Vec<TemporalChange>> {
        // Try to extract JSON from response
        let json_start = response.find('[').ok_or_else(|| LlmWriterError::ResponseParseError {
            reason: "No JSON array found in response".to_string(),
        })?;

        let json_end = response.rfind(']').ok_or_else(|| LlmWriterError::ResponseParseError {
            reason: "Unclosed JSON array in response".to_string(),
        })?;

        let json_str = &response[json_start..=json_end];

        let generated_changes: Vec<GeneratedChange> = serde_json::from_str(json_str)
            .map_err(|e| LlmWriterError::ResponseParseError {
                reason: format!("Invalid JSON: {}", e),
            })?;

        // Convert generated changes to temporal changes
        let mut temporal_changes = Vec::new();
        for change in generated_changes {
            let temporal_change = TemporalChange {
                isgl1_key: change.isgl1_key,
                action: match change.change_type {
                    ChangeType::ModifyFunction => TemporalAction::Edit,
                    ChangeType::AddFunction => TemporalAction::Create,
                    ChangeType::RemoveFunction => TemporalAction::Delete,
                    _ => TemporalAction::Edit,
                },
                future_code: Some(change.future_code),
                updated_signature: None,
            };

            temporal_changes.push(temporal_change);
        }

        Ok(temporal_changes)
    }
}

#[async_trait::async_trait]
impl LlmClient for LlmClientImpl {
    async fn generate_changes(&self, entities: &[CodeEntity]) -> Result<Vec<TemporalChange>> {
        if entities.is_empty() {
            return Ok(Vec::new());
        }

        let request = LlmRequest {
            model: self.config.model.clone(),
            messages: vec![
                LlmMessage {
                    role: MessageRole::System,
                    content: self.create_system_prompt(),
                },
                LlmMessage {
                    role: MessageRole::User,
                    content: self.create_user_prompt(entities),
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
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmWriterError::LlmApiError {
                status: 0,
                message: format!("Request failed: {}", e),
            })?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(LlmWriterError::LlmApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let llm_response: LlmResponse = response.json().await.map_err(|e| {
            LlmWriterError::ResponseParseError {
                reason: format!("Failed to parse JSON response: {}", e),
            }
        })?;

        if llm_response.choices.is_empty() {
            return Err(LlmWriterError::ResponseParseError {
                reason: "No choices in LLM response".to_string(),
            });
        }

        let content = &llm_response.choices[0].message.content;
        self.parse_response(content)
    }

    fn capabilities(&self) -> LlmCapabilities {
        self.capabilities.clone()
    }

    fn validate_config(&self) -> Result<()> {
        if self.config.llm_api_key.is_empty() {
            return Err(LlmWriterError::ConfigurationError {
                field: "llm_api_key".to_string(),
                reason: "API key cannot be empty".to_string(),
            });
        }

        if self.config.llm_endpoint.is_empty() {
            return Err(LlmWriterError::ConfigurationError {
                field: "llm_endpoint".to_string(),
                reason: "Endpoint cannot be empty".to_string(),
            });
        }

        if self.config.model.is_empty() {
            return Err(LlmWriterError::ConfigurationError {
                field: "model".to_string(),
                reason: "Model cannot be empty".to_string(),
            });
        }

        Ok(())
    }
}

/// Factory for creating LLM clients
pub struct LlmClientFactory;

impl LlmClientFactory {
    /// Create new LLM client instance
    pub fn new(config: crate::LlmWriterConfig) -> Arc<LlmClientImpl> {
        Arc::new(LlmClientImpl::new(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parseltongue_core::entities::{EntityMetadata, Language, EntityType};
    use crate::LlmWriterConfig;

    #[test]
    fn test_system_prompt_creation() {
        let config = LlmWriterConfig::default();
        let client = LlmClientImpl::new(config);
        let prompt = client.create_system_prompt();

        assert!(prompt.contains("Rust code optimizer"));
        assert!(prompt.contains("ISGL1 key"));
        assert!(prompt.contains("JSON format"));
    }

    #[test]
    fn test_user_prompt_creation() {
        let config = LlmWriterConfig::default();
        let client = LlmClientImpl::new(config);

        let interface_signature = InterfaceSignature {
            entity_type: EntityType::Function,
            name: "test_function".to_string(),
            visibility: Visibility::Public,
            file_path: "test.rs".into(),
            line_range: LineRange::new(1, 5).unwrap(),
            module_path: vec![],
            documentation: None,
            language_specific: parseltongue_core::entities::LanguageSpecificSignature::Rust(
                parseltongue_core::entities::RustSignature {
                    generics: vec![],
                    lifetimes: vec![],
                    where_clauses: vec![],
                    attributes: vec![],
                    trait_impl: None,
                }
            ),
        };

        let entity = CodeEntity::new(
            "rust:fn:test_function:test.rs:1-5".to_string(),
            interface_signature,
        ).unwrap();

        let prompt = client.create_user_prompt(&[entity]);
        assert!(prompt.contains("rust:fn:test_function:test.rs:1-5"));
    }

    #[test]
    fn test_config_validation() {
        let mut config = LlmWriterConfig::default();
        config.llm_api_key = "test-key".to_string();

        let client = LlmClientImpl::new(config);
        assert!(client.validate_config().is_ok());

        let invalid_config = LlmWriterConfig::default();
        let invalid_client = LlmClientImpl::new(invalid_config);
        assert!(invalid_client.validate_config().is_err());
    }

    #[test]
    fn test_response_parsing() {
        let config = LlmWriterConfig::default();
        let client = LlmClientImpl::new(config);

        let response = r#"
        Here are the suggested improvements:
        [
            {
                "isgl1_key": "rust:fn:old_function:src/lib.rs:10-15",
                "change_type": "ModifyFunction",
                "description": "Optimize function performance",
                "future_code": "fn optimized_function() { /* improved implementation */ }",
                "confidence": 0.85
            }
        ]
        "#;

        let changes = client.parse_response(response).unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].isgl1_key, "rust:fn:old_function:src/lib.rs:10-15");
    }
}
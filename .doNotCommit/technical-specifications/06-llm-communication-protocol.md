# LLM Communication Protocol Specification

## Overview

This specification defines the executable contract for LLM communication in Parseltongue, following TDD-first principles with measurable outcomes, structured error handling, and performance validation. The protocol enables reliable interaction between the reasoning orchestrator and external LLM APIs.

## Executable Specification Contract

### Performance Requirements (Test-Validated)

```rust
#[cfg(test)]
mod performance_contracts {
    use super::*;
    use std::time::Instant;

    /// **Executable Contract**: LLM requests must complete within 30 seconds
    #[test]
    fn llm_request_performance_contract() {
        let client = create_test_llm_client();
        let start = Instant::now();

        let result = client.send_request(test_context_request()).unwrap();

        let duration = start.elapsed();
        assert!(duration < Duration::from_secs(30),
            "LLM request took {:?}, expected < 30s", duration);
        assert!(result.confidence_score >= 0.8,
            "Confidence score {}, expected >= 0.8", result.confidence_score);
    }

    /// **Executable Contract**: Token counting must be accurate within ±5%
    #[test]
    fn token_counting_accuracy_contract() {
        let counter = TokenCounter::new();
        let test_text = include_str!("../test-data/sample_context.json");

        let estimated_tokens = counter.count_tokens(test_text);
        let actual_tokens = get_actual_token_count_from_api(test_text).unwrap();

        let accuracy_ratio = estimated_tokens as f64 / actual_tokens as f64;
        assert!((0.95..=1.05).contains(&accuracy_ratio),
            "Token accuracy {:.3}, expected 0.95-1.05", accuracy_ratio);
    }
}
```

## Core Data Structures

### LLM Request Types

```rust
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// **Executable Contract**: All LLM requests must be serializable and deserializable
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LLMRequest {
    /// Unique request identifier for tracking and debugging
    pub request_id: Uuid,

    /// Context data formatted according to CodeGraphContext schema
    pub context: CodeGraphContext,

    /// Task specification with clear success criteria
    pub task: TaskSpecification,

    /// Expected response format for validation
    pub expected_format: ResponseFormat,

    /// Performance constraints
    pub constraints: RequestConstraints,
}

/// **Executable Contract**: Task specification must have measurable success criteria
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskSpecification {
    /// Type of reasoning task
    pub task_type: TaskType,

    /// Natural language instruction for the LLM
    pub instruction: String,

    /// Expected outputs with validation rules
    pub expected_outputs: Vec<ExpectedOutput>,

    /// Success criteria for this task
    pub success_criteria: SuccessCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    /// Analyze code changes and propose modifications
    ChangeReasoning,

    /// Validate proposed changes for consistency
    ValidationCheck,

    /// Generate context for specific entities
    ContextGeneration,

    /// Analyze dependency impact
    DependencyAnalysis,
}

/// **Executable Contract**: Success criteria must be objectively measurable
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuccessCriteria {
    /// Minimum confidence threshold (0.0 to 1.0)
    pub min_confidence: f64,

    /// Maximum allowed reasoning time
    pub max_duration: Duration,

    /// Required output validation rules
    pub validation_rules: Vec<ValidationRule>,

    /// Performance benchmarks
    pub performance_targets: PerformanceTargets,
}
```

### LLM Response Types

```rust
/// **Executable Contract**: All LLM responses must validate against expected format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LLMResponse {
    /// Corresponding request identifier
    pub request_id: Uuid,

    /// LLM's reasoning and analysis
    pub reasoning: String,

    /// Structured changes proposed by the LLM
    pub proposed_changes: Vec<ProposedChange>,

    /// Confidence score for the proposed solution
    pub confidence_score: f64,

    /// Performance metrics
    pub metrics: ResponseMetrics,

    /// Validation status
    pub validation_status: ValidationStatus,
}

/// **Executable Contract**: Proposed changes must be verifiable
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProposedChange {
    /// Target entity ISGL1 key
    pub target_entity: String,

    /// Type of change to apply
    pub change_type: ChangeType,

    /// New content for the entity
    pub new_content: String,

    /// Justification for this change
    pub justification: String,

    /// Dependencies affected by this change
    pub affected_dependencies: Vec<String>,

    /// Validation checklist for this change
    pub validation_checklist: ValidationChecklist,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    Create,
    Edit,
    Delete,
}

/// **Executable Contract**: Validation must be comprehensive and automated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationChecklist {
    /// Syntax validation passed
    pub syntax_valid: bool,

    /// Dependency consistency check passed
    pub dependencies_consistent: bool,

    /// No circular dependencies introduced
    pub no_circular_deps: bool,

    /// All referenced entities exist
    pub references_valid: bool,

    /// Performance impact acceptable
    pub performance_acceptable: bool,
}
```

## Error Handling Contract

### Structured Error Types

```rust
/// **Executable Contract**: All errors must be structured and actionable
#[derive(Debug, Error)]
pub enum LLMCommunicationError {
    #[error("Request timeout after {duration:?}")]
    RequestTimeout { duration: Duration },

    #[error("API rate limit exceeded. Retry after {retry_after:?}")]
    RateLimitExceeded { retry_after: Duration },

    #[error("Invalid response format: {details}")]
    InvalidResponseFormat { details: String },

    #[error("Confidence score {score:.3} below threshold {threshold:.3}")]
    ConfidenceTooLow { score: f64, threshold: f64 },

    #[error("Token limit exceeded: {used}/{limit}")]
    TokenLimitExceeded { used: usize, limit: usize },

    #[error("Network error: {source}")]
    NetworkError { #[from] source: reqwest::Error },

    #[error("Authentication failed: {reason}")]
    AuthenticationFailed { reason: String },

    #[error("Response validation failed: {errors:?}")]
    ValidationFailed { errors: Vec<ValidationError> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub expected: String,
    pub actual: String,
    pub message: String,
}
```

### Recovery Strategies

```rust
/// **Executable Contract**: Recovery strategies must be testable and deterministic
pub trait RecoveryStrategy {
    /// Attempt recovery from the given error
    fn attempt_recovery(&self, error: &LLMCommunicationError) -> Result<RecoveryAction, RecoveryError>;

    /// Check if this strategy can handle the given error
    fn can_handle(&self, error: &LLMCommunicationError) -> bool;
}

#[derive(Debug, Clone)]
pub struct ExponentialBackoffRetry {
    pub max_retries: usize,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl RecoveryStrategy for ExponentialBackoffRetry {
    fn attempt_recovery(&self, error: &LLMCommunicationError) -> Result<RecoveryAction, RecoveryError> {
        match error {
            LLMCommunicationError::RequestTimeout { .. } |
            LLMCommunicationError::NetworkError { .. } => {
                Ok(RecoveryAction::RetryWithDelay(self.calculate_delay()))
            }
            LLMCommunicationError::RateLimitExceeded { retry_after } => {
                Ok(RecoveryAction::RetryWithDelay(*retry_after))
            }
            _ => Err(RecoveryError::CannotRecover(error.clone())),
        }
    }

    fn can_handle(&self, error: &LLMCommunicationError) -> bool {
        matches!(
            error,
            LLMCommunicationError::RequestTimeout { .. } |
            LLMCommunicationError::NetworkError { .. } |
            LLMCommunicationError::RateLimitExceeded { .. }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryAction {
    RetryWithDelay(Duration),
    RetryWithModifiedRequest(LLMRequest),
    FailWithError(LLMCommunicationError),
}
```

## LLM Client Implementation

### Primary Client Interface

```rust
/// **Executable Contract**: LLM client must implement all required behaviors
pub trait LLMClient: Send + Sync {
    /// Send request to LLM with timeout and retry logic
    async fn send_request(&self, request: LLMRequest) -> Result<LLMResponse, LLMCommunicationError>;

    /// Validate response format and content
    fn validate_response(&self, response: &LLMResponse, request: &LLMRequest) -> Result<(), ValidationError>;

    /// Get current rate limit status
    async fn get_rate_limit_status(&self) -> Result<RateLimitStatus, LLMCommunicationError>;

    /// Estimate token count for given content
    fn estimate_tokens(&self, content: &str) -> usize;
}

/// **Executable Contract**: Claude API implementation with full feature support
pub struct ClaudeAPIClient {
    client: reqwest::Client,
    api_key: String,
    model: ClaudeModel,
    rate_limiter: Arc<RwLock<RateLimiter>>,
    token_counter: TokenCounter,
    recovery_strategies: Vec<Box<dyn RecoveryStrategy>>,
}

impl ClaudeAPIClient {
    /// Create new client with authentication and configuration
    pub fn new(api_key: String, model: ClaudeModel) -> Result<Self, LLMCommunicationError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| LLMCommunicationError::NetworkError { source: e })?;

        Ok(Self {
            client,
            api_key,
            model,
            rate_limiter: Arc::new(RwLock::new(RateLimiter::new())),
            token_counter: TokenCounter::new(),
            recovery_strategies: vec![
                Box::new(ExponentialBackoffRetry {
                    max_retries: 3,
                    base_delay: Duration::from_millis(1000),
                    max_delay: Duration::from_secs(30),
                }),
            ],
        })
    }
}

impl LLMClient for ClaudeAPIClient {
    async fn send_request(&self, request: LLMRequest) -> Result<LLMResponse, LLMCommunicationError> {
        // Check rate limits first
        self.check_rate_limits().await?;

        // Validate request format
        self.validate_request(&request)?;

        // Send request with retry logic
        self.send_with_retry(request).await
    }

    fn validate_response(&self, response: &LLMResponse, request: &LLMRequest) -> Result<(), ValidationError> {
        // Check response ID matches request
        if response.request_id != request.request_id {
            return Err(ValidationError {
                field: "request_id".to_string(),
                expected: request.request_id.to_string(),
                actual: response.request_id.to_string(),
                message: "Response ID does not match request ID".to_string(),
            });
        }

        // Validate confidence score meets minimum threshold
        if response.confidence_score < request.constraints.min_confidence {
            return Err(ValidationError {
                field: "confidence_score".to_string(),
                expected: format!(">={}", request.constraints.min_confidence),
                actual: response.confidence_score.to_string(),
                message: "Confidence score below minimum threshold".to_string(),
            });
        }

        // Validate all proposed changes
        for change in &response.proposed_changes {
            self.validate_proposed_change(change)?;
        }

        Ok(())
    }

    async fn get_rate_limit_status(&self) -> Result<RateLimitStatus, LLMCommunicationError> {
        let rate_limiter = self.rate_limiter.read().await;
        Ok(rate_limiter.current_status())
    }

    fn estimate_tokens(&self, content: &str) -> usize {
        self.token_counter.count_tokens(content)
    }
}
```

### Request/Response Processing

```rust
impl ClaudeAPIClient {
    /// **Executable Contract**: Send request with comprehensive error handling
    async fn send_with_retry(&self, request: LLMRequest) -> Result<LLMResponse, LLMCommunicationError> {
        let mut attempt = 0;
        let mut current_request = request;

        loop {
            match self.attempt_send(&current_request).await {
                Ok(response) => {
                    // Validate response format
                    self.validate_response(&response, &current_request)
                        .map_err(|e| LLMCommunicationError::ValidationFailed {
                            errors: vec![e]
                        })?;
                    return Ok(response);
                }
                Err(error) => {
                    attempt += 1;

                    // Find applicable recovery strategy
                    let recovery_action = self.find_recovery_action(&error, attempt)?;

                    match recovery_action {
                        RecoveryAction::RetryWithDelay(delay) => {
                            tokio::time::sleep(delay).await;
                            continue;
                        }
                        RecoveryAction::RetryWithModifiedRequest(modified_request) => {
                            current_request = modified_request;
                            continue;
                        }
                        RecoveryAction::FailWithError(fatal_error) => {
                            return Err(fatal_error);
                        }
                    }
                }
            }
        }
    }

    /// **Executable Contract**: Single attempt with comprehensive error handling
    async fn attempt_send(&self, request: &LLMRequest) -> Result<LLMResponse, LLMCommunicationError> {
        // Update rate limiter
        {
            let mut rate_limiter = self.rate_limiter.write().await;
            rate_limiter.record_request()?;
        }

        // Build Claude API request
        let claude_request = self.build_claude_request(request)?;

        // Send HTTP request
        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&claude_request)
            .send()
            .await
            .map_err(|e| LLMCommunicationError::NetworkError { source: e })?;

        // Check response status
        if response.status() == 429 {
            let retry_after = response.headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .map(|secs| Duration::from_secs(secs))
                .unwrap_or(Duration::from_secs(60));

            return Err(LLMCommunicationError::RateLimitExceeded { retry_after });
        }

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(LLMCommunicationError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        // Parse response
        let claude_response: ClaudeApiResponse = response.json().await
            .map_err(|e| LLMCommunicationError::InvalidResponseFormat {
                details: format!("JSON parsing failed: {}", e)
            })?;

        // Convert to internal format
        self.convert_claude_response(claude_response, request)
    }
}
```

## Test Infrastructure

### Test Doubles and Mocks

```rust
/// **Executable Contract**: Test double implementation for unit testing
pub struct MockLLMClient {
    responses: Arc<RwLock<Vec<(Uuid, Result<LLMResponse, LLMCommunicationError>)>>>,
    request_log: Arc<RwLock<Vec<LLMRequest>>>,
}

impl MockLLMClient {
    pub fn new() -> Self {
        Self {
            responses: Arc::new(RwLock::new(Vec::new())),
            request_log: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Setup predefined response for specific request ID
    pub fn setup_response(&self, request_id: Uuid, response: Result<LLMResponse, LLMCommunicationError>) {
        self.responses.write().unwrap().push((request_id, response));
    }

    /// Get all requests made to this client
    pub fn get_requests(&self) -> Vec<LLMRequest> {
        self.request_log.read().unwrap().clone()
    }
}

impl LLMClient for MockLLMClient {
    async fn send_request(&self, request: LLMRequest) -> Result<LLMResponse, LLMCommunicationError> {
        // Log request for test verification
        self.request_log.write().unwrap().push(request.clone());

        // Find predefined response
        let responses = self.responses.read().unwrap();
        if let Some((_, response)) = responses.iter().find(|(id, _)| *id == request.request_id) {
            response.clone()
        } else {
            Err(LLMCommunicationError::InvalidResponseFormat {
                details: format!("No mock response configured for request ID: {}", request.request_id)
            })
        }
    }

    fn validate_response(&self, response: &LLMResponse, request: &LLMRequest) -> Result<(), ValidationError> {
        // Simple validation for tests
        if response.request_id != request.request_id {
            return Err(ValidationError {
                field: "request_id".to_string(),
                expected: request.request_id.to_string(),
                actual: response.request_id.to_string(),
                message: "Response ID mismatch".to_string(),
            });
        }
        Ok(())
    }

    async fn get_rate_limit_status(&self) -> Result<RateLimitStatus, LLMCommunicationError> {
        Ok(RateLimitStatus::default())
    }

    fn estimate_tokens(&self, content: &str) -> usize {
        // Simple estimation: 4 characters per token
        content.len() / 4
    }
}
```

### Integration Test Examples

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    /// **Executable Contract**: End-to-end LLM communication workflow
    #[tokio::test]
    async fn complete_llm_workflow_contract() {
        // Setup
        let client = create_test_claude_client().await;
        let context = create_test_codegraph_context();
        let request = LLMRequest {
            request_id: Uuid::new_v4(),
            context: context.clone(),
            task: TaskSpecification {
                task_type: TaskType::ChangeReasoning,
                instruction: "Analyze the proposed changes and suggest improvements".to_string(),
                expected_outputs: vec![],
                success_criteria: SuccessCriteria {
                    min_confidence: 0.8,
                    max_duration: Duration::from_secs(30),
                    validation_rules: vec![],
                    performance_targets: PerformanceTargets::default(),
                },
            },
            expected_format: ResponseFormat::StructuredChanges,
            constraints: RequestConstraints {
                max_tokens: 100000,
                temperature: 0.7,
                min_confidence: 0.8,
            },
        };

        // Execute
        let start = Instant::now();
        let response = client.send_request(request.clone()).await.unwrap();
        let duration = start.elapsed();

        // Validate contract compliance
        assert_eq!(response.request_id, request.request_id);
        assert!(response.confidence_score >= 0.8);
        assert!(duration < Duration::from_secs(30));
        assert!(!response.proposed_changes.is_empty());

        // Validate each proposed change
        for change in &response.proposed_changes {
            assert!(!change.target_entity.is_empty());
            assert!(!change.new_content.is_empty());
            assert!(!change.justification.is_empty());

            // Validate that the target entity exists in context
            assert!(context.entities.iter().any(|e| e.isgl1_key == change.target_entity));
        }
    }
}
```

## Performance Monitoring

### Metrics Collection

```rust
/// **Executable Contract**: Performance metrics must be automatically collected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMPerformanceMetrics {
    /// Request duration histogram
    pub request_durations: Vec<Duration>,

    /// Token usage statistics
    pub token_usage: TokenUsageStats,

    /// Error rates by type
    pub error_rates: HashMap<String, f64>,

    /// Rate limit statistics
    pub rate_limit_stats: RateLimitStats,

    /// Cache hit rates
    pub cache_hit_rates: CacheStats,
}

impl LLMPerformanceMetrics {
    /// **Executable Contract**: Performance targets must be continuously validated
    pub fn validate_performance_targets(&self) -> Result<(), PerformanceViolation> {
        // Check average request duration
        let avg_duration = self.request_durations.iter().sum::<Duration>() / self.request_durations.len() as u32;
        if avg_duration > Duration::from_secs(30) {
            return Err(PerformanceViolation::RequestDurationTooLong {
                actual: avg_duration,
                target: Duration::from_secs(30)
            });
        }

        // Check error rates
        for (error_type, rate) in &self.error_rates {
            if *rate > 0.05 { // 5% error rate threshold
                return Err(PerformanceViolation::ErrorRateTooHigh {
                    error_type: error_type.clone(),
                    actual_rate: *rate,
                    max_rate: 0.05,
                });
            }
        }

        Ok(())
    }
}
```

This LLM communication protocol specification provides a comprehensive, testable foundation for reliable LLM integration while following the steering docs principles of TDD-first development, executable specifications, and structured error handling.
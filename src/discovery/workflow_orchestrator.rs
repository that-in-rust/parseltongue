//! Workflow Orchestration Layer for Parseltongue v2
//! 
//! Combines discovery commands into complete user journeys following JTBD patterns.
//! Provides high-level workflow abstractions that orchestrate multiple discovery
//! operations to deliver complete solutions for common developer tasks.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use crate::discovery::{DiscoveryEngine, DiscoveryError, EntityInfo, FileLocation};

/// Core trait for workflow orchestration
/// 
/// # Contract
/// - All workflows must complete within specified time limits
/// - Results must be cacheable and serializable
/// - Workflows must be composable and testable
#[async_trait]
pub trait WorkflowOrchestrator {
    /// Execute onboarding workflow for new codebase
    /// 
    /// # Preconditions
    /// - Codebase has been ingested into discovery engine
    /// - Discovery indexes are available
    /// 
    /// # Postconditions
    /// - Returns comprehensive onboarding overview
    /// - Completes within 15 minutes for typical codebases
    /// - Provides actionable next steps
    async fn onboard(&self, target_dir: &str) -> Result<OnboardingResult, WorkflowError>;
    
    /// Execute feature planning workflow
    /// 
    /// # Preconditions
    /// - Entity exists in codebase
    /// - Discovery engine is initialized
    /// 
    /// # Postconditions
    /// - Returns impact analysis and scope guidance
    /// - Provides test recommendations
    /// - Completes within 5 minutes
    async fn feature_start(&self, entity_name: &str) -> Result<FeaturePlanResult, WorkflowError>;
    
    /// Execute debugging workflow
    /// 
    /// # Preconditions
    /// - Entity exists in codebase
    /// - Caller traces are available
    /// 
    /// # Postconditions
    /// - Returns caller traces and usage sites
    /// - Provides minimal change scope recommendations
    /// - Completes within 2 minutes
    async fn debug(&self, entity_name: &str) -> Result<DebugResult, WorkflowError>;
    
    /// Execute refactoring safety check workflow
    /// 
    /// # Preconditions
    /// - Entity exists in codebase
    /// - Dependency graph is available
    /// 
    /// # Postconditions
    /// - Returns risk assessment
    /// - Provides change checklist and reviewer guidance
    /// - Completes within 3 minutes
    async fn refactor_check(&self, entity_name: &str) -> Result<RefactorResult, WorkflowError>;
}

/// Result of onboarding workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingResult {
    /// Timestamp when workflow completed
    pub timestamp: DateTime<Utc>,
    /// Total execution time
    pub execution_time: Duration,
    /// Codebase overview statistics
    pub overview: CodebaseOverview,
    /// Key entry points and routes
    pub entry_points: Vec<EntryPoint>,
    /// Important contexts for understanding the codebase
    pub key_contexts: Vec<KeyContext>,
    /// Recommended next steps for onboarding
    pub next_steps: Vec<String>,
}

/// Result of feature planning workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturePlanResult {
    /// Timestamp when workflow completed
    pub timestamp: DateTime<Utc>,
    /// Total execution time
    pub execution_time: Duration,
    /// Target entity being modified
    pub target_entity: String,
    /// Impact analysis results
    pub impact_analysis: ImpactAnalysis,
    /// Scope guidance for the feature
    pub scope_guidance: ScopeGuidance,
    /// Test recommendations
    pub test_recommendations: Vec<TestRecommendation>,
}

/// Result of debugging workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugResult {
    /// Timestamp when workflow completed
    pub timestamp: DateTime<Utc>,
    /// Total execution time
    pub execution_time: Duration,
    /// Target entity being debugged
    pub target_entity: String,
    /// Caller trace information
    pub caller_traces: Vec<CallerTrace>,
    /// Usage sites for the entity
    pub usage_sites: Vec<UsageSite>,
    /// Minimal change scope recommendations
    pub minimal_scope: ChangeScope,
}

/// Result of refactoring safety check workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorResult {
    /// Timestamp when workflow completed
    pub timestamp: DateTime<Utc>,
    /// Total execution time
    pub execution_time: Duration,
    /// Target entity being refactored
    pub target_entity: String,
    /// Risk assessment
    pub risk_assessment: RiskAssessment,
    /// Change checklist
    pub change_checklist: Vec<ChecklistItem>,
    /// Reviewer guidance
    pub reviewer_guidance: ReviewerGuidance,
}

/// Codebase overview statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebaseOverview {
    /// Total number of files
    pub total_files: usize,
    /// Total number of entities
    pub total_entities: usize,
    /// Entities by type
    pub entities_by_type: std::collections::HashMap<String, usize>,
    /// Key modules and their purposes
    pub key_modules: Vec<ModuleInfo>,
    /// Architecture patterns detected
    pub architecture_patterns: Vec<String>,
}

/// Entry point information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    /// Name of the entry point
    pub name: String,
    /// Type of entry point (main, lib, test, etc.)
    pub entry_type: String,
    /// File location
    pub location: FileLocation,
    /// Description of what this entry point does
    pub description: String,
}

/// Key context for understanding codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyContext {
    /// Context name
    pub name: String,
    /// Context type (trait, struct, module, etc.)
    pub context_type: String,
    /// Why this context is important
    pub importance: String,
    /// Related entities
    pub related_entities: Vec<String>,
    /// File location
    pub location: FileLocation,
}

/// Impact analysis for feature planning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysis {
    /// Entities that will be directly affected
    pub direct_impact: Vec<EntityInfo>,
    /// Entities that will be indirectly affected
    pub indirect_impact: Vec<EntityInfo>,
    /// Risk level of the change
    pub risk_level: RiskLevel,
    /// Estimated complexity
    pub complexity_estimate: ComplexityLevel,
}

/// Scope guidance for feature development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeGuidance {
    /// Recommended scope boundaries
    pub boundaries: Vec<String>,
    /// Files that should be modified
    pub files_to_modify: Vec<String>,
    /// Files that should NOT be modified
    pub files_to_avoid: Vec<String>,
    /// Integration points to consider
    pub integration_points: Vec<String>,
}

/// Test recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRecommendation {
    /// Type of test (unit, integration, etc.)
    pub test_type: String,
    /// What should be tested
    pub test_target: String,
    /// Why this test is important
    pub rationale: String,
    /// Suggested test location
    pub suggested_location: String,
}

/// Caller trace information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallerTrace {
    /// Calling entity
    pub caller: EntityInfo,
    /// Call chain depth
    pub depth: usize,
    /// Call context (direct, indirect, conditional)
    pub call_context: String,
    /// Frequency estimate (if available)
    pub frequency: Option<String>,
}

/// Usage site information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSite {
    /// Entity using the target
    pub user: EntityInfo,
    /// Type of usage (call, import, inherit, etc.)
    pub usage_type: String,
    /// File location of usage
    pub location: FileLocation,
    /// Context around the usage
    pub context: String,
}

/// Change scope recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeScope {
    /// Minimal set of files to change
    pub minimal_files: Vec<String>,
    /// Safe change boundaries
    pub safe_boundaries: Vec<String>,
    /// Potential side effects to watch for
    pub side_effects: Vec<String>,
    /// Rollback strategy
    pub rollback_strategy: String,
}

/// Risk assessment for refactoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk level
    pub overall_risk: RiskLevel,
    /// Specific risk factors
    pub risk_factors: Vec<RiskFactor>,
    /// Mitigation strategies
    pub mitigations: Vec<String>,
    /// Confidence level in the assessment
    pub confidence: ConfidenceLevel,
}

/// Individual checklist item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    /// Item description
    pub description: String,
    /// Priority level
    pub priority: Priority,
    /// Whether this item is completed
    pub completed: bool,
    /// Additional notes
    pub notes: Option<String>,
}

/// Reviewer guidance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewerGuidance {
    /// Key areas to focus review on
    pub focus_areas: Vec<String>,
    /// Potential issues to look for
    pub potential_issues: Vec<String>,
    /// Testing recommendations for reviewers
    pub testing_recommendations: Vec<String>,
    /// Approval criteria
    pub approval_criteria: Vec<String>,
}

/// Module information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Module name
    pub name: String,
    /// Module purpose/description
    pub purpose: String,
    /// Key entities in the module
    pub key_entities: Vec<String>,
    /// Dependencies
    pub dependencies: Vec<String>,
}

/// Risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Factor description
    pub description: String,
    /// Risk level for this factor
    pub level: RiskLevel,
    /// Impact if this risk materializes
    pub impact: String,
}

/// Risk level enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Complexity level enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

/// Confidence level enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Priority level enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Workflow execution errors
#[derive(Debug, thiserror::Error)]
pub enum WorkflowError {
    #[error("Discovery error: {0}")]
    Discovery(#[from] DiscoveryError),
    
    #[error("Workflow timeout: {workflow} took {elapsed:?} (limit: {limit:?})")]
    Timeout {
        workflow: String,
        elapsed: Duration,
        limit: Duration,
    },
    
    #[error("Entity not found: {entity}")]
    EntityNotFound { entity: String },
    
    #[error("Invalid workflow state: {message}")]
    InvalidState { message: String },
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // TDD: RED phase - Test workflow result structures
    #[test]
    fn test_onboarding_result_serialization() {
        let result = OnboardingResult {
            timestamp: Utc::now(),
            execution_time: Duration::from_secs(30),
            overview: CodebaseOverview {
                total_files: 100,
                total_entities: 500,
                entities_by_type: HashMap::new(),
                key_modules: vec![],
                architecture_patterns: vec!["MVC".to_string()],
            },
            entry_points: vec![],
            key_contexts: vec![],
            next_steps: vec!["Read main.rs".to_string()],
        };
        
        // Should serialize and deserialize without errors
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: OnboardingResult = serde_json::from_str(&json).unwrap();
        
        assert_eq!(result.overview.total_files, deserialized.overview.total_files);
        assert_eq!(result.next_steps, deserialized.next_steps);
    }

    #[test]
    fn test_feature_plan_result_serialization() {
        let result = FeaturePlanResult {
            timestamp: Utc::now(),
            execution_time: Duration::from_secs(60),
            target_entity: "test_function".to_string(),
            impact_analysis: ImpactAnalysis {
                direct_impact: vec![],
                indirect_impact: vec![],
                risk_level: RiskLevel::Medium,
                complexity_estimate: ComplexityLevel::Moderate,
            },
            scope_guidance: ScopeGuidance {
                boundaries: vec!["module_boundary".to_string()],
                files_to_modify: vec!["src/lib.rs".to_string()],
                files_to_avoid: vec!["src/main.rs".to_string()],
                integration_points: vec!["API endpoint".to_string()],
            },
            test_recommendations: vec![],
        };
        
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: FeaturePlanResult = serde_json::from_str(&json).unwrap();
        
        assert_eq!(result.target_entity, deserialized.target_entity);
        assert_eq!(result.impact_analysis.risk_level, deserialized.impact_analysis.risk_level);
    }

    #[test]
    fn test_debug_result_serialization() {
        let result = DebugResult {
            timestamp: Utc::now(),
            execution_time: Duration::from_secs(45),
            target_entity: "debug_target".to_string(),
            caller_traces: vec![],
            usage_sites: vec![],
            minimal_scope: ChangeScope {
                minimal_files: vec!["src/target.rs".to_string()],
                safe_boundaries: vec!["module boundary".to_string()],
                side_effects: vec!["cache invalidation".to_string()],
                rollback_strategy: "revert commit".to_string(),
            },
        };
        
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: DebugResult = serde_json::from_str(&json).unwrap();
        
        assert_eq!(result.target_entity, deserialized.target_entity);
        assert_eq!(result.minimal_scope.rollback_strategy, deserialized.minimal_scope.rollback_strategy);
    }

    #[test]
    fn test_refactor_result_serialization() {
        let result = RefactorResult {
            timestamp: Utc::now(),
            execution_time: Duration::from_secs(90),
            target_entity: "refactor_target".to_string(),
            risk_assessment: RiskAssessment {
                overall_risk: RiskLevel::High,
                risk_factors: vec![],
                mitigations: vec!["Add tests".to_string()],
                confidence: ConfidenceLevel::High,
            },
            change_checklist: vec![
                ChecklistItem {
                    description: "Update tests".to_string(),
                    priority: Priority::High,
                    completed: false,
                    notes: Some("Focus on integration tests".to_string()),
                }
            ],
            reviewer_guidance: ReviewerGuidance {
                focus_areas: vec!["Error handling".to_string()],
                potential_issues: vec!["Race conditions".to_string()],
                testing_recommendations: vec!["Load testing".to_string()],
                approval_criteria: vec!["All tests pass".to_string()],
            },
        };
        
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: RefactorResult = serde_json::from_str(&json).unwrap();
        
        assert_eq!(result.target_entity, deserialized.target_entity);
        assert_eq!(result.risk_assessment.overall_risk, deserialized.risk_assessment.overall_risk);
        assert_eq!(result.change_checklist.len(), deserialized.change_checklist.len());
    }

    #[test]
    fn test_workflow_error_types() {
        // Test different error types
        let discovery_error = WorkflowError::Discovery(DiscoveryError::EntityNotFound("test".to_string()));
        assert!(matches!(discovery_error, WorkflowError::Discovery(_)));
        
        let timeout_error = WorkflowError::Timeout {
            workflow: "onboard".to_string(),
            elapsed: Duration::from_secs(20),
            limit: Duration::from_secs(15),
        };
        assert!(matches!(timeout_error, WorkflowError::Timeout { .. }));
        
        let entity_error = WorkflowError::EntityNotFound {
            entity: "missing_entity".to_string(),
        };
        assert!(matches!(entity_error, WorkflowError::EntityNotFound { .. }));
    }

    #[test]
    fn test_risk_level_ordering() {
        // Risk levels should be orderable
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);
    }

    #[test]
    fn test_complexity_level_ordering() {
        // Complexity levels should be orderable
        assert!(ComplexityLevel::Simple < ComplexityLevel::Moderate);
        assert!(ComplexityLevel::Moderate < ComplexityLevel::Complex);
        assert!(ComplexityLevel::Complex < ComplexityLevel::VeryComplex);
    }
}
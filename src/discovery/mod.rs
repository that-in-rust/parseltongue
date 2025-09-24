//! Discovery Infrastructure for Parseltongue v2
//! 
//! Discovery-first architectural intelligence tool that transforms entity discovery
//! from a 5+ minute bottleneck to a <30 second interactive experience.
//! 
//! Core components:
//! - String interning system for memory-efficient file path storage
//! - DiscoveryEngine trait for entity exploration
//! - Discovery indexes for fast entity listing and filtering

pub mod string_interning;
pub mod engine;
pub mod types;
pub mod error;
pub mod enhanced_isg_node;
pub mod simple_discovery_engine;
pub mod file_navigation_provider;

#[cfg(test)]
mod integration_test;

pub mod file_navigation_tests;
pub mod blast_radius_analyzer;
pub mod indexes;
pub mod concurrent_discovery_engine;
pub mod performance_metrics;
pub mod performance_regression_tests;
pub mod workspace_manager;
pub mod workflow_orchestrator;
pub mod concrete_workflow_orchestrator;
pub mod output_formatter;

#[cfg(test)]
pub mod workflow_integration_tests;

// Re-export core types for convenience
pub use string_interning::{FileId, FileInterner};
pub use engine::DiscoveryEngine;
pub use types::{EntityInfo, FileLocation, DiscoveryQuery, DiscoveryResult};
pub use error::{DiscoveryError, DiscoveryResult as Result};
pub use enhanced_isg_node::{EnhancedIsgNode, NodeConverter};
pub use simple_discovery_engine::SimpleDiscoveryEngine;
pub use file_navigation_provider::ISGFileNavigationProvider;
pub use file_navigation_tests::{FileNavigationProvider, FileStats, MockFileNavigationProvider};
pub use blast_radius_analyzer::{BlastRadiusAnalyzer, BlastRadiusAnalysis, ImpactGroup, ImpactedEntity, RiskLevel};
pub use indexes::{DiscoveryIndexes, CompactEntityInfo, IndexError};
pub use concurrent_discovery_engine::ConcurrentDiscoveryEngine;
pub use performance_metrics::{DiscoveryMetrics, Counter, Histogram, MetricsError, ContractValidation, MemoryStats as MetricsMemoryStats};
pub use performance_regression_tests::{PerformanceRegressionTester, PerformanceTestResults};
pub use workspace_manager::{WorkspaceManager, AnalysisSession, WorkspaceError};
pub use workflow_orchestrator::{
    WorkflowOrchestrator, OnboardingResult, FeaturePlanResult, DebugResult, RefactorResult,
    WorkflowError, CodebaseOverview, EntryPoint, KeyContext, ImpactAnalysis, ScopeGuidance,
    TestRecommendation, CallerTrace, UsageSite, ChangeScope, RiskAssessment, ChecklistItem,
    ReviewerGuidance, RiskLevel as WorkflowRiskLevel, ComplexityLevel, ConfidenceLevel, Priority, ModuleInfo,
    RiskFactor
};
pub use concrete_workflow_orchestrator::ConcreteWorkflowOrchestrator;
pub use output_formatter::{
    OutputFormatter, FormattingError, HumanFormatter, JsonFormatter, 
    PrSummaryFormatter, CiFormatter, CiPlatform, FormatterFactory
};
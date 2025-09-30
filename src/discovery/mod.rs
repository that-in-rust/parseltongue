//! Discovery Infrastructure for Parseltongue v2
//!
//! Discovery-first architectural intelligence tool that transforms entity discovery
//! from a 5+ minute bottleneck to a <30 second interactive experience.
//!
//! Core components:
//! - String interning system for memory-efficient file path storage
//! - DiscoveryEngine trait for entity exploration
//! - Discovery indexes for fast entity listing and filtering

pub mod engine;
pub mod enhanced_isg_node;
pub mod error;
pub mod file_navigation_provider;
pub mod simple_discovery_engine;
pub mod string_interning;
pub mod types;

#[cfg(test)]
mod integration_test;

pub mod blast_radius_analyzer;
pub mod concrete_workflow_orchestrator;
pub mod concurrent_discovery_engine;
pub mod file_navigation_tests;
pub mod indexes;
pub mod output_formatter;
pub mod workflow_orchestrator;
pub mod workspace_manager;

// Experimental performance and testing modules
#[cfg(feature = "experimental")]
pub mod performance_metrics;
#[cfg(feature = "experimental")]
pub mod performance_regression_tests;

#[cfg(test)]
pub mod workflow_integration_tests;

// #[cfg(test)]
// pub mod output_formatter_integration_test;

// Re-export core types for convenience
pub use blast_radius_analyzer::{
    BlastRadiusAnalysis, BlastRadiusAnalyzer, ImpactGroup, ImpactedEntity, RiskLevel,
};
pub use concrete_workflow_orchestrator::ConcreteWorkflowOrchestrator;
pub use concurrent_discovery_engine::ConcurrentDiscoveryEngine;
pub use engine::DiscoveryEngine;
pub use enhanced_isg_node::{EnhancedIsgNode, NodeConverter};
pub use error::{DiscoveryError, DiscoveryResult as Result};
pub use file_navigation_provider::ISGFileNavigationProvider;
pub use file_navigation_tests::{FileNavigationProvider, FileStats, MockFileNavigationProvider};
pub use indexes::{CompactEntityInfo, DiscoveryIndexes, IndexError};
pub use output_formatter::{
    CiFormatter, CiPlatform, FormatterFactory, FormattingError, HumanFormatter, JsonFormatter,
    OutputFormatter, PrSummaryFormatter,
};
#[cfg(feature = "experimental")]
pub use performance_metrics::{
    ContractValidation, Counter, DiscoveryMetrics, Histogram, MemoryStats as MetricsMemoryStats,
    MetricsError,
};
#[cfg(feature = "experimental")]
pub use performance_regression_tests::{PerformanceRegressionTester, PerformanceTestResults};
pub use simple_discovery_engine::SimpleDiscoveryEngine;
pub use string_interning::{FileId, FileInterner};
pub use types::{DiscoveryQuery, DiscoveryResult, EntityInfo, FileLocation};
pub use workflow_orchestrator::{
    CallerTrace, ChangeScope, ChecklistItem, CodebaseOverview, ComplexityLevel, ConfidenceLevel,
    DebugResult, EntryPoint, FeaturePlanResult, ImpactAnalysis, KeyContext, ModuleInfo,
    OnboardingResult, Priority, RefactorResult, ReviewerGuidance, RiskAssessment, RiskFactor,
    RiskLevel as WorkflowRiskLevel, ScopeGuidance, TestRecommendation, UsageSite, WorkflowError,
    WorkflowOrchestrator,
};
pub use workspace_manager::{AnalysisSession, WorkspaceError, WorkspaceManager};

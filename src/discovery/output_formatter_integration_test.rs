//! Integration tests for output formatting system
//! 
//! Tests the complete output formatting workflow with realistic data,
//! validates cross-format consistency, and ensures copy-pastable outputs.

use std::time::{Duration, Instant};
use std::collections::HashMap;
use chrono::Utc;

use crate::discovery::{
    OutputFormatter, HumanFormatter, JsonFormatter, PrSummaryFormatter, CiFormatter,
    CiPlatform, FormatterFactory, FormattingError,
    OnboardingResult, FeaturePlanResult, DebugResult, RefactorResult,
    CodebaseOverview, EntryPoint, KeyContext, ImpactAnalysis, ScopeGuidance,
    TestRecommendation, CallerTrace, UsageSite, ChangeScope, RiskAssessment,
    ChecklistItem, ReviewerGuidance, ModuleInfo, RiskFactor, ComplexityLevel,
    ConfidenceLevel, Priority, FileLocation, WorkflowRiskLevel
};

/// Integration test fixture with realistic workflow results
struct IntegrationTestFixture {
    onboarding_result: OnboardingResult,
    feature_plan_result: FeaturePlanResult,
    debug_result: DebugResult,
    refactor_result: RefactorResult,
}

impl IntegrationTestFixture {
    /// Create fixture with realistic test data based on actual Parseltongue codebase
    fn new() -> Self {
        Self {
            onboarding_result: Self::create_realistic_onboarding_result(),
            feature_plan_result: Self::create_realistic_feature_plan_result(),
            debug_result: Self::create_realistic_debug_result(),
            refactor_result: Self::create_realistic_refactor_result(),
        }
    }
    
    // Create realistic onboarding result based on Parseltongue's actual structure
    fn create_realistic_onboarding_result() -> OnboardingResult {
        let mut entities_by_type = HashMap::new();
        entities_by_type.insert("Function".to_string(), 245);
        entities_by_type.insert("Struct".to_string(), 89);
        entities_by_type.insert("Trait".to_string(), 23);
        entities_by_type.insert("Enum".to_string(), 15);
        entities_by_type.insert("Impl".to_string(), 156);
        entities_by_type.insert("Module".to_string(), 12);

        OnboardingResult {
            timestamp: Utc::now(),
            execution_time: Duration::from_secs(8), // Under 15 minute target
            overview: CodebaseOverview {
                total_files: 42,
                total_entities: 540,
                entities_by_type,
                key_modules: vec![
                    ModuleInfo {
                        name: "discovery".to_string(),
                        purpose: "Core discovery engine and entity analysis".to_string(),
                        key_entities: vec![
                            "DiscoveryEngine".to_string(),
                            "ConcurrentDiscoveryEngine".to_string(),
                            "BlastRadiusAnalyzer".to_string(),
                        ],
                        dependencies: vec!["isg".to_string(), "std".to_string()],
                    },
                    ModuleInfo {
                        name: "isg".to_string(),
                        purpose: "In-memory semantic graph for Rust code analysis".to_string(),
                        key_entities: vec![
                            "InMemoryIsg".to_string(),
                            "OptimizedISG".to_string(),
                            "SigHash".to_string(),
                        ],
                        dependencies: vec!["std".to_string(), "petgraph".to_string()],
                    },
                ],
                architecture_patterns: vec![
                    "Layered Architecture".to_string(),
                    "Repository Pattern".to_string(),
                ],
            },
            entry_points: vec![
                EntryPoint {
                    name: "main".to_string(),
                    entry_type: "binary".to_string(),
                    location: FileLocation::with_line("src/main.rs".to_string(), 1),
                    description: "Primary CLI entry point".to_string(),
                },
            ],
            key_contexts: vec![
                KeyContext {
                    name: "InMemoryIsg".to_string(),
                    context_type: "struct".to_string(),
                    importance: "Core data structure".to_string(),
                    related_entities: vec!["SigHash".to_string()],
                    location: FileLocation::with_line("src/isg.rs".to_string(), 45),
                },
            ],
            next_steps: vec![
                "Start with `src/main.rs` to understand the CLI interface".to_string(),
                "Explore `src/isg.rs` to understand the core semantic graph".to_string(),
            ],
        }
    }    //
/ Create realistic feature planning result for a complex entity
    fn create_realistic_feature_plan_result() -> FeaturePlanResult {
        FeaturePlanResult {
            timestamp: Utc::now(),
            execution_time: Duration::from_secs(45), // Under 5 minute target
            target_entity: "ConcurrentDiscoveryEngine".to_string(),
            impact_analysis: ImpactAnalysis {
                direct_impact: vec![
                    "DiscoveryEngine trait".to_string(),
                    "SimpleDiscoveryEngine".to_string(),
                ],
                indirect_impact: vec![
                    "CLI commands".to_string(),
                    "WorkflowOrchestrator".to_string(),
                ],
                risk_level: WorkflowRiskLevel::High,
                complexity_estimate: ComplexityLevel::High,
            },
            scope_guidance: ScopeGuidance {
                boundaries: vec![
                    "Keep changes within discovery module".to_string(),
                ],
                files_to_modify: vec![
                    "src/discovery/concurrent_discovery_engine.rs".to_string(),
                ],
                files_to_avoid: vec![
                    "src/isg.rs".to_string(),
                ],
                integration_points: vec![
                    "CLI command handlers".to_string(),
                ],
            },
            test_recommendations: vec![
                TestRecommendation {
                    test_type: "unit".to_string(),
                    test_target: "ConcurrentDiscoveryEngine".to_string(),
                    rationale: "Verify thread safety".to_string(),
                    suggested_location: "src/discovery/concurrent_discovery_engine.rs".to_string(),
                },
            ],
        }
    }

    /// Create realistic debug result for a complex function
    fn create_realistic_debug_result() -> DebugResult {
        DebugResult {
            timestamp: Utc::now(),
            execution_time: Duration::from_secs(23), // Under 2 minute target
            target_entity: "calculate_blast_radius".to_string(),
            caller_traces: vec![
                CallerTrace {
                    caller: crate::discovery::EntityInfo {
                        name: "BlastRadiusAnalyzer::analyze".to_string(),
                        file_path: "src/discovery/blast_radius_analyzer.rs".to_string(),
                        entity_type: crate::discovery::EntityType::Function,
                        line_number: Some(156),
                    },
                    depth: 1,
                    call_context: "Primary analysis entry point".to_string(),
                    frequency: Some("High - called for every blast radius query".to_string()),
                },
            ],
            usage_sites: vec![
                UsageSite {
                    user: crate::discovery::EntityInfo {
                        name: "test_blast_radius_performance".to_string(),
                        file_path: "src/discovery/blast_radius_analyzer.rs".to_string(),
                        entity_type: crate::discovery::EntityType::Function,
                        line_number: Some(445),
                    },
                    usage_type: "test".to_string(),
                    context: "Performance regression test".to_string(),
                    location: FileLocation::with_line("src/discovery/blast_radius_analyzer.rs".to_string(), 445),
                },
            ],
            minimal_scope: ChangeScope {
                minimal_files: vec![
                    "src/discovery/blast_radius_analyzer.rs".to_string(),
                ],
                safe_boundaries: vec![
                    "Keep changes within BlastRadiusAnalyzer impl".to_string(),
                ],
                side_effects: vec![
                    "May affect blast radius query performance".to_string(),
                ],
                rollback_strategy: "Revert to previous commit".to_string(),
            },
        }
    } 
   /// Create realistic refactor result for a high-risk change
    fn create_realistic_refactor_result() -> RefactorResult {
        RefactorResult {
            timestamp: Utc::now(),
            execution_time: Duration::from_secs(67), // Under 3 minute target
            target_entity: "InMemoryIsg".to_string(),
            risk_assessment: RiskAssessment {
                overall_risk: WorkflowRiskLevel::Critical,
                risk_factors: vec![
                    RiskFactor {
                        description: "Core data structure used throughout the system".to_string(),
                        level: WorkflowRiskLevel::Critical,
                        impact: "Changes could break all discovery functionality".to_string(),
                    },
                ],
                mitigations: vec![
                    "Comprehensive test suite with 95%+ coverage".to_string(),
                ],
                confidence: ConfidenceLevel::High,
            },
            change_checklist: vec![
                ChecklistItem {
                    description: "Run full test suite including performance tests".to_string(),
                    priority: Priority::Critical,
                    completed: false,
                    notes: Some("Must include memory usage and timing validation".to_string()),
                },
            ],
            reviewer_guidance: ReviewerGuidance {
                focus_areas: vec![
                    "Memory safety and ownership patterns".to_string(),
                ],
                potential_issues: vec![
                    "Memory leaks in graph traversal".to_string(),
                ],
                testing_recommendations: vec![
                    "Run memory profiler on large codebases".to_string(),
                ],
                approval_criteria: vec![
                    "All tests pass including performance benchmarks".to_string(),
                ],
            },
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_integration_module_works() {
        // Simple test to verify the module is being compiled
        assert!(true);
    }    
/// Test complete workflow formatting across all output formats
    #[test]
    fn test_complete_workflow_formatting_integration() {
        let fixture = IntegrationTestFixture::new();
        
        // Test all formatters with all workflow types
        let formatters: Vec<(&str, Box<dyn OutputFormatter>)> = vec![
            ("human", Box::new(HumanFormatter::new())),
            ("json", Box::new(JsonFormatter::new())),
            ("pr-summary", Box::new(PrSummaryFormatter::new())),
            ("ci-github", Box::new(CiFormatter::for_platform(CiPlatform::GitHub))),
        ];

        for (format_name, formatter) in formatters {
            // Test onboarding workflow
            let onboard_result = formatter.format_onboarding(&fixture.onboarding_result);
            assert!(onboard_result.is_ok(), 
                    "Onboarding formatting failed for {}: {:?}", format_name, onboard_result.err());
            
            let onboard_output = onboard_result.unwrap();
            assert!(!onboard_output.is_empty(), "Onboarding output should not be empty for {}", format_name);
            
            // Test feature planning workflow
            let feature_result = formatter.format_feature_plan(&fixture.feature_plan_result);
            assert!(feature_result.is_ok(), 
                    "Feature planning formatting failed for {}: {:?}", format_name, feature_result.err());
            
            // Test debug workflow
            let debug_result = formatter.format_debug(&fixture.debug_result);
            assert!(debug_result.is_ok(), 
                    "Debug formatting failed for {}: {:?}", format_name, debug_result.err());
            
            // Test refactor workflow
            let refactor_result = formatter.format_refactor(&fixture.refactor_result);
            assert!(refactor_result.is_ok(), 
                    "Refactor formatting failed for {}: {:?}", format_name, refactor_result.err());
        }
    }

    /// Test performance contracts across all formatters and workflows
    #[test]
    fn test_formatting_performance_contracts_integration() {
        let fixture = IntegrationTestFixture::new();
        let formatters: Vec<Box<dyn OutputFormatter>> = vec![
            Box::new(HumanFormatter::new()),
            Box::new(JsonFormatter::new()),
            Box::new(PrSummaryFormatter::new()),
            Box::new(CiFormatter::new()),
        ];

        for formatter in formatters {
            // Test onboarding performance
            let start = Instant::now();
            let result = formatter.format_onboarding(&fixture.onboarding_result).unwrap();
            let elapsed = start.elapsed();
            assert!(elapsed < Duration::from_millis(100), 
                    "Onboarding formatting took {:?}, expected <100ms", elapsed);
            assert!(!result.is_empty());

            // Test feature planning performance
            let start = Instant::now();
            let result = formatter.format_feature_plan(&fixture.feature_plan_result).unwrap();
            let elapsed = start.elapsed();
            assert!(elapsed < Duration::from_millis(100), 
                    "Feature planning formatting took {:?}, expected <100ms", elapsed);
            assert!(!result.is_empty());
        }
    }
    
    // Test cross-format data consistency
    #[test]
    fn test_cross_format_data_consistency() {
        let fixture = IntegrationTestFixture::new();
        
        let human_formatter = HumanFormatter::new();
        let json_formatter = JsonFormatter::new();
        let pr_formatter = PrSummaryFormatter::new();
        
        // Test onboarding data consistency
        let human_output = human_formatter.format_onboarding(&fixture.onboarding_result).unwrap();
        let json_output = json_formatter.format_onboarding(&fixture.onboarding_result).unwrap();
        let pr_output = pr_formatter.format_onboarding(&fixture.onboarding_result).unwrap();
        
        // All formats should contain core data
        let total_files = fixture.onboarding_result.overview.total_files.to_string();
        let total_entities = fixture.onboarding_result.overview.total_entities.to_string();
        
        assert!(human_output.contains(&total_files), "Human format missing total files");
        assert!(human_output.contains(&total_entities), "Human format missing total entities");
        
        assert!(json_output.contains(&total_files), "JSON format missing total files");
        assert!(json_output.contains(&total_entities), "JSON format missing total entities");
        
        assert!(pr_output.contains(&total_files), "PR format missing total files");
        assert!(pr_output.contains(&total_entities), "PR format missing total entities");
    }

    /// Test copy-pastable output validation across formats
    #[test]
    fn test_copy_pastable_output_integration() {
        let fixture = IntegrationTestFixture::new();
        
        // Human formatter should produce clean terminal output
        let human_formatter = HumanFormatter::new();
        let human_output = human_formatter.format_onboarding(&fixture.onboarding_result).unwrap();
        
        // Should not contain problematic control characters
        assert!(!human_output.contains('\x1b'), "Human output contains ANSI escape codes");
        assert!(!human_output.contains('\x07'), "Human output contains bell character");
        
        // JSON formatter should produce valid JSON
        let json_formatter = JsonFormatter::new();
        let json_output = json_formatter.format_feature_plan(&fixture.feature_plan_result).unwrap();
        
        // Should parse as valid JSON
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json_output);
        assert!(parsed.is_ok(), "JSON output is not valid JSON: {:?}", parsed.err());
        
        let json_value = parsed.unwrap();
        assert!(json_value.is_object(), "JSON output should be an object");
        assert!(json_value["workflow"].is_string(), "JSON should contain workflow field");
    }

    /// Test CI/CD platform-specific output formats
    #[test]
    fn test_ci_platform_specific_formatting() {
        let fixture = IntegrationTestFixture::new();
        
        // GitHub Actions format
        let github_formatter = CiFormatter::for_platform(CiPlatform::GitHub);
        let github_output = github_formatter.format_refactor(&fixture.refactor_result).unwrap();
        
        // Should contain GitHub Actions annotations
        assert!(github_output.contains("::error") || github_output.contains("::warning") || github_output.contains("::notice"),
                "GitHub output should contain workflow annotations");
        assert!(github_output.contains("$GITHUB_ENV"), "GitHub output should set environment variables");
        
        // GitLab CI format
        let gitlab_formatter = CiFormatter::for_platform(CiPlatform::GitLab);
        let gitlab_output = gitlab_formatter.format_feature_plan(&fixture.feature_plan_result).unwrap();
        
        // Should contain GitLab CI format
        assert!(gitlab_output.contains("echo "), "GitLab output should use echo commands");
        assert!(gitlab_output.contains("export "), "GitLab output should export variables");
    }

    /// Test formatter factory integration
    #[test]
    fn test_formatter_factory_integration() {
        let fixture = IntegrationTestFixture::new();
        
        let format_strings = vec!["human", "json", "pr-summary", "ci"];
        
        for format_str in format_strings {
            let formatter = FormatterFactory::create_formatter(format_str)
                .expect(&format!("Should create formatter for {}", format_str));
            
            // Test with all workflow types
            assert!(formatter.format_onboarding(&fixture.onboarding_result).is_ok(),
                    "Factory-created {} formatter should handle onboarding", format_str);
            assert!(formatter.format_feature_plan(&fixture.feature_plan_result).is_ok(),
                    "Factory-created {} formatter should handle feature planning", format_str);
        }
        
        // Test invalid format
        let invalid_result = FormatterFactory::create_formatter("invalid-format");
        assert!(invalid_result.is_err(), "Should reject invalid format strings");
        
        match invalid_result.unwrap_err() {
            FormattingError::InvalidFormat { format } => {
                assert_eq!(format, "invalid-format");
            }
            _ => panic!("Should return InvalidFormat error"),
        }
    }
}
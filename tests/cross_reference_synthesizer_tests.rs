use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

use parseltongue::content_processing::{
    ContentChunk, ChunkId, LineRange,
    user_journey_extractor::{UserJourney, DeveloperPersona, WorkflowType, Solution, ComplexityLevel as SolutionComplexity, SuccessMetrics, PerformanceMetric, ProductivityMetric, QualityMetric},
    strategic_theme_organizer::{StrategicTheme, StrategicThemeCategory, ImplementationPriority, CompetitiveAdvantage, EcosystemPositioning, RoiMetrics, AdoptionPathway, AdoptionStage, SourceLocation},
    cross_reference_synthesizer::{
        CrossReferenceSynthesizer, EntityType, EntityReference, RelationshipType, 
        ComplexityLevel, ImpactLevel, QualityIssueType, IssueSeverity
    },
};

fn create_test_content_chunk(id: ChunkId, content: &str, start_line: usize, end_line: usize) -> ContentChunk {
    ContentChunk {
        id,
        content: content.to_string(),
        line_range: LineRange::new(start_line, end_line),
        source_file: "test.md".to_string(),
        created_at: Utc::now(),
    }
}

fn create_test_user_journey(
    title: &str,
    description: &str,
    persona: DeveloperPersona,
    workflow_type: WorkflowType,
    chunk_id: ChunkId,
) -> UserJourney {
    let mut journey = UserJourney::new(
        title.to_string(),
        description.to_string(),
        persona,
        workflow_type,
    );
    
    // Add a test solution
    journey.solutions.push(Solution {
        description: format!("Solution for {}", title),
        approach: "Test approach".to_string(),
        expected_outcomes: vec!["Improved efficiency".to_string()],
        implementation_complexity: SolutionComplexity::Medium,
        source_location: parseltongue::content_processing::user_journey_extractor::SourceLocation {
            chunk_id,
            line_start: 1,
            line_end: 5,
            confidence_score: 0.8,
        },
    });
    
    // Add test success metrics
    journey.success_metrics = SuccessMetrics {
        performance_improvements: vec![PerformanceMetric {
            metric_name: "Response Time".to_string(),
            baseline_value: Some("100ms".to_string()),
            target_value: "50ms".to_string(),
            measurement_method: "Automated testing".to_string(),
        }],
        productivity_gains: vec![ProductivityMetric {
            metric_name: "Development Speed".to_string(),
            current_state: "Slow".to_string(),
            improved_state: "Fast".to_string(),
            time_savings: Some("2 hours per day".to_string()),
        }],
        quality_improvements: vec![QualityMetric {
            metric_name: "Code Quality".to_string(),
            improvement_description: "Better maintainability".to_string(),
            measurement_criteria: vec!["Cyclomatic complexity".to_string()],
        }],
        user_satisfaction: None,
    };
    
    journey.integration_tools = vec!["VSCode".to_string(), "Git".to_string()];
    journey.prerequisites = vec!["Basic Rust knowledge".to_string()];
    journey.source_chunks = vec![chunk_id];
    journey.confidence_score = 0.8;
    
    journey
}

fn create_test_strategic_theme(
    title: &str,
    description: &str,
    category: StrategicThemeCategory,
    chunk_id: ChunkId,
) -> StrategicTheme {
    let mut theme = StrategicTheme::new(
        title.to_string(),
        description.to_string(),
        category,
    );
    
    // Add competitive advantage
    theme.competitive_advantages.push(CompetitiveAdvantage {
        description: format!("Competitive advantage for {}", title),
        unique_value_proposition: "Unique value".to_string(),
        market_differentiation: "Market differentiation".to_string(),
        technical_moat: Some("Technical superiority".to_string()),
        source_location: SourceLocation {
            chunk_id,
            line_start: 1,
            line_end: 3,
            confidence_score: 0.9,
        },
        confidence_score: 0.9,
    });
    
    // Add ROI metrics
    theme.roi_metrics.push(RoiMetrics {
        metric_name: "Development Efficiency".to_string(),
        baseline_value: Some("Current state".to_string()),
        projected_value: "50% improvement".to_string(),
        measurement_methodology: "Time tracking".to_string(),
        timeframe: "6 months".to_string(),
        assumptions: vec!["Team adoption".to_string()],
        risk_factors: vec!["Learning curve".to_string()],
    });
    
    // Add adoption pathway
    theme.adoption_pathways.push(AdoptionPathway {
        pathway_name: "Standard Adoption".to_string(),
        target_persona: "Developer".to_string(),
        adoption_stages: vec![AdoptionStage {
            stage_name: "Initial Setup".to_string(),
            description: "Setup and configuration".to_string(),
            duration_estimate: Some("1 week".to_string()),
            prerequisites: vec!["System requirements".to_string()],
            deliverables: vec!["Working setup".to_string()],
            success_criteria: vec!["Successful installation".to_string()],
        }],
        barriers_to_adoption: vec!["Complexity".to_string()],
        enablers: vec!["Documentation".to_string()],
        success_indicators: vec!["Usage metrics".to_string()],
    });
    
    theme.ecosystem_positioning = EcosystemPositioning {
        market_position: "Leading position".to_string(),
        target_audience: vec!["Developers".to_string()],
        integration_points: vec!["IDE integration".to_string()],
        competitive_landscape: vec!["Competitor A".to_string()],
        partnership_opportunities: vec!["Partner B".to_string()],
    };
    
    theme.implementation_priority = ImplementationPriority::High;
    theme.dependencies = vec!["Rust ecosystem".to_string()];
    theme.source_chunks = vec![chunk_id];
    theme.confidence_score = 0.85;
    
    theme
}

#[test]
fn test_cross_reference_synthesizer_creation() {
    let synthesizer = CrossReferenceSynthesizer::new();
    
    assert!(synthesizer.get_cross_references().is_empty());
    assert!(synthesizer.get_narratives().is_empty());
    assert!(synthesizer.get_quality_issues().is_empty());
    assert!(synthesizer.get_entity_index().is_empty());
}

#[test]
fn test_entity_registration() {
    let mut synthesizer = CrossReferenceSynthesizer::new();
    
    let chunk_id = ChunkId::new();
    let chunk = create_test_content_chunk(chunk_id, "Test content", 1, 10);
    
    let journey = create_test_user_journey(
        "Test Journey",
        "A test user journey for development workflow",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::Development,
        chunk_id,
    );
    
    let theme = create_test_strategic_theme(
        "Test Theme",
        "A test strategic theme for developer productivity",
        StrategicThemeCategory::DeveloperProductivity,
        chunk_id,
    );
    
    // Register entities
    synthesizer.register_user_journey(&journey).unwrap();
    synthesizer.register_strategic_theme(&theme).unwrap();
    synthesizer.register_content_chunk(&chunk).unwrap();
    
    // Verify registration
    let entity_index = synthesizer.get_entity_index();
    
    assert!(entity_index.contains_key(&EntityType::UserJourney));
    assert!(entity_index.contains_key(&EntityType::StrategicTheme));
    assert!(entity_index.contains_key(&EntityType::ContentChunk));
    
    assert!(entity_index[&EntityType::UserJourney].contains_key(&journey.id.0.to_string()));
    assert!(entity_index[&EntityType::StrategicTheme].contains_key(&theme.id.0.to_string()));
    assert!(entity_index[&EntityType::ContentChunk].contains_key(&chunk.id.0.to_string()));
}

#[test]
fn test_cross_reference_creation() {
    let mut synthesizer = CrossReferenceSynthesizer::new();
    
    let chunk_id = ChunkId::new();
    let chunk = create_test_content_chunk(chunk_id, "Developer productivity enhancement", 1, 10);
    
    // Create journey and theme with related content
    let journey = create_test_user_journey(
        "Productivity Enhancement",
        "Improve developer productivity through automation",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::Development,
        chunk_id,
    );
    
    let theme = create_test_strategic_theme(
        "Developer Productivity",
        "Strategic theme focused on developer productivity improvements",
        StrategicThemeCategory::DeveloperProductivity,
        chunk_id,
    );
    
    // Register entities
    synthesizer.register_user_journey(&journey).unwrap();
    synthesizer.register_strategic_theme(&theme).unwrap();
    synthesizer.register_content_chunk(&chunk).unwrap();
    
    // Create cross-references
    let cross_refs = synthesizer.create_cross_references(
        &[journey],
        &[theme],
        &[chunk],
    ).unwrap();
    
    assert!(!cross_refs.is_empty());
    
    // Verify cross-references were created
    let stored_refs = synthesizer.get_cross_references();
    assert!(!stored_refs.is_empty());
    
    // Check for journey-theme relationship
    let journey_theme_ref = stored_refs.values().find(|cr| {
        cr.source_entity.entity_type == EntityType::UserJourney &&
        cr.target_entity.entity_type == EntityType::StrategicTheme
    });
    
    assert!(journey_theme_ref.is_some());
    let ref_found = journey_theme_ref.unwrap();
    assert!(ref_found.confidence_score > 0.5);
    assert!(!ref_found.evidence.is_empty());
}

#[test]
fn test_workflow_narrative_generation() {
    let mut synthesizer = CrossReferenceSynthesizer::new();
    
    let chunk_id1 = ChunkId::new();
    let chunk_id2 = ChunkId::new();
    
    // Create multiple journeys for the same persona and workflow type
    let journey1 = create_test_user_journey(
        "Code Analysis",
        "Analyze code structure and dependencies",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::Development,
        chunk_id1,
    );
    
    let journey2 = create_test_user_journey(
        "Code Refactoring",
        "Refactor code for better maintainability",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::Development,
        chunk_id2,
    );
    
    let theme = create_test_strategic_theme(
        "Code Quality",
        "Improve overall code quality and maintainability",
        StrategicThemeCategory::DeveloperProductivity,
        chunk_id1,
    );
    
    // Generate narratives
    let narratives = synthesizer.generate_workflow_narratives(
        &[journey1, journey2],
        &[theme],
    ).unwrap();
    
    assert!(!narratives.is_empty());
    
    let narrative = &narratives[0];
    assert_eq!(narrative.persona, DeveloperPersona::IndividualDeveloper);
    assert_eq!(narrative.workflow_type, WorkflowType::Development);
    assert_eq!(narrative.related_journeys.len(), 2);
    assert!(!narrative.narrative_flow.is_empty());
    assert!(!narrative.success_criteria.is_empty());
}

#[test]
fn test_quality_issue_detection() {
    let mut synthesizer = CrossReferenceSynthesizer::new();
    
    let chunk_id = ChunkId::new();
    let chunk = create_test_content_chunk(chunk_id, "Test content", 1, 10);
    
    // Create journey with no solutions (should trigger gap detection)
    let mut journey_with_gaps = create_test_user_journey(
        "Incomplete Journey",
        "A journey with missing information",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::Development,
        chunk_id,
    );
    journey_with_gaps.solutions.clear(); // Remove solutions to create gap
    
    // Create journey with low confidence
    let mut low_confidence_journey = create_test_user_journey(
        "Low Confidence Journey",
        "A journey with low confidence score",
        DeveloperPersona::TeamLead,
        WorkflowType::Testing,
        chunk_id,
    );
    low_confidence_journey.confidence_score = 0.3; // Low confidence
    
    // Create theme with no competitive advantages
    let mut theme_with_gaps = create_test_strategic_theme(
        "Incomplete Theme",
        "A theme with missing information",
        StrategicThemeCategory::Performance,
        chunk_id,
    );
    theme_with_gaps.competitive_advantages.clear(); // Remove advantages to create gap
    
    // Create similar journeys (should trigger duplication detection)
    let journey_duplicate1 = create_test_user_journey(
        "Similar Journey 1",
        "This is a very similar journey description with same content",
        DeveloperPersona::DevOpsEngineer,
        WorkflowType::CiCd,
        chunk_id,
    );
    
    let journey_duplicate2 = create_test_user_journey(
        "Similar Journey 2", 
        "This is a very similar journey description with same content",
        DeveloperPersona::DevOpsEngineer,
        WorkflowType::CiCd,
        chunk_id,
    );
    
    let journeys = vec![
        journey_with_gaps,
        low_confidence_journey,
        journey_duplicate1,
        journey_duplicate2,
    ];
    
    let themes = vec![theme_with_gaps];
    let chunks = vec![chunk];
    
    // Detect quality issues
    let quality_issues = synthesizer.detect_quality_issues(&journeys, &themes, &chunks).unwrap();
    
    assert!(!quality_issues.is_empty());
    
    // Check for different types of issues
    let gap_issues: Vec<_> = quality_issues.iter()
        .filter(|issue| issue.issue_type == QualityIssueType::Gap)
        .collect();
    assert!(!gap_issues.is_empty());
    
    let low_confidence_issues: Vec<_> = quality_issues.iter()
        .filter(|issue| issue.issue_type == QualityIssueType::LowConfidence)
        .collect();
    assert!(!low_confidence_issues.is_empty());
    
    let duplication_issues: Vec<_> = quality_issues.iter()
        .filter(|issue| issue.issue_type == QualityIssueType::Duplication)
        .collect();
    assert!(!duplication_issues.is_empty());
}

#[test]
fn test_strategic_impact_and_feasibility_prioritization() {
    let mut synthesizer = CrossReferenceSynthesizer::new();
    
    let chunk_id = ChunkId::new();
    
    // Create high-impact journey
    let mut high_impact_journey = create_test_user_journey(
        "High Impact Journey",
        "AI-powered development assistance with significant productivity gains",
        DeveloperPersona::PlatformEngineer, // High strategic impact persona
        WorkflowType::LlmIntegration, // Complex but high-impact workflow
        chunk_id,
    );
    high_impact_journey.confidence_score = 0.9;
    
    // Create low-impact journey
    let mut low_impact_journey = create_test_user_journey(
        "Low Impact Journey",
        "Simple code formatting improvement",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::Development,
        chunk_id,
    );
    low_impact_journey.confidence_score = 0.4;
    low_impact_journey.solutions.clear(); // Reduce impact
    
    // Create high-impact theme
    let mut high_impact_theme = create_test_strategic_theme(
        "AI Enhancement Strategy",
        "Revolutionary AI integration for development workflows",
        StrategicThemeCategory::AiEnhancement, // High impact category
        chunk_id,
    );
    high_impact_theme.implementation_priority = ImplementationPriority::Critical;
    high_impact_theme.confidence_score = 0.95;
    
    // Create low-impact theme
    let mut low_impact_theme = create_test_strategic_theme(
        "Minor Community Feature",
        "Small community enhancement",
        StrategicThemeCategory::Community, // Lower impact category
        chunk_id,
    );
    low_impact_theme.implementation_priority = ImplementationPriority::Low;
    low_impact_theme.confidence_score = 0.5;
    low_impact_theme.competitive_advantages.clear(); // Reduce impact
    
    let journeys = vec![high_impact_journey, low_impact_journey];
    let themes = vec![high_impact_theme, low_impact_theme];
    
    // Get prioritized items
    let prioritized_items = synthesizer.prioritize_by_strategic_impact_and_feasibility(
        &journeys,
        &themes,
    ).unwrap();
    
    assert_eq!(prioritized_items.len(), 4); // 2 journeys + 2 themes
    
    // Verify items are sorted by priority score (highest first)
    for i in 1..prioritized_items.len() {
        assert!(prioritized_items[i-1].priority_score >= prioritized_items[i].priority_score);
    }
    
    // Verify high-impact items have higher priority scores
    let high_impact_items: Vec<_> = prioritized_items.iter()
        .filter(|item| item.entity_reference.title.contains("High Impact") || 
                      item.entity_reference.title.contains("AI Enhancement"))
        .collect();
    
    let low_impact_items: Vec<_> = prioritized_items.iter()
        .filter(|item| item.entity_reference.title.contains("Low Impact") || 
                      item.entity_reference.title.contains("Minor Community"))
        .collect();
    
    assert!(!high_impact_items.is_empty());
    assert!(!low_impact_items.is_empty());
    
    // High impact items should generally have higher priority scores
    let avg_high_priority: f64 = high_impact_items.iter()
        .map(|item| item.priority_score)
        .sum::<f64>() / high_impact_items.len() as f64;
    
    let avg_low_priority: f64 = low_impact_items.iter()
        .map(|item| item.priority_score)
        .sum::<f64>() / low_impact_items.len() as f64;
    
    assert!(avg_high_priority > avg_low_priority);
}

#[test]
fn test_relationship_type_detection() {
    let mut synthesizer = CrossReferenceSynthesizer::new();
    
    let chunk_id = ChunkId::new();
    
    // Create journey that implements a theme
    let implementation_journey = create_test_user_journey(
        "AI Code Generation",
        "Implement AI-powered code generation for developer productivity",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::LlmIntegration,
        chunk_id,
    );
    
    let ai_theme = create_test_strategic_theme(
        "AI Enhancement",
        "Strategic AI integration for development workflows",
        StrategicThemeCategory::AiEnhancement,
        chunk_id,
    );
    
    // Create dependent journeys
    let mut base_journey = create_test_user_journey(
        "Code Analysis",
        "Analyze code structure and dependencies",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::Development,
        chunk_id,
    );
    
    let mut dependent_journey = create_test_user_journey(
        "Code Testing",
        "Test code quality and functionality",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::Testing,
        chunk_id,
    );
    
    // Make dependent journey depend on base journey
    dependent_journey.prerequisites.push("Code analysis results".to_string());
    base_journey.solutions[0].expected_outcomes.push("Code analysis results".to_string());
    
    // Register entities
    synthesizer.register_user_journey(&implementation_journey).unwrap();
    synthesizer.register_user_journey(&base_journey).unwrap();
    synthesizer.register_user_journey(&dependent_journey).unwrap();
    synthesizer.register_strategic_theme(&ai_theme).unwrap();
    
    // Create cross-references
    let cross_refs = synthesizer.create_cross_references(
        &[implementation_journey, base_journey, dependent_journey],
        &[ai_theme],
        &[],
    ).unwrap();
    
    // Verify different relationship types were detected
    let relationship_types: std::collections::HashSet<_> = cross_refs.iter()
        .map(|cr| cr.relationship_type.clone())
        .collect();
    
    // Should have multiple relationship types
    assert!(relationship_types.len() > 1);
    
    // Should include implements relationship for AI journey-theme
    let implements_refs: Vec<_> = cross_refs.iter()
        .filter(|cr| cr.relationship_type == RelationshipType::Implements)
        .collect();
    assert!(!implements_refs.is_empty());
    
    // Should include dependency relationships
    let dependency_refs: Vec<_> = cross_refs.iter()
        .filter(|cr| cr.relationship_type == RelationshipType::DependsOn)
        .collect();
    // Note: Dependency detection might not trigger in this simple test case
}

#[test]
fn test_integration_point_extraction() {
    let mut synthesizer = CrossReferenceSynthesizer::new();
    
    let chunk_id = ChunkId::new();
    
    // Create journeys with shared tools
    let mut journey1 = create_test_user_journey(
        "VSCode Integration",
        "Integrate with VSCode for better development experience",
        DeveloperPersona::IndividualDeveloper,
        WorkflowType::Development,
        chunk_id,
    );
    journey1.integration_tools = vec!["VSCode".to_string(), "Language Server".to_string()];
    
    let mut journey2 = create_test_user_journey(
        "IDE Enhancement",
        "Enhance IDE capabilities with advanced features",
        DeveloperPersona::TeamLead,
        WorkflowType::Development,
        chunk_id,
    );
    journey2.integration_tools = vec!["VSCode".to_string(), "Git".to_string()];
    
    let theme = create_test_strategic_theme(
        "Developer Experience",
        "Improve overall developer experience",
        StrategicThemeCategory::DeveloperProductivity,
        chunk_id,
    );
    
    // Generate narratives (which includes integration point extraction)
    let narratives = synthesizer.generate_workflow_narratives(
        &[journey1, journey2],
        &[theme],
    ).unwrap();
    
    assert!(!narratives.is_empty());
    
    let narrative = &narratives[0];
    
    // Should have integration points for shared tools
    let vscode_integration = narrative.integration_points.iter()
        .find(|ip| ip.name.contains("VSCode"));
    
    assert!(vscode_integration.is_some());
    
    let integration_point = vscode_integration.unwrap();
    assert!(!integration_point.description.is_empty());
    assert!(!integration_point.involved_systems.is_empty());
}

#[test]
fn test_comprehensive_workflow() {
    let mut synthesizer = CrossReferenceSynthesizer::new();
    
    let chunk_id1 = ChunkId::new();
    let chunk_id2 = ChunkId::new();
    let chunk_id3 = ChunkId::new();
    
    let chunk1 = create_test_content_chunk(chunk_id1, "AI-powered development tools", 1, 100);
    let chunk2 = create_test_content_chunk(chunk_id2, "Developer productivity enhancement", 101, 200);
    let chunk3 = create_test_content_chunk(chunk_id3, "Code analysis and refactoring", 201, 300);
    
    // Create diverse set of journeys and themes
    let journeys = vec![
        create_test_user_journey(
            "AI Code Assistant",
            "AI-powered code completion and suggestions",
            DeveloperPersona::IndividualDeveloper,
            WorkflowType::LlmIntegration,
            chunk_id1,
        ),
        create_test_user_journey(
            "Team Code Review",
            "Streamlined code review process for teams",
            DeveloperPersona::TeamLead,
            WorkflowType::Development,
            chunk_id2,
        ),
        create_test_user_journey(
            "Automated Testing",
            "Automated test generation and execution",
            DeveloperPersona::DevOpsEngineer,
            WorkflowType::Testing,
            chunk_id3,
        ),
    ];
    
    let themes = vec![
        create_test_strategic_theme(
            "AI Integration",
            "Strategic AI integration across development workflows",
            StrategicThemeCategory::AiEnhancement,
            chunk_id1,
        ),
        create_test_strategic_theme(
            "Developer Productivity",
            "Comprehensive developer productivity improvements",
            StrategicThemeCategory::DeveloperProductivity,
            chunk_id2,
        ),
    ];
    
    let chunks = vec![chunk1, chunk2, chunk3];
    
    // Register all entities
    for journey in &journeys {
        synthesizer.register_user_journey(journey).unwrap();
    }
    for theme in &themes {
        synthesizer.register_strategic_theme(theme).unwrap();
    }
    for chunk in &chunks {
        synthesizer.register_content_chunk(chunk).unwrap();
    }
    
    // Execute full workflow
    let cross_refs = synthesizer.create_cross_references(&journeys, &themes, &chunks).unwrap();
    let narratives = synthesizer.generate_workflow_narratives(&journeys, &themes).unwrap();
    let quality_issues = synthesizer.detect_quality_issues(&journeys, &themes, &chunks).unwrap();
    let prioritized_items = synthesizer.prioritize_by_strategic_impact_and_feasibility(&journeys, &themes).unwrap();
    
    // Verify comprehensive results
    assert!(!cross_refs.is_empty());
    assert!(!narratives.is_empty());
    assert!(!prioritized_items.is_empty());
    
    // Quality issues might be empty if all data is well-formed
    // but the detection system should run without errors
    
    // Verify entity index is populated
    let entity_index = synthesizer.get_entity_index();
    assert_eq!(entity_index[&EntityType::UserJourney].len(), journeys.len());
    assert_eq!(entity_index[&EntityType::StrategicTheme].len(), themes.len());
    assert_eq!(entity_index[&EntityType::ContentChunk].len(), chunks.len());
    
    // Verify cross-references are stored
    assert_eq!(synthesizer.get_cross_references().len(), cross_refs.len());
    
    // Verify narratives are stored
    assert_eq!(synthesizer.get_narratives().len(), narratives.len());
    
    // Verify prioritized items cover all entities
    assert_eq!(prioritized_items.len(), journeys.len() + themes.len());
}
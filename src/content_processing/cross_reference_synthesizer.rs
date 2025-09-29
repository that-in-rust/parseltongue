use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use thiserror::Error;

use crate::content_processing::{
    ContentChunk, ChunkId, Result, ContentProcessingError,
    user_journey_extractor::{UserJourney, UserJourneyId, DeveloperPersona, WorkflowType},
    strategic_theme_organizer::{StrategicTheme, StrategyThemeId, StrategicThemeCategory},
};

/// Unique identifier for cross-references
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CrossReferenceId(pub Uuid);

impl CrossReferenceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for CrossReferenceId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<CrossReferenceId> for Uuid {
    fn from(id: CrossReferenceId) -> Self {
        id.0
    }
}

/// Types of entities that can be cross-referenced
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    UserJourney,
    StrategicTheme,
    TechnicalInsight,
    ContentChunk,
}

/// Reference to an entity in the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityReference {
    pub entity_type: EntityType,
    pub entity_id: String, // Generic string ID to handle different ID types
    pub title: String,
    pub description: String,
}

/// Types of relationships between entities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    /// One entity supports or enables another
    Supports,
    /// One entity depends on another
    DependsOn,
    /// Entities are related or similar
    RelatedTo,
    /// One entity contradicts another
    Contradicts,
    /// One entity implements another
    Implements,
    /// Entities are part of the same workflow
    PartOfWorkflow,
    /// One entity enhances another
    Enhances,
    /// Entities share common themes
    SharesTheme,
}

impl RelationshipType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Supports => "supports",
            Self::DependsOn => "depends_on",
            Self::RelatedTo => "related_to",
            Self::Contradicts => "contradicts",
            Self::Implements => "implements",
            Self::PartOfWorkflow => "part_of_workflow",
            Self::Enhances => "enhances",
            Self::SharesTheme => "shares_theme",
        }
    }
}

/// Cross-reference link between entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub id: CrossReferenceId,
    pub source_entity: EntityReference,
    pub target_entity: EntityReference,
    pub relationship_type: RelationshipType,
    pub confidence_score: f64,
    pub evidence: Vec<String>,
    pub source_chunks: Vec<ChunkId>,
    pub created_at: DateTime<Utc>,
}

impl CrossReference {
    pub fn new(
        source_entity: EntityReference,
        target_entity: EntityReference,
        relationship_type: RelationshipType,
        confidence_score: f64,
    ) -> Self {
        Self {
            id: CrossReferenceId::new(),
            source_entity,
            target_entity,
            relationship_type,
            confidence_score,
            evidence: Vec::new(),
            source_chunks: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

/// Coherent workflow narrative combining multiple insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNarrative {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub persona: DeveloperPersona,
    pub workflow_type: WorkflowType,
    pub related_journeys: Vec<UserJourneyId>,
    pub supporting_themes: Vec<StrategyThemeId>,
    pub narrative_flow: Vec<NarrativeStep>,
    pub integration_points: Vec<IntegrationPoint>,
    pub success_criteria: Vec<String>,
    pub implementation_complexity: ComplexityLevel,
    pub strategic_impact: ImpactLevel,
    pub created_at: DateTime<Utc>,
}

/// Individual step in a workflow narrative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeStep {
    pub step_number: usize,
    pub title: String,
    pub description: String,
    pub related_entities: Vec<EntityReference>,
    pub prerequisites: Vec<String>,
    pub outcomes: Vec<String>,
    pub tools_involved: Vec<String>,
}

/// Integration point between different workflows or systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationPoint {
    pub name: String,
    pub description: String,
    pub integration_type: IntegrationType,
    pub involved_systems: Vec<String>,
    pub data_flow: Vec<String>,
    pub complexity: ComplexityLevel,
}

/// Types of integration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationType {
    ApiIntegration,
    PluginSystem,
    DataPipeline,
    WorkflowOrchestration,
    EventDriven,
    DirectIntegration,
}

/// Complexity levels for implementation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl ComplexityLevel {
    pub fn score(&self) -> u8 {
        match self {
            Self::Low => 1,
            Self::Medium => 2,
            Self::High => 3,
            Self::VeryHigh => 4,
        }
    }
}

/// Strategic impact levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl ImpactLevel {
    pub fn score(&self) -> u8 {
        match self {
            Self::Low => 1,
            Self::Medium => 2,
            Self::High => 3,
            Self::Critical => 4,
        }
    }
}

/// Gap or contradiction detected in the analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    pub id: Uuid,
    pub issue_type: QualityIssueType,
    pub severity: IssueSeverity,
    pub description: String,
    pub affected_entities: Vec<EntityReference>,
    pub evidence: Vec<String>,
    pub suggested_resolution: Option<String>,
    pub source_chunks: Vec<ChunkId>,
    pub detected_at: DateTime<Utc>,
}

/// Types of quality issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityIssueType {
    /// Missing information or incomplete analysis
    Gap,
    /// Conflicting information between entities
    Contradiction,
    /// Inconsistent categorization or classification
    Inconsistency,
    /// Duplicate or redundant information
    Duplication,
    /// Missing cross-references that should exist
    MissingReference,
    /// Low confidence scores indicating uncertain analysis
    LowConfidence,
}

/// Severity levels for quality issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl IssueSeverity {
    pub fn score(&self) -> u8 {
        match self {
            Self::Info => 1,
            Self::Warning => 2,
            Self::Error => 3,
            Self::Critical => 4,
        }
    }
}

/// Cross-reference and synthesis system errors
#[derive(Error, Debug)]
pub enum SynthesisError {
    #[error("Entity not found: {entity_type:?} with ID {entity_id}")]
    EntityNotFound { entity_type: EntityType, entity_id: String },
    
    #[error("Invalid relationship: {relationship:?} between {source:?} and {target:?}")]
    InvalidRelationship { 
        relationship: RelationshipType, 
        source: EntityType, 
        target: EntityType 
    },
    
    #[error("Synthesis failed: {reason}")]
    SynthesisFailed { reason: String },
    
    #[error("Quality check failed: {reason}")]
    QualityCheckFailed { reason: String },
    
    #[error("Content processing error: {0}")]
    ContentProcessing(#[from] ContentProcessingError),
}

/// Main cross-reference and synthesis engine
pub struct CrossReferenceSynthesizer {
    /// All cross-references in the system
    cross_references: HashMap<CrossReferenceId, CrossReference>,
    
    /// Index of entities by type and ID
    entity_index: HashMap<EntityType, HashMap<String, EntityReference>>,
    
    /// Relationship patterns for automatic detection
    relationship_patterns: HashMap<RelationshipType, Vec<String>>,
    
    /// Quality issue patterns
    quality_patterns: HashMap<QualityIssueType, Vec<String>>,
    
    /// Generated workflow narratives
    narratives: HashMap<Uuid, WorkflowNarrative>,
    
    /// Detected quality issues
    quality_issues: HashMap<Uuid, QualityIssue>,
}

impl CrossReferenceSynthesizer {
    pub fn new() -> Self {
        Self {
            cross_references: HashMap::new(),
            entity_index: HashMap::new(),
            relationship_patterns: Self::build_relationship_patterns(),
            quality_patterns: Self::build_quality_patterns(),
            narratives: HashMap::new(),
            quality_issues: HashMap::new(),
        }
    }
    
    /// Register entities in the system for cross-referencing
    pub fn register_user_journey(&mut self, journey: &UserJourney) -> Result<()> {
        let entity_ref = EntityReference {
            entity_type: EntityType::UserJourney,
            entity_id: journey.id.0.to_string(),
            title: journey.title.clone(),
            description: journey.description.clone(),
        };
        
        self.entity_index
            .entry(EntityType::UserJourney)
            .or_insert_with(HashMap::new)
            .insert(journey.id.0.to_string(), entity_ref);
        
        Ok(())
    }
    
    pub fn register_strategic_theme(&mut self, theme: &StrategicTheme) -> Result<()> {
        let entity_ref = EntityReference {
            entity_type: EntityType::StrategicTheme,
            entity_id: theme.id.0.to_string(),
            title: theme.title.clone(),
            description: theme.description.clone(),
        };
        
        self.entity_index
            .entry(EntityType::StrategicTheme)
            .or_insert_with(HashMap::new)
            .insert(theme.id.0.to_string(), entity_ref);
        
        Ok(())
    }
    
    pub fn register_content_chunk(&mut self, chunk: &ContentChunk) -> Result<()> {
        let entity_ref = EntityReference {
            entity_type: EntityType::ContentChunk,
            entity_id: chunk.id.0.to_string(),
            title: format!("Chunk {}-{}", chunk.line_range.start, chunk.line_range.end),
            description: chunk.content.chars().take(200).collect::<String>() + "...",
        };
        
        self.entity_index
            .entry(EntityType::ContentChunk)
            .or_insert_with(HashMap::new)
            .insert(chunk.id.0.to_string(), entity_ref);
        
        Ok(())
    }
    
    /// Implement cross-reference linking system to connect related insights
    pub fn create_cross_references(
        &mut self,
        journeys: &[UserJourney],
        themes: &[StrategicTheme],
        chunks: &[ContentChunk],
    ) -> Result<Vec<CrossReference>> {
        let mut new_references = Vec::new();
        
        // Cross-reference journeys with themes
        for journey in journeys {
            for theme in themes {
                if let Some(cross_ref) = self.analyze_journey_theme_relationship(journey, theme)? {
                    self.cross_references.insert(cross_ref.id, cross_ref.clone());
                    new_references.push(cross_ref);
                }
            }
        }
        
        // Cross-reference journeys with each other
        for (i, journey1) in journeys.iter().enumerate() {
            for journey2 in journeys.iter().skip(i + 1) {
                if let Some(cross_ref) = self.analyze_journey_journey_relationship(journey1, journey2)? {
                    self.cross_references.insert(cross_ref.id, cross_ref.clone());
                    new_references.push(cross_ref);
                }
            }
        }
        
        // Cross-reference themes with each other
        for (i, theme1) in themes.iter().enumerate() {
            for theme2 in themes.iter().skip(i + 1) {
                if let Some(cross_ref) = self.analyze_theme_theme_relationship(theme1, theme2)? {
                    self.cross_references.insert(cross_ref.id, cross_ref.clone());
                    new_references.push(cross_ref);
                }
            }
        }
        
        // Cross-reference with content chunks
        for chunk in chunks {
            for journey in journeys {
                if journey.source_chunks.contains(&chunk.id) {
                    if let Some(cross_ref) = self.create_chunk_journey_reference(chunk, journey)? {
                        self.cross_references.insert(cross_ref.id, cross_ref.clone());
                        new_references.push(cross_ref);
                    }
                }
            }
            
            for theme in themes {
                if theme.source_chunks.contains(&chunk.id) {
                    if let Some(cross_ref) = self.create_chunk_theme_reference(chunk, theme)? {
                        self.cross_references.insert(cross_ref.id, cross_ref.clone());
                        new_references.push(cross_ref);
                    }
                }
            }
        }
        
        Ok(new_references)
    }
    
    /// Create coherent workflow narrative generation from distributed insights
    pub fn generate_workflow_narratives(
        &mut self,
        journeys: &[UserJourney],
        themes: &[StrategicTheme],
    ) -> Result<Vec<WorkflowNarrative>> {
        let mut narratives = Vec::new();
        
        // Group journeys by persona and workflow type
        let mut journey_groups: HashMap<(DeveloperPersona, WorkflowType), Vec<&UserJourney>> = HashMap::new();
        
        for journey in journeys {
            journey_groups
                .entry((journey.persona.clone(), journey.workflow_type.clone()))
                .or_insert_with(Vec::new)
                .push(journey);
        }
        
        // Generate narrative for each group
        for ((persona, workflow_type), group_journeys) in journey_groups {
            if group_journeys.len() >= 2 { // Only create narratives for groups with multiple journeys
                let narrative = self.synthesize_narrative_from_journeys(
                    &persona,
                    &workflow_type,
                    &group_journeys,
                    themes,
                )?;
                
                self.narratives.insert(narrative.id, narrative.clone());
                narratives.push(narrative);
            }
        }
        
        Ok(narratives)
    }
    
    /// Build gap and contradiction detection system for quality assurance
    pub fn detect_quality_issues(
        &mut self,
        journeys: &[UserJourney],
        themes: &[StrategicTheme],
        chunks: &[ContentChunk],
    ) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();
        
        // Detect gaps
        issues.extend(self.detect_gaps(journeys, themes, chunks)?);
        
        // Detect contradictions
        issues.extend(self.detect_contradictions(journeys, themes)?);
        
        // Detect inconsistencies
        issues.extend(self.detect_inconsistencies(journeys, themes)?);
        
        // Detect duplications
        issues.extend(self.detect_duplications(journeys, themes)?);
        
        // Detect missing references
        issues.extend(self.detect_missing_references(journeys, themes)?);
        
        // Detect low confidence issues
        issues.extend(self.detect_low_confidence_issues(journeys, themes)?);
        
        // Store issues
        for issue in &issues {
            self.quality_issues.insert(issue.id, issue.clone());
        }
        
        Ok(issues)
    }
    
    /// Implement strategic impact and implementation feasibility prioritization
    pub fn prioritize_by_strategic_impact_and_feasibility(
        &self,
        journeys: &[UserJourney],
        themes: &[StrategicTheme],
    ) -> Result<Vec<PrioritizedItem>> {
        let mut prioritized_items = Vec::new();
        
        // Prioritize journeys
        for journey in journeys {
            let strategic_impact = self.calculate_journey_strategic_impact(journey, themes)?;
            let implementation_feasibility = self.calculate_journey_implementation_feasibility(journey)?;
            let priority_score = self.calculate_priority_score(strategic_impact, implementation_feasibility);
            
            prioritized_items.push(PrioritizedItem {
                entity_reference: EntityReference {
                    entity_type: EntityType::UserJourney,
                    entity_id: journey.id.0.to_string(),
                    title: journey.title.clone(),
                    description: journey.description.clone(),
                },
                strategic_impact,
                implementation_feasibility,
                priority_score,
                supporting_evidence: self.gather_journey_evidence(journey)?,
            });
        }
        
        // Prioritize themes
        for theme in themes {
            let strategic_impact = self.calculate_theme_strategic_impact(theme)?;
            let implementation_feasibility = self.calculate_theme_implementation_feasibility(theme)?;
            let priority_score = self.calculate_priority_score(strategic_impact, implementation_feasibility);
            
            prioritized_items.push(PrioritizedItem {
                entity_reference: EntityReference {
                    entity_type: EntityType::StrategicTheme,
                    entity_id: theme.id.0.to_string(),
                    title: theme.title.clone(),
                    description: theme.description.clone(),
                },
                strategic_impact,
                implementation_feasibility,
                priority_score,
                supporting_evidence: self.gather_theme_evidence(theme)?,
            });
        }
        
        // Sort by priority score (highest first)
        prioritized_items.sort_by(|a, b| b.priority_score.partial_cmp(&a.priority_score).unwrap());
        
        Ok(prioritized_items)
    }
    
    // Helper methods for relationship analysis
    fn analyze_journey_theme_relationship(
        &self,
        journey: &UserJourney,
        theme: &StrategicTheme,
    ) -> Result<Option<CrossReference>> {
        let mut confidence_score = 0.0;
        let mut evidence = Vec::new();
        let mut relationship_type = RelationshipType::RelatedTo;
        
        // Check if journey persona aligns with theme category
        if self.persona_aligns_with_theme_category(&journey.persona, &theme.category) {
            confidence_score += 0.3;
            evidence.push(format!("Persona {} aligns with theme category {}", 
                journey.persona.as_str(), theme.category.as_str()));
        }
        
        // Check for keyword overlap
        let journey_keywords = self.extract_keywords(&journey.description);
        let theme_keywords = self.extract_keywords(&theme.description);
        let overlap = self.calculate_keyword_overlap(&journey_keywords, &theme_keywords);
        
        if overlap > 0.2 {
            confidence_score += overlap * 0.4;
            evidence.push(format!("Keyword overlap: {:.2}", overlap));
        }
        
        // Check if journey implements theme
        if self.journey_implements_theme(journey, theme) {
            relationship_type = RelationshipType::Implements;
            confidence_score += 0.4;
            evidence.push("Journey implements strategic theme".to_string());
        }
        
        // Only create reference if confidence is above threshold
        if confidence_score > 0.5 {
            let source_ref = EntityReference {
                entity_type: EntityType::UserJourney,
                entity_id: journey.id.0.to_string(),
                title: journey.title.clone(),
                description: journey.description.clone(),
            };
            
            let target_ref = EntityReference {
                entity_type: EntityType::StrategicTheme,
                entity_id: theme.id.0.to_string(),
                title: theme.title.clone(),
                description: theme.description.clone(),
            };
            
            let mut cross_ref = CrossReference::new(
                source_ref,
                target_ref,
                relationship_type,
                confidence_score,
            );
            cross_ref.evidence = evidence;
            
            return Ok(Some(cross_ref));
        }
        
        Ok(None)
    }
    
    fn analyze_journey_journey_relationship(
        &self,
        journey1: &UserJourney,
        journey2: &UserJourney,
    ) -> Result<Option<CrossReference>> {
        let mut confidence_score = 0.0;
        let mut evidence = Vec::new();
        let mut relationship_type = RelationshipType::RelatedTo;
        
        // Same persona and workflow type = part of same workflow
        if journey1.persona == journey2.persona && journey1.workflow_type == journey2.workflow_type {
            relationship_type = RelationshipType::PartOfWorkflow;
            confidence_score += 0.5;
            evidence.push("Same persona and workflow type".to_string());
        }
        
        // Check for dependency relationships
        if self.journey_depends_on_journey(journey1, journey2) {
            relationship_type = RelationshipType::DependsOn;
            confidence_score += 0.4;
            evidence.push("Journey dependency detected".to_string());
        }
        
        // Check for enhancement relationships
        if self.journey_enhances_journey(journey1, journey2) {
            relationship_type = RelationshipType::Enhances;
            confidence_score += 0.3;
            evidence.push("Journey enhancement detected".to_string());
        }
        
        // Check for tool overlap
        let tool_overlap = self.calculate_tool_overlap(&journey1.integration_tools, &journey2.integration_tools);
        if tool_overlap > 0.3 {
            confidence_score += tool_overlap * 0.2;
            evidence.push(format!("Tool overlap: {:.2}", tool_overlap));
        }
        
        if confidence_score > 0.4 {
            let source_ref = EntityReference {
                entity_type: EntityType::UserJourney,
                entity_id: journey1.id.0.to_string(),
                title: journey1.title.clone(),
                description: journey1.description.clone(),
            };
            
            let target_ref = EntityReference {
                entity_type: EntityType::UserJourney,
                entity_id: journey2.id.0.to_string(),
                title: journey2.title.clone(),
                description: journey2.description.clone(),
            };
            
            let mut cross_ref = CrossReference::new(
                source_ref,
                target_ref,
                relationship_type,
                confidence_score,
            );
            cross_ref.evidence = evidence;
            
            return Ok(Some(cross_ref));
        }
        
        Ok(None)
    }
    
    fn analyze_theme_theme_relationship(
        &self,
        theme1: &StrategicTheme,
        theme2: &StrategicTheme,
    ) -> Result<Option<CrossReference>> {
        let mut confidence_score = 0.0;
        let mut evidence = Vec::new();
        let mut relationship_type = RelationshipType::RelatedTo;
        
        // Same category = shares theme
        if theme1.category == theme2.category {
            relationship_type = RelationshipType::SharesTheme;
            confidence_score += 0.4;
            evidence.push("Same strategic category".to_string());
        }
        
        // Check for support relationships
        if self.theme_supports_theme(theme1, theme2) {
            relationship_type = RelationshipType::Supports;
            confidence_score += 0.3;
            evidence.push("Theme support relationship detected".to_string());
        }
        
        // Check for contradictions
        if self.theme_contradicts_theme(theme1, theme2) {
            relationship_type = RelationshipType::Contradicts;
            confidence_score += 0.6; // High confidence for contradictions
            evidence.push("Theme contradiction detected".to_string());
        }
        
        // Check competitive advantage overlap
        let advantage_overlap = self.calculate_competitive_advantage_overlap(theme1, theme2);
        if advantage_overlap > 0.2 {
            confidence_score += advantage_overlap * 0.3;
            evidence.push(format!("Competitive advantage overlap: {:.2}", advantage_overlap));
        }
        
        if confidence_score > 0.3 {
            let source_ref = EntityReference {
                entity_type: EntityType::StrategicTheme,
                entity_id: theme1.id.0.to_string(),
                title: theme1.title.clone(),
                description: theme1.description.clone(),
            };
            
            let target_ref = EntityReference {
                entity_type: EntityType::StrategicTheme,
                entity_id: theme2.id.0.to_string(),
                title: theme2.title.clone(),
                description: theme2.description.clone(),
            };
            
            let mut cross_ref = CrossReference::new(
                source_ref,
                target_ref,
                relationship_type,
                confidence_score,
            );
            cross_ref.evidence = evidence;
            
            return Ok(Some(cross_ref));
        }
        
        Ok(None)
    }
    
    fn create_chunk_journey_reference(
        &self,
        chunk: &ContentChunk,
        journey: &UserJourney,
    ) -> Result<Option<CrossReference>> {
        let source_ref = EntityReference {
            entity_type: EntityType::ContentChunk,
            entity_id: chunk.id.0.to_string(),
            title: format!("Chunk {}-{}", chunk.line_range.start, chunk.line_range.end),
            description: chunk.content.chars().take(100).collect::<String>() + "...",
        };
        
        let target_ref = EntityReference {
            entity_type: EntityType::UserJourney,
            entity_id: journey.id.0.to_string(),
            title: journey.title.clone(),
            description: journey.description.clone(),
        };
        
        let mut cross_ref = CrossReference::new(
            source_ref,
            target_ref,
            RelationshipType::Supports,
            0.9, // High confidence since journey was extracted from this chunk
        );
        cross_ref.evidence = vec!["Journey extracted from this content chunk".to_string()];
        cross_ref.source_chunks = vec![chunk.id];
        
        Ok(Some(cross_ref))
    }
    
    fn create_chunk_theme_reference(
        &self,
        chunk: &ContentChunk,
        theme: &StrategicTheme,
    ) -> Result<Option<CrossReference>> {
        let source_ref = EntityReference {
            entity_type: EntityType::ContentChunk,
            entity_id: chunk.id.0.to_string(),
            title: format!("Chunk {}-{}", chunk.line_range.start, chunk.line_range.end),
            description: chunk.content.chars().take(100).collect::<String>() + "...",
        };
        
        let target_ref = EntityReference {
            entity_type: EntityType::StrategicTheme,
            entity_id: theme.id.0.to_string(),
            title: theme.title.clone(),
            description: theme.description.clone(),
        };
        
        let mut cross_ref = CrossReference::new(
            source_ref,
            target_ref,
            RelationshipType::Supports,
            0.9, // High confidence since theme was extracted from this chunk
        );
        cross_ref.evidence = vec!["Strategic theme extracted from this content chunk".to_string()];
        cross_ref.source_chunks = vec![chunk.id];
        
        Ok(Some(cross_ref))
    }
    
    // Pattern building methods
    fn build_relationship_patterns() -> HashMap<RelationshipType, Vec<String>> {
        let mut patterns = HashMap::new();
        
        patterns.insert(RelationshipType::Supports, vec![
            "enables".to_string(),
            "facilitates".to_string(),
            "supports".to_string(),
            "helps".to_string(),
            "assists".to_string(),
            "provides".to_string(),
        ]);
        
        patterns.insert(RelationshipType::DependsOn, vec![
            "requires".to_string(),
            "needs".to_string(),
            "depends on".to_string(),
            "relies on".to_string(),
            "prerequisite".to_string(),
            "based on".to_string(),
        ]);
        
        patterns.insert(RelationshipType::Contradicts, vec![
            "conflicts with".to_string(),
            "contradicts".to_string(),
            "opposes".to_string(),
            "incompatible".to_string(),
            "mutually exclusive".to_string(),
        ]);
        
        patterns.insert(RelationshipType::Implements, vec![
            "implements".to_string(),
            "realizes".to_string(),
            "executes".to_string(),
            "delivers".to_string(),
            "achieves".to_string(),
        ]);
        
        patterns.insert(RelationshipType::Enhances, vec![
            "enhances".to_string(),
            "improves".to_string(),
            "optimizes".to_string(),
            "augments".to_string(),
            "extends".to_string(),
        ]);
        
        patterns
    }
    
    fn build_quality_patterns() -> HashMap<QualityIssueType, Vec<String>> {
        let mut patterns = HashMap::new();
        
        patterns.insert(QualityIssueType::Gap, vec![
            "missing".to_string(),
            "incomplete".to_string(),
            "undefined".to_string(),
            "unclear".to_string(),
            "to be defined".to_string(),
        ]);
        
        patterns.insert(QualityIssueType::Contradiction, vec![
            "however".to_string(),
            "but".to_string(),
            "contradicts".to_string(),
            "conflicts".to_string(),
            "opposite".to_string(),
        ]);
        
        patterns.insert(QualityIssueType::Inconsistency, vec![
            "inconsistent".to_string(),
            "varies".to_string(),
            "different".to_string(),
            "conflicting".to_string(),
        ]);
        
        patterns
    }
}

/// Prioritized item with strategic impact and feasibility scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrioritizedItem {
    pub entity_reference: EntityReference,
    pub strategic_impact: ImpactLevel,
    pub implementation_feasibility: ComplexityLevel,
    pub priority_score: f64,
    pub supporting_evidence: Vec<String>,
}
impl C
rossReferenceSynthesizer {
    // Helper methods for analysis
    fn persona_aligns_with_theme_category(&self, persona: &DeveloperPersona, category: &StrategicThemeCategory) -> bool {
        match (persona, category) {
            (DeveloperPersona::IndividualDeveloper, StrategicThemeCategory::DeveloperProductivity) => true,
            (DeveloperPersona::TeamLead, StrategicThemeCategory::Community) => true,
            (DeveloperPersona::DevOpsEngineer, StrategicThemeCategory::Performance) => true,
            (DeveloperPersona::DevOpsEngineer, StrategicThemeCategory::Security) => true,
            (DeveloperPersona::PlatformEngineer, StrategicThemeCategory::EcosystemIntegration) => true,
            (DeveloperPersona::PlatformEngineer, StrategicThemeCategory::Performance) => true,
            (_, StrategicThemeCategory::AiEnhancement) => true, // AI enhancement applies to all personas
            _ => false,
        }
    }
    
    fn extract_keywords(&self, text: &str) -> HashSet<String> {
        text.to_lowercase()
            .split_whitespace()
            .filter(|word| word.len() > 3)
            .filter(|word| !self.is_stop_word(word))
            .map(|word| word.to_string())
            .collect()
    }
    
    fn is_stop_word(&self, word: &str) -> bool {
        matches!(word, "the" | "and" | "or" | "but" | "in" | "on" | "at" | "to" | "for" | "of" | "with" | "by" | "from" | "this" | "that" | "these" | "those" | "a" | "an")
    }
    
    fn calculate_keyword_overlap(&self, keywords1: &HashSet<String>, keywords2: &HashSet<String>) -> f64 {
        if keywords1.is_empty() || keywords2.is_empty() {
            return 0.0;
        }
        
        let intersection_size = keywords1.intersection(keywords2).count();
        let union_size = keywords1.union(keywords2).count();
        
        intersection_size as f64 / union_size as f64
    }
    
    fn journey_implements_theme(&self, journey: &UserJourney, theme: &StrategicTheme) -> bool {
        // Check if journey's solutions align with theme's competitive advantages
        for solution in &journey.solutions {
            for advantage in &theme.competitive_advantages {
                if self.text_similarity(&solution.description, &advantage.description) > 0.3 {
                    return true;
                }
            }
        }
        
        // Check if journey's workflow type aligns with theme category
        match (&journey.workflow_type, &theme.category) {
            (WorkflowType::Development, StrategicThemeCategory::DeveloperProductivity) => true,
            (WorkflowType::LlmIntegration, StrategicThemeCategory::AiEnhancement) => true,
            (WorkflowType::Security, StrategicThemeCategory::Security) => true,
            (WorkflowType::ArchitectureAnalysis, StrategicThemeCategory::EcosystemIntegration) => true,
            _ => false,
        }
    }
    
    fn journey_depends_on_journey(&self, journey1: &UserJourney, journey2: &UserJourney) -> bool {
        // Check if journey1's prerequisites mention journey2's outcomes
        for prerequisite in &journey1.prerequisites {
            for solution in &journey2.solutions {
                for outcome in &solution.expected_outcomes {
                    if self.text_similarity(prerequisite, outcome) > 0.4 {
                        return true;
                    }
                }
            }
        }
        
        // Check workflow sequence dependencies
        match (&journey1.workflow_type, &journey2.workflow_type) {
            (WorkflowType::CiCd, WorkflowType::Development) => true,
            (WorkflowType::Testing, WorkflowType::Development) => true,
            (WorkflowType::Security, WorkflowType::Development) => true,
            _ => false,
        }
    }
    
    fn journey_enhances_journey(&self, journey1: &UserJourney, journey2: &UserJourney) -> bool {
        // Check if journey1's solutions enhance journey2's outcomes
        for solution1 in &journey1.solutions {
            for solution2 in &journey2.solutions {
                if solution1.description.to_lowercase().contains("enhance") ||
                   solution1.description.to_lowercase().contains("improve") ||
                   solution1.description.to_lowercase().contains("optimize") {
                    if self.text_similarity(&solution1.description, &solution2.description) > 0.2 {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    fn calculate_tool_overlap(&self, tools1: &[String], tools2: &[String]) -> f64 {
        if tools1.is_empty() || tools2.is_empty() {
            return 0.0;
        }
        
        let set1: HashSet<_> = tools1.iter().collect();
        let set2: HashSet<_> = tools2.iter().collect();
        
        let intersection_size = set1.intersection(&set2).count();
        let union_size = set1.union(&set2).count();
        
        intersection_size as f64 / union_size as f64
    }
    
    fn theme_supports_theme(&self, theme1: &StrategicTheme, theme2: &StrategicTheme) -> bool {
        // Check if theme1's competitive advantages support theme2's goals
        for advantage1 in &theme1.competitive_advantages {
            for advantage2 in &theme2.competitive_advantages {
                if advantage1.description.to_lowercase().contains("enable") ||
                   advantage1.description.to_lowercase().contains("support") ||
                   advantage1.description.to_lowercase().contains("facilitate") {
                    if self.text_similarity(&advantage1.description, &advantage2.description) > 0.2 {
                        return true;
                    }
                }
            }
        }
        
        // Check category relationships
        match (&theme1.category, &theme2.category) {
            (StrategicThemeCategory::Performance, StrategicThemeCategory::DeveloperProductivity) => true,
            (StrategicThemeCategory::Security, StrategicThemeCategory::EcosystemIntegration) => true,
            (StrategicThemeCategory::AiEnhancement, _) => true, // AI enhances everything
            _ => false,
        }
    }
    
    fn theme_contradicts_theme(&self, theme1: &StrategicTheme, theme2: &StrategicTheme) -> bool {
        // Check for contradictory competitive advantages
        for advantage1 in &theme1.competitive_advantages {
            for advantage2 in &theme2.competitive_advantages {
                if self.detect_contradiction(&advantage1.description, &advantage2.description) {
                    return true;
                }
            }
        }
        
        // Check for contradictory ROI metrics
        for roi1 in &theme1.roi_metrics {
            for roi2 in &theme2.roi_metrics {
                if roi1.metric_name == roi2.metric_name {
                    if self.detect_metric_contradiction(&roi1.projected_value, &roi2.projected_value) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    fn calculate_competitive_advantage_overlap(&self, theme1: &StrategicTheme, theme2: &StrategicTheme) -> f64 {
        if theme1.competitive_advantages.is_empty() || theme2.competitive_advantages.is_empty() {
            return 0.0;
        }
        
        let mut total_similarity = 0.0;
        let mut comparisons = 0;
        
        for advantage1 in &theme1.competitive_advantages {
            for advantage2 in &theme2.competitive_advantages {
                total_similarity += self.text_similarity(&advantage1.description, &advantage2.description);
                comparisons += 1;
            }
        }
        
        if comparisons > 0 {
            total_similarity / comparisons as f64
        } else {
            0.0
        }
    }
    
    fn text_similarity(&self, text1: &str, text2: &str) -> f64 {
        let words1 = self.extract_keywords(text1);
        let words2 = self.extract_keywords(text2);
        self.calculate_keyword_overlap(&words1, &words2)
    }
    
    fn detect_contradiction(&self, text1: &str, text2: &str) -> bool {
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();
        
        // Simple contradiction detection patterns
        let contradiction_pairs = vec![
            ("fast", "slow"),
            ("increase", "decrease"),
            ("improve", "worsen"),
            ("enable", "disable"),
            ("secure", "insecure"),
            ("efficient", "inefficient"),
            ("simple", "complex"),
            ("automated", "manual"),
        ];
        
        for (word1, word2) in contradiction_pairs {
            if (text1_lower.contains(word1) && text2_lower.contains(word2)) ||
               (text1_lower.contains(word2) && text2_lower.contains(word1)) {
                return true;
            }
        }
        
        false
    }
    
    fn detect_metric_contradiction(&self, value1: &str, value2: &str) -> bool {
        // Simple numeric contradiction detection
        // This is a placeholder - in practice, you'd want more sophisticated parsing
        value1.to_lowercase().contains("increase") && value2.to_lowercase().contains("decrease") ||
        value1.to_lowercase().contains("faster") && value2.to_lowercase().contains("slower") ||
        value1.to_lowercase().contains("more") && value2.to_lowercase().contains("less")
    }
    
    // Narrative synthesis methods
    fn synthesize_narrative_from_journeys(
        &self,
        persona: &DeveloperPersona,
        workflow_type: &WorkflowType,
        journeys: &[&UserJourney],
        themes: &[StrategicTheme],
    ) -> Result<WorkflowNarrative> {
        let narrative_id = Uuid::new_v4();
        
        // Generate title and description
        let title = format!("{} {} Workflow", persona.as_str(), workflow_type.as_str());
        let description = self.generate_narrative_description(journeys)?;
        
        // Extract related journey IDs
        let related_journeys: Vec<UserJourneyId> = journeys.iter().map(|j| j.id).collect();
        
        // Find supporting themes
        let supporting_themes = self.find_supporting_themes(journeys, themes)?;
        
        // Generate narrative flow
        let narrative_flow = self.generate_narrative_flow(journeys)?;
        
        // Extract integration points
        let integration_points = self.extract_integration_points(journeys)?;
        
        // Generate success criteria
        let success_criteria = self.generate_success_criteria(journeys)?;
        
        // Calculate complexity and impact
        let implementation_complexity = self.calculate_narrative_complexity(journeys)?;
        let strategic_impact = self.calculate_narrative_impact(journeys, themes)?;
        
        Ok(WorkflowNarrative {
            id: narrative_id,
            title,
            description,
            persona: persona.clone(),
            workflow_type: workflow_type.clone(),
            related_journeys,
            supporting_themes,
            narrative_flow,
            integration_points,
            success_criteria,
            implementation_complexity,
            strategic_impact,
            created_at: Utc::now(),
        })
    }
    
    fn generate_narrative_description(&self, journeys: &[&UserJourney]) -> Result<String> {
        let mut description_parts = Vec::new();
        
        // Combine journey descriptions
        for journey in journeys {
            description_parts.push(journey.description.clone());
        }
        
        // Create coherent narrative
        let combined = description_parts.join(" ");
        let words: Vec<&str> = combined.split_whitespace().take(50).collect();
        
        Ok(words.join(" ") + "...")
    }
    
    fn find_supporting_themes(&self, journeys: &[&UserJourney], themes: &[StrategicTheme]) -> Result<Vec<StrategyThemeId>> {
        let mut supporting_themes = Vec::new();
        
        for theme in themes {
            let mut support_score = 0.0;
            
            for journey in journeys {
                // Check if theme supports this journey
                if self.persona_aligns_with_theme_category(&journey.persona, &theme.category) {
                    support_score += 0.3;
                }
                
                // Check keyword overlap
                let journey_keywords = self.extract_keywords(&journey.description);
                let theme_keywords = self.extract_keywords(&theme.description);
                let overlap = self.calculate_keyword_overlap(&journey_keywords, &theme_keywords);
                support_score += overlap * 0.4;
            }
            
            if support_score > 0.5 {
                supporting_themes.push(theme.id);
            }
        }
        
        Ok(supporting_themes)
    }
    
    fn generate_narrative_flow(&self, journeys: &[&UserJourney]) -> Result<Vec<NarrativeStep>> {
        let mut steps = Vec::new();
        
        // Sort journeys by complexity (simpler first)
        let mut sorted_journeys = journeys.to_vec();
        sorted_journeys.sort_by(|a, b| {
            let complexity_a = a.solutions.iter().map(|s| s.implementation_complexity.score()).sum::<u8>();
            let complexity_b = b.solutions.iter().map(|s| s.implementation_complexity.score()).sum::<u8>();
            complexity_a.cmp(&complexity_b)
        });
        
        for (idx, journey) in sorted_journeys.iter().enumerate() {
            let step = NarrativeStep {
                step_number: idx + 1,
                title: journey.title.clone(),
                description: journey.description.clone(),
                related_entities: vec![EntityReference {
                    entity_type: EntityType::UserJourney,
                    entity_id: journey.id.0.to_string(),
                    title: journey.title.clone(),
                    description: journey.description.clone(),
                }],
                prerequisites: journey.prerequisites.clone(),
                outcomes: journey.solutions.iter()
                    .flat_map(|s| s.expected_outcomes.clone())
                    .collect(),
                tools_involved: journey.integration_tools.clone(),
            };
            steps.push(step);
        }
        
        Ok(steps)
    }
    
    fn extract_integration_points(&self, journeys: &[&UserJourney]) -> Result<Vec<IntegrationPoint>> {
        let mut integration_points = Vec::new();
        let mut tool_groups: HashMap<String, Vec<&UserJourney>> = HashMap::new();
        
        // Group journeys by shared tools
        for journey in journeys {
            for tool in &journey.integration_tools {
                tool_groups.entry(tool.clone()).or_insert_with(Vec::new).push(journey);
            }
        }
        
        // Create integration points for shared tools
        for (tool, tool_journeys) in tool_groups {
            if tool_journeys.len() > 1 {
                let integration_point = IntegrationPoint {
                    name: format!("{} Integration", tool),
                    description: format!("Integration point for {} across multiple workflows", tool),
                    integration_type: self.determine_integration_type(&tool),
                    involved_systems: vec![tool.clone()],
                    data_flow: self.extract_data_flow(&tool_journeys)?,
                    complexity: self.determine_integration_complexity(&tool_journeys),
                };
                integration_points.push(integration_point);
            }
        }
        
        Ok(integration_points)
    }
    
    fn generate_success_criteria(&self, journeys: &[&UserJourney]) -> Result<Vec<String>> {
        let mut criteria = Vec::new();
        
        for journey in journeys {
            // Extract performance improvements
            for perf_metric in &journey.success_metrics.performance_improvements {
                criteria.push(format!("{}: {}", perf_metric.metric_name, perf_metric.target_value));
            }
            
            // Extract productivity gains
            for prod_metric in &journey.success_metrics.productivity_gains {
                criteria.push(format!("{}: {}", prod_metric.metric_name, prod_metric.improved_state));
            }
            
            // Extract quality improvements
            for quality_metric in &journey.success_metrics.quality_improvements {
                criteria.push(format!("{}: {}", quality_metric.metric_name, quality_metric.improvement_description));
            }
        }
        
        // Remove duplicates
        criteria.sort();
        criteria.dedup();
        
        Ok(criteria)
    }
    
    fn calculate_narrative_complexity(&self, journeys: &[&UserJourney]) -> Result<ComplexityLevel> {
        let total_complexity: u8 = journeys.iter()
            .flat_map(|j| j.solutions.iter())
            .map(|s| s.implementation_complexity.score())
            .sum();
        
        let avg_complexity = total_complexity as f64 / journeys.len() as f64;
        
        Ok(match avg_complexity {
            x if x <= 1.5 => ComplexityLevel::Low,
            x if x <= 2.5 => ComplexityLevel::Medium,
            x if x <= 3.5 => ComplexityLevel::High,
            _ => ComplexityLevel::VeryHigh,
        })
    }
    
    fn calculate_narrative_impact(&self, journeys: &[&UserJourney], themes: &[StrategicTheme]) -> Result<ImpactLevel> {
        let mut impact_score = 0.0;
        
        // Base impact from number of journeys
        impact_score += journeys.len() as f64 * 0.2;
        
        // Impact from supporting themes
        let supporting_themes = self.find_supporting_themes(journeys, themes)?;
        impact_score += supporting_themes.len() as f64 * 0.3;
        
        // Impact from journey confidence scores
        let avg_confidence: f64 = journeys.iter().map(|j| j.confidence_score).sum::<f64>() / journeys.len() as f64;
        impact_score += avg_confidence * 0.5;
        
        Ok(match impact_score {
            x if x <= 1.0 => ImpactLevel::Low,
            x if x <= 2.0 => ImpactLevel::Medium,
            x if x <= 3.0 => ImpactLevel::High,
            _ => ImpactLevel::Critical,
        })
    }
    
    fn determine_integration_type(&self, tool: &str) -> IntegrationType {
        let tool_lower = tool.to_lowercase();
        
        if tool_lower.contains("api") || tool_lower.contains("rest") || tool_lower.contains("graphql") {
            IntegrationType::ApiIntegration
        } else if tool_lower.contains("plugin") || tool_lower.contains("extension") {
            IntegrationType::PluginSystem
        } else if tool_lower.contains("pipeline") || tool_lower.contains("stream") {
            IntegrationType::DataPipeline
        } else if tool_lower.contains("workflow") || tool_lower.contains("orchestrat") {
            IntegrationType::WorkflowOrchestration
        } else if tool_lower.contains("event") || tool_lower.contains("message") {
            IntegrationType::EventDriven
        } else {
            IntegrationType::DirectIntegration
        }
    }
    
    fn extract_data_flow(&self, journeys: &[&UserJourney]) -> Result<Vec<String>> {
        let mut data_flows = Vec::new();
        
        for journey in journeys {
            for solution in &journey.solutions {
                if solution.description.to_lowercase().contains("data") ||
                   solution.description.to_lowercase().contains("flow") ||
                   solution.description.to_lowercase().contains("process") {
                    data_flows.push(solution.description.clone());
                }
            }
        }
        
        Ok(data_flows)
    }
    
    fn determine_integration_complexity(&self, journeys: &[&UserJourney]) -> ComplexityLevel {
        let complexity_sum: u8 = journeys.iter()
            .flat_map(|j| j.solutions.iter())
            .map(|s| s.implementation_complexity.score())
            .sum();
        
        let avg_complexity = complexity_sum as f64 / journeys.len() as f64;
        
        match avg_complexity {
            x if x <= 1.5 => ComplexityLevel::Low,
            x if x <= 2.5 => ComplexityLevel::Medium,
            x if x <= 3.5 => ComplexityLevel::High,
            _ => ComplexityLevel::VeryHigh,
        }
    }
}    // Qu
ality assurance methods
    fn detect_gaps(&self, journeys: &[UserJourney], themes: &[StrategicTheme], chunks: &[ContentChunk]) -> Result<Vec<QualityIssue>> {
        let mut gaps = Vec::new();
        
        // Check for journeys without solutions
        for journey in journeys {
            if journey.solutions.is_empty() {
                gaps.push(QualityIssue {
                    id: Uuid::new_v4(),
                    issue_type: QualityIssueType::Gap,
                    severity: IssueSeverity::Warning,
                    description: format!("Journey '{}' has no solutions defined", journey.title),
                    affected_entities: vec![EntityReference {
                        entity_type: EntityType::UserJourney,
                        entity_id: journey.id.0.to_string(),
                        title: journey.title.clone(),
                        description: journey.description.clone(),
                    }],
                    evidence: vec!["Empty solutions array".to_string()],
                    suggested_resolution: Some("Add solution descriptions and implementation details".to_string()),
                    source_chunks: journey.source_chunks.clone(),
                    detected_at: Utc::now(),
                });
            }
        }
        
        // Check for themes without competitive advantages
        for theme in themes {
            if theme.competitive_advantages.is_empty() {
                gaps.push(QualityIssue {
                    id: Uuid::new_v4(),
                    issue_type: QualityIssueType::Gap,
                    severity: IssueSeverity::Warning,
                    description: format!("Strategic theme '{}' has no competitive advantages defined", theme.title),
                    affected_entities: vec![EntityReference {
                        entity_type: EntityType::StrategicTheme,
                        entity_id: theme.id.0.to_string(),
                        title: theme.title.clone(),
                        description: theme.description.clone(),
                    }],
                    evidence: vec!["Empty competitive advantages array".to_string()],
                    suggested_resolution: Some("Define competitive advantages and value propositions".to_string()),
                    source_chunks: theme.source_chunks.clone(),
                    detected_at: Utc::now(),
                });
            }
        }
        
        // Check for missing success metrics
        for journey in journeys {
            if journey.success_metrics.performance_improvements.is_empty() &&
               journey.success_metrics.productivity_gains.is_empty() &&
               journey.success_metrics.quality_improvements.is_empty() {
                gaps.push(QualityIssue {
                    id: Uuid::new_v4(),
                    issue_type: QualityIssueType::Gap,
                    severity: IssueSeverity::Error,
                    description: format!("Journey '{}' has no success metrics defined", journey.title),
                    affected_entities: vec![EntityReference {
                        entity_type: EntityType::UserJourney,
                        entity_id: journey.id.0.to_string(),
                        title: journey.title.clone(),
                        description: journey.description.clone(),
                    }],
                    evidence: vec!["All success metrics arrays are empty".to_string()],
                    suggested_resolution: Some("Define measurable success criteria and performance targets".to_string()),
                    source_chunks: journey.source_chunks.clone(),
                    detected_at: Utc::now(),
                });
            }
        }
        
        Ok(gaps)
    }
    
    fn detect_contradictions(&self, journeys: &[UserJourney], themes: &[StrategicTheme]) -> Result<Vec<QualityIssue>> {
        let mut contradictions = Vec::new();
        
        // Check for contradictory solutions between journeys
        for (i, journey1) in journeys.iter().enumerate() {
            for journey2 in journeys.iter().skip(i + 1) {
                for solution1 in &journey1.solutions {
                    for solution2 in &journey2.solutions {
                        if self.detect_contradiction(&solution1.description, &solution2.description) {
                            contradictions.push(QualityIssue {
                                id: Uuid::new_v4(),
                                issue_type: QualityIssueType::Contradiction,
                                severity: IssueSeverity::Error,
                                description: format!("Contradictory solutions between journeys '{}' and '{}'", 
                                    journey1.title, journey2.title),
                                affected_entities: vec![
                                    EntityReference {
                                        entity_type: EntityType::UserJourney,
                                        entity_id: journey1.id.0.to_string(),
                                        title: journey1.title.clone(),
                                        description: journey1.description.clone(),
                                    },
                                    EntityReference {
                                        entity_type: EntityType::UserJourney,
                                        entity_id: journey2.id.0.to_string(),
                                        title: journey2.title.clone(),
                                        description: journey2.description.clone(),
                                    },
                                ],
                                evidence: vec![
                                    format!("Solution 1: {}", solution1.description),
                                    format!("Solution 2: {}", solution2.description),
                                ],
                                suggested_resolution: Some("Review and reconcile contradictory approaches".to_string()),
                                source_chunks: [journey1.source_chunks.clone(), journey2.source_chunks.clone()].concat(),
                                detected_at: Utc::now(),
                            });
                        }
                    }
                }
            }
        }
        
        // Check for contradictory themes
        for (i, theme1) in themes.iter().enumerate() {
            for theme2 in themes.iter().skip(i + 1) {
                if self.theme_contradicts_theme(theme1, theme2) {
                    contradictions.push(QualityIssue {
                        id: Uuid::new_v4(),
                        issue_type: QualityIssueType::Contradiction,
                        severity: IssueSeverity::Warning,
                        description: format!("Contradictory strategic themes: '{}' and '{}'", 
                            theme1.title, theme2.title),
                        affected_entities: vec![
                            EntityReference {
                                entity_type: EntityType::StrategicTheme,
                                entity_id: theme1.id.0.to_string(),
                                title: theme1.title.clone(),
                                description: theme1.description.clone(),
                            },
                            EntityReference {
                                entity_type: EntityType::StrategicTheme,
                                entity_id: theme2.id.0.to_string(),
                                title: theme2.title.clone(),
                                description: theme2.description.clone(),
                            },
                        ],
                        evidence: vec!["Contradictory competitive advantages or ROI metrics detected".to_string()],
                        suggested_resolution: Some("Clarify strategic positioning and resolve conflicts".to_string()),
                        source_chunks: [theme1.source_chunks.clone(), theme2.source_chunks.clone()].concat(),
                        detected_at: Utc::now(),
                    });
                }
            }
        }
        
        Ok(contradictions)
    }
    
    fn detect_inconsistencies(&self, journeys: &[UserJourney], themes: &[StrategicTheme]) -> Result<Vec<QualityIssue>> {
        let mut inconsistencies = Vec::new();
        
        // Check for inconsistent persona classifications
        let mut persona_keywords: HashMap<DeveloperPersona, HashSet<String>> = HashMap::new();
        
        for journey in journeys {
            let keywords = self.extract_keywords(&journey.description);
            persona_keywords.entry(journey.persona.clone()).or_insert_with(HashSet::new).extend(keywords);
        }
        
        // Look for journeys that might be misclassified
        for journey in journeys {
            let journey_keywords = self.extract_keywords(&journey.description);
            
            for (persona, persona_keyword_set) in &persona_keywords {
                if persona != &journey.persona {
                    let overlap = self.calculate_keyword_overlap(&journey_keywords, persona_keyword_set);
                    if overlap > 0.6 {
                        inconsistencies.push(QualityIssue {
                            id: Uuid::new_v4(),
                            issue_type: QualityIssueType::Inconsistency,
                            severity: IssueSeverity::Warning,
                            description: format!("Journey '{}' may be misclassified as {} instead of {}", 
                                journey.title, journey.persona.as_str(), persona.as_str()),
                            affected_entities: vec![EntityReference {
                                entity_type: EntityType::UserJourney,
                                entity_id: journey.id.0.to_string(),
                                title: journey.title.clone(),
                                description: journey.description.clone(),
                            }],
                            evidence: vec![format!("High keyword overlap ({:.2}) with {} persona", overlap, persona.as_str())],
                            suggested_resolution: Some("Review persona classification".to_string()),
                            source_chunks: journey.source_chunks.clone(),
                            detected_at: Utc::now(),
                        });
                    }
                }
            }
        }
        
        // Check for inconsistent theme categorization
        for theme in themes {
            let theme_keywords = self.extract_keywords(&theme.description);
            
            for category in StrategicThemeCategory::all() {
                if category != theme.category {
                    if let Some(category_patterns) = self.category_patterns.get(&category) {
                        let category_keyword_set: HashSet<String> = category_patterns.iter().cloned().collect();
                        let overlap = self.calculate_keyword_overlap(&theme_keywords, &category_keyword_set);
                        
                        if overlap > 0.5 {
                            inconsistencies.push(QualityIssue {
                                id: Uuid::new_v4(),
                                issue_type: QualityIssueType::Inconsistency,
                                severity: IssueSeverity::Info,
                                description: format!("Theme '{}' may belong to {} category instead of {}", 
                                    theme.title, category.as_str(), theme.category.as_str()),
                                affected_entities: vec![EntityReference {
                                    entity_type: EntityType::StrategicTheme,
                                    entity_id: theme.id.0.to_string(),
                                    title: theme.title.clone(),
                                    description: theme.description.clone(),
                                }],
                                evidence: vec![format!("High keyword overlap ({:.2}) with {} category", overlap, category.as_str())],
                                suggested_resolution: Some("Review theme categorization".to_string()),
                                source_chunks: theme.source_chunks.clone(),
                                detected_at: Utc::now(),
                            });
                        }
                    }
                }
            }
        }
        
        Ok(inconsistencies)
    }
    
    fn detect_duplications(&self, journeys: &[UserJourney], themes: &[StrategicTheme]) -> Result<Vec<QualityIssue>> {
        let mut duplications = Vec::new();
        
        // Check for duplicate journeys
        for (i, journey1) in journeys.iter().enumerate() {
            for journey2 in journeys.iter().skip(i + 1) {
                let similarity = self.text_similarity(&journey1.description, &journey2.description);
                if similarity > 0.8 {
                    duplications.push(QualityIssue {
                        id: Uuid::new_v4(),
                        issue_type: QualityIssueType::Duplication,
                        severity: IssueSeverity::Warning,
                        description: format!("Potential duplicate journeys: '{}' and '{}'", 
                            journey1.title, journey2.title),
                        affected_entities: vec![
                            EntityReference {
                                entity_type: EntityType::UserJourney,
                                entity_id: journey1.id.0.to_string(),
                                title: journey1.title.clone(),
                                description: journey1.description.clone(),
                            },
                            EntityReference {
                                entity_type: EntityType::UserJourney,
                                entity_id: journey2.id.0.to_string(),
                                title: journey2.title.clone(),
                                description: journey2.description.clone(),
                            },
                        ],
                        evidence: vec![format!("High similarity score: {:.2}", similarity)],
                        suggested_resolution: Some("Merge duplicate journeys or clarify differences".to_string()),
                        source_chunks: [journey1.source_chunks.clone(), journey2.source_chunks.clone()].concat(),
                        detected_at: Utc::now(),
                    });
                }
            }
        }
        
        // Check for duplicate themes
        for (i, theme1) in themes.iter().enumerate() {
            for theme2 in themes.iter().skip(i + 1) {
                let similarity = self.text_similarity(&theme1.description, &theme2.description);
                if similarity > 0.8 {
                    duplications.push(QualityIssue {
                        id: Uuid::new_v4(),
                        issue_type: QualityIssueType::Duplication,
                        severity: IssueSeverity::Warning,
                        description: format!("Potential duplicate themes: '{}' and '{}'", 
                            theme1.title, theme2.title),
                        affected_entities: vec![
                            EntityReference {
                                entity_type: EntityType::StrategicTheme,
                                entity_id: theme1.id.0.to_string(),
                                title: theme1.title.clone(),
                                description: theme1.description.clone(),
                            },
                            EntityReference {
                                entity_type: EntityType::StrategicTheme,
                                entity_id: theme2.id.0.to_string(),
                                title: theme2.title.clone(),
                                description: theme2.description.clone(),
                            },
                        ],
                        evidence: vec![format!("High similarity score: {:.2}", similarity)],
                        suggested_resolution: Some("Merge duplicate themes or clarify differences".to_string()),
                        source_chunks: [theme1.source_chunks.clone(), theme2.source_chunks.clone()].concat(),
                        detected_at: Utc::now(),
                    });
                }
            }
        }
        
        Ok(duplications)
    }
    
    fn detect_missing_references(&self, journeys: &[UserJourney], themes: &[StrategicTheme]) -> Result<Vec<QualityIssue>> {
        let mut missing_refs = Vec::new();
        
        // Check for journeys that should be cross-referenced but aren't
        for journey in journeys {
            let mut has_theme_references = false;
            
            // Check if journey has any theme cross-references
            for cross_ref in self.cross_references.values() {
                if cross_ref.source_entity.entity_id == journey.id.0.to_string() &&
                   cross_ref.target_entity.entity_type == EntityType::StrategicTheme {
                    has_theme_references = true;
                    break;
                }
            }
            
            if !has_theme_references && !themes.is_empty() {
                missing_refs.push(QualityIssue {
                    id: Uuid::new_v4(),
                    issue_type: QualityIssueType::MissingReference,
                    severity: IssueSeverity::Info,
                    description: format!("Journey '{}' has no strategic theme references", journey.title),
                    affected_entities: vec![EntityReference {
                        entity_type: EntityType::UserJourney,
                        entity_id: journey.id.0.to_string(),
                        title: journey.title.clone(),
                        description: journey.description.clone(),
                    }],
                    evidence: vec!["No cross-references to strategic themes found".to_string()],
                    suggested_resolution: Some("Review and add relevant strategic theme connections".to_string()),
                    source_chunks: journey.source_chunks.clone(),
                    detected_at: Utc::now(),
                });
            }
        }
        
        Ok(missing_refs)
    }
    
    fn detect_low_confidence_issues(&self, journeys: &[UserJourney], themes: &[StrategicTheme]) -> Result<Vec<QualityIssue>> {
        let mut low_confidence_issues = Vec::new();
        
        // Check for low confidence journeys
        for journey in journeys {
            if journey.confidence_score < 0.5 {
                low_confidence_issues.push(QualityIssue {
                    id: Uuid::new_v4(),
                    issue_type: QualityIssueType::LowConfidence,
                    severity: IssueSeverity::Warning,
                    description: format!("Journey '{}' has low confidence score: {:.2}", 
                        journey.title, journey.confidence_score),
                    affected_entities: vec![EntityReference {
                        entity_type: EntityType::UserJourney,
                        entity_id: journey.id.0.to_string(),
                        title: journey.title.clone(),
                        description: journey.description.clone(),
                    }],
                    evidence: vec![format!("Confidence score: {:.2}", journey.confidence_score)],
                    suggested_resolution: Some("Review extraction quality and add more evidence".to_string()),
                    source_chunks: journey.source_chunks.clone(),
                    detected_at: Utc::now(),
                });
            }
        }
        
        // Check for low confidence themes
        for theme in themes {
            if theme.confidence_score < 0.5 {
                low_confidence_issues.push(QualityIssue {
                    id: Uuid::new_v4(),
                    issue_type: QualityIssueType::LowConfidence,
                    severity: IssueSeverity::Warning,
                    description: format!("Theme '{}' has low confidence score: {:.2}", 
                        theme.title, theme.confidence_score),
                    affected_entities: vec![EntityReference {
                        entity_type: EntityType::StrategicTheme,
                        entity_id: theme.id.0.to_string(),
                        title: theme.title.clone(),
                        description: theme.description.clone(),
                    }],
                    evidence: vec![format!("Confidence score: {:.2}", theme.confidence_score)],
                    suggested_resolution: Some("Review extraction quality and add more evidence".to_string()),
                    source_chunks: theme.source_chunks.clone(),
                    detected_at: Utc::now(),
                });
            }
        }
        
        Ok(low_confidence_issues)
    }
    
    // Prioritization methods
    fn calculate_journey_strategic_impact(&self, journey: &UserJourney, themes: &[StrategicTheme]) -> Result<ImpactLevel> {
        let mut impact_score = 0.0;
        
        // Base impact from confidence score
        impact_score += journey.confidence_score * 0.3;
        
        // Impact from number of solutions
        impact_score += (journey.solutions.len() as f64 * 0.1).min(0.5);
        
        // Impact from success metrics
        let metrics_count = journey.success_metrics.performance_improvements.len() +
                           journey.success_metrics.productivity_gains.len() +
                           journey.success_metrics.quality_improvements.len();
        impact_score += (metrics_count as f64 * 0.1).min(0.4);
        
        // Impact from theme connections
        let theme_connections = self.count_theme_connections(journey, themes);
        impact_score += (theme_connections as f64 * 0.2).min(0.6);
        
        // Impact from persona (some personas have higher strategic impact)
        match journey.persona {
            DeveloperPersona::PlatformEngineer => impact_score += 0.3,
            DeveloperPersona::TeamLead => impact_score += 0.2,
            DeveloperPersona::DevOpsEngineer => impact_score += 0.2,
            DeveloperPersona::IndividualDeveloper => impact_score += 0.1,
        }
        
        Ok(match impact_score {
            x if x <= 1.0 => ImpactLevel::Low,
            x if x <= 2.0 => ImpactLevel::Medium,
            x if x <= 3.0 => ImpactLevel::High,
            _ => ImpactLevel::Critical,
        })
    }
    
    fn calculate_journey_implementation_feasibility(&self, journey: &UserJourney) -> Result<ComplexityLevel> {
        let mut complexity_score = 0.0;
        
        // Complexity from solutions
        for solution in &journey.solutions {
            complexity_score += solution.implementation_complexity.score() as f64 * 0.3;
        }
        
        // Complexity from number of integration tools
        complexity_score += (journey.integration_tools.len() as f64 * 0.1).min(0.5);
        
        // Complexity from number of prerequisites
        complexity_score += (journey.prerequisites.len() as f64 * 0.1).min(0.4);
        
        // Complexity from workflow type
        match journey.workflow_type {
            WorkflowType::LlmIntegration => complexity_score += 0.4,
            WorkflowType::ArchitectureAnalysis => complexity_score += 0.3,
            WorkflowType::Security => complexity_score += 0.3,
            WorkflowType::CiCd => complexity_score += 0.2,
            WorkflowType::Testing => complexity_score += 0.2,
            WorkflowType::Development => complexity_score += 0.1,
        }
        
        Ok(match complexity_score {
            x if x <= 1.0 => ComplexityLevel::Low,
            x if x <= 2.0 => ComplexityLevel::Medium,
            x if x <= 3.0 => ComplexityLevel::High,
            _ => ComplexityLevel::VeryHigh,
        })
    }
    
    fn calculate_theme_strategic_impact(&self, theme: &StrategicTheme) -> Result<ImpactLevel> {
        let mut impact_score = 0.0;
        
        // Base impact from confidence score
        impact_score += theme.confidence_score * 0.3;
        
        // Impact from competitive advantages
        impact_score += (theme.competitive_advantages.len() as f64 * 0.2).min(0.6);
        
        // Impact from ROI metrics
        impact_score += (theme.roi_metrics.len() as f64 * 0.2).min(0.5);
        
        // Impact from adoption pathways
        impact_score += (theme.adoption_pathways.len() as f64 * 0.1).min(0.4);
        
        // Impact from implementation priority
        impact_score += theme.implementation_priority.score() as f64 * 0.2;
        
        // Impact from category (some categories have higher strategic impact)
        match theme.category {
            StrategicThemeCategory::AiEnhancement => impact_score += 0.4,
            StrategicThemeCategory::DeveloperProductivity => impact_score += 0.3,
            StrategicThemeCategory::EcosystemIntegration => impact_score += 0.3,
            StrategicThemeCategory::Performance => impact_score += 0.2,
            StrategicThemeCategory::Security => impact_score += 0.2,
            StrategicThemeCategory::Community => impact_score += 0.1,
        }
        
        Ok(match impact_score {
            x if x <= 1.0 => ImpactLevel::Low,
            x if x <= 2.0 => ImpactLevel::Medium,
            x if x <= 3.0 => ImpactLevel::High,
            _ => ImpactLevel::Critical,
        })
    }
    
    fn calculate_theme_implementation_feasibility(&self, theme: &StrategicTheme) -> Result<ComplexityLevel> {
        let mut complexity_score = 0.0;
        
        // Complexity from implementation priority (inverse relationship)
        complexity_score += match theme.implementation_priority {
            crate::content_processing::strategic_theme_organizer::ImplementationPriority::Critical => 1.0,
            crate::content_processing::strategic_theme_organizer::ImplementationPriority::High => 2.0,
            crate::content_processing::strategic_theme_organizer::ImplementationPriority::Medium => 3.0,
            crate::content_processing::strategic_theme_organizer::ImplementationPriority::Low => 4.0,
        };
        
        // Complexity from number of dependencies
        complexity_score += (theme.dependencies.len() as f64 * 0.2).min(1.0);
        
        // Complexity from adoption pathways
        let total_stages: usize = theme.adoption_pathways.iter()
            .map(|pathway| pathway.adoption_stages.len())
            .sum();
        complexity_score += (total_stages as f64 * 0.1).min(0.8);
        
        // Complexity from category
        match theme.category {
            StrategicThemeCategory::AiEnhancement => complexity_score += 0.4,
            StrategicThemeCategory::EcosystemIntegration => complexity_score += 0.3,
            StrategicThemeCategory::Security => complexity_score += 0.3,
            StrategicThemeCategory::Performance => complexity_score += 0.2,
            StrategicThemeCategory::DeveloperProductivity => complexity_score += 0.2,
            StrategicThemeCategory::Community => complexity_score += 0.1,
        }
        
        Ok(match complexity_score {
            x if x <= 1.5 => ComplexityLevel::Low,
            x if x <= 2.5 => ComplexityLevel::Medium,
            x if x <= 3.5 => ComplexityLevel::High,
            _ => ComplexityLevel::VeryHigh,
        })
    }
    
    fn calculate_priority_score(&self, impact: ImpactLevel, feasibility: ComplexityLevel) -> f64 {
        let impact_score = impact.score() as f64;
        let feasibility_score = 5.0 - feasibility.score() as f64; // Invert complexity for feasibility
        
        // Weighted combination: 60% impact, 40% feasibility
        (impact_score * 0.6) + (feasibility_score * 0.4)
    }
    
    fn count_theme_connections(&self, journey: &UserJourney, themes: &[StrategicTheme]) -> usize {
        let mut connections = 0;
        
        for cross_ref in self.cross_references.values() {
            if cross_ref.source_entity.entity_id == journey.id.0.to_string() &&
               cross_ref.target_entity.entity_type == EntityType::StrategicTheme {
                connections += 1;
            }
        }
        
        connections
    }
    
    fn gather_journey_evidence(&self, journey: &UserJourney) -> Result<Vec<String>> {
        let mut evidence = Vec::new();
        
        evidence.push(format!("Confidence score: {:.2}", journey.confidence_score));
        evidence.push(format!("Solutions count: {}", journey.solutions.len()));
        evidence.push(format!("Integration tools: {}", journey.integration_tools.len()));
        evidence.push(format!("Prerequisites: {}", journey.prerequisites.len()));
        
        // Add cross-reference evidence
        for cross_ref in self.cross_references.values() {
            if cross_ref.source_entity.entity_id == journey.id.0.to_string() {
                evidence.push(format!("Cross-reference: {} to {}", 
                    cross_ref.relationship_type.as_str(), 
                    cross_ref.target_entity.title));
            }
        }
        
        Ok(evidence)
    }
    
    fn gather_theme_evidence(&self, theme: &StrategicTheme) -> Result<Vec<String>> {
        let mut evidence = Vec::new();
        
        evidence.push(format!("Confidence score: {:.2}", theme.confidence_score));
        evidence.push(format!("Competitive advantages: {}", theme.competitive_advantages.len()));
        evidence.push(format!("ROI metrics: {}", theme.roi_metrics.len()));
        evidence.push(format!("Adoption pathways: {}", theme.adoption_pathways.len()));
        evidence.push(format!("Implementation priority: {}", theme.implementation_priority.as_str()));
        
        // Add cross-reference evidence
        for cross_ref in self.cross_references.values() {
            if cross_ref.source_entity.entity_id == theme.id.0.to_string() {
                evidence.push(format!("Cross-reference: {} to {}", 
                    cross_ref.relationship_type.as_str(), 
                    cross_ref.target_entity.title));
            }
        }
        
        Ok(evidence)
    }
    
    // Getter methods for accessing internal state
    pub fn get_cross_references(&self) -> &HashMap<CrossReferenceId, CrossReference> {
        &self.cross_references
    }
    
    pub fn get_narratives(&self) -> &HashMap<Uuid, WorkflowNarrative> {
        &self.narratives
    }
    
    pub fn get_quality_issues(&self) -> &HashMap<Uuid, QualityIssue> {
        &self.quality_issues
    }
    
    pub fn get_entity_index(&self) -> &HashMap<EntityType, HashMap<String, EntityReference>> {
        &self.entity_index
    }
}

impl Default for CrossReferenceSynthesizer {
    fn default() -> Self {
        Self::new()
    }
}
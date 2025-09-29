use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::content_processing::{ContentChunk, ChunkId, Result, ContentProcessingError, JourneyCandidate};

/// Unique identifier for a user journey
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserJourneyId(pub Uuid);

impl UserJourneyId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for UserJourneyId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserJourneyId> for Uuid {
    fn from(id: UserJourneyId) -> Self {
        id.0
    }
}

/// Developer persona categories as per requirements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeveloperPersona {
    IndividualDeveloper,
    TeamLead,
    DevOpsEngineer,
    PlatformEngineer,
}

impl DeveloperPersona {
    pub fn all() -> Vec<Self> {
        vec![
            Self::IndividualDeveloper,
            Self::TeamLead,
            Self::DevOpsEngineer,
            Self::PlatformEngineer,
        ]
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IndividualDeveloper => "Individual Developer",
            Self::TeamLead => "Team Lead",
            Self::DevOpsEngineer => "DevOps Engineer",
            Self::PlatformEngineer => "Platform Engineer",
        }
    }
}

/// Workflow type classifications as per requirements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowType {
    Development,
    CiCd,
    ArchitectureAnalysis,
    LlmIntegration,
    Testing,
    Security,
}

impl WorkflowType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Development,
            Self::CiCd,
            Self::ArchitectureAnalysis,
            Self::LlmIntegration,
            Self::Testing,
            Self::Security,
        ]
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "Development",
            Self::CiCd => "CI/CD",
            Self::ArchitectureAnalysis => "Architecture Analysis",
            Self::LlmIntegration => "LLM Integration",
            Self::Testing => "Testing",
            Self::Security => "Security",
        }
    }
}

/// Pain point extracted from content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainPoint {
    pub description: String,
    pub severity: PainPointSeverity,
    pub context: String,
    pub source_location: SourceLocation,
}

/// Severity levels for pain points
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PainPointSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Solution extracted from content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    pub description: String,
    pub approach: String,
    pub expected_outcomes: Vec<String>,
    pub implementation_complexity: ComplexityLevel,
    pub source_location: SourceLocation,
}

/// Complexity levels for solutions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Success metrics for solutions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetrics {
    pub performance_improvements: Vec<PerformanceMetric>,
    pub productivity_gains: Vec<ProductivityMetric>,
    pub quality_improvements: Vec<QualityMetric>,
    pub user_satisfaction: Option<SatisfactionMetric>,
}

/// Performance metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub metric_name: String,
    pub baseline_value: Option<String>,
    pub target_value: String,
    pub measurement_method: String,
}

/// Productivity metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductivityMetric {
    pub metric_name: String,
    pub current_state: String,
    pub improved_state: String,
    pub time_savings: Option<String>,
}

/// Quality metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetric {
    pub metric_name: String,
    pub improvement_description: String,
    pub measurement_criteria: Vec<String>,
}

/// Satisfaction metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatisfactionMetric {
    pub aspect: String,
    pub improvement_description: String,
    pub measurement_method: String,
}

/// Source location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub chunk_id: ChunkId,
    pub line_start: usize,
    pub line_end: usize,
    pub confidence_score: f64,
}

/// Complete user journey extracted from content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserJourney {
    pub id: UserJourneyId,
    pub title: String,
    pub description: String,
    pub persona: DeveloperPersona,
    pub workflow_type: WorkflowType,
    pub pain_points: Vec<PainPoint>,
    pub solutions: Vec<Solution>,
    pub success_metrics: SuccessMetrics,
    pub integration_tools: Vec<String>,
    pub prerequisites: Vec<String>,
    pub source_chunks: Vec<ChunkId>,
    pub confidence_score: f64,
    pub extracted_at: DateTime<Utc>,
}

impl UserJourney {
    pub fn new(
        title: String,
        description: String,
        persona: DeveloperPersona,
        workflow_type: WorkflowType,
    ) -> Self {
        Self {
            id: UserJourneyId::new(),
            title,
            description,
            persona,
            workflow_type,
            pain_points: Vec::new(),
            solutions: Vec::new(),
            success_metrics: SuccessMetrics {
                performance_improvements: Vec::new(),
                productivity_gains: Vec::new(),
                quality_improvements: Vec::new(),
                user_satisfaction: None,
            },
            integration_tools: Vec::new(),
            prerequisites: Vec::new(),
            source_chunks: Vec::new(),
            confidence_score: 0.0,
            extracted_at: Utc::now(),
        }
    }
}

/// User journey extraction engine
pub struct UserJourneyExtractor {
    persona_patterns: HashMap<DeveloperPersona, Vec<String>>,
    workflow_patterns: HashMap<WorkflowType, Vec<String>>,
    pain_point_indicators: Vec<String>,
    solution_indicators: Vec<String>,
    metrics_indicators: Vec<String>,
}

impl UserJourneyExtractor {
    pub fn new() -> Self {
        Self {
            persona_patterns: Self::build_persona_patterns(),
            workflow_patterns: Self::build_workflow_patterns(),
            pain_point_indicators: Self::build_pain_point_indicators(),
            solution_indicators: Self::build_solution_indicators(),
            metrics_indicators: Self::build_metrics_indicators(),
        }
    }
    
    /// Extract user journeys from a content chunk
    pub fn extract_journeys(&self, chunk: &ContentChunk) -> Result<Vec<UserJourney>> {
        let mut journeys = Vec::new();
        
        // Analyze content for journey patterns
        let journey_candidates = self.identify_journey_candidates(chunk)?;
        
        for candidate in journey_candidates {
            if let Some(journey) = self.build_journey_from_candidate(candidate, chunk)? {
                journeys.push(journey);
            }
        }
        
        Ok(journeys)
    }
    
    /// Identify potential journey candidates in content
    fn identify_journey_candidates(&self, chunk: &ContentChunk) -> Result<Vec<JourneyCandidate>> {
        let mut candidates = Vec::new();
        let lines: Vec<&str> = chunk.content.lines().collect();
        
        let mut current_candidate: Option<JourneyCandidate> = None;
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check for journey start indicators
            if self.is_journey_start(&line_lower) {
                // Save previous candidate if exists
                if let Some(candidate) = current_candidate.take() {
                    candidates.push(candidate);
                }
                
                // Start new candidate
                current_candidate = Some(JourneyCandidate {
                    start_line: chunk.line_range.start + line_idx,
                    end_line: chunk.line_range.start + line_idx,
                    content_lines: vec![line.to_string()],
                    persona_hints: self.extract_persona_hints(&line_lower),
                    workflow_hints: self.extract_workflow_hints(&line_lower),
                });
            } else if let Some(ref mut candidate) = current_candidate {
                // Continue building current candidate
                candidate.end_line = chunk.line_range.start + line_idx;
                candidate.content_lines.push(line.to_string());
                
                // Update hints
                candidate.persona_hints.extend(self.extract_persona_hints(&line_lower));
                candidate.workflow_hints.extend(self.extract_workflow_hints(&line_lower));
                
                // Check for journey end
                if self.is_journey_end(&line_lower) {
                    candidates.push(current_candidate.take().unwrap());
                }
            }
        }
        
        // Add final candidate if exists
        if let Some(candidate) = current_candidate {
            candidates.push(candidate);
        }
        
        Ok(candidates)
    }
    
    /// Build a complete user journey from a candidate
    fn build_journey_from_candidate(
        &self,
        candidate: JourneyCandidate,
        chunk: &ContentChunk,
    ) -> Result<Option<UserJourney>> {
        let content = candidate.content_lines.join("\n");
        
        // Determine persona and workflow type
        let persona = self.classify_persona(&candidate.persona_hints, &content)?;
        let workflow_type = self.classify_workflow(&candidate.workflow_hints, &content)?;
        
        // Extract title and description
        let title = self.extract_title(&content)?;
        let description = self.extract_description(&content)?;
        
        // Skip if we can't determine basic information
        if title.is_empty() || persona.is_none() || workflow_type.is_none() {
            return Ok(None);
        }
        
        let mut journey = UserJourney::new(
            title,
            description,
            persona.unwrap(),
            workflow_type.unwrap(),
        );
        
        // Extract pain points
        journey.pain_points = self.extract_pain_points(&content, chunk.id, candidate.start_line)?;
        
        // Extract solutions
        journey.solutions = self.extract_solutions(&content, chunk.id, candidate.start_line)?;
        
        // Extract success metrics
        journey.success_metrics = self.extract_success_metrics(&content)?;
        
        // Extract integration tools and prerequisites
        journey.integration_tools = self.extract_integration_tools(&content)?;
        journey.prerequisites = self.extract_prerequisites(&content)?;
        
        // Set source information
        journey.source_chunks = vec![chunk.id];
        journey.confidence_score = self.calculate_confidence_score(&journey, &candidate)?;
        
        Ok(Some(journey))
    }
    
    // Pattern matching methods
    fn build_persona_patterns() -> HashMap<DeveloperPersona, Vec<String>> {
        let mut patterns = HashMap::new();
        
        patterns.insert(DeveloperPersona::IndividualDeveloper, vec![
            "individual developer".to_string(),
            "solo developer".to_string(),
            "single developer".to_string(),
            "developer working alone".to_string(),
            "personal project".to_string(),
            "my code".to_string(),
            "I need to".to_string(),
            "when I'm coding".to_string(),
        ]);
        
        patterns.insert(DeveloperPersona::TeamLead, vec![
            "team lead".to_string(),
            "team leader".to_string(),
            "engineering manager".to_string(),
            "tech lead".to_string(),
            "leading a team".to_string(),
            "team coordination".to_string(),
            "team productivity".to_string(),
            "code review".to_string(),
            "team standards".to_string(),
        ]);
        
        patterns.insert(DeveloperPersona::DevOpsEngineer, vec![
            "devops".to_string(),
            "ci/cd".to_string(),
            "deployment".to_string(),
            "infrastructure".to_string(),
            "pipeline".to_string(),
            "automation".to_string(),
            "monitoring".to_string(),
            "production".to_string(),
            "release".to_string(),
        ]);
        
        patterns.insert(DeveloperPersona::PlatformEngineer, vec![
            "platform engineer".to_string(),
            "platform team".to_string(),
            "infrastructure platform".to_string(),
            "developer platform".to_string(),
            "tooling".to_string(),
            "developer experience".to_string(),
            "internal tools".to_string(),
            "scalability".to_string(),
            "enterprise".to_string(),
        ]);
        
        patterns
    }
    
    fn build_workflow_patterns() -> HashMap<WorkflowType, Vec<String>> {
        let mut patterns = HashMap::new();
        
        patterns.insert(WorkflowType::Development, vec![
            "coding".to_string(),
            "development".to_string(),
            "programming".to_string(),
            "writing code".to_string(),
            "implementation".to_string(),
            "feature development".to_string(),
            "bug fixing".to_string(),
            "refactoring".to_string(),
        ]);
        
        patterns.insert(WorkflowType::CiCd, vec![
            "ci/cd".to_string(),
            "continuous integration".to_string(),
            "continuous deployment".to_string(),
            "build pipeline".to_string(),
            "automated testing".to_string(),
            "deployment pipeline".to_string(),
            "release automation".to_string(),
        ]);
        
        patterns.insert(WorkflowType::ArchitectureAnalysis, vec![
            "architecture".to_string(),
            "system design".to_string(),
            "code analysis".to_string(),
            "dependency analysis".to_string(),
            "architectural review".to_string(),
            "system understanding".to_string(),
            "codebase exploration".to_string(),
        ]);
        
        patterns.insert(WorkflowType::LlmIntegration, vec![
            "llm".to_string(),
            "ai integration".to_string(),
            "language model".to_string(),
            "ai assistance".to_string(),
            "code generation".to_string(),
            "ai-powered".to_string(),
            "machine learning".to_string(),
        ]);
        
        patterns.insert(WorkflowType::Testing, vec![
            "testing".to_string(),
            "test automation".to_string(),
            "quality assurance".to_string(),
            "test coverage".to_string(),
            "unit tests".to_string(),
            "integration tests".to_string(),
            "test strategy".to_string(),
        ]);
        
        patterns.insert(WorkflowType::Security, vec![
            "security".to_string(),
            "vulnerability".to_string(),
            "security analysis".to_string(),
            "threat modeling".to_string(),
            "security review".to_string(),
            "compliance".to_string(),
            "security scanning".to_string(),
        ]);
        
        patterns
    }
    
    fn build_pain_point_indicators() -> Vec<String> {
        vec![
            "problem".to_string(),
            "issue".to_string(),
            "challenge".to_string(),
            "difficulty".to_string(),
            "pain point".to_string(),
            "frustration".to_string(),
            "bottleneck".to_string(),
            "slow".to_string(),
            "inefficient".to_string(),
            "manual".to_string(),
            "time-consuming".to_string(),
            "error-prone".to_string(),
            "complex".to_string(),
            "hard to".to_string(),
            "difficult to".to_string(),
            "struggle with".to_string(),
        ]
    }
    
    fn build_solution_indicators() -> Vec<String> {
        vec![
            "solution".to_string(),
            "approach".to_string(),
            "method".to_string(),
            "technique".to_string(),
            "strategy".to_string(),
            "implementation".to_string(),
            "improvement".to_string(),
            "optimization".to_string(),
            "enhancement".to_string(),
            "automation".to_string(),
            "streamline".to_string(),
            "simplify".to_string(),
            "accelerate".to_string(),
            "enable".to_string(),
            "provide".to_string(),
        ]
    }
    
    fn build_metrics_indicators() -> Vec<String> {
        vec![
            "performance".to_string(),
            "speed".to_string(),
            "time".to_string(),
            "efficiency".to_string(),
            "productivity".to_string(),
            "quality".to_string(),
            "accuracy".to_string(),
            "reliability".to_string(),
            "scalability".to_string(),
            "throughput".to_string(),
            "latency".to_string(),
            "response time".to_string(),
            "success rate".to_string(),
            "error rate".to_string(),
            "satisfaction".to_string(),
        ]
    }
    
    // Helper methods for pattern matching and extraction
    fn is_journey_start(&self, line: &str) -> bool {
        line.contains("user story") ||
        line.contains("workflow") ||
        line.contains("journey") ||
        line.contains("use case") ||
        line.contains("scenario") ||
        line.starts_with("as a") ||
        line.starts_with("when") ||
        line.contains("developer needs")
    }
    
    fn is_journey_end(&self, line: &str) -> bool {
        line.is_empty() ||
        line.starts_with("---") ||
        line.starts_with("###") ||
        line.starts_with("##")
    }
    
    fn extract_persona_hints(&self, line: &str) -> Vec<DeveloperPersona> {
        let mut hints = Vec::new();
        
        for (persona, patterns) in &self.persona_patterns {
            for pattern in patterns {
                if line.contains(pattern) {
                    hints.push(persona.clone());
                    break;
                }
            }
        }
        
        hints
    }
    
    fn extract_workflow_hints(&self, line: &str) -> Vec<WorkflowType> {
        let mut hints = Vec::new();
        
        for (workflow, patterns) in &self.workflow_patterns {
            for pattern in patterns {
                if line.contains(pattern) {
                    hints.push(workflow.clone());
                    break;
                }
            }
        }
        
        hints
    }
    
    fn classify_persona(&self, hints: &[DeveloperPersona], content: &str) -> Result<Option<DeveloperPersona>> {
        if hints.is_empty() {
            // Fallback to content analysis
            let content_lower = content.to_lowercase();
            
            for (persona, patterns) in &self.persona_patterns {
                let matches = patterns.iter()
                    .filter(|pattern| content_lower.contains(*pattern))
                    .count();
                
                if matches >= 2 {
                    return Ok(Some(persona.clone()));
                }
            }
            
            return Ok(None);
        }
        
        // Use most frequent hint
        let mut counts = HashMap::new();
        for hint in hints {
            *counts.entry(hint.clone()).or_insert(0) += 1;
        }
        
        let most_frequent = counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(persona, _)| persona);
        
        Ok(most_frequent)
    }
    
    fn classify_workflow(&self, hints: &[WorkflowType], content: &str) -> Result<Option<WorkflowType>> {
        if hints.is_empty() {
            // Fallback to content analysis
            let content_lower = content.to_lowercase();
            
            for (workflow, patterns) in &self.workflow_patterns {
                let matches = patterns.iter()
                    .filter(|pattern| content_lower.contains(*pattern))
                    .count();
                
                if matches >= 2 {
                    return Ok(Some(workflow.clone()));
                }
            }
            
            return Ok(None);
        }
        
        // Use most frequent hint
        let mut counts = HashMap::new();
        for hint in hints {
            *counts.entry(hint.clone()).or_insert(0) += 1;
        }
        
        let most_frequent = counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(workflow, _)| workflow);
        
        Ok(most_frequent)
    }
    
    fn extract_title(&self, content: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        
        // Look for title patterns
        for line in &lines {
            let line_trimmed = line.trim();
            
            if line_trimmed.starts_with("# ") {
                return Ok(line_trimmed[2..].to_string());
            }
            
            if line_trimmed.starts_with("## ") {
                return Ok(line_trimmed[3..].to_string());
            }
            
            if line_trimmed.starts_with("User Story:") {
                return Ok(line_trimmed[11..].trim().to_string());
            }
            
            if line_trimmed.starts_with("Workflow:") {
                return Ok(line_trimmed[9..].trim().to_string());
            }
        }
        
        // Fallback: use first non-empty line
        for line in &lines {
            let line_trimmed = line.trim();
            if !line_trimmed.is_empty() {
                return Ok(line_trimmed.chars().take(100).collect());
            }
        }
        
        Ok("Untitled Journey".to_string())
    }
    
    fn extract_description(&self, content: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut description_lines = Vec::new();
        let mut in_description = false;
        
        for line in &lines {
            let line_trimmed = line.trim();
            
            if line_trimmed.starts_with("Description:") || 
               line_trimmed.starts_with("Summary:") ||
               line_trimmed.starts_with("Overview:") {
                in_description = true;
                if line_trimmed.len() > line_trimmed.find(':').unwrap() + 1 {
                    description_lines.push(line_trimmed[line_trimmed.find(':').unwrap() + 1..].trim());
                }
                continue;
            }
            
            if in_description {
                if line_trimmed.is_empty() || line_trimmed.starts_with('#') {
                    break;
                }
                description_lines.push(line_trimmed);
            }
        }
        
        if description_lines.is_empty() {
            // Use first few lines as description
            let first_lines: Vec<&str> = lines.iter()
                .take(3)
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect();
            
            return Ok(first_lines.join(" "));
        }
        
        Ok(description_lines.join(" "))
    }
    
    fn extract_pain_points(&self, content: &str, chunk_id: ChunkId, start_line: usize) -> Result<Vec<PainPoint>> {
        let mut pain_points = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check for pain point indicators
            for indicator in &self.pain_point_indicators {
                if line_lower.contains(indicator) {
                    let pain_point = PainPoint {
                        description: line.trim().to_string(),
                        severity: self.determine_pain_point_severity(&line_lower),
                        context: self.extract_context(&lines, line_idx),
                        source_location: SourceLocation {
                            chunk_id,
                            line_start: start_line + line_idx,
                            line_end: start_line + line_idx,
                            confidence_score: 0.7,
                        },
                    };
                    pain_points.push(pain_point);
                    break;
                }
            }
        }
        
        Ok(pain_points)
    }
    
    fn extract_solutions(&self, content: &str, chunk_id: ChunkId, start_line: usize) -> Result<Vec<Solution>> {
        let mut solutions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check for solution indicators
            for indicator in &self.solution_indicators {
                if line_lower.contains(indicator) {
                    let solution = Solution {
                        description: line.trim().to_string(),
                        approach: self.extract_approach(&lines, line_idx),
                        expected_outcomes: self.extract_expected_outcomes(&lines, line_idx),
                        implementation_complexity: self.determine_complexity(&line_lower),
                        source_location: SourceLocation {
                            chunk_id,
                            line_start: start_line + line_idx,
                            line_end: start_line + line_idx,
                            confidence_score: 0.7,
                        },
                    };
                    solutions.push(solution);
                    break;
                }
            }
        }
        
        Ok(solutions)
    }
    
    fn extract_success_metrics(&self, content: &str) -> Result<SuccessMetrics> {
        let lines: Vec<&str> = content.lines().collect();
        let mut performance_improvements = Vec::new();
        let mut productivity_gains = Vec::new();
        let mut quality_improvements = Vec::new();
        let mut user_satisfaction = None;
        
        for line in &lines {
            let line_lower = line.to_lowercase();
            
            // Check for metrics indicators
            for indicator in &self.metrics_indicators {
                if line_lower.contains(indicator) {
                    if line_lower.contains("performance") {
                        performance_improvements.push(PerformanceMetric {
                            metric_name: indicator.clone(),
                            baseline_value: None,
                            target_value: line.trim().to_string(),
                            measurement_method: "To be defined".to_string(),
                        });
                    } else if line_lower.contains("productivity") {
                        productivity_gains.push(ProductivityMetric {
                            metric_name: indicator.clone(),
                            current_state: "Current".to_string(),
                            improved_state: line.trim().to_string(),
                            time_savings: None,
                        });
                    } else if line_lower.contains("quality") {
                        quality_improvements.push(QualityMetric {
                            metric_name: indicator.clone(),
                            improvement_description: line.trim().to_string(),
                            measurement_criteria: vec!["To be defined".to_string()],
                        });
                    } else if line_lower.contains("satisfaction") {
                        user_satisfaction = Some(SatisfactionMetric {
                            aspect: indicator.clone(),
                            improvement_description: line.trim().to_string(),
                            measurement_method: "Survey".to_string(),
                        });
                    }
                    break;
                }
            }
        }
        
        Ok(SuccessMetrics {
            performance_improvements,
            productivity_gains,
            quality_improvements,
            user_satisfaction,
        })
    }
    
    fn extract_integration_tools(&self, content: &str) -> Result<Vec<String>> {
        let mut tools = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for line in &lines {
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("integration") || line_lower.contains("tool") {
                // Extract tool names from the line
                let words: Vec<&str> = line.split_whitespace().collect();
                for word in words {
                    if word.len() > 3 && !word.chars().all(|c| c.is_ascii_lowercase()) {
                        tools.push(word.to_string());
                    }
                }
            }
        }
        
        Ok(tools)
    }
    
    fn extract_prerequisites(&self, content: &str) -> Result<Vec<String>> {
        let mut prerequisites = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for line in &lines {
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("prerequisite") || 
               line_lower.contains("requires") ||
               line_lower.contains("depends on") {
                prerequisites.push(line.trim().to_string());
            }
        }
        
        Ok(prerequisites)
    }
    
    fn calculate_confidence_score(&self, journey: &UserJourney, candidate: &JourneyCandidate) -> Result<f64> {
        let mut score = 0.0;
        let mut factors = 0;
        
        // Factor 1: Persona classification confidence
        if !candidate.persona_hints.is_empty() {
            score += 0.2;
        }
        factors += 1;
        
        // Factor 2: Workflow classification confidence
        if !candidate.workflow_hints.is_empty() {
            score += 0.2;
        }
        factors += 1;
        
        // Factor 3: Pain points found
        if !journey.pain_points.is_empty() {
            score += 0.2;
        }
        factors += 1;
        
        // Factor 4: Solutions found
        if !journey.solutions.is_empty() {
            score += 0.2;
        }
        factors += 1;
        
        // Factor 5: Content quality (length and structure)
        let content_length = candidate.content_lines.len();
        if content_length > 5 {
            score += 0.2;
        }
        factors += 1;
        
        Ok(score)
    }
    
    // Helper methods
    fn determine_pain_point_severity(&self, line: &str) -> PainPointSeverity {
        if line.contains("critical") || line.contains("severe") {
            PainPointSeverity::Critical
        } else if line.contains("major") || line.contains("significant") {
            PainPointSeverity::High
        } else if line.contains("minor") || line.contains("small") {
            PainPointSeverity::Low
        } else {
            PainPointSeverity::Medium
        }
    }
    
    fn extract_context(&self, lines: &[&str], line_idx: usize) -> String {
        let start = line_idx.saturating_sub(1);
        let end = std::cmp::min(line_idx + 2, lines.len());
        
        lines[start..end]
            .iter()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    fn extract_approach(&self, lines: &[&str], line_idx: usize) -> String {
        // Look for approach description in surrounding lines
        let start = line_idx.saturating_sub(1);
        let end = std::cmp::min(line_idx + 3, lines.len());
        
        for i in start..end {
            let line = lines[i].trim();
            if line.to_lowercase().contains("approach") {
                return line.to_string();
            }
        }
        
        "To be defined".to_string()
    }
    
    fn extract_expected_outcomes(&self, lines: &[&str], line_idx: usize) -> Vec<String> {
        let mut outcomes = Vec::new();
        let start = line_idx;
        let end = std::cmp::min(line_idx + 5, lines.len());
        
        for i in start..end {
            let line = lines[i].trim();
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("outcome") || 
               line_lower.contains("result") ||
               line_lower.contains("benefit") {
                outcomes.push(line.to_string());
            }
        }
        
        if outcomes.is_empty() {
            outcomes.push("Improved workflow efficiency".to_string());
        }
        
        outcomes
    }
    
    fn determine_complexity(&self, line: &str) -> ComplexityLevel {
        if line.contains("simple") || line.contains("easy") {
            ComplexityLevel::Low
        } else if line.contains("complex") || line.contains("difficult") {
            ComplexityLevel::High
        } else if line.contains("very complex") || line.contains("extremely difficult") {
            ComplexityLevel::VeryHigh
        } else {
            ComplexityLevel::Medium
        }
    }
}

impl Default for UserJourneyExtractor {
    fn default() -> Self {
        Self::new()
    }
}
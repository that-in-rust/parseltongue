use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::content_processing::{ContentChunk, ChunkId, Result, ContentProcessingError};

/// Unique identifier for strategic themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StrategyThemeId(pub Uuid);

impl StrategyThemeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for StrategyThemeId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<StrategyThemeId> for Uuid {
    fn from(id: StrategyThemeId) -> Self {
        id.0
    }
}

/// Strategic theme categories as per requirements 3.1, 3.2, 3.3, 3.5
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrategicThemeCategory {
    DeveloperProductivity,
    AiEnhancement,
    EcosystemIntegration,
    Performance,
    Security,
    Community,
}

impl StrategicThemeCategory {
    pub fn all() -> Vec<Self> {
        vec![
            Self::DeveloperProductivity,
            Self::AiEnhancement,
            Self::EcosystemIntegration,
            Self::Performance,
            Self::Security,
            Self::Community,
        ]
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeveloperProductivity => "Developer Productivity",
            Self::AiEnhancement => "AI Enhancement",
            Self::EcosystemIntegration => "Ecosystem Integration",
            Self::Performance => "Performance",
            Self::Security => "Security",
            Self::Community => "Community",
        }
    }
}

/// Implementation priority levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImplementationPriority {
    Critical,
    High,
    Medium,
    Low,
}

impl ImplementationPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Critical => "Critical",
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
        }
    }
    
    pub fn score(&self) -> u8 {
        match self {
            Self::Critical => 4,
            Self::High => 3,
            Self::Medium => 2,
            Self::Low => 1,
        }
    }
}

/// Competitive advantage extracted from content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitiveAdvantage {
    pub description: String,
    pub unique_value_proposition: String,
    pub market_differentiation: String,
    pub technical_moat: Option<String>,
    pub source_location: SourceLocation,
    pub confidence_score: f64,
}

/// Ecosystem positioning information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemPositioning {
    pub market_position: String,
    pub target_audience: Vec<String>,
    pub integration_points: Vec<String>,
    pub competitive_landscape: Vec<String>,
    pub partnership_opportunities: Vec<String>,
}

/// ROI metrics with measurement methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoiMetrics {
    pub metric_name: String,
    pub baseline_value: Option<String>,
    pub projected_value: String,
    pub measurement_methodology: String,
    pub timeframe: String,
    pub assumptions: Vec<String>,
    pub risk_factors: Vec<String>,
}

/// Adoption pathway mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionPathway {
    pub pathway_name: String,
    pub target_persona: String,
    pub adoption_stages: Vec<AdoptionStage>,
    pub barriers_to_adoption: Vec<String>,
    pub enablers: Vec<String>,
    pub success_indicators: Vec<String>,
}

/// Individual adoption stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionStage {
    pub stage_name: String,
    pub description: String,
    pub duration_estimate: Option<String>,
    pub prerequisites: Vec<String>,
    pub deliverables: Vec<String>,
    pub success_criteria: Vec<String>,
}

/// Source location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub chunk_id: ChunkId,
    pub line_start: usize,
    pub line_end: usize,
    pub confidence_score: f64,
}

/// Complete strategic theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategicTheme {
    pub id: StrategyThemeId,
    pub title: String,
    pub description: String,
    pub category: StrategicThemeCategory,
    pub competitive_advantages: Vec<CompetitiveAdvantage>,
    pub ecosystem_positioning: EcosystemPositioning,
    pub roi_metrics: Vec<RoiMetrics>,
    pub adoption_pathways: Vec<AdoptionPathway>,
    pub implementation_priority: ImplementationPriority,
    pub dependencies: Vec<String>,
    pub source_chunks: Vec<ChunkId>,
    pub confidence_score: f64,
    pub extracted_at: DateTime<Utc>,
}

impl StrategicTheme {
    pub fn new(
        title: String,
        description: String,
        category: StrategicThemeCategory,
    ) -> Self {
        Self {
            id: StrategyThemeId::new(),
            title,
            description,
            category,
            competitive_advantages: Vec::new(),
            ecosystem_positioning: EcosystemPositioning {
                market_position: String::new(),
                target_audience: Vec::new(),
                integration_points: Vec::new(),
                competitive_landscape: Vec::new(),
                partnership_opportunities: Vec::new(),
            },
            roi_metrics: Vec::new(),
            adoption_pathways: Vec::new(),
            implementation_priority: ImplementationPriority::Medium,
            dependencies: Vec::new(),
            source_chunks: Vec::new(),
            confidence_score: 0.0,
            extracted_at: Utc::now(),
        }
    }
}

/// Strategic insight synthesis engine
pub struct StrategicInsightSynthesizer {
    category_patterns: HashMap<StrategicThemeCategory, Vec<String>>,
    competitive_advantage_indicators: Vec<String>,
    roi_indicators: Vec<String>,
    adoption_indicators: Vec<String>,
    ecosystem_indicators: Vec<String>,
}

impl StrategicInsightSynthesizer {
    pub fn new() -> Self {
        Self {
            category_patterns: Self::build_category_patterns(),
            competitive_advantage_indicators: Self::build_competitive_advantage_indicators(),
            roi_indicators: Self::build_roi_indicators(),
            adoption_indicators: Self::build_adoption_indicators(),
            ecosystem_indicators: Self::build_ecosystem_indicators(),
        }
    }
    
    /// Extract strategic themes from a content chunk
    pub fn extract_strategic_themes(&self, chunk: &ContentChunk) -> Result<Vec<StrategicTheme>> {
        let mut themes = Vec::new();
        
        // Analyze content for strategic patterns
        let theme_candidates = self.identify_theme_candidates(chunk)?;
        
        for candidate in theme_candidates {
            if let Some(theme) = self.build_theme_from_candidate(candidate, chunk)? {
                themes.push(theme);
            }
        }
        
        Ok(themes)
    }
    
    /// Synthesize competitive advantages from content
    pub fn synthesize_competitive_advantages(&self, content: &str, chunk_id: ChunkId) -> Result<Vec<CompetitiveAdvantage>> {
        let mut advantages = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check for competitive advantage indicators
            for indicator in &self.competitive_advantage_indicators {
                if line_lower.contains(indicator) {
                    if let Some(advantage) = self.extract_competitive_advantage(
                        &lines, line_idx, chunk_id
                    )? {
                        advantages.push(advantage);
                    }
                    break;
                }
            }
        }
        
        Ok(advantages)
    }
    
    /// Build ROI metrics capture system
    pub fn capture_roi_metrics(&self, content: &str, chunk_id: ChunkId) -> Result<Vec<RoiMetrics>> {
        let mut metrics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check for ROI indicators
            for indicator in &self.roi_indicators {
                if line_lower.contains(indicator) {
                    if let Some(roi_metric) = self.extract_roi_metric(
                        &lines, line_idx, chunk_id
                    )? {
                        metrics.push(roi_metric);
                    }
                    break;
                }
            }
        }
        
        Ok(metrics)
    }
    
    /// Create adoption pathway mapping system
    pub fn map_adoption_pathways(&self, content: &str, chunk_id: ChunkId) -> Result<Vec<AdoptionPathway>> {
        let mut pathways = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check for adoption indicators
            for indicator in &self.adoption_indicators {
                if line_lower.contains(indicator) {
                    if let Some(pathway) = self.extract_adoption_pathway(
                        &lines, line_idx, chunk_id
                    )? {
                        pathways.push(pathway);
                    }
                    break;
                }
            }
        }
        
        Ok(pathways)
    }
    
    // Pattern building methods
    fn build_category_patterns() -> HashMap<StrategicThemeCategory, Vec<String>> {
        let mut patterns = HashMap::new();
        
        patterns.insert(StrategicThemeCategory::DeveloperProductivity, vec![
            "developer productivity".to_string(),
            "development efficiency".to_string(),
            "coding speed".to_string(),
            "workflow optimization".to_string(),
            "developer experience".to_string(),
            "time savings".to_string(),
            "automation".to_string(),
            "streamline development".to_string(),
            "reduce friction".to_string(),
            "faster development".to_string(),
        ]);
        
        patterns.insert(StrategicThemeCategory::AiEnhancement, vec![
            "ai enhancement".to_string(),
            "artificial intelligence".to_string(),
            "machine learning".to_string(),
            "llm integration".to_string(),
            "ai-powered".to_string(),
            "intelligent assistance".to_string(),
            "automated analysis".to_string(),
            "smart recommendations".to_string(),
            "zero hallucination".to_string(),
            "context generation".to_string(),
        ]);
        
        patterns.insert(StrategicThemeCategory::EcosystemIntegration, vec![
            "ecosystem integration".to_string(),
            "tool integration".to_string(),
            "platform compatibility".to_string(),
            "interoperability".to_string(),
            "api integration".to_string(),
            "plugin system".to_string(),
            "extensibility".to_string(),
            "third-party tools".to_string(),
            "ecosystem positioning".to_string(),
            "market integration".to_string(),
        ]);
        
        patterns.insert(StrategicThemeCategory::Performance, vec![
            "performance".to_string(),
            "speed".to_string(),
            "efficiency".to_string(),
            "optimization".to_string(),
            "scalability".to_string(),
            "throughput".to_string(),
            "latency".to_string(),
            "memory usage".to_string(),
            "cpu utilization".to_string(),
            "benchmark".to_string(),
        ]);
        
        patterns.insert(StrategicThemeCategory::Security, vec![
            "security".to_string(),
            "vulnerability".to_string(),
            "threat model".to_string(),
            "compliance".to_string(),
            "authentication".to_string(),
            "authorization".to_string(),
            "encryption".to_string(),
            "secure coding".to_string(),
            "security analysis".to_string(),
            "risk assessment".to_string(),
        ]);
        
        patterns.insert(StrategicThemeCategory::Community, vec![
            "community".to_string(),
            "open source".to_string(),
            "collaboration".to_string(),
            "contribution".to_string(),
            "community driven".to_string(),
            "user feedback".to_string(),
            "community support".to_string(),
            "extensible".to_string(),
            "plugin ecosystem".to_string(),
            "developer community".to_string(),
        ]);
        
        patterns
    }
    
    fn build_competitive_advantage_indicators() -> Vec<String> {
        vec![
            "competitive advantage".to_string(),
            "unique value".to_string(),
            "differentiation".to_string(),
            "market position".to_string(),
            "unique selling point".to_string(),
            "competitive edge".to_string(),
            "market differentiation".to_string(),
            "value proposition".to_string(),
            "strategic advantage".to_string(),
            "competitive moat".to_string(),
            "first mover advantage".to_string(),
            "technical superiority".to_string(),
        ]
    }
    
    fn build_roi_indicators() -> Vec<String> {
        vec![
            "roi".to_string(),
            "return on investment".to_string(),
            "cost savings".to_string(),
            "time savings".to_string(),
            "efficiency gains".to_string(),
            "productivity improvement".to_string(),
            "revenue impact".to_string(),
            "cost reduction".to_string(),
            "business value".to_string(),
            "economic benefit".to_string(),
            "financial impact".to_string(),
            "measurable benefit".to_string(),
        ]
    }
    
    fn build_adoption_indicators() -> Vec<String> {
        vec![
            "adoption".to_string(),
            "onboarding".to_string(),
            "user journey".to_string(),
            "implementation path".to_string(),
            "rollout strategy".to_string(),
            "migration path".to_string(),
            "adoption pathway".to_string(),
            "getting started".to_string(),
            "user adoption".to_string(),
            "deployment strategy".to_string(),
            "implementation plan".to_string(),
            "adoption barriers".to_string(),
        ]
    }
    
    fn build_ecosystem_indicators() -> Vec<String> {
        vec![
            "ecosystem".to_string(),
            "integration".to_string(),
            "compatibility".to_string(),
            "interoperability".to_string(),
            "platform".to_string(),
            "api".to_string(),
            "plugin".to_string(),
            "extension".to_string(),
            "third-party".to_string(),
            "marketplace".to_string(),
            "partner".to_string(),
            "collaboration".to_string(),
        ]
    }
}

/// Theme candidate for extraction
#[derive(Debug, Clone)]
pub struct ThemeCandidate {
    pub start_line: usize,
    pub end_line: usize,
    pub content_lines: Vec<String>,
    pub category_hints: Vec<StrategicThemeCategory>,
    pub competitive_advantage_hints: Vec<String>,
    pub roi_hints: Vec<String>,
    pub adoption_hints: Vec<String>,
}
impl StrategicInsightSynthesizer {
    /// Identify potential theme candidates in content
    fn identify_theme_candidates(&self, chunk: &ContentChunk) -> Result<Vec<ThemeCandidate>> {
        let mut candidates = Vec::new();
        let lines: Vec<&str> = chunk.content.lines().collect();
        
        let mut current_candidate: Option<ThemeCandidate> = None;
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check for theme start indicators
            if self.is_theme_start(&line_lower) {
                // Save previous candidate if exists
                if let Some(candidate) = current_candidate.take() {
                    candidates.push(candidate);
                }
                
                // Start new candidate
                current_candidate = Some(ThemeCandidate {
                    start_line: chunk.line_range.start + line_idx,
                    end_line: chunk.line_range.start + line_idx,
                    content_lines: vec![line.to_string()],
                    category_hints: self.extract_category_hints(&line_lower),
                    competitive_advantage_hints: self.extract_competitive_advantage_hints(&line_lower),
                    roi_hints: self.extract_roi_hints(&line_lower),
                    adoption_hints: self.extract_adoption_hints(&line_lower),
                });
            } else if let Some(ref mut candidate) = current_candidate {
                // Continue building current candidate
                candidate.end_line = chunk.line_range.start + line_idx;
                candidate.content_lines.push(line.to_string());
                
                // Update hints
                candidate.category_hints.extend(self.extract_category_hints(&line_lower));
                candidate.competitive_advantage_hints.extend(self.extract_competitive_advantage_hints(&line_lower));
                candidate.roi_hints.extend(self.extract_roi_hints(&line_lower));
                candidate.adoption_hints.extend(self.extract_adoption_hints(&line_lower));
                
                // Check for theme end
                if self.is_theme_end(&line_lower) {
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
    
    /// Build a complete strategic theme from a candidate
    fn build_theme_from_candidate(
        &self,
        candidate: ThemeCandidate,
        chunk: &ContentChunk,
    ) -> Result<Option<StrategicTheme>> {
        let content = candidate.content_lines.join("\n");
        
        // Determine category
        let category = self.classify_category(&candidate.category_hints, &content)?;
        
        // Extract title and description
        let title = self.extract_theme_title(&content)?;
        let description = self.extract_theme_description(&content)?;
        
        // Skip if we can't determine basic information
        if title.is_empty() || category.is_none() {
            return Ok(None);
        }
        
        let mut theme = StrategicTheme::new(
            title,
            description,
            category.unwrap(),
        );
        
        // Extract competitive advantages
        theme.competitive_advantages = self.synthesize_competitive_advantages(&content, chunk.id)?;
        
        // Extract ROI metrics
        theme.roi_metrics = self.capture_roi_metrics(&content, chunk.id)?;
        
        // Extract adoption pathways
        theme.adoption_pathways = self.map_adoption_pathways(&content, chunk.id)?;
        
        // Extract ecosystem positioning
        theme.ecosystem_positioning = self.extract_ecosystem_positioning(&content)?;
        
        // Determine implementation priority
        theme.implementation_priority = self.determine_implementation_priority(&content, &candidate)?;
        
        // Extract dependencies
        theme.dependencies = self.extract_dependencies(&content)?;
        
        // Set source information
        theme.source_chunks = vec![chunk.id];
        theme.confidence_score = self.calculate_theme_confidence_score(&theme, &candidate)?;
        
        Ok(Some(theme))
    }
    
    /// Extract competitive advantage from content lines
    fn extract_competitive_advantage(
        &self,
        lines: &[&str],
        start_idx: usize,
        chunk_id: ChunkId,
    ) -> Result<Option<CompetitiveAdvantage>> {
        let mut description = String::new();
        let mut unique_value_proposition = String::new();
        let mut market_differentiation = String::new();
        let mut technical_moat = None;
        
        // Look for advantage description in surrounding lines
        let start = start_idx.saturating_sub(2);
        let end = std::cmp::min(start_idx + 5, lines.len());
        
        for i in start..end {
            let line = lines[i].trim();
            if line.is_empty() {
                continue;
            }
            
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("unique value") || line_lower.contains("value proposition") {
                unique_value_proposition = line.to_string();
            } else if line_lower.contains("differentiation") || line_lower.contains("competitive edge") {
                market_differentiation = line.to_string();
            } else if line_lower.contains("technical moat") || line_lower.contains("technical superiority") {
                technical_moat = Some(line.to_string());
            } else if description.is_empty() {
                description = line.to_string();
            }
        }
        
        if description.is_empty() {
            return Ok(None);
        }
        
        Ok(Some(CompetitiveAdvantage {
            description,
            unique_value_proposition,
            market_differentiation,
            technical_moat,
            source_location: SourceLocation {
                chunk_id,
                line_start: start_idx,
                line_end: end.saturating_sub(1),
                confidence_score: 0.8,
            },
            confidence_score: 0.8,
        }))
    }
    
    /// Extract ROI metric from content lines
    fn extract_roi_metric(
        &self,
        lines: &[&str],
        start_idx: usize,
        chunk_id: ChunkId,
    ) -> Result<Option<RoiMetrics>> {
        let mut metric_name = String::new();
        let mut baseline_value = None;
        let mut projected_value = String::new();
        let mut measurement_methodology = String::new();
        let mut timeframe = String::new();
        let mut assumptions = Vec::new();
        let mut risk_factors = Vec::new();
        
        // Look for ROI details in surrounding lines
        let start = start_idx.saturating_sub(2);
        let end = std::cmp::min(start_idx + 8, lines.len());
        
        for i in start..end {
            let line = lines[i].trim();
            if line.is_empty() {
                continue;
            }
            
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("baseline") {
                baseline_value = Some(self.extract_value_from_line(line));
            } else if line_lower.contains("target") || line_lower.contains("projected") {
                projected_value = self.extract_value_from_line(line);
            } else if line_lower.contains("measurement") || line_lower.contains("methodology") {
                measurement_methodology = line.to_string();
            } else if line_lower.contains("timeframe") || line_lower.contains("timeline") {
                timeframe = self.extract_value_from_line(line);
            } else if line_lower.contains("assumption") {
                assumptions.push(line.to_string());
            } else if line_lower.contains("risk") {
                risk_factors.push(line.to_string());
            } else if metric_name.is_empty() {
                metric_name = line.to_string();
            }
        }
        
        if metric_name.is_empty() {
            return Ok(None);
        }
        
        Ok(Some(RoiMetrics {
            metric_name,
            baseline_value,
            projected_value,
            measurement_methodology,
            timeframe,
            assumptions,
            risk_factors,
        }))
    }
    
    /// Extract adoption pathway from content lines
    fn extract_adoption_pathway(
        &self,
        lines: &[&str],
        start_idx: usize,
        chunk_id: ChunkId,
    ) -> Result<Option<AdoptionPathway>> {
        let mut pathway_name = String::new();
        let mut target_persona = String::new();
        let mut adoption_stages = Vec::new();
        let mut barriers_to_adoption = Vec::new();
        let mut enablers = Vec::new();
        let mut success_indicators = Vec::new();
        
        // Look for adoption details in surrounding lines
        let start = start_idx.saturating_sub(2);
        let end = std::cmp::min(start_idx + 10, lines.len());
        
        for i in start..end {
            let line = lines[i].trim();
            if line.is_empty() {
                continue;
            }
            
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("target") && line_lower.contains("persona") {
                target_persona = self.extract_value_from_line(line);
            } else if line_lower.contains("stage") {
                if let Some(stage) = self.extract_adoption_stage(line) {
                    adoption_stages.push(stage);
                }
            } else if line_lower.contains("barrier") || line_lower.contains("obstacle") {
                barriers_to_adoption.push(line.to_string());
            } else if line_lower.contains("enabler") || line_lower.contains("facilitator") {
                enablers.push(line.to_string());
            } else if line_lower.contains("success") && line_lower.contains("indicator") {
                success_indicators.push(line.to_string());
            } else if pathway_name.is_empty() {
                pathway_name = line.to_string();
            }
        }
        
        if pathway_name.is_empty() {
            return Ok(None);
        }
        
        Ok(Some(AdoptionPathway {
            pathway_name,
            target_persona,
            adoption_stages,
            barriers_to_adoption,
            enablers,
            success_indicators,
        }))
    }
    
    // Helper methods for pattern matching and extraction
    fn is_theme_start(&self, line: &str) -> bool {
        line.contains("strategic theme") ||
        line.contains("competitive advantage") ||
        line.contains("market position") ||
        line.contains("value proposition") ||
        line.starts_with("strategy:") ||
        line.starts_with("theme:") ||
        line.contains("roi analysis") ||
        line.contains("adoption strategy")
    }
    
    fn is_theme_end(&self, line: &str) -> bool {
        line.is_empty() ||
        line.starts_with("---") ||
        line.starts_with("###") ||
        line.starts_with("##")
    }
    
    fn extract_category_hints(&self, line: &str) -> Vec<StrategicThemeCategory> {
        let mut hints = Vec::new();
        
        for (category, patterns) in &self.category_patterns {
            for pattern in patterns {
                if line.contains(pattern) {
                    hints.push(category.clone());
                    break;
                }
            }
        }
        
        hints
    }
    
    fn extract_competitive_advantage_hints(&self, line: &str) -> Vec<String> {
        let mut hints = Vec::new();
        
        for indicator in &self.competitive_advantage_indicators {
            if line.contains(indicator) {
                hints.push(indicator.clone());
            }
        }
        
        hints
    }
    
    fn extract_roi_hints(&self, line: &str) -> Vec<String> {
        let mut hints = Vec::new();
        
        for indicator in &self.roi_indicators {
            if line.contains(indicator) {
                hints.push(indicator.clone());
            }
        }
        
        hints
    }
    
    fn extract_adoption_hints(&self, line: &str) -> Vec<String> {
        let mut hints = Vec::new();
        
        for indicator in &self.adoption_indicators {
            if line.contains(indicator) {
                hints.push(indicator.clone());
            }
        }
        
        hints
    }
    
    fn classify_category(&self, hints: &[StrategicThemeCategory], content: &str) -> Result<Option<StrategicThemeCategory>> {
        if hints.is_empty() {
            // Fallback to content analysis
            let content_lower = content.to_lowercase();
            
            for (category, patterns) in &self.category_patterns {
                let matches = patterns.iter()
                    .filter(|pattern| content_lower.contains(*pattern))
                    .count();
                
                if matches >= 2 {
                    return Ok(Some(category.clone()));
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
            .map(|(category, _)| category);
        
        Ok(most_frequent)
    }
    
    fn extract_theme_title(&self, content: &str) -> Result<String> {
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
            
            if line_trimmed.starts_with("Strategic Theme:") {
                return Ok(line_trimmed[16..].trim().to_string());
            }
            
            if line_trimmed.starts_with("Theme:") {
                return Ok(line_trimmed[6..].trim().to_string());
            }
        }
        
        // Fallback: use first non-empty line
        for line in &lines {
            let line_trimmed = line.trim();
            if !line_trimmed.is_empty() {
                return Ok(line_trimmed.chars().take(100).collect());
            }
        }
        
        Ok("Untitled Strategic Theme".to_string())
    }
    
    fn extract_theme_description(&self, content: &str) -> Result<String> {
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
    
    fn extract_ecosystem_positioning(&self, content: &str) -> Result<EcosystemPositioning> {
        let mut market_position = String::new();
        let mut target_audience = Vec::new();
        let mut integration_points = Vec::new();
        let mut competitive_landscape = Vec::new();
        let mut partnership_opportunities = Vec::new();
        
        let lines: Vec<&str> = content.lines().collect();
        
        for line in &lines {
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("market position") {
                market_position = self.extract_value_from_line(line);
            } else if line_lower.contains("target audience") {
                target_audience.push(self.extract_value_from_line(line));
            } else if line_lower.contains("integration point") {
                integration_points.push(self.extract_value_from_line(line));
            } else if line_lower.contains("competitive landscape") {
                competitive_landscape.push(self.extract_value_from_line(line));
            } else if line_lower.contains("partnership") {
                partnership_opportunities.push(self.extract_value_from_line(line));
            }
        }
        
        Ok(EcosystemPositioning {
            market_position,
            target_audience,
            integration_points,
            competitive_landscape,
            partnership_opportunities,
        })
    }
    
    fn determine_implementation_priority(&self, content: &str, candidate: &ThemeCandidate) -> Result<ImplementationPriority> {
        let content_lower = content.to_lowercase();
        
        // Check for priority indicators
        if content_lower.contains("critical") || content_lower.contains("urgent") {
            return Ok(ImplementationPriority::Critical);
        }
        
        if content_lower.contains("high priority") || content_lower.contains("important") {
            return Ok(ImplementationPriority::High);
        }
        
        if content_lower.contains("low priority") || content_lower.contains("nice to have") {
            return Ok(ImplementationPriority::Low);
        }
        
        // Default to medium priority
        Ok(ImplementationPriority::Medium)
    }
    
    fn extract_dependencies(&self, content: &str) -> Result<Vec<String>> {
        let mut dependencies = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for line in &lines {
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("depends on") || 
               line_lower.contains("requires") ||
               line_lower.contains("prerequisite") {
                dependencies.push(self.extract_value_from_line(line));
            }
        }
        
        Ok(dependencies)
    }
    
    fn extract_value_from_line(&self, line: &str) -> String {
        // Extract value after colon or other separators
        if let Some(colon_pos) = line.find(':') {
            return line[colon_pos + 1..].trim().to_string();
        }
        
        if let Some(dash_pos) = line.find('-') {
            return line[dash_pos + 1..].trim().to_string();
        }
        
        line.trim().to_string()
    }
    
    fn extract_adoption_stage(&self, line: &str) -> Option<AdoptionStage> {
        let stage_name = self.extract_value_from_line(line);
        
        if stage_name.is_empty() {
            return None;
        }
        
        Some(AdoptionStage {
            stage_name,
            description: String::new(),
            duration_estimate: None,
            prerequisites: Vec::new(),
            deliverables: Vec::new(),
            success_criteria: Vec::new(),
        })
    }
    
    fn calculate_theme_confidence_score(&self, theme: &StrategicTheme, candidate: &ThemeCandidate) -> Result<f64> {
        let mut score = 0.0;
        let mut factors = 0;
        
        // Factor 1: Category classification confidence
        if !candidate.category_hints.is_empty() {
            score += 0.2;
        }
        factors += 1;
        
        // Factor 2: Competitive advantages found
        if !theme.competitive_advantages.is_empty() {
            score += 0.2;
        }
        factors += 1;
        
        // Factor 3: ROI metrics found
        if !theme.roi_metrics.is_empty() {
            score += 0.2;
        }
        factors += 1;
        
        // Factor 4: Adoption pathways found
        if !theme.adoption_pathways.is_empty() {
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
}

/// Strategic theme organization system
pub struct StrategicThemeOrganizer {
    synthesizer: StrategicInsightSynthesizer,
    themes_by_category: HashMap<StrategicThemeCategory, Vec<StrategicTheme>>,
    themes_by_priority: HashMap<ImplementationPriority, Vec<StrategicTheme>>,
}

impl StrategicThemeOrganizer {
    pub fn new() -> Self {
        Self {
            synthesizer: StrategicInsightSynthesizer::new(),
            themes_by_category: HashMap::new(),
            themes_by_priority: HashMap::new(),
        }
    }
    
    /// Process content chunk and organize extracted themes
    pub fn process_chunk(&mut self, chunk: &ContentChunk) -> Result<Vec<StrategicTheme>> {
        let themes = self.synthesizer.extract_strategic_themes(chunk)?;
        
        // Organize themes by category and priority
        for theme in &themes {
            self.themes_by_category
                .entry(theme.category.clone())
                .or_insert_with(Vec::new)
                .push(theme.clone());
            
            self.themes_by_priority
                .entry(theme.implementation_priority.clone())
                .or_insert_with(Vec::new)
                .push(theme.clone());
        }
        
        Ok(themes)
    }
    
    /// Get themes by category
    pub fn get_themes_by_category(&self, category: &StrategicThemeCategory) -> Vec<&StrategicTheme> {
        self.themes_by_category
            .get(category)
            .map(|themes| themes.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get themes by priority
    pub fn get_themes_by_priority(&self, priority: &ImplementationPriority) -> Vec<&StrategicTheme> {
        self.themes_by_priority
            .get(priority)
            .map(|themes| themes.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get all themes sorted by priority
    pub fn get_all_themes_by_priority(&self) -> Vec<&StrategicTheme> {
        let mut all_themes = Vec::new();
        
        // Add themes in priority order
        for priority in [
            ImplementationPriority::Critical,
            ImplementationPriority::High,
            ImplementationPriority::Medium,
            ImplementationPriority::Low,
        ] {
            all_themes.extend(self.get_themes_by_priority(&priority));
        }
        
        all_themes
    }
    
    /// Generate strategic theme summary report
    pub fn generate_summary_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Strategic Theme Organization Summary\n\n");
        
        // Summary by category
        report.push_str("## Themes by Category\n\n");
        for category in StrategicThemeCategory::all() {
            let themes = self.get_themes_by_category(&category);
            report.push_str(&format!("### {}\n", category.as_str()));
            report.push_str(&format!("- Count: {}\n", themes.len()));
            
            for theme in themes {
                report.push_str(&format!("  - {}\n", theme.title));
            }
            report.push_str("\n");
        }
        
        // Summary by priority
        report.push_str("## Themes by Implementation Priority\n\n");
        for priority in [
            ImplementationPriority::Critical,
            ImplementationPriority::High,
            ImplementationPriority::Medium,
            ImplementationPriority::Low,
        ] {
            let themes = self.get_themes_by_priority(&priority);
            report.push_str(&format!("### {} Priority\n", priority.as_str()));
            report.push_str(&format!("- Count: {}\n", themes.len()));
            
            for theme in themes {
                report.push_str(&format!("  - {} ({})\n", theme.title, theme.category.as_str()));
            }
            report.push_str("\n");
        }
        
        report
    }
}

impl Default for StrategicInsightSynthesizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StrategicThemeOrganizer {
    fn default() -> Self {
        Self::new()
    }
}
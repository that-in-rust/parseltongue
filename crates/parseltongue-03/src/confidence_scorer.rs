//! Confidence scoring module for Tool 2
//!
//! Provides confidence scoring and threshold enforcement for simulation results

use crate::{ChangeRequest, SimulationPlan};
use parseltongue_01::{streaming::CodeGraph, types::CoreResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a confidence score for a simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceScore {
    /// Overall confidence score (0.0 to 1.0)
    pub score: f64,

    /// Individual component scores
    pub components: HashMap<String, f64>,

    /// Reasoning for the score
    pub reasoning: String,

    /// Timestamp when score was calculated
    pub calculated_at: chrono::DateTime<chrono::Utc>,
}

impl PartialEq for ConfidenceScore {
    fn eq(&self, other: &Self) -> bool {
        // Use epsilon comparison for floating point values
        const EPSILON: f64 = 1e-9;

        // Compare overall scores
        if (self.score - other.score).abs() > EPSILON {
            return false;
        }

        // Compare component scores
        if self.components.len() != other.components.len() {
            return false;
        }

        for (key, &value) in &self.components {
            match other.components.get(key) {
                Some(&other_value) => {
                    if (value - other_value).abs() > EPSILON {
                        return false;
                    }
                }
                None => return false,
            }
        }

        // Compare reasoning and timestamp
        self.reasoning == other.reasoning && self.calculated_at == other.calculated_at
    }
}

/// Confidence threshold for validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceThreshold {
    /// Minimum acceptable confidence score
    pub minimum_score: f64,

    /// Whether to enforce the threshold strictly
    pub strict: bool,

    /// Custom thresholds for different components
    pub component_thresholds: HashMap<String, f64>,
}

/// Confidence scoring engine
#[derive(Debug, Clone)]
pub struct ConfidenceScorer {
    /// Default threshold
    default_threshold: f64,

    /// Component weights for scoring
    component_weights: HashMap<String, f64>,
}

impl ConfidenceScore {
    /// Create a new confidence score
    pub fn new(score: f64, reasoning: String) -> Self {
        Self {
            score: score.clamp(0.0, 1.0),
            components: HashMap::new(),
            reasoning,
            calculated_at: chrono::Utc::now(),
        }
    }

    /// Create a confidence score with components
    pub fn with_components(
        score: f64,
        components: HashMap<String, f64>,
        reasoning: String,
    ) -> Self {
        Self {
            score: score.clamp(0.0, 1.0),
            components,
            reasoning,
            calculated_at: chrono::Utc::now(),
        }
    }

    /// Get the confidence score
    pub fn score(&self) -> f64 {
        self.score
    }

    /// Get reasoning for the score
    pub fn reasoning(&self) -> &str {
        &self.reasoning
    }

    /// Get component scores
    pub fn components(&self) -> &HashMap<String, f64> {
        &self.components
    }

    /// Check if the score meets a threshold
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.score >= threshold
    }

    /// Check if specific component meets threshold
    pub fn component_meets_threshold(&self, component: &str, threshold: f64) -> bool {
        self.components
            .get(component)
            .map(|&score| score >= threshold)
            .unwrap_or(false)
    }

    /// Get the lowest component score
    pub fn lowest_component_score(&self) -> f64 {
        self.components
            .values()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap_or(self.score)
    }

    /// Get the highest component score
    pub fn highest_component_score(&self) -> f64 {
        self.components
            .values()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap_or(self.score)
    }
}

impl ConfidenceThreshold {
    /// Create a new confidence threshold
    pub fn new(minimum_score: f64) -> Self {
        Self {
            minimum_score: minimum_score.clamp(0.0, 1.0),
            strict: true,
            component_thresholds: HashMap::new(),
        }
    }

    /// Create a non-strict threshold
    pub fn non_strict(minimum_score: f64) -> Self {
        Self {
            minimum_score: minimum_score.clamp(0.0, 1.0),
            strict: false,
            component_thresholds: HashMap::new(),
        }
    }

    /// Set a component threshold
    pub fn with_component_threshold(mut self, component: String, threshold: f64) -> Self {
        self.component_thresholds
            .insert(component, threshold.clamp(0.0, 1.0));
        self
    }

    /// Check if a confidence score meets the threshold
    pub fn meets_threshold(&self, score: &ConfidenceScore) -> bool {
        if !score.meets_threshold(self.minimum_score) {
            return false;
        }

        if self.strict {
            // In strict mode, all component thresholds must be met
            for (component, &threshold) in &self.component_thresholds {
                if !score.component_meets_threshold(component, threshold) {
                    return false;
                }
            }
        }

        true
    }

    /// Get validation errors for a score that doesn't meet threshold
    pub fn validation_errors(&self, score: &ConfidenceScore) -> Vec<String> {
        let mut errors = Vec::new();

        if score.score() < self.minimum_score {
            errors.push(format!(
                "Overall confidence {:.3} is below minimum threshold {:.3}",
                score.score(),
                self.minimum_score
            ));
        }

        for (component, &threshold) in &self.component_thresholds {
            if !score.component_meets_threshold(component, threshold) {
                if let Some(component_score) = score.components().get(component) {
                    errors.push(format!(
                        "Component '{}' confidence {:.3} is below threshold {:.3}",
                        component, component_score, threshold
                    ));
                } else {
                    errors.push(format!(
                        "Component '{}' is missing from confidence score",
                        component
                    ));
                }
            }
        }

        errors
    }
}

impl ConfidenceScorer {
    /// Create a new confidence scorer with default settings
    pub fn new() -> Self {
        let mut component_weights = HashMap::new();
        component_weights.insert("change_complexity".to_string(), 0.2);
        component_weights.insert("impact_analysis".to_string(), 0.25);
        component_weights.insert("dependency_coverage".to_string(), 0.2);
        component_weights.insert("test_coverage".to_string(), 0.15);
        component_weights.insert("reasoning_quality".to_string(), 0.2);

        Self {
            default_threshold: 0.8,
            component_weights,
        }
    }

    /// Create a confidence scorer with custom component weights
    pub fn with_weights(component_weights: HashMap<String, f64>) -> Self {
        Self {
            default_threshold: 0.8,
            component_weights,
        }
    }

    /// Calculate confidence score for a change request and simulation plan
    pub async fn calculate_confidence(
        &self,
        change_request: &ChangeRequest,
        code_graph: &CodeGraph,
        simulation_plan: &SimulationPlan,
    ) -> CoreResult<ConfidenceScore> {
        let mut components = HashMap::new();

        // Component 1: Change complexity analysis
        let complexity_score = self.analyze_change_complexity(change_request);
        components.insert("change_complexity".to_string(), complexity_score);

        // Component 2: Impact analysis quality
        let impact_score = self.analyze_impact_quality(simulation_plan, code_graph)?;
        components.insert("impact_analysis".to_string(), impact_score);

        // Component 3: Dependency coverage
        let dependency_score = self.analyze_dependency_coverage(change_request, code_graph);
        components.insert("dependency_coverage".to_string(), dependency_score);

        // Component 4: Test coverage (mocked for now)
        let test_score = self.analyze_test_coverage(change_request, code_graph);
        components.insert("test_coverage".to_string(), test_score);

        // Component 5: Reasoning quality (mocked for now)
        let reasoning_score = self.analyze_reasoning_quality(change_request, simulation_plan);
        components.insert("reasoning_quality".to_string(), reasoning_score);

        // Calculate weighted overall score
        let overall_score = self.calculate_weighted_score(&components);

        // Generate reasoning
        let reasoning = self.generate_reasoning(&components, overall_score);

        Ok(ConfidenceScore::with_components(
            overall_score,
            components,
            reasoning,
        ))
    }

    /// Analyze change complexity
    fn analyze_change_complexity(&self, change_request: &ChangeRequest) -> f64 {
        match change_request.metadata.complexity {
            crate::change_request::Complexity::Simple => 0.95,
            crate::change_request::Complexity::Moderate => 0.80,
            crate::change_request::Complexity::Complex => 0.65,
            crate::change_request::Complexity::VeryComplex => 0.50,
        }
    }

    /// Analyze impact analysis quality
    fn analyze_impact_quality(
        &self,
        simulation_plan: &SimulationPlan,
        _code_graph: &CodeGraph,
    ) -> CoreResult<f64> {
        // Check if plan has required phases
        let phases = simulation_plan.phases();
        let has_analysis = phases.iter().any(|p| p.starts_with('A'));
        let has_impact = phases.iter().any(|p| p.starts_with('B'));
        let has_application = phases.iter().any(|p| p.starts_with('C'));
        let has_validation = phases.iter().any(|p| p.starts_with('D'));

        let mut score = 0.0;
        if has_analysis {
            score += 0.25;
        }
        if has_impact {
            score += 0.25;
        }
        if has_application {
            score += 0.25;
        }
        if has_validation {
            score += 0.25;
        }

        // Check for validation criteria in steps
        let steps_with_validation = simulation_plan
            .steps()
            .iter()
            .filter(|step| !step.validation_criteria.is_empty())
            .count();

        if !simulation_plan.steps().is_empty() {
            let validation_ratio =
                steps_with_validation as f64 / simulation_plan.steps().len() as f64;
            score = score * 0.6 + validation_ratio * 0.4;
        }

        Ok(score.clamp(0.0, 1.0))
    }

    /// Analyze dependency coverage
    fn analyze_dependency_coverage(
        &self,
        _change_request: &ChangeRequest,
        _code_graph: &CodeGraph,
    ) -> f64 {
        // Mock implementation - in real version this would analyze actual dependencies
        0.85
    }

    /// Analyze test coverage
    fn analyze_test_coverage(
        &self,
        _change_request: &ChangeRequest,
        _code_graph: &CodeGraph,
    ) -> f64 {
        // Mock implementation - in real version this would analyze actual test coverage
        0.75
    }

    /// Analyze reasoning quality
    fn analyze_reasoning_quality(
        &self,
        change_request: &ChangeRequest,
        simulation_plan: &SimulationPlan,
    ) -> f64 {
        let mut score: f64 = 0.5; // Base score

        // Bonus for having reasoning
        if change_request.metadata.reasoning.is_some() {
            score += 0.2;
        }

        // Bonus for detailed description
        if change_request.description.len() > 50 {
            score += 0.15;
        }

        // Bonus for comprehensive simulation plan
        if simulation_plan.steps().len() >= 4 {
            score += 0.15;
        }

        score.clamp(0.0, 1.0)
    }

    /// Calculate weighted score from components
    fn calculate_weighted_score(&self, components: &HashMap<String, f64>) -> f64 {
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for (component, &score) in components {
            let weight = self.component_weights.get(component).unwrap_or(&0.2);
            weighted_sum += score * weight;
            total_weight += weight;
        }

        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        }
    }

    /// Generate reasoning for the confidence score
    fn generate_reasoning(&self, components: &HashMap<String, f64>, overall_score: f64) -> String {
        let mut reasoning = format!("Overall confidence: {:.3}\n", overall_score);

        for (component, &score) in components {
            let weight = self.component_weights.get(component).unwrap_or(&0.2);
            reasoning.push_str(&format!(
                "- {}: {:.3} (weight: {:.2})\n",
                component.replace('_', " "),
                score,
                weight
            ));
        }

        if overall_score >= 0.8 {
            reasoning.push_str("High confidence: Simulation is likely to be accurate");
        } else if overall_score >= 0.6 {
            reasoning.push_str("Moderate confidence: Simulation should be reviewed carefully");
        } else {
            reasoning.push_str("Low confidence: Simulation results may be unreliable");
        }

        reasoning
    }

    /// Get default threshold
    pub fn default_threshold(&self) -> f64 {
        self.default_threshold
    }

    /// Set default threshold
    pub fn set_default_threshold(&mut self, threshold: f64) {
        self.default_threshold = threshold.clamp(0.0, 1.0);
    }
}

impl Default for ConfidenceScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::change_request::{ChangeType, Complexity};
    use parseltongue_01::{streaming::CodeNode, types::ISGL1Key};
    use std::path::PathBuf;

    fn create_test_change_request() -> ChangeRequest {
        let key = ISGL1Key::new(
            PathBuf::from("/test/src/lib.rs"),
            "lib.rs".to_string(),
            "test_function".to_string(),
        );

        ChangeRequest::new(
            key,
            ChangeType::Modify,
            "Add error handling to test function".to_string(),
            "fn test_function() {}".to_string(),
            "fn test_function() -> Result<(), Error> { Ok(()) }".to_string(),
        )
        .with_complexity(Complexity::Moderate)
        .with_reasoning("Need proper error handling".to_string())
    }

    fn create_test_simulation_plan() -> SimulationPlan {
        SimulationPlan::mock()
    }

    fn create_test_code_graph() -> CodeGraph {
        let mut graph = CodeGraph::new();
        let key = ISGL1Key::new(
            PathBuf::from("/test/src/lib.rs"),
            "lib.rs".to_string(),
            "test_function".to_string(),
        );
        let node = CodeNode {
            current_code: "fn test_function() {}".to_string(),
            future_code: None,
            interface_signature: Some("fn test_function()".to_string()),
            tdd_classification: Some("unit_test".to_string()),
            current_id: 1,
            future_id: 1,
            lsp_meta_data: None,
        };
        graph.insert_node(key, node).unwrap();
        graph
    }

    #[test]
    fn test_confidence_score_creation() {
        let score = ConfidenceScore::new(0.85, "Test reasoning".to_string());

        assert_eq!(score.score(), 0.85);
        assert_eq!(score.reasoning(), "Test reasoning");
        assert!(score.meets_threshold(0.8));
        assert!(!score.meets_threshold(0.9));
    }

    #[test]
    fn test_confidence_score_clamping() {
        let high_score = ConfidenceScore::new(1.5, "Too high".to_string());
        assert_eq!(high_score.score(), 1.0);

        let low_score = ConfidenceScore::new(-0.5, "Too low".to_string());
        assert_eq!(low_score.score(), 0.0);
    }

    #[test]
    fn test_confidence_score_with_components() {
        let mut components = HashMap::new();
        components.insert("test".to_string(), 0.9);

        let score =
            ConfidenceScore::with_components(0.85, components, "Test reasoning".to_string());

        assert_eq!(score.score(), 0.85);
        assert_eq!(score.components().len(), 1);
        assert!(score.component_meets_threshold("test", 0.8));
        assert!(!score.component_meets_threshold("test", 0.95));
    }

    #[test]
    fn test_confidence_threshold() {
        let threshold = ConfidenceThreshold::new(0.8);

        let good_score = ConfidenceScore::new(0.85, "Good score".to_string());
        assert!(threshold.meets_threshold(&good_score));

        let bad_score = ConfidenceScore::new(0.75, "Bad score".to_string());
        assert!(!threshold.meets_threshold(&bad_score));
    }

    #[test]
    fn test_confidence_threshold_with_components() {
        let threshold =
            ConfidenceThreshold::new(0.8).with_component_threshold("test".to_string(), 0.9);

        let mut components = HashMap::new();
        components.insert("test".to_string(), 0.85);

        let score =
            ConfidenceScore::with_components(0.85, components, "Test reasoning".to_string());

        // Overall score meets threshold but component doesn't
        assert!(!threshold.meets_threshold(&score));
    }

    #[test]
    fn test_non_strict_threshold() {
        let threshold =
            ConfidenceThreshold::non_strict(0.8).with_component_threshold("test".to_string(), 0.9);

        let mut components = HashMap::new();
        components.insert("test".to_string(), 0.85);

        let score =
            ConfidenceScore::with_components(0.85, components, "Test reasoning".to_string());

        // Non-strict mode: overall score meets threshold, so it passes
        assert!(threshold.meets_threshold(&score));
    }

    #[test]
    fn test_validation_errors() {
        let threshold =
            ConfidenceThreshold::new(0.8).with_component_threshold("test".to_string(), 0.9);

        let score = ConfidenceScore::new(0.75, "Low score".to_string());
        let errors = threshold.validation_errors(&score);

        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("below minimum threshold")));
    }

    #[tokio::test]
    async fn test_confidence_scorer() {
        let scorer = ConfidenceScorer::new();
        let change_request = create_test_change_request();
        let simulation_plan = create_test_simulation_plan();
        let code_graph = create_test_code_graph();

        let score = scorer
            .calculate_confidence(&change_request, &code_graph, &simulation_plan)
            .await
            .unwrap();

        assert!(score.score() >= 0.0);
        assert!(score.score() <= 1.0);
        assert!(!score.reasoning().is_empty());
        assert!(!score.components().is_empty());
    }

    #[test]
    fn test_default_confidence_scorer() {
        let scorer = ConfidenceScorer::default();
        assert_eq!(scorer.default_threshold(), 0.8);
    }

    #[test]
    fn test_custom_weights() {
        let mut weights = HashMap::new();
        weights.insert("custom".to_string(), 1.0);

        let scorer = ConfidenceScorer::with_weights(weights);
        assert_eq!(scorer.default_threshold(), 0.8);
    }

    #[test]
    fn test_extreme_scores() {
        let perfect_score = ConfidenceScore::new(1.0, "Perfect".to_string());
        assert_eq!(perfect_score.highest_component_score(), 1.0);
        assert_eq!(perfect_score.lowest_component_score(), 1.0);

        let mut components = HashMap::new();
        components.insert("high".to_string(), 0.9);
        components.insert("low".to_string(), 0.1);

        let varied_score = ConfidenceScore::with_components(0.5, components, "Varied".to_string());
        assert_eq!(varied_score.highest_component_score(), 0.9);
        assert_eq!(varied_score.lowest_component_score(), 0.1);
    }
}

//! Debugging info module for Tool 2
//!
//! Provides rubber duck debugging artifacts and validation information

use crate::{ChangeRequest, ConfidenceScore, SimulationPlan};
use parseltongue_01::types::CoreResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Comprehensive debugging information for simulation validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebuggingInfo {
    /// Unique identifier for this debugging info
    pub id: Uuid,

    /// Change request being debugged
    pub change_request: ChangeRequest,

    /// Simulation plan being validated
    pub simulation_plan: SimulationPlan,

    /// Confidence score analysis
    pub confidence_score: Option<ConfidenceScore>,

    /// Rubber duck debugging questions
    pub rubber_duck_questions: Vec<RubberDuckQuestion>,

    /// Step-by-step explanations
    pub step_explanations: Vec<StepExplanation>,

    /// Validation checklist
    pub validation_checklist: ValidationChecklist,

    /// Common pitfalls and warnings
    pub pitfalls: Vec<Pitfall>,

    /// Metadata about debugging session
    pub metadata: DebuggingMetadata,
}

/// Rubber duck debugging question for self-validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RubberDuckQuestion {
    /// Unique identifier for the question
    pub id: Uuid,

    /// The question to ask
    pub question: String,

    /// Category of the question
    pub category: QuestionCategory,

    /// Expected answer format
    pub answer_format: AnswerFormat,

    /// Why this question is important
    pub reasoning: String,

    /// Tips for answering
    pub tips: Vec<String>,
}

/// Categories of rubber duck questions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestionCategory {
    /// Questions about change scope
    Scope,
    /// Questions about implementation details
    Implementation,
    /// Questions about testing strategy
    Testing,
    /// Questions about impact assessment
    Impact,
    /// Questions about validation approach
    Validation,
    /// Questions about alternatives considered
    Alternatives,
}

/// Expected answer format for questions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnswerFormat {
    /// Free text answer
    Text,
    /// Yes/No answer
    YesNo,
    /// Multiple choice
    MultipleChoice(Vec<String>),
    /// Numeric answer
    Numeric,
    /// List of items
    List,
}

/// Detailed explanation for each simulation step
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepExplanation {
    /// Simulation step this explains
    pub step_id: Uuid,

    /// Title of the explanation
    pub title: String,

    /// What this step does in simple terms
    pub what_it_does: String,

    /// Why this step is necessary
    pub why_its_necessary: String,

    /// How to validate this step completed successfully
    pub how_to_validate: String,

    /// Common issues and solutions
    pub common_issues: Vec<CommonIssue>,

    /// Success criteria
    pub success_criteria: Vec<String>,
}

/// Common issue that might occur during a step
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonIssue {
    /// Description of the issue
    pub issue: String,

    /// Symptoms to look for
    pub symptoms: Vec<String>,

    /// Solutions to try
    pub solutions: Vec<String>,

    /// Whether this issue blocks progress
    pub is_blocking: bool,
}

/// Validation checklist for the simulation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationChecklist {
    /// Items to validate before starting
    pub pre_validation: Vec<ChecklistItem>,

    /// Items to validate during simulation
    pub during_validation: Vec<ChecklistItem>,

    /// Items to validate after simulation
    pub post_validation: Vec<ChecklistItem>,

    /// Overall validation score (0.0 to 1.0)
    pub overall_score: f64,
}

/// Individual checklist item
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecklistItem {
    /// Unique identifier
    pub id: Uuid,

    /// Description of what to check
    pub description: String,

    /// Whether this item is required
    pub required: bool,

    /// Current status
    pub status: ChecklistStatus,

    /// Notes about this check
    pub notes: String,

    /// Who should perform this check
    pub responsible_party: String,
}

/// Status of checklist items
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChecklistStatus {
    /// Not yet checked
    Pending,
    /// In progress
    InProgress,
    /// Passed validation
    Passed,
    /// Failed validation
    Failed,
    /// Not applicable
    NotApplicable,
}

/// Common pitfall to watch out for
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pitfall {
    /// Title of the pitfall
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Why this is a pitfall
    pub why_its_pitfall: String,

    /// How to avoid it
    pub avoidance_strategies: Vec<String>,

    /// Severity level
    pub severity: PitfallSeverity,

    /// Which phase this affects
    pub affected_phases: Vec<String>,
}

/// Severity levels for pitfalls
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PitfallSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Metadata for debugging information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggingMetadata {
    /// When debugging info was created
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Who created this debugging info
    pub author: String,

    /// Version of debugging format
    pub version: String,

    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Main debugging info generator
#[derive(Debug, Clone)]
pub struct DebuggingInfoGenerator {
    /// Configuration for generation
    config: GeneratorConfig,
}

/// Configuration for debugging info generation
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Whether to include detailed explanations
    pub include_detailed_explanations: bool,

    /// Whether to include rubber duck questions
    pub include_rubber_duck_questions: bool,

    /// Whether to include validation checklist
    pub include_validation_checklist: bool,

    /// Which categories of questions to include
    pub question_categories: Vec<QuestionCategory>,

    /// Maximum number of questions per category
    pub max_questions_per_category: usize,
}

impl DebuggingInfo {
    /// Create a new debugging info structure
    pub fn new(change_request: ChangeRequest, simulation_plan: SimulationPlan) -> Self {
        Self {
            id: Uuid::new_v4(),
            change_request,
            simulation_plan,
            confidence_score: None,
            rubber_duck_questions: Vec::new(),
            step_explanations: Vec::new(),
            validation_checklist: ValidationChecklist {
                pre_validation: Vec::new(),
                during_validation: Vec::new(),
                post_validation: Vec::new(),
                overall_score: 0.0,
            },
            pitfalls: Vec::new(),
            metadata: DebuggingMetadata {
                created_at: chrono::Utc::now(),
                author: "parseltongue-03".to_string(),
                version: "1.0".to_string(),
                tags: Vec::new(),
            },
        }
    }

    /// Add rubber duck questions
    pub fn with_rubber_duck_questions(mut self, questions: Vec<RubberDuckQuestion>) -> Self {
        self.rubber_duck_questions = questions;
        self
    }

    /// Add step explanations
    pub fn with_step_explanations(mut self, explanations: Vec<StepExplanation>) -> Self {
        self.step_explanations = explanations;
        self
    }

    /// Add validation checklist
    pub fn with_validation_checklist(mut self, checklist: ValidationChecklist) -> Self {
        self.validation_checklist = checklist;
        self
    }

    /// Add pitfalls
    pub fn with_pitfalls(mut self, pitfalls: Vec<Pitfall>) -> Self {
        self.pitfalls = pitfalls;
        self
    }

    /// Set confidence score
    pub fn with_confidence_score(mut self, confidence_score: ConfidenceScore) -> Self {
        self.confidence_score = Some(confidence_score);
        self
    }

    /// Check if has rubber duck questions
    pub fn has_questions(&self) -> bool {
        !self.rubber_duck_questions.is_empty()
    }

    /// Check if has step explanations
    pub fn has_step_explanations(&self) -> bool {
        !self.step_explanations.is_empty()
    }

    /// Check if has validation checklist
    pub fn has_validation_checklist(&self) -> bool {
        !self.validation_checklist.pre_validation.is_empty()
            || !self.validation_checklist.during_validation.is_empty()
            || !self.validation_checklist.post_validation.is_empty()
    }

    /// Export debugging artifacts
    pub fn export_artifacts(&self) -> CoreResult<HashMap<String, String>> {
        let mut artifacts = HashMap::new();

        // Export rubber duck questions
        if self.has_questions() {
            let questions_text = self.export_rubber_duck_questions()?;
            artifacts.insert("rubber_duck_questions.md".to_string(), questions_text);
        }

        // Export step explanations
        if self.has_step_explanations() {
            let explanations_text = self.export_step_explanations()?;
            artifacts.insert("step_explanations.md".to_string(), explanations_text);
        }

        // Export validation checklist
        if self.has_validation_checklist() {
            let checklist_text = self.export_validation_checklist()?;
            artifacts.insert("validation_checklist.md".to_string(), checklist_text);
        }

        // Export pitfalls
        if !self.pitfalls.is_empty() {
            let pitfalls_text = self.export_pitfalls()?;
            artifacts.insert("pitfalls.md".to_string(), pitfalls_text);
        }

        // Export summary
        let summary_text = self.export_summary()?;
        artifacts.insert("debugging_summary.md".to_string(), summary_text);

        Ok(artifacts)
    }

    /// Export rubber duck questions as markdown
    fn export_rubber_duck_questions(&self) -> CoreResult<String> {
        let mut output = String::new();
        output.push_str("# Rubber Duck Debugging Questions\n\n");

        for question in &self.rubber_duck_questions {
            output.push_str(&format!("## {}\n\n", question.question));
            output.push_str(&format!("**Category:** {:?}\n\n", question.category));
            output.push_str(&format!("**Why this matters:** {}\n\n", question.reasoning));

            if !question.tips.is_empty() {
                output.push_str("**Tips for answering:**\n");
                for tip in &question.tips {
                    output.push_str(&format!("- {}\n", tip));
                }
                output.push_str("\n");
            }

            output.push_str("---\n\n");
        }

        Ok(output)
    }

    /// Export step explanations as markdown
    fn export_step_explanations(&self) -> CoreResult<String> {
        let mut output = String::new();
        output.push_str("# Step-by-Step Explanations\n\n");

        for explanation in &self.step_explanations {
            output.push_str(&format!("## {}\n\n", explanation.title));
            output.push_str(&format!(
                "**What it does:** {}\n\n",
                explanation.what_it_does
            ));
            output.push_str(&format!(
                "**Why it's necessary:** {}\n\n",
                explanation.why_its_necessary
            ));
            output.push_str(&format!(
                "**How to validate:** {}\n\n",
                explanation.how_to_validate
            ));

            if !explanation.success_criteria.is_empty() {
                output.push_str("**Success criteria:**\n");
                for criterion in &explanation.success_criteria {
                    output.push_str(&format!("- [ ] {}\n", criterion));
                }
                output.push_str("\n");
            }

            if !explanation.common_issues.is_empty() {
                output.push_str("**Common issues:**\n");
                for issue in &explanation.common_issues {
                    output.push_str(&format!(
                        "- **{}**: {}\n",
                        issue.issue,
                        issue.solutions.join(", ")
                    ));
                }
                output.push_str("\n");
            }

            output.push_str("---\n\n");
        }

        Ok(output)
    }

    /// Export validation checklist as markdown
    fn export_validation_checklist(&self) -> CoreResult<String> {
        let mut output = String::new();
        output.push_str("# Validation Checklist\n\n");

        // Pre-validation
        if !self.validation_checklist.pre_validation.is_empty() {
            output.push_str("## Pre-Simulation Validation\n\n");
            for item in &self.validation_checklist.pre_validation {
                let checkbox = match item.status {
                    ChecklistStatus::Passed => "- [x]",
                    ChecklistStatus::Failed => "- [ ] ❌",
                    ChecklistStatus::InProgress => "- [ ] 🔄",
                    _ => "- [ ]",
                };
                output.push_str(&format!(
                    "{} {} ({})\n",
                    checkbox, item.description, item.responsible_party
                ));
            }
            output.push_str("\n");
        }

        // During validation
        if !self.validation_checklist.during_validation.is_empty() {
            output.push_str("## During Simulation Validation\n\n");
            for item in &self.validation_checklist.during_validation {
                let checkbox = match item.status {
                    ChecklistStatus::Passed => "- [x]",
                    ChecklistStatus::Failed => "- [ ] ❌",
                    ChecklistStatus::InProgress => "- [ ] 🔄",
                    _ => "- [ ]",
                };
                output.push_str(&format!(
                    "{} {} ({})\n",
                    checkbox, item.description, item.responsible_party
                ));
            }
            output.push_str("\n");
        }

        // Post-validation
        if !self.validation_checklist.post_validation.is_empty() {
            output.push_str("## Post-Simulation Validation\n\n");
            for item in &self.validation_checklist.post_validation {
                let checkbox = match item.status {
                    ChecklistStatus::Passed => "- [x]",
                    ChecklistStatus::Failed => "- [ ] ❌",
                    ChecklistStatus::InProgress => "- [ ] 🔄",
                    _ => "- [ ]",
                };
                output.push_str(&format!(
                    "{} {} ({})\n",
                    checkbox, item.description, item.responsible_party
                ));
            }
            output.push_str("\n");
        }

        output.push_str(&format!(
            "**Overall Validation Score:** {:.1}%\n\n",
            self.validation_checklist.overall_score * 100.0
        ));

        Ok(output)
    }

    /// Export pitfalls as markdown
    fn export_pitfalls(&self) -> CoreResult<String> {
        let mut output = String::new();
        output.push_str("# Common Pitfalls to Avoid\n\n");

        for pitfall in &self.pitfalls {
            let severity_emoji = match pitfall.severity {
                PitfallSeverity::Low => "🟡",
                PitfallSeverity::Medium => "🟠",
                PitfallSeverity::High => "🔴",
                PitfallSeverity::Critical => "💀",
            };

            output.push_str(&format!(
                "## {} {} {}\n\n",
                severity_emoji, pitfall.title, severity_emoji
            ));
            output.push_str(&format!("**Description:** {}\n\n", pitfall.description));
            output.push_str(&format!(
                "**Why it's a pitfall:** {}\n\n",
                pitfall.why_its_pitfall
            ));

            if !pitfall.affected_phases.is_empty() {
                output.push_str(&format!(
                    "**Affected phases:** {}\n\n",
                    pitfall.affected_phases.join(", ")
                ));
            }

            output.push_str("**How to avoid:**\n");
            for strategy in &pitfall.avoidance_strategies {
                output.push_str(&format!("- {}\n", strategy));
            }
            output.push_str("\n---\n\n");
        }

        Ok(output)
    }

    /// Export summary as markdown
    fn export_summary(&self) -> CoreResult<String> {
        let mut output = String::new();
        output.push_str("# Debugging Summary\n\n");

        output.push_str("## Overview\n\n");
        output.push_str(&format!(
            "**Change:** {}\n",
            self.change_request.description
        ));
        output.push_str(&format!(
            "**Target:** {}\n",
            self.change_request.target.key.interface_name
        ));
        output.push_str(&format!(
            "**Type:** {:?}\n",
            self.change_request.change_type
        ));
        output.push_str(&format!(
            "**Complexity:** {:?}\n\n",
            self.change_request.metadata.complexity
        ));

        if let Some(confidence) = &self.confidence_score {
            output.push_str(&format!(
                "**Confidence Score:** {:.1}%\n",
                confidence.score() * 100.0
            ));
            output.push_str(&format!(
                "**Confidence Reasoning:** {}\n\n",
                confidence.reasoning()
            ));
        }

        output.push_str("## Available Artifacts\n\n");
        output.push_str("- [Rubber Duck Questions](rubber_duck_questions.md)\n");
        output.push_str("- [Step Explanations](step_explanations.md)\n");
        output.push_str("- [Validation Checklist](validation_checklist.md)\n");
        output.push_str("- [Common Pitfalls](pitfalls.md)\n\n");

        output.push_str("## Quick Start\n\n");
        output.push_str("1. Start with the rubber duck questions to validate your understanding\n");
        output.push_str("2. Review the step explanations to understand what each phase does\n");
        output.push_str("3. Use the validation checklist during simulation\n");
        output.push_str("4. Watch out for the common pitfalls\n\n");

        Ok(output)
    }

    /// Generate rubber duck debugging artifacts
    pub async fn generate_rubber_duck_artifacts(
        &mut self,
        change_request: &ChangeRequest,
        simulation_plan: &SimulationPlan,
    ) -> CoreResult<()> {
        let generator = DebuggingInfoGenerator::new();
        let additional_info = generator
            .generate_debugging_info(
                change_request,
                simulation_plan,
                self.confidence_score.as_ref(),
            )
            .await?;

        // Merge the additional info
        self.rubber_duck_questions
            .extend(additional_info.rubber_duck_questions);
        self.step_explanations
            .extend(additional_info.step_explanations);
        self.pitfalls.extend(additional_info.pitfalls);

        Ok(())
    }
}

impl DebuggingInfoGenerator {
    /// Create a new debugging info generator
    pub fn new() -> Self {
        let config = GeneratorConfig {
            include_detailed_explanations: true,
            include_rubber_duck_questions: true,
            include_validation_checklist: true,
            question_categories: vec![
                QuestionCategory::Scope,
                QuestionCategory::Implementation,
                QuestionCategory::Testing,
                QuestionCategory::Impact,
                QuestionCategory::Validation,
            ],
            max_questions_per_category: 5,
        };

        Self { config }
    }

    /// Create a generator with custom configuration
    pub fn with_config(config: GeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate comprehensive debugging artifacts
    pub async fn generate_debugging_info(
        &self,
        change_request: &ChangeRequest,
        simulation_plan: &SimulationPlan,
        confidence_score: Option<&ConfidenceScore>,
    ) -> CoreResult<DebuggingInfo> {
        let mut debugging_info =
            DebuggingInfo::new(change_request.clone(), simulation_plan.clone());

        // Generate rubber duck questions
        if self.config.include_rubber_duck_questions {
            let questions = self.generate_rubber_duck_questions(change_request, simulation_plan)?;
            debugging_info = debugging_info.with_rubber_duck_questions(questions);
        }

        // Generate step explanations
        if self.config.include_detailed_explanations {
            let explanations = self.generate_step_explanations(simulation_plan)?;
            debugging_info = debugging_info.with_step_explanations(explanations);
        }

        // Generate validation checklist
        if self.config.include_validation_checklist {
            let checklist = self.generate_validation_checklist(change_request, simulation_plan)?;
            debugging_info = debugging_info.with_validation_checklist(checklist);
        }

        // Generate pitfalls
        let pitfalls = self.generate_pitfalls(change_request, simulation_plan)?;
        debugging_info = debugging_info.with_pitfalls(pitfalls);

        // Add confidence score if provided
        if let Some(score) = confidence_score {
            debugging_info = debugging_info.with_confidence_score(score.clone());
        }

        Ok(debugging_info)
    }

    /// Generate rubber duck questions
    fn generate_rubber_duck_questions(
        &self,
        _change_request: &ChangeRequest,
        _simulation_plan: &SimulationPlan,
    ) -> CoreResult<Vec<RubberDuckQuestion>> {
        let mut questions = Vec::new();

        // Scope questions
        questions.push(RubberDuckQuestion {
            id: Uuid::new_v4(),
            question: "What exactly are you trying to change and why?".to_string(),
            category: QuestionCategory::Scope,
            answer_format: AnswerFormat::Text,
            reasoning: "Understanding the exact scope helps prevent scope creep and ensures we're solving the right problem.".to_string(),
            tips: vec![
                "Be specific about what needs to change".to_string(),
                "Explain the business or technical motivation".to_string(),
                "Consider if there are alternative approaches".to_string(),
            ],
        });

        questions.push(RubberDuckQuestion {
            id: Uuid::new_v4(),
            question: "What will stay the same after this change?".to_string(),
            category: QuestionCategory::Scope,
            answer_format: AnswerFormat::List,
            reasoning: "Identifying what doesn't change helps define boundaries and reduces risk."
                .to_string(),
            tips: vec![
                "List APIs that remain unchanged".to_string(),
                "Identify data structures that stay the same".to_string(),
                "Consider user-facing behavior that doesn't change".to_string(),
            ],
        });

        // Implementation questions
        questions.push(RubberDuckQuestion {
            id: Uuid::new_v4(),
            question: "Have you considered the edge cases?".to_string(),
            category: QuestionCategory::Implementation,
            answer_format: AnswerFormat::List,
            reasoning: "Edge cases are where most bugs hide. Thinking about them upfront prevents issues later.".to_string(),
            tips: vec![
                "Consider empty inputs, null values, boundary conditions".to_string(),
                "Think about error conditions and failure modes".to_string(),
                "Consider concurrent access scenarios".to_string(),
            ],
        });

        questions.push(RubberDuckQuestion {
            id: Uuid::new_v4(),
            question: "What assumptions are you making?".to_string(),
            category: QuestionCategory::Implementation,
            answer_format: AnswerFormat::List,
            reasoning: "Hidden assumptions can lead to unexpected behavior. Making them explicit helps validation.".to_string(),
            tips: vec![
                "List assumptions about input data".to_string(),
                "Consider assumptions about the environment".to_string(),
                "Think about assumptions about dependencies".to_string(),
            ],
        });

        // Testing questions
        questions.push(RubberDuckQuestion {
            id: Uuid::new_v4(),
            question: "How will you test this change works?".to_string(),
            category: QuestionCategory::Testing,
            answer_format: AnswerFormat::List,
            reasoning:
                "Having a clear testing strategy ensures quality and enables confident deployment."
                    .to_string(),
            tips: vec![
                "Consider unit tests for individual components".to_string(),
                "Think about integration tests".to_string(),
                "Consider manual testing scenarios".to_string(),
            ],
        });

        // Impact questions
        questions.push(RubberDuckQuestion {
            id: Uuid::new_v4(),
            question: "What could break if this change goes wrong?".to_string(),
            category: QuestionCategory::Impact,
            answer_format: AnswerFormat::List,
            reasoning: "Understanding potential failure modes helps prepare rollback strategies and monitoring.".to_string(),
            tips: vec![
                "Think about dependent systems".to_string(),
                "Consider user experience impacts".to_string(),
                "Think about data consistency issues".to_string(),
            ],
        });

        // Validation questions
        questions.push(RubberDuckQuestion {
            id: Uuid::new_v4(),
            question: "How will you know the change was successful?".to_string(),
            category: QuestionCategory::Validation,
            answer_format: AnswerFormat::List,
            reasoning:
                "Clear success criteria help validate that the change achieved its intended effect."
                    .to_string(),
            tips: vec![
                "Define measurable success metrics".to_string(),
                "Consider both technical and business metrics".to_string(),
                "Think about short and long term indicators".to_string(),
            ],
        });

        Ok(questions)
    }

    /// Generate step explanations
    fn generate_step_explanations(
        &self,
        simulation_plan: &SimulationPlan,
    ) -> CoreResult<Vec<StepExplanation>> {
        let mut explanations = Vec::new();

        for step in simulation_plan.steps() {
            let explanation = match step.phase.as_str() {
                phase if phase.starts_with('A') => self.generate_analysis_explanation(step),
                phase if phase.starts_with('B') => self.generate_impact_explanation(step),
                "C" => self.generate_application_explanation(step),
                "D" => self.generate_validation_explanation(step),
                _ => self.generate_generic_explanation(step),
            };

            explanations.push(explanation);
        }

        Ok(explanations)
    }

    /// Generate analysis phase explanation
    fn generate_analysis_explanation(
        &self,
        step: &crate::simulation_plan::SimulationStep,
    ) -> StepExplanation {
        StepExplanation {
            step_id: step.id,
            title: step.title.clone(),
            what_it_does: format!("Analyzes {} to understand the current state and proposed changes",
                if step.phase == "A01" { "the current code" } else { "the proposed changes" }),
            why_its_necessary: "Understanding the current implementation and proposed changes helps identify potential issues before they affect the system.".to_string(),
            how_to_validate: "Review the analysis output to ensure it captures all relevant aspects of the code and changes.".to_string(),
            common_issues: vec![
                CommonIssue {
                    issue: "Incomplete analysis".to_string(),
                    symptoms: vec!["Missing dependencies in analysis".to_string(), "Incomplete code understanding".to_string()],
                    solutions: vec!["Review analysis manually".to_string(), "Expand analysis scope".to_string(), "Add additional analysis tools".to_string()],
                    is_blocking: true,
                },
            ],
            success_criteria: vec![
                "All relevant code components identified".to_string(),
                "Dependencies correctly mapped".to_string(),
                "Analysis covers all change aspects".to_string(),
            ],
        }
    }

    /// Generate impact assessment explanation
    fn generate_impact_explanation(
        &self,
        step: &crate::simulation_plan::SimulationStep,
    ) -> StepExplanation {
        StepExplanation {
            step_id: step.id,
            title: step.title.clone(),
            what_it_does: "Assesses how the changes will affect other parts of the system".to_string(),
            why_its_necessary: "Understanding impact helps prevent unintended consequences and prepare mitigation strategies.".to_string(),
            how_to_validate: "Review the impact assessment for completeness and accuracy of affected components.".to_string(),
            common_issues: vec![
                CommonIssue {
                    issue: "Missed dependencies".to_string(),
                    symptoms: vec!["Unexpected runtime errors".to_string(), "Missing integration points".to_string()],
                    solutions: vec!["Expand dependency analysis".to_string(), "Manual code review".to_string(), "Static analysis tools".to_string()],
                    is_blocking: true,
                },
            ],
            success_criteria: vec![
                "All affected components identified".to_string(),
                "Impact severity correctly assessed".to_string(),
                "Mitigation strategies prepared".to_string(),
            ],
        }
    }

    /// Generate change application explanation
    fn generate_application_explanation(
        &self,
        step: &crate::simulation_plan::SimulationStep,
    ) -> StepExplanation {
        StepExplanation {
            step_id: step.id,
            title: step.title.clone(),
            what_it_does: "Applies the proposed code changes in a controlled simulation environment".to_string(),
            why_its_necessary: "Actually applying the changes tests whether they work as intended and interact correctly with existing code.".to_string(),
            how_to_validate: "Verify that changes apply correctly and don't break existing functionality.".to_string(),
            common_issues: vec![
                CommonIssue {
                    issue: "Compilation errors".to_string(),
                    symptoms: vec!["Build fails".to_string(), "Type errors".to_string()],
                    solutions: vec!["Fix syntax errors".to_string(), "Resolve type mismatches".to_string(), "Add missing imports".to_string()],
                    is_blocking: true,
                },
            ],
            success_criteria: vec![
                "Code compiles successfully".to_string(),
                "Basic functionality preserved".to_string(),
                "New features working as expected".to_string(),
            ],
        }
    }

    /// Generate validation explanation
    fn generate_validation_explanation(
        &self,
        step: &crate::simulation_plan::SimulationStep,
    ) -> StepExplanation {
        StepExplanation {
            step_id: step.id,
            title: step.title.clone(),
            what_it_does: "Validates that the applied changes meet all requirements and don't introduce regressions".to_string(),
            why_its_necessary: "Validation ensures quality and confirms that the changes achieve their intended purpose without side effects.".to_string(),
            how_to_validate: "Run tests, review code quality metrics, and verify functional requirements are met.".to_string(),
            common_issues: vec![
                CommonIssue {
                    issue: "Test failures".to_string(),
                    symptoms: vec!["Unit tests failing".to_string(), "Integration test errors".to_string()],
                    solutions: vec!["Fix failing tests".to_string(), "Update test expectations".to_string(), "Add missing test coverage".to_string()],
                    is_blocking: false,
                },
            ],
            success_criteria: vec![
                "All tests pass".to_string(),
                "Code quality standards met".to_string(),
                "Functional requirements satisfied".to_string(),
            ],
        }
    }

    /// Generate generic explanation for unknown step types
    fn generate_generic_explanation(
        &self,
        step: &crate::simulation_plan::SimulationStep,
    ) -> StepExplanation {
        StepExplanation {
            step_id: step.id,
            title: step.title.clone(),
            what_it_does: step.description.clone(),
            why_its_necessary: "This step is part of the simulation process and helps ensure a comprehensive analysis.".to_string(),
            how_to_validate: "Follow the step's validation criteria to ensure successful completion.".to_string(),
            common_issues: vec![],
            success_criteria: step.validation_criteria.clone(),
        }
    }

    /// Generate validation checklist
    fn generate_validation_checklist(
        &self,
        change_request: &ChangeRequest,
        _simulation_plan: &SimulationPlan,
    ) -> CoreResult<ValidationChecklist> {
        let pre_validation = vec![
            ChecklistItem {
                id: Uuid::new_v4(),
                description: "Change request has clear description".to_string(),
                required: true,
                status: if change_request.description.trim().is_empty() {
                    ChecklistStatus::Failed
                } else {
                    ChecklistStatus::Passed
                },
                notes: String::new(),
                responsible_party: "Developer".to_string(),
            },
            ChecklistItem {
                id: Uuid::new_v4(),
                description: "Proposed code is provided".to_string(),
                required: true,
                status: if change_request.proposed_code.trim().is_empty() {
                    ChecklistStatus::Failed
                } else {
                    ChecklistStatus::Passed
                },
                notes: String::new(),
                responsible_party: "Developer".to_string(),
            },
            ChecklistItem {
                id: Uuid::new_v4(),
                description: "Simulation plan includes all required phases".to_string(),
                required: true,
                status: ChecklistStatus::Pending,
                notes: String::new(),
                responsible_party: "System".to_string(),
            },
        ];

        let during_validation = vec![
            ChecklistItem {
                id: Uuid::new_v4(),
                description: "Each simulation step completes successfully".to_string(),
                required: true,
                status: ChecklistStatus::Pending,
                notes: String::new(),
                responsible_party: "System".to_string(),
            },
            ChecklistItem {
                id: Uuid::new_v4(),
                description: "Confidence score meets minimum threshold".to_string(),
                required: true,
                status: ChecklistStatus::Pending,
                notes: String::new(),
                responsible_party: "System".to_string(),
            },
        ];

        let post_validation = vec![
            ChecklistItem {
                id: Uuid::new_v4(),
                description: "All validation criteria met".to_string(),
                required: true,
                status: ChecklistStatus::Pending,
                notes: String::new(),
                responsible_party: "Developer".to_string(),
            },
            ChecklistItem {
                id: Uuid::new_v4(),
                description: "Debugging artifacts reviewed".to_string(),
                required: false,
                status: ChecklistStatus::Pending,
                notes: String::new(),
                responsible_party: "Developer".to_string(),
            },
        ];

        Ok(ValidationChecklist {
            pre_validation,
            during_validation,
            post_validation,
            overall_score: 0.0,
        })
    }

    /// Generate common pitfalls
    fn generate_pitfalls(
        &self,
        _change_request: &ChangeRequest,
        _simulation_plan: &SimulationPlan,
    ) -> CoreResult<Vec<Pitfall>> {
        Ok(vec![
            Pitfall {
                title: "Scope Creep".to_string(),
                description: "Gradually expanding the scope of changes beyond the original requirements".to_string(),
                why_its_pitfall: "Expanded scope increases complexity, risk, and can lead to delays or incomplete implementation".to_string(),
                avoidance_strategies: vec![
                    "Clearly define and document the scope upfront".to_string(),
                    "Review any scope changes through a formal process".to_string(),
                    "Break larger changes into smaller, manageable pieces".to_string(),
                ],
                severity: PitfallSeverity::High,
                affected_phases: vec!["A".to_string(), "B".to_string()],
            },
            Pitfall {
                title: "Insufficient Testing".to_string(),
                description: "Not adequately testing the changes before deployment".to_string(),
                why_its_pitfall: "Insufficient testing can lead to bugs in production and user dissatisfaction".to_string(),
                avoidance_strategies: vec![
                    "Write tests before implementing changes".to_string(),
                    "Include both unit and integration tests".to_string(),
                    "Perform manual testing for complex scenarios".to_string(),
                ],
                severity: PitfallSeverity::Critical,
                affected_phases: vec!["C".to_string(), "D".to_string()],
            },
            Pitfall {
                title: "Ignoring Edge Cases".to_string(),
                description: "Failing to consider unusual or boundary conditions in the implementation".to_string(),
                why_its_pitfall: "Edge cases often contain hidden bugs that can cause system failures in production".to_string(),
                avoidance_strategies: vec![
                    "Brainstorm potential edge cases during design".to_string(),
                    "Write specific tests for edge cases".to_string(),
                    "Review code with edge cases in mind".to_string(),
                ],
                severity: PitfallSeverity::Medium,
                affected_phases: vec!["A".to_string(), "C".to_string()],
            },
            Pitfall {
                title: "Overconfidence in Analysis".to_string(),
                description: "Trusting the automated analysis without manual verification".to_string(),
                why_its_pitfall: "Automated tools can miss subtle dependencies or complex interactions that humans might catch".to_string(),
                avoidance_strategies: vec![
                    "Review analysis results manually".to_string(),
                    "Get peer review of complex changes".to_string(),
                    "Cross-validate with multiple analysis methods".to_string(),
                ],
                severity: PitfallSeverity::Medium,
                affected_phases: vec!["A".to_string(), "B".to_string()],
            },
        ])
    }
}

impl Default for DebuggingInfoGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::change_request::{ChangeType, Complexity};
    use parseltongue_01::types::ISGL1Key;
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
            "Add error handling".to_string(),
            "fn test_function() {}".to_string(),
            "fn test_function() -> Result<(), Error> { Ok(()) }".to_string(),
        )
        .with_complexity(Complexity::Moderate)
    }

    fn create_test_simulation_plan() -> crate::SimulationPlan {
        crate::SimulationPlan::mock()
    }

    #[tokio::test]
    async fn test_debugging_info_creation() {
        let change_request = create_test_change_request();
        let simulation_plan = create_test_simulation_plan();

        let debugging_info = DebuggingInfo::new(change_request, simulation_plan);

        assert!(!debugging_info.has_questions());
        assert!(!debugging_info.has_step_explanations());
        assert!(!debugging_info.has_validation_checklist());
        assert_eq!(debugging_info.pitfalls.len(), 0);
    }

    #[tokio::test]
    async fn test_debugging_info_generator() {
        let generator = DebuggingInfoGenerator::new();
        let change_request = create_test_change_request();
        let simulation_plan = create_test_simulation_plan();

        let debugging_info = generator
            .generate_debugging_info(&change_request, &simulation_plan, None)
            .await
            .unwrap();

        assert!(debugging_info.has_questions());
        assert!(debugging_info.has_step_explanations());
        assert!(debugging_info.has_validation_checklist());
        assert!(!debugging_info.pitfalls.is_empty());
    }

    #[test]
    fn test_rubber_duck_question() {
        let question = RubberDuckQuestion {
            id: Uuid::new_v4(),
            question: "Test question".to_string(),
            category: QuestionCategory::Scope,
            answer_format: AnswerFormat::Text,
            reasoning: "Test reasoning".to_string(),
            tips: vec!["Test tip".to_string()],
        };

        assert_eq!(question.question, "Test question");
        assert_eq!(question.category, QuestionCategory::Scope);
        assert_eq!(question.answer_format, AnswerFormat::Text);
    }

    #[test]
    fn test_step_explanation() {
        let explanation = StepExplanation {
            step_id: Uuid::new_v4(),
            title: "Test Step".to_string(),
            what_it_does: "Test what it does".to_string(),
            why_its_necessary: "Test why necessary".to_string(),
            how_to_validate: "Test how to validate".to_string(),
            common_issues: vec![],
            success_criteria: vec!["Test criterion".to_string()],
        };

        assert_eq!(explanation.title, "Test Step");
        assert_eq!(explanation.what_it_does, "Test what it does");
    }

    #[test]
    fn test_validation_checklist() {
        let checklist = ValidationChecklist {
            pre_validation: vec![],
            during_validation: vec![],
            post_validation: vec![],
            overall_score: 0.85,
        };

        assert_eq!(checklist.overall_score, 0.85);
    }

    #[test]
    fn test_pitfall() {
        let pitfall = Pitfall {
            title: "Test Pitfall".to_string(),
            description: "Test description".to_string(),
            why_its_pitfall: "Test why".to_string(),
            avoidance_strategies: vec!["Test strategy".to_string()],
            severity: PitfallSeverity::High,
            affected_phases: vec!["A".to_string(), "B".to_string()],
        };

        assert_eq!(pitfall.title, "Test Pitfall");
        assert_eq!(pitfall.severity, PitfallSeverity::High);
        assert_eq!(pitfall.affected_phases.len(), 2);
    }

    #[tokio::test]
    async fn test_export_artifacts() {
        let generator = DebuggingInfoGenerator::new();
        let change_request = create_test_change_request();
        let simulation_plan = create_test_simulation_plan();

        let debugging_info = generator
            .generate_debugging_info(&change_request, &simulation_plan, None)
            .await
            .unwrap();

        let artifacts = debugging_info.export_artifacts().unwrap();

        assert!(artifacts.contains_key("rubber_duck_questions.md"));
        assert!(artifacts.contains_key("step_explanations.md"));
        assert!(artifacts.contains_key("validation_checklist.md"));
        assert!(artifacts.contains_key("pitfalls.md"));
        assert!(artifacts.contains_key("debugging_summary.md"));
    }

    #[test]
    fn test_generator_config() {
        let config = GeneratorConfig {
            include_detailed_explanations: false,
            include_rubber_duck_questions: false,
            include_validation_checklist: false,
            question_categories: vec![QuestionCategory::Scope],
            max_questions_per_category: 3,
        };

        let generator = DebuggingInfoGenerator::with_config(config);
        assert!(!generator.config.include_detailed_explanations);
        assert!(!generator.config.include_rubber_duck_questions);
        assert!(!generator.config.include_validation_checklist);
        assert_eq!(generator.config.max_questions_per_category, 3);
    }

    #[test]
    fn test_default_generator() {
        let generator = DebuggingInfoGenerator::default();
        assert!(generator.config.include_detailed_explanations);
        assert!(generator.config.include_rubber_duck_questions);
        assert!(generator.config.include_validation_checklist);
    }
}

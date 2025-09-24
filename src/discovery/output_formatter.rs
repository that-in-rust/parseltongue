//! Output Formatting System for Parseltongue v2
//! 
//! Provides multiple output formats for workflow results:
//! - Human: Readable terminal output with emojis and structure
//! - JSON: Structured data for LLM consumption and API integration
//! - PR Summary: Markdown format for pull request descriptions
//! - CI: GitHub Actions compatible output with risk levels and gates

use std::time::Duration;
use thiserror::Error;

use crate::discovery::{
    OnboardingResult, FeaturePlanResult, DebugResult, RefactorResult,
    ComplexityLevel, ConfidenceLevel, Priority
};
use crate::discovery::workflow_orchestrator::RiskLevel;

/// Core trait for output formatting
/// 
/// # Contract
/// - All formatters must handle the same workflow result types
/// - Formatting must complete within 100ms for responsiveness
/// - Output must be consistent and copy-pastable where appropriate
/// 
/// # Error Conditions
/// - FormattingError::SerializationFailed for JSON serialization issues
/// - FormattingError::TemplateError for template rendering failures
/// - FormattingError::InvalidFormat for unsupported formats
pub trait OutputFormatter {
    /// Format onboarding workflow result
    /// 
    /// # Preconditions
    /// - OnboardingResult is valid and complete
    /// 
    /// # Postconditions
    /// - Returns formatted string appropriate for the output type
    /// - Completes within 100ms
    /// 
    /// # Error Conditions
    /// - FormattingError if formatting fails
    fn format_onboarding(&self, result: &OnboardingResult) -> Result<String, FormattingError>;
    
    /// Format feature planning workflow result
    /// 
    /// # Preconditions
    /// - FeaturePlanResult contains valid impact analysis and scope guidance
    /// 
    /// # Postconditions
    /// - Returns formatted output emphasizing risk and scope
    /// - Highlights test recommendations and boundaries
    fn format_feature_plan(&self, result: &FeaturePlanResult) -> Result<String, FormattingError>;
    
    /// Format debugging workflow result
    /// 
    /// # Preconditions
    /// - DebugResult contains caller traces and usage sites
    /// 
    /// # Postconditions
    /// - Returns formatted output focusing on minimal change scope
    /// - Emphasizes rollback strategy and side effects
    fn format_debug(&self, result: &DebugResult) -> Result<String, FormattingError>;
    
    /// Format refactoring safety check result
    /// 
    /// # Preconditions
    /// - RefactorResult contains risk assessment and reviewer guidance
    /// 
    /// # Postconditions
    /// - Returns formatted output emphasizing safety and review process
    /// - Highlights risk factors and approval criteria
    fn format_refactor(&self, result: &RefactorResult) -> Result<String, FormattingError>;
}

/// Formatting errors with structured error hierarchy
#[derive(Error, Debug)]
pub enum FormattingError {
    #[error("JSON serialization failed: {message}")]
    SerializationFailed { message: String },
    
    #[error("Template rendering failed: {template} - {error}")]
    TemplateError { template: String, error: String },
    
    #[error("Invalid output format: {format}")]
    InvalidFormat { format: String },
    
    #[error("IO error during formatting: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Formatting timeout: took {elapsed:?}, limit {limit:?}")]
    Timeout { elapsed: Duration, limit: Duration },
}

/// Human-readable formatter for terminal output
/// 
/// Produces copy-pastable output with emojis, clear structure, and actionable information.
/// Optimized for developer reading and terminal display.
pub struct HumanFormatter {
    /// Whether to include performance timing information
    include_timing: bool,
    /// Whether to use emoji icons in output
    use_emojis: bool,
}

impl HumanFormatter {
    /// Create new human formatter with default settings
    pub fn new() -> Self {
        Self {
            include_timing: true,
            use_emojis: true,
        }
    }
    
    /// Create human formatter with custom settings
    pub fn with_options(include_timing: bool, use_emojis: bool) -> Self {
        Self {
            include_timing,
            use_emojis,
        }
    }
}

impl Default for HumanFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for HumanFormatter {
    fn format_onboarding(&self, result: &OnboardingResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        
        // Avengers-themed emojis for onboarding (Captain America - leadership & guidance)
        let emoji_prefix = if self.use_emojis { "🛡️  " } else { "" };
        let mut output = String::new();
        
        // Header with Avengers theme
        output.push_str(&format!("{}PARSELTONGUE ONBOARDING PROTOCOL ACTIVATED\n", emoji_prefix));
        output.push_str("═══════════════════════════════════════════════\n\n");
        output.push_str("🎯 Mission: Codebase Intelligence Gathering Complete\n\n");
        
        // Overview section (Iron Man - tech analysis)
        let overview_emoji = if self.use_emojis { "🤖 " } else { "" };
        output.push_str(&format!("{}JARVIS Analysis Report:\n", overview_emoji));
        output.push_str(&format!("  ⚡ Total files scanned: {}\n", result.overview.total_files));
        output.push_str(&format!("  ⚡ Total entities discovered: {}\n", result.overview.total_entities));
        output.push('\n');
        
        // Entities by type (Thor - power classification)
        if !result.overview.entities_by_type.is_empty() {
            let entities_emoji = if self.use_emojis { "⚡ " } else { "" };
            output.push_str(&format!("{}Power Classification by Type:\n", entities_emoji));
            for (entity_type, count) in &result.overview.entities_by_type {
                let type_emoji = match entity_type.as_str() {
                    "Function" => "🔨", // Thor's hammer for functions
                    "Struct" => "🛡️", // Captain America's shield for structs
                    "Trait" => "💎", // Infinity stones for traits
                    "Impl" => "🔧", // Iron Man's tech for implementations
                    _ => "⚡"
                };
                output.push_str(&format!("  {} {}: {}\n", type_emoji, entity_type, count));
            }
            output.push('\n');
        }
        
        // Key modules (Nick Fury - strategic overview)
        if !result.overview.key_modules.is_empty() {
            let modules_emoji = if self.use_emojis { "👁️  " } else { "" };
            output.push_str(&format!("{}S.H.I.E.L.D. Strategic Modules:\n", modules_emoji));
            for module in &result.overview.key_modules {
                output.push_str(&format!("  🎯 {}: {}\n", module.name, module.purpose));
            }
            output.push('\n');
        }
        
        // Entry points (Captain America - entry strategy)
        if !result.entry_points.is_empty() {
            let entry_emoji = if self.use_emojis { "🚪 " } else { "" };
            output.push_str(&format!("{}Mission Entry Points:\n", entry_emoji));
            for entry in &result.entry_points {
                let entry_type_emoji = match entry.entry_type.as_str() {
                    "main" => "🚀", // Launch point
                    "library" => "📚", // Knowledge base
                    _ => "🎯"
                };
                output.push_str(&format!("  {} {} ({}): {}\n", entry_type_emoji, entry.name, entry.entry_type, entry.description));
                output.push_str(&format!("    📍 Location: {}\n", entry.location.format_for_editor()));
            }
            output.push('\n');
        }
        
        // Key contexts (Doctor Strange - mystical knowledge)
        if !result.key_contexts.is_empty() {
            let context_emoji = if self.use_emojis { "🔮 " } else { "" };
            output.push_str(&format!("{}Sanctum Sanctorum - Key Knowledge:\n", context_emoji));
            for context in &result.key_contexts {
                let context_type_emoji = match context.context_type.as_str() {
                    "trait" => "💎", // Infinity stone
                    "service" => "⚙️", // Machinery
                    "controller" => "🎮", // Control
                    _ => "🔮"
                };
                output.push_str(&format!("  {} {} ({}): {}\n", context_type_emoji, context.name, context.context_type, context.importance));
                output.push_str(&format!("    📍 Location: {}\n", context.location.format_for_editor()));
            }
            output.push('\n');
        }
        
        // Next steps (Captain America - mission briefing)
        if !result.next_steps.is_empty() {
            let steps_emoji = if self.use_emojis { "📋 " } else { "" };
            output.push_str(&format!("{}Mission Briefing - Next Objectives:\n", steps_emoji));
            for (i, step) in result.next_steps.iter().enumerate() {
                output.push_str(&format!("  {}. 🎯 {}\n", i + 1, step));
            }
            output.push('\n');
        }
        
        // Timing information (Quicksilver - speed)
        if self.include_timing {
            let timing_emoji = if self.use_emojis { "💨 " } else { "" };
            let time_status = if result.execution_time.as_secs() < 15 * 60 {
                "✅ MISSION ACCOMPLISHED"
            } else {
                "⚠️  MISSION EXTENDED"
            };
            output.push_str(&format!("{}Speed Analysis: {:.2}s {} (target: <15 minutes)\n", 
                                   timing_emoji, result.execution_time.as_secs_f64(), time_status));
        }
        
        output.push_str("\n🛡️  Parseltongue Protocol Complete - Ready for Action! 🛡️\n");
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
    
    fn format_feature_plan(&self, result: &FeaturePlanResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        
        // Iron Man theme - tech planning and analysis
        let emoji_prefix = if self.use_emojis { "🤖 " } else { "" };
        let mut output = String::new();
        
        // Header with Iron Man tech theme
        output.push_str(&format!("{}STARK INDUSTRIES FEATURE DEVELOPMENT PROTOCOL\n", emoji_prefix));
        output.push_str("═══════════════════════════════════════════════════\n\n");
        output.push_str(&format!("🎯 Target System: {}\n\n", result.target_entity));
        
        // Impact analysis (FRIDAY AI analysis)
        let analysis_emoji = if self.use_emojis { "🔬 " } else { "" };
        output.push_str(&format!("{}FRIDAY Impact Analysis:\n", analysis_emoji));
        
        let risk_emoji = match result.impact_analysis.risk_level {
            RiskLevel::Low => "🟢",
            RiskLevel::Medium => "🟡", 
            RiskLevel::High => "🟠",
            RiskLevel::Critical => "🔴"
        };
        output.push_str(&format!("  {} Risk Level: {:?}\n", risk_emoji, result.impact_analysis.risk_level));
        
        let complexity_emoji = match result.impact_analysis.complexity_estimate {
            ComplexityLevel::Simple => "⚡",
            ComplexityLevel::Moderate => "🔧",
            ComplexityLevel::Complex => "⚙️",
            ComplexityLevel::VeryComplex => "🛠️"
        };
        output.push_str(&format!("  {} Complexity: {:?}\n", complexity_emoji, result.impact_analysis.complexity_estimate));
        output.push_str(&format!("  💥 Direct Impact: {} entities\n", result.impact_analysis.direct_impact.len()));
        output.push_str(&format!("  🌊 Ripple Effect: {} entities\n", result.impact_analysis.indirect_impact.len()));
        output.push('\n');
        
        // Scope guidance (Tactical planning)
        let scope_emoji = if self.use_emojis { "🎯 " } else { "" };
        output.push_str(&format!("{}Tactical Deployment Scope:\n", scope_emoji));
        if !result.scope_guidance.boundaries.is_empty() {
            output.push_str("  🛡️  Security Perimeter:\n");
            for boundary in &result.scope_guidance.boundaries {
                output.push_str(&format!("    🔒 {}\n", boundary));
            }
        }
        if !result.scope_guidance.files_to_modify.is_empty() {
            output.push_str("  🔧 Systems to Upgrade:\n");
            for file in &result.scope_guidance.files_to_modify {
                output.push_str(&format!("    ⚡ {}\n", file));
            }
        }
        if !result.scope_guidance.files_to_avoid.is_empty() {
            output.push_str("  ⚠️  Critical Systems (Do Not Touch):\n");
            for file in &result.scope_guidance.files_to_avoid {
                output.push_str(&format!("    🚫 {}\n", file));
            }
        }
        output.push('\n');
        
        // Test recommendations (Quality assurance protocols)
        if !result.test_recommendations.is_empty() {
            let test_emoji = if self.use_emojis { "🧪 " } else { "" };
            output.push_str(&format!("{}Quality Assurance Protocols:\n", test_emoji));
            for test in &result.test_recommendations {
                let test_type_emoji = match test.test_type.as_str() {
                    "unit" => "🔬",
                    "integration" => "🔗",
                    "performance" => "⚡",
                    _ => "🧪"
                };
                output.push_str(&format!("  {} {} ({}): {}\n", test_type_emoji, test.test_target, test.test_type, test.rationale));
                output.push_str(&format!("    📍 Deploy at: {}\n", test.suggested_location));
            }
            output.push('\n');
        }
        
        // Timing information (Efficiency metrics)
        if self.include_timing {
            let timing_emoji = if self.use_emojis { "💨 " } else { "" };
            let time_status = if result.execution_time.as_secs() < 5 * 60 {
                "✅ OPTIMAL EFFICIENCY"
            } else {
                "⚠️  PERFORMANCE REVIEW NEEDED"
            };
            output.push_str(&format!("{}Stark Tech Efficiency: {:.2}s {} (target: <5 minutes)\n", 
                                   timing_emoji, result.execution_time.as_secs_f64(), time_status));
        }
        
        output.push_str("\n🤖 Feature Development Protocol Ready for Deployment! 🤖\n");
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
    
    fn format_debug(&self, result: &DebugResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        
        // Spider-Man theme - web tracing and debugging
        let emoji_prefix = if self.use_emojis { "🕷️  " } else { "" };
        let mut output = String::new();
        
        // Header with Spider-Man web theme
        output.push_str(&format!("{}SPIDER-SENSE DEBUG PROTOCOL ACTIVATED\n", emoji_prefix));
        output.push_str("═══════════════════════════════════════════\n\n");
        output.push_str(&format!("🎯 Web Target: {}\n\n", result.target_entity));
        
        // Caller traces (Web tracing)
        if !result.caller_traces.is_empty() {
            let caller_emoji = if self.use_emojis { "🕸️  " } else { "" };
            output.push_str(&format!("{}Web Trace Analysis:\n", caller_emoji));
            for trace in &result.caller_traces {
                let depth_emoji = match trace.depth {
                    1 => "🔗", // Direct connection
                    2..=3 => "🕸️", // Web connection
                    _ => "🌐" // Deep web
                };
                output.push_str(&format!("  {} {} (depth: {}, context: {})\n", 
                                       depth_emoji, trace.caller.name, trace.depth, trace.call_context));
                output.push_str(&format!("    📍 Web Node: {}\n", trace.caller.file_path));
                if let Some(freq) = &trace.frequency {
                    output.push_str(&format!("    🔄 Frequency: {}\n", freq));
                }
            }
            output.push('\n');
        }
        
        // Usage sites (Spider-sense detection)
        if !result.usage_sites.is_empty() {
            let usage_emoji = if self.use_emojis { "🕷️  " } else { "" };
            output.push_str(&format!("{}Spider-Sense Detection:\n", usage_emoji));
            for usage in &result.usage_sites {
                let usage_type_emoji = match usage.usage_type.as_str() {
                    "call" => "📞",
                    "reference" => "👁️",
                    "dependency" => "🔗",
                    _ => "🎯"
                };
                output.push_str(&format!("  {} {} ({}): {}\n", usage_type_emoji, usage.user.name, usage.usage_type, usage.context));
                output.push_str(&format!("    📍 Location: {}\n", usage.location.format_for_editor()));
            }
            output.push('\n');
        }
        
        // Minimal change scope (Precision targeting)
        let scope_emoji = if self.use_emojis { "🎯 " } else { "" };
        output.push_str(&format!("{}Precision Strike Zone:\n", scope_emoji));
        if !result.minimal_scope.minimal_files.is_empty() {
            output.push_str("  🎯 Primary Targets:\n");
            for file in &result.minimal_scope.minimal_files {
                output.push_str(&format!("    🔧 {}\n", file));
            }
        }
        if !result.minimal_scope.safe_boundaries.is_empty() {
            output.push_str("  🛡️  Safe Zones:\n");
            for boundary in &result.minimal_scope.safe_boundaries {
                output.push_str(&format!("    ✅ {}\n", boundary));
            }
        }
        if !result.minimal_scope.side_effects.is_empty() {
            output.push_str("  ⚠️  Spider-Sense Warnings:\n");
            for effect in &result.minimal_scope.side_effects {
                output.push_str(&format!("    🚨 {}\n", effect));
            }
        }
        output.push_str(&format!("  🔄 Web Restoration Plan: {}\n", result.minimal_scope.rollback_strategy));
        output.push('\n');
        
        // Timing information (Web-slinger speed)
        if self.include_timing {
            let timing_emoji = if self.use_emojis { "💨 " } else { "" };
            let time_status = if result.execution_time.as_secs() < 2 * 60 {
                "✅ WEB-SLINGER SPEED"
            } else {
                "⚠️  NEED MORE SPIDER-POWER"
            };
            output.push_str(&format!("{}Web-Swing Speed: {:.2}s {} (target: <2 minutes)\n", 
                                   timing_emoji, result.execution_time.as_secs_f64(), time_status));
        }
        
        output.push_str("\n🕷️  Debug Web Complete - Your Friendly Neighborhood Debugger! 🕷️\n");
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
    
    fn format_refactor(&self, result: &RefactorResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        
        // Hulk theme - careful transformation with strength
        let emoji_prefix = if self.use_emojis { "💚 " } else { "" };
        let mut output = String::new();
        
        // Header with Hulk transformation theme
        output.push_str(&format!("{}HULK SMASH... CAREFULLY! REFACTOR PROTOCOL\n", emoji_prefix));
        output.push_str("═══════════════════════════════════════════════\n\n");
        output.push_str(&format!("🎯 Transformation Target: {}\n\n", result.target_entity));
        
        // Risk assessment (Banner's scientific analysis)
        let risk_emoji = if self.use_emojis { "🧬 " } else { "" };
        output.push_str(&format!("{}Dr. Banner's Risk Analysis:\n", risk_emoji));
        
        let overall_risk_emoji = match result.risk_assessment.overall_risk {
            RiskLevel::Low => "🟢",
            RiskLevel::Medium => "🟡",
            RiskLevel::High => "🟠", 
            RiskLevel::Critical => "🔴"
        };
        output.push_str(&format!("  {} Overall Risk: {:?}\n", overall_risk_emoji, result.risk_assessment.overall_risk));
        
        let confidence_emoji = match result.risk_assessment.confidence {
            ConfidenceLevel::Low => "🤔",
            ConfidenceLevel::Medium => "🧐",
            ConfidenceLevel::High => "💪",
            ConfidenceLevel::VeryHigh => "🎯"
        };
        output.push_str(&format!("  {} Confidence Level: {:?}\n", confidence_emoji, result.risk_assessment.confidence));
        
        if !result.risk_assessment.risk_factors.is_empty() {
            output.push_str("  ⚠️  Gamma Radiation Levels (Risk Factors):\n");
            for factor in &result.risk_assessment.risk_factors {
                let factor_emoji = match factor.level {
                    RiskLevel::Low => "🟢",
                    RiskLevel::Medium => "🟡",
                    RiskLevel::High => "🟠",
                    RiskLevel::Critical => "🔴"
                };
                output.push_str(&format!("    {} {} ({:?}): {}\n", factor_emoji, factor.description, factor.level, factor.impact));
            }
        }
        
        if !result.risk_assessment.mitigations.is_empty() {
            output.push_str("  🛡️  Containment Protocols:\n");
            for mitigation in &result.risk_assessment.mitigations {
                output.push_str(&format!("    🔬 {}\n", mitigation));
            }
        }
        output.push('\n');
        
        // Change checklist (Hulk's careful approach)
        if !result.change_checklist.is_empty() {
            let checklist_emoji = if self.use_emojis { "💪 " } else { "" };
            output.push_str(&format!("{}Hulk's Careful Transformation Steps:\n", checklist_emoji));
            for item in &result.change_checklist {
                let status = if item.completed { "✅" } else { "⬜" };
                let priority_emoji = match item.priority {
                    Priority::Critical => "🔴",
                    Priority::High => "🟠",
                    Priority::Medium => "🟡",
                    Priority::Low => "🟢"
                };
                output.push_str(&format!("  {} {} {} ({:?})\n", status, priority_emoji, item.description, item.priority));
                if let Some(notes) = &item.notes {
                    output.push_str(&format!("    📝 Banner's Notes: {}\n", notes));
                }
            }
            output.push('\n');
        }
        
        // Reviewer guidance (Avengers team review)
        let reviewer_emoji = if self.use_emojis { "🦸‍♂️ " } else { "" };
        output.push_str(&format!("{}Avengers Team Review Protocol:\n", reviewer_emoji));
        if !result.reviewer_guidance.focus_areas.is_empty() {
            output.push_str("  🎯 Mission Critical Areas:\n");
            for area in &result.reviewer_guidance.focus_areas {
                output.push_str(&format!("    🔍 {}\n", area));
            }
        }
        if !result.reviewer_guidance.potential_issues.is_empty() {
            output.push_str("  ⚠️  Threat Assessment:\n");
            for issue in &result.reviewer_guidance.potential_issues {
                output.push_str(&format!("    🚨 {}\n", issue));
            }
        }
        if !result.reviewer_guidance.testing_recommendations.is_empty() {
            output.push_str("  🧪 S.H.I.E.L.D. Testing Protocols:\n");
            for rec in &result.reviewer_guidance.testing_recommendations {
                output.push_str(&format!("    🔬 {}\n", rec));
            }
        }
        if !result.reviewer_guidance.approval_criteria.is_empty() {
            output.push_str("  ✅ Avengers Assembly Approval:\n");
            for criteria in &result.reviewer_guidance.approval_criteria {
                output.push_str(&format!("    🛡️  {}\n", criteria));
            }
        }
        output.push('\n');
        
        // Timing information (Hulk's controlled power)
        if self.include_timing {
            let timing_emoji = if self.use_emojis { "💨 " } else { "" };
            let time_status = if result.execution_time.as_secs() < 3 * 60 {
                "✅ HULK CONTROLLED POWER"
            } else {
                "⚠️  HULK NEED MORE TIME"
            };
            output.push_str(&format!("{}Transformation Speed: {:.2}s {} (target: <3 minutes)\n", 
                                   timing_emoji, result.execution_time.as_secs_f64(), time_status));
        }
        
        output.push_str("\n💚 Hulk Refactor Complete - Hulk Strongest Coder There Is! 💚\n");
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
}

/// JSON formatter for structured data output
/// 
/// Produces valid JSON suitable for LLM consumption, API integration, and programmatic processing.
/// Includes metadata and timing information for analysis.
pub struct JsonFormatter {
    /// Whether to pretty-print JSON output
    pretty_print: bool,
    /// Whether to include metadata fields
    include_metadata: bool,
}

impl JsonFormatter {
    /// Create new JSON formatter with default settings
    pub fn new() -> Self {
        Self {
            pretty_print: true,
            include_metadata: true,
        }
    }
    
    /// Create JSON formatter with custom settings
    pub fn with_options(pretty_print: bool, include_metadata: bool) -> Self {
        Self {
            pretty_print,
            include_metadata,
        }
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for JsonFormatter {
    fn format_onboarding(&self, result: &OnboardingResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        
        let mut output = serde_json::json!({
            "workflow": "onboard",
            "result": result
        });
        
        if self.include_metadata {
            output["execution_time_s"] = serde_json::json!(result.execution_time.as_secs_f64());
            output["performance_target_s"] = serde_json::json!(15 * 60);
            output["timestamp"] = serde_json::json!(chrono::Utc::now().to_rfc3339());
        }
        
        let json_string = if self.pretty_print {
            serde_json::to_string_pretty(&output)
        } else {
            serde_json::to_string(&output)
        }.map_err(|e| FormattingError::SerializationFailed {
            message: e.to_string(),
        })?;
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(json_string)
    }
    
    fn format_feature_plan(&self, result: &FeaturePlanResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        
        let mut output = serde_json::json!({
            "workflow": "feature-start",
            "result": result
        });
        
        if self.include_metadata {
            output["execution_time_s"] = serde_json::json!(result.execution_time.as_secs_f64());
            output["performance_target_s"] = serde_json::json!(5 * 60);
            output["timestamp"] = serde_json::json!(chrono::Utc::now().to_rfc3339());
        }
        
        let json_string = if self.pretty_print {
            serde_json::to_string_pretty(&output)
        } else {
            serde_json::to_string(&output)
        }.map_err(|e| FormattingError::SerializationFailed {
            message: e.to_string(),
        })?;
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(json_string)
    }
    
    fn format_debug(&self, result: &DebugResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        
        let mut output = serde_json::json!({
            "workflow": "debug",
            "result": result
        });
        
        if self.include_metadata {
            output["execution_time_s"] = serde_json::json!(result.execution_time.as_secs_f64());
            output["performance_target_s"] = serde_json::json!(2 * 60);
            output["timestamp"] = serde_json::json!(chrono::Utc::now().to_rfc3339());
        }
        
        let json_string = if self.pretty_print {
            serde_json::to_string_pretty(&output)
        } else {
            serde_json::to_string(&output)
        }.map_err(|e| FormattingError::SerializationFailed {
            message: e.to_string(),
        })?;
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(json_string)
    }
    
    fn format_refactor(&self, result: &RefactorResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        
        let mut output = serde_json::json!({
            "workflow": "refactor-check",
            "result": result
        });
        
        if self.include_metadata {
            output["execution_time_s"] = serde_json::json!(result.execution_time.as_secs_f64());
            output["performance_target_s"] = serde_json::json!(3 * 60);
            output["timestamp"] = serde_json::json!(chrono::Utc::now().to_rfc3339());
        }
        
        let json_string = if self.pretty_print {
            serde_json::to_string_pretty(&output)
        } else {
            serde_json::to_string(&output)
        }.map_err(|e| FormattingError::SerializationFailed {
            message: e.to_string(),
        })?;
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(json_string)
    }
}

/// PR Summary formatter for pull request descriptions
/// 
/// Produces markdown format with architectural context, impact analysis, and actionable checklists.
/// Optimized for code review and team communication.
pub struct PrSummaryFormatter {
    /// Whether to include architectural diagrams (mermaid)
    include_diagrams: bool,
    /// Whether to generate actionable checklists
    include_checklists: bool,
}

impl PrSummaryFormatter {
    /// Create new PR summary formatter with default settings
    pub fn new() -> Self {
        Self {
            include_diagrams: true,
            include_checklists: true,
        }
    }
    
    /// Create PR summary formatter with custom settings
    pub fn with_options(include_diagrams: bool, include_checklists: bool) -> Self {
        Self {
            include_diagrams,
            include_checklists,
        }
    }
}

impl Default for PrSummaryFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for PrSummaryFormatter {
    fn format_onboarding(&self, result: &OnboardingResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        let mut output = String::new();
        
        // Header
        output.push_str("# Codebase Onboarding Summary\n\n");
        
        // Overview
        output.push_str("## Architectural Overview\n\n");
        output.push_str(&format!("- **Total Files**: {}\n", result.overview.total_files));
        output.push_str(&format!("- **Total Entities**: {}\n", result.overview.total_entities));
        
        if !result.overview.architecture_patterns.is_empty() {
            output.push_str("- **Architecture Patterns**: ");
            output.push_str(&result.overview.architecture_patterns.join(", "));
            output.push('\n');
        }
        output.push('\n');
        
        // Entity breakdown
        if !result.overview.entities_by_type.is_empty() {
            output.push_str("### Entity Distribution\n\n");
            for (entity_type, count) in &result.overview.entities_by_type {
                output.push_str(&format!("- **{}**: {}\n", entity_type, count));
            }
            output.push('\n');
        }
        
        // Key modules
        if !result.overview.key_modules.is_empty() {
            output.push_str("### Key Modules\n\n");
            for module in &result.overview.key_modules {
                output.push_str(&format!("- **{}**: {}\n", module.name, module.purpose));
                if !module.key_entities.is_empty() {
                    output.push_str(&format!("  - Key entities: {}\n", module.key_entities.join(", ")));
                }
            }
            output.push('\n');
        }
        
        // Entry points
        if !result.entry_points.is_empty() {
            output.push_str("## Entry Points\n\n");
            for entry in &result.entry_points {
                output.push_str(&format!("### {} ({})\n\n", entry.name, entry.entry_type));
                output.push_str(&format!("{}\n\n", entry.description));
                output.push_str(&format!("**Location**: `{}`\n\n", entry.location.format_for_editor()));
            }
        }
        
        // Recommended actions
        if !result.next_steps.is_empty() && self.include_checklists {
            output.push_str("## Recommended Actions\n\n");
            for (_i, step) in result.next_steps.iter().enumerate() {
                output.push_str(&format!("- [ ] {}\n", step));
            }
            output.push('\n');
        }
        
        // Architecture diagram
        if self.include_diagrams && !result.overview.key_modules.is_empty() {
            output.push_str("## Architecture Diagram\n\n");
            output.push_str("```mermaid\n");
            output.push_str("graph TD\n");
            for (i, module) in result.overview.key_modules.iter().enumerate() {
                let node_id = format!("M{}", i);
                output.push_str(&format!("    {}[{}]\n", node_id, module.name));
            }
            output.push_str("```\n\n");
        }
        
        // Metadata
        output.push_str("---\n");
        output.push_str(&format!("**Analysis completed in**: {:.2}s\n", result.execution_time.as_secs_f64()));
        output.push_str(&format!("**Generated at**: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
    
    fn format_feature_plan(&self, result: &FeaturePlanResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        let mut output = String::new();
        
        // Header
        output.push_str("# Feature Development Plan\n\n");
        output.push_str(&format!("**Target Entity**: `{}`\n\n", result.target_entity));
        
        // Risk assessment
        output.push_str("## Risk Assessment\n\n");
        output.push_str(&format!("- **Risk Level**: {:?}\n", result.impact_analysis.risk_level));
        output.push_str(&format!("- **Complexity**: {:?}\n", result.impact_analysis.complexity_estimate));
        output.push_str(&format!("- **Direct Impact**: {} entities\n", result.impact_analysis.direct_impact.len()));
        output.push_str(&format!("- **Indirect Impact**: {} entities\n", result.impact_analysis.indirect_impact.len()));
        output.push('\n');
        
        // Scope boundaries
        output.push_str("## Scope Boundaries\n\n");
        if !result.scope_guidance.boundaries.is_empty() {
            output.push_str("### Recommended Boundaries\n\n");
            for boundary in &result.scope_guidance.boundaries {
                output.push_str(&format!("- {}\n", boundary));
            }
            output.push('\n');
        }
        
        if !result.scope_guidance.files_to_modify.is_empty() {
            output.push_str("### Files to Modify\n\n");
            for file in &result.scope_guidance.files_to_modify {
                output.push_str(&format!("- `{}`\n", file));
            }
            output.push('\n');
        }
        
        if !result.scope_guidance.files_to_avoid.is_empty() {
            output.push_str("### Files to Avoid\n\n");
            for file in &result.scope_guidance.files_to_avoid {
                output.push_str(&format!("- `{}` ⚠️\n", file));
            }
            output.push('\n');
        }
        
        // Testing strategy
        if !result.test_recommendations.is_empty() {
            output.push_str("## Testing Strategy\n\n");
            for test in &result.test_recommendations {
                output.push_str(&format!("### {} Tests\n\n", test.test_type.to_uppercase()));
                output.push_str(&format!("**Target**: `{}`\n\n", test.test_target));
                output.push_str(&format!("**Rationale**: {}\n\n", test.rationale));
                output.push_str(&format!("**Suggested Location**: `{}`\n\n", test.suggested_location));
            }
        }
        
        // Pre-development checklist
        if self.include_checklists {
            output.push_str("## Pre-Development Checklist\n\n");
            output.push_str("- [ ] Review impact analysis and risk assessment\n");
            output.push_str("- [ ] Confirm scope boundaries with team\n");
            output.push_str("- [ ] Set up test framework for recommended tests\n");
            output.push_str("- [ ] Create feature branch\n");
            output.push_str("- [ ] Document expected behavior changes\n");
            output.push('\n');
        }
        
        // Impact diagram
        if self.include_diagrams {
            output.push_str("## Impact Diagram\n\n");
            output.push_str("```mermaid\n");
            output.push_str("graph LR\n");
            output.push_str(&format!("    Target[{}]\n", result.target_entity));
            output.push_str("    Target --> DirectImpact[Direct Impact]\n");
            output.push_str("    Target --> IndirectImpact[Indirect Impact]\n");
            output.push_str("```\n\n");
        }
        
        // Metadata
        output.push_str("---\n");
        output.push_str(&format!("**Analysis completed in**: {:.2}s\n", result.execution_time.as_secs_f64()));
        output.push_str(&format!("**Generated at**: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
    
    fn format_debug(&self, result: &DebugResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        let mut output = String::new();
        
        // Header
        output.push_str("# Debug Analysis Report\n\n");
        output.push_str(&format!("**Target Entity**: `{}`\n\n", result.target_entity));
        
        // Caller analysis
        if !result.caller_traces.is_empty() {
            output.push_str("## Caller Analysis\n\n");
            for trace in &result.caller_traces {
                output.push_str(&format!("### {}\n\n", trace.caller.name));
                output.push_str(&format!("- **Call Depth**: {}\n", trace.depth));
                output.push_str(&format!("- **Context**: {}\n", trace.call_context));
                output.push_str(&format!("- **Location**: `{}`\n", trace.caller.file_path));
                if let Some(freq) = &trace.frequency {
                    output.push_str(&format!("- **Frequency**: {}\n", freq));
                }
                output.push('\n');
            }
        }
        
        // Usage analysis
        if !result.usage_sites.is_empty() {
            output.push_str("## Usage Sites\n\n");
            for usage in &result.usage_sites {
                output.push_str(&format!("### {}\n\n", usage.user.name));
                output.push_str(&format!("- **Usage Type**: {}\n", usage.usage_type));
                output.push_str(&format!("- **Context**: {}\n", usage.context));
                output.push_str(&format!("- **Location**: `{}`\n", usage.location.format_for_editor()));
                output.push('\n');
            }
        }
        
        // Change recommendations
        output.push_str("## Change Recommendations\n\n");
        output.push_str("### Minimal Change Scope\n\n");
        
        if !result.minimal_scope.minimal_files.is_empty() {
            output.push_str("**Files to modify**:\n");
            for file in &result.minimal_scope.minimal_files {
                output.push_str(&format!("- `{}`\n", file));
            }
            output.push('\n');
        }
        
        if !result.minimal_scope.safe_boundaries.is_empty() {
            output.push_str("**Safe boundaries**:\n");
            for boundary in &result.minimal_scope.safe_boundaries {
                output.push_str(&format!("- {}\n", boundary));
            }
            output.push('\n');
        }
        
        output.push_str(&format!("**Rollback strategy**: {}\n\n", result.minimal_scope.rollback_strategy));
        
        // Safety checklist
        if self.include_checklists {
            output.push_str("## Safety Checklist\n\n");
            output.push_str("- [ ] Backup current state\n");
            output.push_str("- [ ] Write tests for current behavior\n");
            output.push_str("- [ ] Make minimal changes only\n");
            output.push_str("- [ ] Test all caller sites\n");
            output.push_str("- [ ] Verify rollback strategy works\n");
            output.push('\n');
        }
        
        // Metadata
        output.push_str("---\n");
        output.push_str(&format!("**Analysis completed in**: {:.2}s\n", result.execution_time.as_secs_f64()));
        output.push_str(&format!("**Generated at**: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
    
    fn format_refactor(&self, result: &RefactorResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        let mut output = String::new();
        
        // Header
        output.push_str("# Refactoring Safety Analysis\n\n");
        output.push_str(&format!("**Target Entity**: `{}`\n\n", result.target_entity));
        
        // Risk factors
        output.push_str("## Risk Factors\n\n");
        output.push_str(&format!("**Overall Risk**: {:?}\n", result.risk_assessment.overall_risk));
        output.push_str(&format!("**Confidence Level**: {:?}\n\n", result.risk_assessment.confidence));
        
        if !result.risk_assessment.risk_factors.is_empty() {
            output.push_str("### Identified Risks\n\n");
            for factor in &result.risk_assessment.risk_factors {
                output.push_str(&format!("#### {} ({:?})\n\n", factor.description, factor.level));
                output.push_str(&format!("{}\n\n", factor.impact));
            }
        }
        
        if !result.risk_assessment.mitigations.is_empty() {
            output.push_str("### Risk Mitigations\n\n");
            for mitigation in &result.risk_assessment.mitigations {
                output.push_str(&format!("- {}\n", mitigation));
            }
            output.push('\n');
        }
        
        // Pre-refactor checklist
        if !result.change_checklist.is_empty() && self.include_checklists {
            output.push_str("## Pre-Refactor Checklist\n\n");
            for item in &result.change_checklist {
                let checkbox = if item.completed { "- [x]" } else { "- [ ]" };
                output.push_str(&format!("{} {} ({:?})\n", checkbox, item.description, item.priority));
                if let Some(notes) = &item.notes {
                    output.push_str(&format!("  - *Note: {}*\n", notes));
                }
            }
            output.push('\n');
        }
        
        // Reviewer focus areas
        output.push_str("## Reviewer Focus Areas\n\n");
        if !result.reviewer_guidance.focus_areas.is_empty() {
            output.push_str("### Key Areas to Review\n\n");
            for area in &result.reviewer_guidance.focus_areas {
                output.push_str(&format!("- {}\n", area));
            }
            output.push('\n');
        }
        
        if !result.reviewer_guidance.potential_issues.is_empty() {
            output.push_str("### Potential Issues to Watch For\n\n");
            for issue in &result.reviewer_guidance.potential_issues {
                output.push_str(&format!("- ⚠️ {}\n", issue));
            }
            output.push('\n');
        }
        
        if !result.reviewer_guidance.testing_recommendations.is_empty() {
            output.push_str("### Testing Recommendations\n\n");
            for rec in &result.reviewer_guidance.testing_recommendations {
                output.push_str(&format!("- {}\n", rec));
            }
            output.push('\n');
        }
        
        if !result.reviewer_guidance.approval_criteria.is_empty() {
            output.push_str("### Approval Criteria\n\n");
            for criteria in &result.reviewer_guidance.approval_criteria {
                output.push_str(&format!("- [ ] {}\n", criteria));
            }
            output.push('\n');
        }
        
        // Risk diagram
        if self.include_diagrams {
            output.push_str("## Risk Assessment Diagram\n\n");
            output.push_str("```mermaid\n");
            output.push_str("graph TD\n");
            output.push_str(&format!("    Target[{}]\n", result.target_entity));
            output.push_str(&format!("    Risk[Overall Risk: {:?}]\n", result.risk_assessment.overall_risk));
            output.push_str("    Target --> Risk\n");
            output.push_str("```\n\n");
        }
        
        // Metadata
        output.push_str("---\n");
        output.push_str(&format!("**Analysis completed in**: {:.2}s\n", result.execution_time.as_secs_f64()));
        output.push_str(&format!("**Generated at**: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
}

/// CI/CD formatter for continuous integration output
/// 
/// Produces GitHub Actions compatible output with risk levels, gates, and actionable recommendations.
/// Includes environment variables and workflow annotations.
pub struct CiFormatter {
    /// CI/CD platform type (github, gitlab, etc.)
    platform: CiPlatform,
    /// Whether to include environment variable exports
    export_variables: bool,
}

/// Supported CI/CD platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CiPlatform {
    /// GitHub Actions
    GitHub,
    /// GitLab CI
    GitLab,
    /// Generic CI (basic output)
    Generic,
}

impl CiFormatter {
    /// Create new CI formatter for GitHub Actions
    pub fn new() -> Self {
        Self {
            platform: CiPlatform::GitHub,
            export_variables: true,
        }
    }
    
    /// Create CI formatter for specific platform
    pub fn for_platform(platform: CiPlatform) -> Self {
        Self {
            platform,
            export_variables: true,
        }
    }
    
    /// Create CI formatter with custom settings
    pub fn with_options(platform: CiPlatform, export_variables: bool) -> Self {
        Self {
            platform,
            export_variables,
        }
    }
}

impl Default for CiFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for CiFormatter {
    fn format_onboarding(&self, result: &OnboardingResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        let mut output = String::new();
        
        match self.platform {
            CiPlatform::GitHub => {
                // GitHub Actions format
                output.push_str("::notice title=Onboarding Complete::Codebase analysis completed successfully\n");
                
                if self.export_variables {
                    output.push_str(&format!("echo \"ONBOARD_STATUS=SUCCESS\" >> $GITHUB_ENV\n"));
                    output.push_str(&format!("echo \"TOTAL_FILES={}\" >> $GITHUB_ENV\n", result.overview.total_files));
                    output.push_str(&format!("echo \"TOTAL_ENTITIES={}\" >> $GITHUB_ENV\n", result.overview.total_entities));
                    
                    if !result.overview.architecture_patterns.is_empty() {
                        let patterns = result.overview.architecture_patterns.join(",");
                        output.push_str(&format!("echo \"ARCHITECTURE_PATTERNS={}\" >> $GITHUB_ENV\n", patterns));
                    }
                    
                    if !result.next_steps.is_empty() {
                        let actions = result.next_steps.join(";");
                        output.push_str(&format!("echo \"NEXT_ACTIONS={}\" >> $GITHUB_ENV\n", actions));
                    }
                }
                
                // Summary
                output.push_str("::group::Onboarding Summary\n");
                output.push_str(&format!("Total files analyzed: {}\n", result.overview.total_files));
                output.push_str(&format!("Total entities found: {}\n", result.overview.total_entities));
                output.push_str(&format!("Entry points identified: {}\n", result.entry_points.len()));
                output.push_str(&format!("Key contexts: {}\n", result.key_contexts.len()));
                output.push_str("::endgroup::\n");
            }
            
            CiPlatform::GitLab => {
                // GitLab CI format
                output.push_str("echo \"✅ Onboarding analysis completed\"\n");
                
                if self.export_variables {
                    output.push_str(&format!("export ONBOARD_STATUS=SUCCESS\n"));
                    output.push_str(&format!("export TOTAL_FILES={}\n", result.overview.total_files));
                    output.push_str(&format!("export TOTAL_ENTITIES={}\n", result.overview.total_entities));
                }
            }
            
            CiPlatform::Generic => {
                // Generic CI format
                output.push_str("# Onboarding Analysis Complete\n");
                output.push_str(&format!("ONBOARD_STATUS=SUCCESS\n"));
                output.push_str(&format!("TOTAL_FILES={}\n", result.overview.total_files));
                output.push_str(&format!("TOTAL_ENTITIES={}\n", result.overview.total_entities));
            }
        }
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
    
    fn format_feature_plan(&self, result: &FeaturePlanResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        let mut output = String::new();
        
        let risk_level_str = format!("{:?}", result.impact_analysis.risk_level).to_uppercase();
        let complexity_str = format!("{:?}", result.impact_analysis.complexity_estimate).to_uppercase();
        
        match self.platform {
            CiPlatform::GitHub => {
                // Risk-based notifications
                match result.impact_analysis.risk_level {
                    crate::discovery::WorkflowRiskLevel::High | crate::discovery::WorkflowRiskLevel::Critical => {
                        output.push_str(&format!("::error title=High Risk Feature::Feature {} has high risk level\n", result.target_entity));
                    }
                    crate::discovery::WorkflowRiskLevel::Medium => {
                        output.push_str(&format!("::warning title=Medium Risk Feature::Feature {} requires careful review\n", result.target_entity));
                    }
                    crate::discovery::WorkflowRiskLevel::Low => {
                        output.push_str(&format!("::notice title=Low Risk Feature::Feature {} is low risk\n", result.target_entity));
                    }
                }
                
                if self.export_variables {
                    output.push_str(&format!("echo \"RISK_LEVEL={}\" >> $GITHUB_ENV\n", risk_level_str));
                    output.push_str(&format!("echo \"COMPLEXITY={}\" >> $GITHUB_ENV\n", complexity_str));
                    output.push_str(&format!("echo \"TARGET_ENTITY={}\" >> $GITHUB_ENV\n", result.target_entity));
                    output.push_str(&format!("echo \"REQUIRED_TESTS={}\" >> $GITHUB_ENV\n", result.test_recommendations.len()));
                    
                    // Set approval requirements based on risk
                    let approval_required = match result.impact_analysis.risk_level {
                        crate::discovery::WorkflowRiskLevel::High | crate::discovery::WorkflowRiskLevel::Critical => "true",
                        _ => "false",
                    };
                    output.push_str(&format!("echo \"APPROVAL_REQUIRED={}\" >> $GITHUB_ENV\n", approval_required));
                }
                
                // Test requirements
                if !result.test_recommendations.is_empty() {
                    output.push_str("::group::Required Tests\n");
                    for test in &result.test_recommendations {
                        output.push_str(&format!("- {} test for {}: {}\n", test.test_type, test.test_target, test.rationale));
                    }
                    output.push_str("::endgroup::\n");
                }
            }
            
            CiPlatform::GitLab => {
                output.push_str(&format!("echo \"🎯 Feature planning for {} complete\"\n", result.target_entity));
                
                if self.export_variables {
                    output.push_str(&format!("export RISK_LEVEL={}\n", risk_level_str));
                    output.push_str(&format!("export COMPLEXITY={}\n", complexity_str));
                    output.push_str(&format!("export TARGET_ENTITY={}\n", result.target_entity));
                }
            }
            
            CiPlatform::Generic => {
                output.push_str(&format!("# Feature Planning: {}\n", result.target_entity));
                output.push_str(&format!("RISK_LEVEL={}\n", risk_level_str));
                output.push_str(&format!("COMPLEXITY={}\n", complexity_str));
                output.push_str(&format!("REQUIRED_TESTS={}\n", result.test_recommendations.len()));
            }
        }
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
    
    fn format_debug(&self, result: &DebugResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        let mut output = String::new();
        
        match self.platform {
            CiPlatform::GitHub => {
                output.push_str(&format!("::notice title=Debug Analysis::Analysis complete for {}\n", result.target_entity));
                
                if self.export_variables {
                    output.push_str(&format!("echo \"DEBUG_TARGET={}\" >> $GITHUB_ENV\n", result.target_entity));
                    output.push_str(&format!("echo \"CALLER_COUNT={}\" >> $GITHUB_ENV\n", result.caller_traces.len()));
                    output.push_str(&format!("echo \"USAGE_SITES={}\" >> $GITHUB_ENV\n", result.usage_sites.len()));
                    output.push_str(&format!("echo \"FILES_TO_CHANGE={}\" >> $GITHUB_ENV\n", result.minimal_scope.minimal_files.len()));
                }
                
                // Change scope summary
                output.push_str("::group::Change Scope\n");
                output.push_str(&format!("Files to modify: {}\n", result.minimal_scope.minimal_files.len()));
                output.push_str(&format!("Rollback strategy: {}\n", result.minimal_scope.rollback_strategy));
                output.push_str("::endgroup::\n");
            }
            
            CiPlatform::GitLab => {
                output.push_str(&format!("echo \"🐛 Debug analysis for {} complete\"\n", result.target_entity));
                
                if self.export_variables {
                    output.push_str(&format!("export DEBUG_TARGET={}\n", result.target_entity));
                    output.push_str(&format!("export CALLER_COUNT={}\n", result.caller_traces.len()));
                }
            }
            
            CiPlatform::Generic => {
                output.push_str(&format!("# Debug Analysis: {}\n", result.target_entity));
                output.push_str(&format!("CALLER_COUNT={}\n", result.caller_traces.len()));
                output.push_str(&format!("USAGE_SITES={}\n", result.usage_sites.len()));
                output.push_str(&format!("FILES_TO_CHANGE={}\n", result.minimal_scope.minimal_files.len()));
            }
        }
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
    
    fn format_refactor(&self, result: &RefactorResult) -> Result<String, FormattingError> {
        let start = std::time::Instant::now();
        let mut output = String::new();
        
        let risk_level_str = format!("{:?}", result.risk_assessment.overall_risk).to_uppercase();
        
        match self.platform {
            CiPlatform::GitHub => {
                // Risk-based notifications
                match result.risk_assessment.overall_risk {
                    crate::discovery::WorkflowRiskLevel::High | crate::discovery::WorkflowRiskLevel::Critical => {
                        output.push_str(&format!("::error title=High Risk Refactor::Refactoring {} is high risk - requires approval\n", result.target_entity));
                    }
                    crate::discovery::WorkflowRiskLevel::Medium => {
                        output.push_str(&format!("::warning title=Medium Risk Refactor::Refactoring {} requires careful review\n", result.target_entity));
                    }
                    crate::discovery::WorkflowRiskLevel::Low => {
                        output.push_str(&format!("::notice title=Low Risk Refactor::Refactoring {} is low risk\n", result.target_entity));
                    }
                }
                
                if self.export_variables {
                    output.push_str(&format!("echo \"REFACTOR_RISK={}\" >> $GITHUB_ENV\n", risk_level_str));
                    output.push_str(&format!("echo \"TARGET_ENTITY={}\" >> $GITHUB_ENV\n", result.target_entity));
                    
                    // Set approval requirements
                    let approval_required = match result.risk_assessment.overall_risk {
                        crate::discovery::WorkflowRiskLevel::High | crate::discovery::WorkflowRiskLevel::Critical => "true",
                        _ => "false",
                    };
                    output.push_str(&format!("echo \"APPROVAL_REQUIRED={}\" >> $GITHUB_ENV\n", approval_required));
                    
                    // Safety checks count
                    let incomplete_checks = result.change_checklist.iter()
                        .filter(|item| !item.completed)
                        .count();
                    output.push_str(&format!("echo \"SAFETY_CHECKS={}\" >> $GITHUB_ENV\n", incomplete_checks));
                }
                
                // Safety checklist
                if !result.change_checklist.is_empty() {
                    output.push_str("::group::Safety Checklist\n");
                    for item in &result.change_checklist {
                        let status = if item.completed { "✅" } else { "❌" };
                        output.push_str(&format!("{} {} ({:?})\n", status, item.description, item.priority));
                    }
                    output.push_str("::endgroup::\n");
                }
            }
            
            CiPlatform::GitLab => {
                output.push_str(&format!("echo \"🔧 Refactor safety check for {} complete\"\n", result.target_entity));
                
                if self.export_variables {
                    output.push_str(&format!("export REFACTOR_RISK={}\n", risk_level_str));
                    output.push_str(&format!("export TARGET_ENTITY={}\n", result.target_entity));
                }
            }
            
            CiPlatform::Generic => {
                output.push_str(&format!("# Refactor Safety: {}\n", result.target_entity));
                output.push_str(&format!("REFACTOR_RISK={}\n", risk_level_str));
                
                let incomplete_checks = result.change_checklist.iter()
                    .filter(|item| !item.completed)
                    .count();
                output.push_str(&format!("SAFETY_CHECKS={}\n", incomplete_checks));
            }
        }
        
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            return Err(FormattingError::Timeout {
                elapsed,
                limit: Duration::from_millis(100),
            });
        }
        
        Ok(output)
    }
}

/// Factory for creating formatters based on format string
pub struct FormatterFactory;

impl FormatterFactory {
    /// Create formatter from format string
    /// 
    /// # Supported Formats
    /// - "human" -> HumanFormatter
    /// - "json" -> JsonFormatter  
    /// - "pr-summary" -> PrSummaryFormatter
    /// - "ci" -> CiFormatter
    /// 
    /// # Error Conditions
    /// - FormattingError::InvalidFormat for unsupported format strings
    pub fn create_formatter(format: &str) -> Result<Box<dyn OutputFormatter>, FormattingError> {
        match format.to_lowercase().as_str() {
            "human" => Ok(Box::new(HumanFormatter::new())),
            "json" => Ok(Box::new(JsonFormatter::new())),
            "pr-summary" | "pr_summary" => Ok(Box::new(PrSummaryFormatter::new())),
            "ci" | "ci-cd" => Ok(Box::new(CiFormatter::new())),
            _ => Err(FormattingError::InvalidFormat {
                format: format.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use crate::discovery::{
        OnboardingResult, FeaturePlanResult, DebugResult, RefactorResult,
        CodebaseOverview, EntryPoint, KeyContext, ImpactAnalysis, ScopeGuidance,
        TestRecommendation, ChangeScope, RiskAssessment, ChecklistItem, ReviewerGuidance,
        ModuleInfo, RiskFactor, ComplexityLevel, ConfidenceLevel, Priority, FileLocation
    };

    // TDD RED PHASE: Test formatter creation
    #[test]
    fn test_formatter_factory_creation() {
        // Test valid format strings
        assert!(FormatterFactory::create_formatter("human").is_ok());
        assert!(FormatterFactory::create_formatter("json").is_ok());
        assert!(FormatterFactory::create_formatter("pr-summary").is_ok());
        assert!(FormatterFactory::create_formatter("ci").is_ok());
        
        // Test invalid format string
        assert!(FormatterFactory::create_formatter("invalid").is_err());
    }

    #[test]
    fn test_formatter_factory_case_insensitive() {
        // Should handle case variations
        assert!(FormatterFactory::create_formatter("HUMAN").is_ok());
        assert!(FormatterFactory::create_formatter("Json").is_ok());
        assert!(FormatterFactory::create_formatter("PR-SUMMARY").is_ok());
        assert!(FormatterFactory::create_formatter("CI").is_ok());
    }

    #[test]
    fn test_formatter_factory_aliases() {
        // Should handle format aliases
        assert!(FormatterFactory::create_formatter("pr_summary").is_ok());
        assert!(FormatterFactory::create_formatter("ci-cd").is_ok());
    }

    // TDD RED PHASE: Test formatter defaults
    #[test]
    fn test_human_formatter_defaults() {
        let formatter = HumanFormatter::new();
        assert!(formatter.include_timing);
        assert!(formatter.use_emojis);
    }

    #[test]
    fn test_json_formatter_defaults() {
        let formatter = JsonFormatter::new();
        assert!(formatter.pretty_print);
        assert!(formatter.include_metadata);
    }

    #[test]
    fn test_pr_summary_formatter_defaults() {
        let formatter = PrSummaryFormatter::new();
        assert!(formatter.include_diagrams);
        assert!(formatter.include_checklists);
    }

    #[test]
    fn test_ci_formatter_defaults() {
        let formatter = CiFormatter::new();
        assert_eq!(formatter.platform, CiPlatform::GitHub);
        assert!(formatter.export_variables);
    }

    // TDD RED PHASE: Test formatter customization
    #[test]
    fn test_human_formatter_customization() {
        let formatter = HumanFormatter::with_options(false, false);
        assert!(!formatter.include_timing);
        assert!(!formatter.use_emojis);
    }

    #[test]
    fn test_json_formatter_customization() {
        let formatter = JsonFormatter::with_options(false, false);
        assert!(!formatter.pretty_print);
        assert!(!formatter.include_metadata);
    }

    #[test]
    fn test_ci_formatter_platform_selection() {
        let github_formatter = CiFormatter::for_platform(CiPlatform::GitHub);
        assert_eq!(github_formatter.platform, CiPlatform::GitHub);
        
        let gitlab_formatter = CiFormatter::for_platform(CiPlatform::GitLab);
        assert_eq!(gitlab_formatter.platform, CiPlatform::GitLab);
        
        let generic_formatter = CiFormatter::for_platform(CiPlatform::Generic);
        assert_eq!(generic_formatter.platform, CiPlatform::Generic);
    }

    // TDD RED PHASE: Test error types
    #[test]
    fn test_formatting_error_types() {
        let serialization_error = FormattingError::SerializationFailed {
            message: "Invalid JSON".to_string(),
        };
        assert!(serialization_error.to_string().contains("JSON serialization failed"));
        
        let template_error = FormattingError::TemplateError {
            template: "test_template".to_string(),
            error: "Missing variable".to_string(),
        };
        assert!(template_error.to_string().contains("Template rendering failed"));
        
        let format_error = FormattingError::InvalidFormat {
            format: "unknown".to_string(),
        };
        assert!(format_error.to_string().contains("Invalid output format"));
        
        let timeout_error = FormattingError::Timeout {
            elapsed: Duration::from_millis(200),
            limit: Duration::from_millis(100),
        };
        assert!(timeout_error.to_string().contains("Formatting timeout"));
    }

    // TDD RED PHASE: Test performance contract
    #[test]
    fn test_formatter_performance_contract() {
        // Contract: Formatter creation should be fast
        let start = Instant::now();
        let _formatter = HumanFormatter::new();
        let elapsed = start.elapsed();
        
        assert!(elapsed < Duration::from_millis(10), 
                "Formatter creation took {:?}, expected <10ms", elapsed);
    }

    // TDD RED PHASE: Test trait object compatibility
    #[test]
    fn test_trait_object_compatibility() {
        // Should be able to use formatters as trait objects
        let formatters: Vec<Box<dyn OutputFormatter>> = vec![
            Box::new(HumanFormatter::new()),
            Box::new(JsonFormatter::new()),
            Box::new(PrSummaryFormatter::new()),
            Box::new(CiFormatter::new()),
        ];
        
        assert_eq!(formatters.len(), 4);
    }

    // TDD GREEN PHASE: Test actual formatting with real data
    #[test]
    fn test_human_formatter_onboarding_output() {
        let formatter = HumanFormatter::new();
        let result = create_test_onboarding_result();
        
        let output = formatter.format_onboarding(&result).unwrap();
        
        // Should contain key sections
        assert!(output.contains("🚀 Codebase Onboarding Complete"));
        assert!(output.contains("📊 Codebase Overview:"));
        assert!(output.contains("Total files: 100"));
        assert!(output.contains("Total entities: 500"));
        assert!(output.contains("⏱️  Workflow completed"));
        
        // Should be copy-pastable (no control characters)
        assert!(!output.contains('\x1b')); // No ANSI escape codes
        
        // Should not contain problematic control characters
        assert!(!output.chars().any(|c| c.is_control() && c != '\n' && c != '\t'));
    }

    #[test]
    fn test_json_formatter_onboarding_output() {
        let formatter = JsonFormatter::new();
        let result = create_test_onboarding_result();
        
        let output = formatter.format_onboarding(&result).unwrap();
        
        // Should be valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["workflow"], "onboard");
        assert!(parsed["result"].is_object());
        assert!(parsed["execution_time_s"].is_number());
        assert!(parsed["timestamp"].is_string());
        
        // Should be pretty-printed by default
        assert!(output.contains("  ")); // Indentation
        assert!(output.contains("\n")); // Line breaks
    }

    #[test]
    fn test_pr_summary_formatter_feature_plan_output() {
        let formatter = PrSummaryFormatter::new();
        let result = create_test_feature_plan_result();
        
        let output = formatter.format_feature_plan(&result).unwrap();
        
        // Should be valid markdown
        assert!(output.starts_with("# Feature Development Plan"));
        assert!(output.contains("**Target Entity**: `test_function`"));
        assert!(output.contains("## Risk Assessment"));
        assert!(output.contains("- **Risk Level**: Medium"));
        assert!(output.contains("```mermaid"));
        assert!(output.contains("## Pre-Development Checklist"));
        assert!(output.contains("- [ ] Review impact analysis"));
        
        // Should be copy-pastable markdown
        assert!(!output.contains('\x1b')); // No ANSI escape codes
        assert!(output.lines().all(|line| line.len() <= 120)); // Reasonable line length
    }

    #[test]
    fn test_ci_formatter_github_actions_output() {
        let formatter = CiFormatter::for_platform(CiPlatform::GitHub);
        let result = create_test_refactor_result();
        
        let output = formatter.format_refactor(&result).unwrap();
        
        // Should contain GitHub Actions annotations
        if result.risk_assessment.overall_risk == crate::discovery::WorkflowRiskLevel::High {
            assert!(output.contains("::error title=High Risk Refactor::"));
        }
        assert!(output.contains("echo \"REFACTOR_RISK="));
        assert!(output.contains(">> $GITHUB_ENV"));
        assert!(output.contains("::group::Safety Checklist"));
        assert!(output.contains("::endgroup::"));
        
        // Should be valid shell commands
        for line in output.lines() {
            if line.starts_with("echo ") {
                assert!(line.contains(">> $GITHUB_ENV") || line.starts_with("echo \""));
            }
        }
    }

    #[test]
    fn test_ci_formatter_gitlab_output() {
        let formatter = CiFormatter::for_platform(CiPlatform::GitLab);
        let result = create_test_debug_result();
        
        let output = formatter.format_debug(&result).unwrap();
        
        // Should contain GitLab CI format
        assert!(output.contains("echo \"🐛 Debug analysis"));
        assert!(output.contains("export DEBUG_TARGET="));
        assert!(output.contains("export CALLER_COUNT="));
        
        // Should be valid shell commands
        for line in output.lines() {
            if line.starts_with("export ") {
                assert!(line.contains("="));
            }
        }
    }

    #[test]
    fn test_formatter_consistency_across_formats() {
        let onboarding_result = create_test_onboarding_result();
        
        let human_formatter = HumanFormatter::new();
        let json_formatter = JsonFormatter::new();
        let pr_formatter = PrSummaryFormatter::new();
        let ci_formatter = CiFormatter::new();
        
        // All formatters should handle the same data without errors
        assert!(human_formatter.format_onboarding(&onboarding_result).is_ok());
        assert!(json_formatter.format_onboarding(&onboarding_result).is_ok());
        assert!(pr_formatter.format_onboarding(&onboarding_result).is_ok());
        assert!(ci_formatter.format_onboarding(&onboarding_result).is_ok());
        
        // All outputs should contain the core information
        let human_output = human_formatter.format_onboarding(&onboarding_result).unwrap();
        let json_output = json_formatter.format_onboarding(&onboarding_result).unwrap();
        let pr_output = pr_formatter.format_onboarding(&onboarding_result).unwrap();
        let ci_output = ci_formatter.format_onboarding(&onboarding_result).unwrap();
        
        // Core data should be present in all formats
        assert!(human_output.contains("100")); // Total files
        assert!(json_output.contains("100"));
        assert!(pr_output.contains("100"));
        assert!(ci_output.contains("100"));
    }

    #[test]
    fn test_formatter_performance_contracts() {
        let result = create_test_onboarding_result();
        let formatters: Vec<Box<dyn OutputFormatter>> = vec![
            Box::new(HumanFormatter::new()),
            Box::new(JsonFormatter::new()),
            Box::new(PrSummaryFormatter::new()),
            Box::new(CiFormatter::new()),
        ];
        
        for formatter in formatters {
            let start = Instant::now();
            let output = formatter.format_onboarding(&result).unwrap();
            let elapsed = start.elapsed();
            
            // Contract: All formatting should complete within 100ms
            assert!(elapsed < Duration::from_millis(100), 
                    "Formatting took {:?}, expected <100ms", elapsed);
            
            // Output should not be empty
            assert!(!output.is_empty());
        }
    }

    #[test]
    fn test_copy_pastable_output_validation() {
        let result = create_test_feature_plan_result();
        
        // Human formatter should produce clean terminal output
        let human_formatter = HumanFormatter::new();
        let human_output = human_formatter.format_feature_plan(&result).unwrap();
        
        // Should not contain control characters that break copy-paste
        assert!(!human_output.contains('\x1b')); // ANSI escape codes
        assert!(!human_output.contains('\x07')); // Bell character
        assert!(!human_output.contains('\x08')); // Backspace
        
        // Should not contain problematic control characters
        assert!(!human_output.chars().any(|c| c.is_control() && c != '\n' && c != '\t'));
        
        // PR formatter should produce valid markdown
        let pr_formatter = PrSummaryFormatter::new();
        let pr_output = pr_formatter.format_feature_plan(&result).unwrap();
        
        // Should be valid markdown structure
        assert!(pr_output.lines().any(|line| line.starts_with("# ")));
        assert!(pr_output.lines().any(|line| line.starts_with("## ")));
        assert!(pr_output.lines().any(|line| line.starts_with("- ")));
        
        // JSON formatter should produce valid JSON
        let json_formatter = JsonFormatter::new();
        let json_output = json_formatter.format_feature_plan(&result).unwrap();
        
        // Should parse as valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json_output).unwrap();
        assert!(parsed.is_object());
    }

    // Helper functions for creating test data
    fn create_test_onboarding_result() -> OnboardingResult {
        use std::collections::HashMap;
        
        OnboardingResult {
            timestamp: chrono::Utc::now(),
            execution_time: Duration::from_secs(30),
            overview: CodebaseOverview {
                total_files: 100,
                total_entities: 500,
                entities_by_type: {
                    let mut map = HashMap::new();
                    map.insert("Function".to_string(), 300);
                    map.insert("Struct".to_string(), 150);
                    map.insert("Trait".to_string(), 50);
                    map
                },
                key_modules: vec![
                    ModuleInfo {
                        name: "core".to_string(),
                        purpose: "Core functionality".to_string(),
                        key_entities: vec!["main".to_string(), "init".to_string()],
                        dependencies: vec!["std".to_string()],
                    }
                ],
                architecture_patterns: vec!["MVC".to_string(), "Repository".to_string()],
            },
            entry_points: vec![
                EntryPoint {
                    name: "main".to_string(),
                    entry_type: "binary".to_string(),
                    location: FileLocation::with_line("src/main.rs".to_string(), 1),
                    description: "Application entry point".to_string(),
                }
            ],
            key_contexts: vec![
                KeyContext {
                    name: "AppState".to_string(),
                    context_type: "struct".to_string(),
                    importance: "Central application state".to_string(),
                    related_entities: vec!["Config".to_string(), "Database".to_string()],
                    location: FileLocation::with_line("src/app.rs".to_string(), 10),
                }
            ],
            next_steps: vec![
                "Read main.rs to understand entry point".to_string(),
                "Explore core module for key functionality".to_string(),
            ],
        }
    }

    fn create_test_feature_plan_result() -> FeaturePlanResult {
        FeaturePlanResult {
            timestamp: chrono::Utc::now(),
            execution_time: Duration::from_secs(60),
            target_entity: "test_function".to_string(),
            impact_analysis: ImpactAnalysis {
                direct_impact: vec![],
                indirect_impact: vec![],
                risk_level: crate::discovery::WorkflowRiskLevel::Medium,
                complexity_estimate: ComplexityLevel::Moderate,
            },
            scope_guidance: ScopeGuidance {
                boundaries: vec!["module_boundary".to_string()],
                files_to_modify: vec!["src/lib.rs".to_string()],
                files_to_avoid: vec!["src/main.rs".to_string()],
                integration_points: vec!["API endpoint".to_string()],
            },
            test_recommendations: vec![
                TestRecommendation {
                    test_type: "unit".to_string(),
                    test_target: "test_function".to_string(),
                    rationale: "Verify core functionality".to_string(),
                    suggested_location: "tests/unit/test_function_test.rs".to_string(),
                }
            ],
        }
    }

    fn create_test_debug_result() -> DebugResult {
        DebugResult {
            timestamp: chrono::Utc::now(),
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
        }
    }

    fn create_test_refactor_result() -> RefactorResult {
        RefactorResult {
            timestamp: chrono::Utc::now(),
            execution_time: Duration::from_secs(90),
            target_entity: "refactor_target".to_string(),
            risk_assessment: RiskAssessment {
                overall_risk: crate::discovery::WorkflowRiskLevel::High,
                risk_factors: vec![
                    RiskFactor {
                        description: "High coupling".to_string(),
                        level: crate::discovery::WorkflowRiskLevel::High,
                        impact: "May break multiple components".to_string(),
                    }
                ],
                mitigations: vec!["Add comprehensive tests".to_string()],
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
        }
    }
}
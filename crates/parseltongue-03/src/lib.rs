//! Tool 2: cozo-code-simulation-sorcerer
//!
//! Code change simulation wizard that uses LLM reasoning to:
//! - Generate step-by-step simulation plans
//! - Create rubber duck debugging artifacts
//! - Score confidence and enforce thresholds
//!
//! Following TDD-first principle - tests first, implementation second

pub mod change_request;
pub mod confidence_scorer;
pub mod debugging_info;
pub mod graph_analyzer;
pub mod reasoning_engine;
pub mod simulation_plan;
pub mod sorcerer;

pub use change_request::ChangeRequest;
pub use confidence_scorer::{ConfidenceScore, ConfidenceScorer, ConfidenceThreshold};
pub use debugging_info::DebuggingInfo;
pub use graph_analyzer::GraphAnalyzer;
pub use reasoning_engine::{ReasoningEngine, MockReasoningEngine, LLMConfig};
pub use simulation_plan::SimulationPlan;
pub use sorcerer::CozoCodeSimulationSorcerer;

/// Simulation error types for structured error handling
#[derive(Debug, Clone, PartialEq)]
pub enum SimulationError {
    ValidationError(String),
    ReasoningError(String),
    ConfidenceError(String),
    GraphAnalysisError(String),
    PlanGenerationError(String),
    ExecutionError(String),
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            SimulationError::ReasoningError(msg) => write!(f, "Reasoning Error: {}", msg),
            SimulationError::ConfidenceError(msg) => write!(f, "Confidence Error: {}", msg),
            SimulationError::GraphAnalysisError(msg) => write!(f, "Graph Analysis Error: {}", msg),
            SimulationError::PlanGenerationError(msg) => write!(f, "Plan Generation Error: {}", msg),
            SimulationError::ExecutionError(msg) => write!(f, "Execution Error: {}", msg),
        }
    }
}

impl std::error::Error for SimulationError {}

/// Tool 2 re-export for convenience
pub struct Tool2 {
    sorcerer: CozoCodeSimulationSorcerer,
}

impl Tool2 {
    pub fn new() -> Self {
        Self {
            sorcerer: CozoCodeSimulationSorcerer::new(),
        }
    }

    pub fn sorcerer(&self) -> &CozoCodeSimulationSorcerer {
        &self.sorcerer
    }
}

impl Default for Tool2 {
    fn default() -> Self {
        Self::new()
    }
}

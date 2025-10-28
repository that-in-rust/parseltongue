//! Tool 3: Rust Preflight Code Simulator
//!
//! This crate provides comprehensive Rust code validation capabilities using rust-analyzer integration.
//! It implements TDD-first validation with performance contracts and Tool 2 integration.

pub mod analyzer;
pub mod performance;
pub mod tool2_integration;
pub mod validation;

// Re-export key types for convenience
pub use analyzer::RustAnalyzerClient;
pub use performance::{
    PerformanceRegressionAnalysis, ValidationPerformanceContract, ValidationPerformanceReport,
};
pub use tool2_integration::{
    IntegrationResult, SimulationToValidationConverter, Tool2SimulationParser,
    Tool2ValidationFormat, Tool2ValidationPipeline, ValidationToTool2Converter,
};
pub use validation::{
    DefaultRustCodeValidator, RustCodeValidator, ValidationError, ValidationOutput,
    ValidationReport, ValidationSeverity, ValidationTestCase, ValidationType,
    ValidatorCapabilities,
};

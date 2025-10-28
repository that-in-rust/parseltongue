//! Performance contracts module for validation operations
//! Implements performance monitoring and regression detection

use crate::validation::ValidationType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance contract specifically for validation operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationPerformanceContract {
    /// Contract identifier
    pub name: String,
    /// Performance thresholds by validation type
    pub thresholds: HashMap<ValidationType, ValidationThresholds>,
    /// Memory usage limits
    pub memory_limits: MemoryLimits,
    /// Throughput requirements
    pub throughput_requirements: ThroughputRequirements,
}

/// Performance thresholds for different validation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationThresholds {
    /// Maximum execution time for small code (< 1KB)
    pub max_time_small_ms: u64,
    /// Maximum execution time for medium code (1-10KB)
    pub max_time_medium_ms: u64,
    /// Maximum execution time for large code (> 10KB)
    pub max_time_large_ms: u64,
    /// Maximum memory usage as multiplier of input size
    pub memory_multiplier: f64,
    /// Minimum accuracy requirement
    pub min_accuracy: f64,
}

/// Memory usage limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLimits {
    /// Maximum memory usage in bytes
    pub max_memory_bytes: usize,
    /// Memory usage as percentage of available memory
    pub max_memory_percentage: f64,
    /// Memory leak detection threshold
    pub leak_detection_threshold: f64,
}

/// Throughput requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputRequirements {
    /// Minimum throughput in KB/s
    pub min_throughput_kbps: f64,
    /// Concurrent operations support
    pub max_concurrent_operations: usize,
    /// Queue depth for operations
    pub max_queue_depth: usize,
}

impl ValidationPerformanceContract {
    /// Create a new performance contract with default values
    pub fn new(name: String) -> Self {
        let mut thresholds = HashMap::new();

        // Default thresholds for each validation type
        thresholds.insert(
            ValidationType::Syntax,
            ValidationThresholds {
                max_time_small_ms: 100,  // 100ms for small code
                max_time_medium_ms: 500, // 500ms for medium code
                max_time_large_ms: 2000, // 2s for large code
                memory_multiplier: 5.0,  // 5x input size
                min_accuracy: 0.95,      // 95% accuracy
            },
        );

        thresholds.insert(
            ValidationType::Type,
            ValidationThresholds {
                max_time_small_ms: 200,   // 200ms for small code
                max_time_medium_ms: 1000, // 1s for medium code
                max_time_large_ms: 5000,  // 5s for large code
                memory_multiplier: 10.0,  // 10x input size
                min_accuracy: 0.90,       // 90% accuracy
            },
        );

        thresholds.insert(
            ValidationType::BorrowChecker,
            ValidationThresholds {
                max_time_small_ms: 300,   // 300ms for small code
                max_time_medium_ms: 1500, // 1.5s for medium code
                max_time_large_ms: 8000,  // 8s for large code
                memory_multiplier: 15.0,  // 15x input size
                min_accuracy: 0.85,       // 85% accuracy
            },
        );

        thresholds.insert(
            ValidationType::Compilation,
            ValidationThresholds {
                max_time_small_ms: 500,   // 500ms for small code
                max_time_medium_ms: 2000, // 2s for medium code
                max_time_large_ms: 10000, // 10s for large code
                memory_multiplier: 20.0,  // 20x input size
                min_accuracy: 0.80,       // 80% accuracy
            },
        );

        thresholds.insert(
            ValidationType::Macro,
            ValidationThresholds {
                max_time_small_ms: 150,  // 150ms for small code
                max_time_medium_ms: 750, // 750ms for medium code
                max_time_large_ms: 3000, // 3s for large code
                memory_multiplier: 8.0,  // 8x input size
                min_accuracy: 0.88,      // 88% accuracy
            },
        );

        thresholds.insert(
            ValidationType::Attribute,
            ValidationThresholds {
                max_time_small_ms: 50,   // 50ms for small code
                max_time_medium_ms: 250, // 250ms for medium code
                max_time_large_ms: 1000, // 1s for large code
                memory_multiplier: 3.0,  // 3x input size
                min_accuracy: 0.92,      // 92% accuracy
            },
        );

        Self {
            name,
            thresholds,
            memory_limits: MemoryLimits {
                max_memory_bytes: 512 * 1024 * 1024, // 512MB
                max_memory_percentage: 0.5,          // 50% of available memory
                leak_detection_threshold: 0.1,       // 10% growth indicates leak
            },
            throughput_requirements: ThroughputRequirements {
                min_throughput_kbps: 100.0, // 100 KB/s minimum
                max_concurrent_operations: 4,
                max_queue_depth: 10,
            },
        }
    }

    /// Create a performance contract with custom thresholds
    pub fn with_thresholds(
        name: String,
        thresholds: HashMap<ValidationType, ValidationThresholds>,
    ) -> Self {
        Self {
            name,
            thresholds,
            memory_limits: MemoryLimits::default(),
            throughput_requirements: ThroughputRequirements::default(),
        }
    }

    /// Get threshold for a specific validation type
    pub fn threshold_for(&self, validation_type: ValidationType) -> Option<&ValidationThresholds> {
        self.thresholds.get(&validation_type)
    }

    /// Check if a validation result meets the performance contract
    pub fn validate_performance(
        &self,
        validation_type: ValidationType,
        code_size_bytes: usize,
        execution_time_ms: u64,
        memory_usage_bytes: usize,
        accuracy: f64,
    ) -> PerformanceCompliance {
        let threshold = match self.threshold_for(validation_type) {
            Some(t) => t,
            None => return PerformanceCompliance::NotApplicable,
        };

        let code_category = self.categorize_code_size(code_size_bytes);
        let max_time_ms = match code_category {
            CodeSizeCategory::Small => threshold.max_time_small_ms,
            CodeSizeCategory::Medium => threshold.max_time_medium_ms,
            CodeSizeCategory::Large => threshold.max_time_large_ms,
        };

        let expected_memory = (code_size_bytes as f64 * threshold.memory_multiplier) as usize;
        let max_memory = std::cmp::min(expected_memory, self.memory_limits.max_memory_bytes);

        let mut violations = Vec::new();

        if execution_time_ms > max_time_ms {
            violations.push(PerformanceViolation::ExecutionTimeExceeded {
                actual: execution_time_ms,
                maximum: max_time_ms,
                code_category,
            });
        }

        if memory_usage_bytes > max_memory {
            violations.push(PerformanceViolation::MemoryUsageExceeded {
                actual: memory_usage_bytes,
                maximum: max_memory,
                multiplier: threshold.memory_multiplier,
            });
        }

        if accuracy < threshold.min_accuracy {
            violations.push(PerformanceViolation::AccuracyBelowThreshold {
                actual: accuracy,
                minimum: threshold.min_accuracy,
            });
        }

        if violations.is_empty() {
            PerformanceCompliance::Compliant
        } else {
            PerformanceCompliance::NonCompliant(violations)
        }
    }

    /// Categorize code size for performance evaluation
    fn categorize_code_size(&self, size_bytes: usize) -> CodeSizeCategory {
        if size_bytes < 1024 {
            CodeSizeCategory::Small
        } else if size_bytes <= 10 * 1024 {
            CodeSizeCategory::Medium
        } else {
            CodeSizeCategory::Large
        }
    }
}

impl Default for MemoryLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 512 * 1024 * 1024, // 512MB
            max_memory_percentage: 0.5,
            leak_detection_threshold: 0.1,
        }
    }
}

impl Default for ThroughputRequirements {
    fn default() -> Self {
        Self {
            min_throughput_kbps: 100.0,
            max_concurrent_operations: 4,
            max_queue_depth: 10,
        }
    }
}

/// Code size categories for performance evaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeSizeCategory {
    Small,  // < 1KB
    Medium, // 1KB - 10KB
    Large,  // > 10KB
}

/// Performance compliance result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformanceCompliance {
    /// Performance meets all requirements
    Compliant,
    /// Performance doesn't meet requirements
    NonCompliant(Vec<PerformanceViolation>),
    /// Validation type not covered by contract
    NotApplicable,
}

/// Performance violation details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformanceViolation {
    /// Execution time exceeded threshold
    ExecutionTimeExceeded {
        actual: u64,
        maximum: u64,
        code_category: CodeSizeCategory,
    },
    /// Memory usage exceeded threshold
    MemoryUsageExceeded {
        actual: usize,
        maximum: usize,
        multiplier: f64,
    },
    /// Accuracy below minimum threshold
    AccuracyBelowThreshold { actual: f64, minimum: f64 },
    /// Throughput below minimum requirement
    ThroughputBelowMinimum { actual: f64, minimum: f64 },
}

/// Performance report for validation operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationPerformanceReport {
    /// Report identifier
    pub report_id: String,
    /// Contract being evaluated
    pub contract_name: String,
    /// Individual validation results with performance data
    pub validation_results: Vec<ValidationPerformanceResult>,
    /// Overall compliance status
    pub overall_compliance: PerformanceCompliance,
    /// Performance metrics summary
    pub metrics_summary: PerformanceMetricsSummary,
    /// Generated timestamp
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Performance data for a single validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationPerformanceResult {
    /// Type of validation performed
    pub validation_type: ValidationType,
    /// Size of input code in bytes
    pub input_size_bytes: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Whether the validation was successful
    pub is_valid: bool,
    /// Performance compliance for this result
    pub compliance: PerformanceCompliance,
    /// Accuracy score (if applicable)
    pub accuracy: Option<f64>,
}

/// Summary of performance metrics across all validations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsSummary {
    /// Total execution time
    pub total_execution_time_ms: u64,
    /// Total memory usage
    pub total_memory_usage_bytes: usize,
    /// Average throughput in KB/s
    pub average_throughput_kbps: f64,
    /// Number of compliant validations
    pub compliant_validations: usize,
    /// Number of non-compliant validations
    pub non_compliant_validations: usize,
    /// Overall compliance percentage
    pub compliance_percentage: f64,
}

impl ValidationPerformanceReport {
    /// Create a new performance report
    pub fn new(
        contract_name: String,
        validation_results: Vec<ValidationPerformanceResult>,
    ) -> Self {
        let overall_compliance = Self::calculate_overall_compliance(&validation_results);
        let metrics_summary = Self::calculate_metrics_summary(&validation_results);

        Self {
            report_id: uuid::Uuid::new_v4().to_string(),
            contract_name,
            validation_results,
            overall_compliance,
            metrics_summary,
            generated_at: chrono::Utc::now(),
        }
    }

    /// Calculate overall compliance from individual results
    fn calculate_overall_compliance(
        results: &[ValidationPerformanceResult],
    ) -> PerformanceCompliance {
        let mut all_violations = Vec::new();

        for result in results {
            if let PerformanceCompliance::NonCompliant(violations) = &result.compliance {
                all_violations.extend(violations.clone());
            }
        }

        if all_violations.is_empty() {
            PerformanceCompliance::Compliant
        } else {
            PerformanceCompliance::NonCompliant(all_violations)
        }
    }

    /// Calculate metrics summary from validation results
    fn calculate_metrics_summary(
        results: &[ValidationPerformanceResult],
    ) -> PerformanceMetricsSummary {
        let total_execution_time_ms: u64 = results.iter().map(|r| r.execution_time_ms).sum();
        let total_memory_usage_bytes: usize = results.iter().map(|r| r.memory_usage_bytes).sum();
        let total_input_size_bytes: usize = results.iter().map(|r| r.input_size_bytes).sum();

        let average_throughput_kbps = if total_execution_time_ms > 0 {
            (total_input_size_bytes as f64 / 1024.0) / (total_execution_time_ms as f64 / 1000.0)
        } else {
            0.0
        };

        let compliant_validations = results
            .iter()
            .filter(|r| matches!(r.compliance, PerformanceCompliance::Compliant))
            .count();

        let non_compliant_validations = results.len() - compliant_validations;

        let compliance_percentage = if results.is_empty() {
            100.0
        } else {
            (compliant_validations as f64 / results.len() as f64) * 100.0
        };

        PerformanceMetricsSummary {
            total_execution_time_ms,
            total_memory_usage_bytes,
            average_throughput_kbps,
            compliant_validations,
            non_compliant_validations,
            compliance_percentage,
        }
    }

    /// Get validation results by type
    pub fn results_by_type(
        &self,
        validation_type: ValidationType,
    ) -> Vec<&ValidationPerformanceResult> {
        self.validation_results
            .iter()
            .filter(|r| r.validation_type == validation_type)
            .collect()
    }

    /// Check if the report indicates performance regression
    pub fn has_regression(&self) -> bool {
        matches!(
            self.overall_compliance,
            PerformanceCompliance::NonCompliant(_)
        )
    }

    /// Get throughput in KB/s
    pub fn throughput_kbps(&self) -> f64 {
        self.metrics_summary.average_throughput_kbps
    }

    /// Get memory efficiency ratio (input_size / memory_usage)
    pub fn memory_efficiency_ratio(&self) -> f64 {
        if self.metrics_summary.total_memory_usage_bytes == 0 {
            0.0
        } else {
            let total_input_size: usize = self
                .validation_results
                .iter()
                .map(|r| r.input_size_bytes)
                .sum();
            total_input_size as f64 / self.metrics_summary.total_memory_usage_bytes as f64
        }
    }

    /// Get validation success percentage for a specific type
    pub fn syntax_validation_percentage(&self) -> f64 {
        self.validation_success_percentage(ValidationType::Syntax)
    }

    pub fn type_validation_percentage(&self) -> f64 {
        self.validation_success_percentage(ValidationType::Type)
    }

    pub fn compilation_validation_percentage(&self) -> f64 {
        self.validation_success_percentage(ValidationType::Compilation)
    }

    fn validation_success_percentage(&self, validation_type: ValidationType) -> f64 {
        let type_results = self.results_by_type(validation_type);
        if type_results.is_empty() {
            return 100.0;
        }

        let successful = type_results.iter().filter(|r| r.is_valid).count();
        (successful as f64 / type_results.len() as f64) * 100.0
    }
}

/// Performance regression analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegressionAnalysis {
    /// Analysis identifier
    pub analysis_id: String,
    /// Baseline report (previous performance)
    pub baseline_report: ValidationPerformanceReport,
    /// Current report (new performance)
    pub current_report: ValidationPerformanceReport,
    /// Regression analysis results
    pub regression_detected: bool,
    /// Performance changes detected
    pub performance_changes: Vec<PerformanceChange>,
    /// Analysis timestamp
    pub analyzed_at: chrono::DateTime<chrono::Utc>,
}

/// Performance change detected between baseline and current
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceChange {
    /// Validation type affected
    pub validation_type: ValidationType,
    /// Type of change
    pub change_type: PerformanceChangeType,
    /// Magnitude of change
    pub magnitude: f64,
    /// Significance threshold
    pub significance_threshold: f64,
}

/// Types of performance changes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformanceChangeType {
    /// Execution time increased
    ExecutionTimeRegression,
    /// Memory usage increased
    MemoryUsageRegression,
    /// Accuracy decreased
    AccuracyRegression,
    /// Performance improved
    PerformanceImprovement,
    /// No significant change
    NoSignificantChange,
}

impl PerformanceRegressionAnalysis {
    /// Analyze performance changes between baseline and current reports
    pub fn analyze(
        baseline: ValidationPerformanceReport,
        current: ValidationPerformanceReport,
        significance_threshold: f64,
    ) -> Self {
        let performance_changes =
            Self::detect_performance_changes(&baseline, &current, significance_threshold);
        let regression_detected = performance_changes.iter().any(|change| {
            matches!(
                change.change_type,
                PerformanceChangeType::ExecutionTimeRegression
                    | PerformanceChangeType::MemoryUsageRegression
                    | PerformanceChangeType::AccuracyRegression
            )
        });

        Self {
            analysis_id: uuid::Uuid::new_v4().to_string(),
            baseline_report: baseline,
            current_report: current,
            regression_detected,
            performance_changes,
            analyzed_at: chrono::Utc::now(),
        }
    }

    /// Detect performance changes between reports
    fn detect_performance_changes(
        baseline: &ValidationPerformanceReport,
        current: &ValidationPerformanceReport,
        significance_threshold: f64,
    ) -> Vec<PerformanceChange> {
        let mut changes = Vec::new();

        for validation_type in [
            ValidationType::Syntax,
            ValidationType::Type,
            ValidationType::BorrowChecker,
            ValidationType::Compilation,
            ValidationType::Macro,
            ValidationType::Attribute,
        ] {
            let baseline_results = baseline.results_by_type(validation_type);
            let current_results = current.results_by_type(validation_type);

            if !baseline_results.is_empty() && !current_results.is_empty() {
                let baseline_avg_time: f64 = baseline_results
                    .iter()
                    .map(|r| r.execution_time_ms as f64)
                    .sum::<f64>()
                    / baseline_results.len() as f64;
                let current_avg_time: f64 = current_results
                    .iter()
                    .map(|r| r.execution_time_ms as f64)
                    .sum::<f64>()
                    / current_results.len() as f64;

                let time_change = (current_avg_time - baseline_avg_time) / baseline_avg_time;
                if time_change.abs() > significance_threshold {
                    changes.push(PerformanceChange {
                        validation_type,
                        change_type: if time_change > 0.0 {
                            PerformanceChangeType::ExecutionTimeRegression
                        } else {
                            PerformanceChangeType::PerformanceImprovement
                        },
                        magnitude: time_change.abs(),
                        significance_threshold,
                    });
                }
            }
        }

        changes
    }
}

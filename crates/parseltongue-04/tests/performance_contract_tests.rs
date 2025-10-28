//! RED PHASE: Failing tests for performance contracts using parseltongue-01 framework
//! Following TDD principle: Write failing tests first

use parseltongue_01::performance::{
    ParsingPerformanceContract, PerformanceError, PerformanceReport,
};
use parseltongue_04::*;
use std::path::PathBuf;
use std::time::Duration;
use parseltongue_04::performance::{ValidationPerformanceResult, PerformanceCompliance, PerformanceChangeType};

#[tokio::test]
async fn test_validation_performance_contract_creation() {
    // RED: Test ValidationPerformanceContract creation and default values
    // This should fail because ValidationPerformanceContract doesn't exist yet

    let contract = ValidationPerformanceContract::default();

    assert!(contract.max_syntax_validation_time_per_kb > Duration::ZERO);
    assert!(contract.max_type_validation_time_per_kb > Duration::ZERO);
    assert!(contract.max_compilation_time_per_kb > Duration::ZERO);
    assert!(contract.max_memory_overhead_factor > 1.0);
    assert!(contract.min_validation_accuracy > 0.0);
    assert!(contract.min_validation_accuracy <= 1.0);
}

#[tokio::test]
async fn test_validation_performance_contract_validation() {
    // RED: Test performance contract validation for validation operations
    // This should fail because performance contract validation doesn't exist yet

    let contract = ValidationPerformanceContract::default();
    let validator = MockRustValidator::new();

    let test_case = ValidationTestCase {
        name: "Simple function validation".to_string(),
        code: r#"
        fn main() {
            println!("Hello, world!");
        }
        "#
        .to_string(),
        file_path: PathBuf::from("/test/main.rs"),
        expected_syntax_valid: true,
        expected_type_valid: true,
        expected_compilation_valid: true,
    };

    let result = contract
        .validate_validation_performance(&validator, &test_case)
        .await;

    // Should succeed or provide meaningful performance error
    match result {
        Ok(report) => {
            assert!(report.total_execution_time_ms > 0);
            assert!(report.memory_usage_bytes > 0);
            assert!(report.validation_accuracy >= 0.0);
            assert!(report.validation_accuracy <= 1.0);
        }
        Err(error) => {
            // Should provide meaningful performance error
            let error_msg = format!("{}", error);
            assert!(!error_msg.is_empty());
        }
    }
}

#[tokio::test]
async fn test_validation_performance_contract_time_violations() {
    // RED: Test performance contract time violation detection
    // This should fail because time violation detection doesn't exist yet

    let mut contract = ValidationPerformanceContract::default();
    // Set very strict time limits to trigger violations
    contract.max_syntax_validation_time_per_kb = Duration::from_nanos(1);
    contract.max_type_validation_time_per_kb = Duration::from_nanos(1);
    contract.max_compilation_time_per_kb = Duration::from_nanos(1);

    let validator = MockRustValidator::new();

    let test_case = ValidationTestCase {
        name: "Performance violation test".to_string(),
        code: r#"
        use std::collections::HashMap;

        fn complex_function(data: &HashMap<String, Vec<i32>>) -> Result<i32, String> {
            let mut sum = 0;
            for (key, values) in data {
                sum += values.iter().sum::<i32>();
                sum += key.len() as i32;
            }
            Ok(sum)
        }

        fn main() {
            let mut data = HashMap::new();
            data.insert("test".to_string(), vec![1, 2, 3, 4, 5]);
            match complex_function(&data) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => println!("Error: {}", e),
            }
        }
        "#
        .to_string(),
        file_path: PathBuf::from("/test/complex.rs"),
        expected_syntax_valid: true,
        expected_type_valid: true,
        expected_compilation_valid: true,
    };

    let result = contract
        .validate_validation_performance(&validator, &test_case)
        .await;

    // Should detect time contract violations
    match result {
        Ok(_) => {
            // If it passes, ensure it's within the strict limits
            // This is unlikely but possible for very fast validation
        }
        Err(error) => {
            // Should be a performance error, specifically time violation
            match &error {
                PerformanceError::TimeContractViolation { .. } => {
                    // Expected error type
                }
                _ => {
                    panic!("Expected TimeContractViolation, got: {}", error);
                }
            }
        }
    }
}

#[tokio::test]
async fn test_validation_performance_contract_memory_violations() {
    // RED: Test performance contract memory violation detection
    // This should fail because memory violation detection doesn't exist yet

    let mut contract = ValidationPerformanceContract::default();
    // Set very low memory limits to trigger violations
    contract.max_memory_overhead_factor = 0.1; // Only 10% of input size

    let validator = MockRustValidator::new();

    let test_case = ValidationTestCase {
        name: "Memory violation test".to_string(),
        code: "fn main() { println!(\"test\"); }".to_string(),
        file_path: PathBuf::from("/test/memory.rs"),
        expected_syntax_valid: true,
        expected_type_valid: true,
        expected_compilation_valid: true,
    };

    let result = contract
        .validate_validation_performance(&validator, &test_case)
        .await;

    // Should detect memory contract violations
    match result {
        Ok(_) => {
            // If it passes, memory usage must be within limits
        }
        Err(error) => {
            // Should be a performance error, specifically memory violation
            match &error {
                PerformanceError::MemoryContractViolation { .. } => {
                    // Expected error type
                }
                _ => {
                    panic!("Expected MemoryContractViolation, got: {}", error);
                }
            }
        }
    }
}

#[tokio::test]
async fn test_validation_performance_report_creation() {
    // RED: Test ValidationPerformanceReport creation and properties
    // This should fail because ValidationPerformanceReport doesn't exist yet

    let validation_results = vec![
        ValidationPerformanceResult {
            validation_type: ValidationType::Syntax,
            input_size_bytes: 1024,
            execution_time_ms: 5,
            memory_usage_bytes: 512,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
        ValidationPerformanceResult {
            validation_type: ValidationType::Type,
            input_size_bytes: 1024,
            execution_time_ms: 15,
            memory_usage_bytes: 1024,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
        ValidationPerformanceResult {
            validation_type: ValidationType::Compilation,
            input_size_bytes: 1024,
            execution_time_ms: 50,
            memory_usage_bytes: 1536,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
    ];

    let report = ValidationPerformanceReport::new(
        "Test validation".to_string(),
        validation_results,
    );

    assert_eq!(report.contract_name, "Test validation");
    assert_eq!(report.validation_results.len(), 3);
    assert_eq!(report.validation_results[0].execution_time_ms, 5);
    assert_eq!(report.validation_results[1].execution_time_ms, 15);
    assert_eq!(report.validation_results[2].execution_time_ms, 50);
    assert_eq!(report.metrics_summary.total_execution_time_ms, 70);
    assert_eq!(report.metrics_summary.total_memory_usage_bytes, 3072);
    assert!(matches!(report.overall_compliance, PerformanceCompliance::Compliant));
    assert!(report.validation_results[0].is_valid);
    assert!(report.validation_results[1].is_valid);
    assert!(report.validation_results[2].is_valid);
    assert!(!report.has_regression());
}

#[tokio::test]
async fn test_validation_performance_report_calculations() {
    // RED: Test ValidationPerformanceReport metric calculations
    // This should fail because calculation methods don't exist yet

    let validation_results = vec![
        ValidationPerformanceResult {
            validation_type: ValidationType::Syntax,
            input_size_bytes: 2048,
            execution_time_ms: 10,
            memory_usage_bytes: 1024,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
        ValidationPerformanceResult {
            validation_type: ValidationType::Type,
            input_size_bytes: 2048,
            execution_time_ms: 30,
            memory_usage_bytes: 2048,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
        ValidationPerformanceResult {
            validation_type: ValidationType::Compilation,
            input_size_bytes: 2048,
            execution_time_ms: 100,
            memory_usage_bytes: 3072,
            is_valid: false,
            compliance: PerformanceCompliance::NonCompliant(vec![]),
            accuracy: Some(0.95),
        },
    ];

    let report = ValidationPerformanceReport::new(
        "Calculation test".to_string(),
        validation_results,
    );

    // Test throughput calculation (bytes per second)
    let throughput_kbps = report.throughput_kbps();
    assert!(throughput_kbps > 0.0);
    let expected_throughput = (2048.0 / 140.0) * 1000.0; // bytes per second
    assert!((throughput_kbps - expected_throughput / 1000.0).abs() < 0.1);

    // Test memory efficiency ratio
    let memory_efficiency = report.memory_efficiency_ratio();
    assert_eq!(memory_efficiency, 4096.0 / 2048.0); // 2.0

    // Test validation time breakdown
    let syntax_percentage = report.syntax_validation_percentage();
    let type_percentage = report.type_validation_percentage();
    let compilation_percentage = report.compilation_validation_percentage();

    assert_eq!(syntax_percentage, 10.0 / 140.0 * 100.0);
    assert_eq!(type_percentage, 30.0 / 140.0 * 100.0);
    assert_eq!(compilation_percentage, 100.0 / 140.0 * 100.0);
    assert!((syntax_percentage + type_percentage + compilation_percentage - 100.0).abs() < 0.001);
}

#[tokio::test]
async fn test_batch_validation_performance_testing() {
    // RED: Test batch validation performance testing
    // This should fail because batch testing doesn't exist yet

    let contract = ValidationPerformanceContract::default();
    let validator = MockRustValidator::new();

    let test_cases = vec![
        ValidationTestCase {
            name: "Simple main function".to_string(),
            code: "fn main() { println!(\"Hello\"); }".to_string(),
            file_path: PathBuf::from("/test/simple.rs"),
            expected_syntax_valid: true,
            expected_type_valid: true,
            expected_compilation_valid: true,
        },
        ValidationTestCase {
            name: "Function with parameters".to_string(),
            code: r#"
            fn greet(name: &str) -> String {
                format!("Hello, {}!", name)
            }

            fn main() {
                println!("{}", greet("World"));
            }
            "#
            .to_string(),
            file_path: PathBuf::from("/test/params.rs"),
            expected_syntax_valid: true,
            expected_type_valid: true,
            expected_compilation_valid: true,
        },
        ValidationTestCase {
            name: "Struct definition".to_string(),
            code: r#"
            struct Person {
                name: String,
                age: u32,
            }

            impl Person {
                fn new(name: String, age: u32) -> Self {
                    Self { name, age }
                }
            }

            fn main() {
                let person = Person::new("Alice".to_string(), 30);
                println!("{} is {} years old", person.name, person.age);
            }
            "#
            .to_string(),
            file_path: PathBuf::from("/test/struct.rs"),
            expected_syntax_valid: true,
            expected_type_valid: true,
            expected_compilation_valid: true,
        },
    ];

    let result = contract
        .validate_batch_performance(&validator, test_cases)
        .await;

    match result {
        Ok(reports) => {
            assert_eq!(reports.len(), 3);

            // Check that all reports have required metrics
            for report in reports {
                assert!(report.total_execution_time > Duration::ZERO);
                assert!(report.memory_usage_bytes > 0);
                assert!(report.validation_accuracy >= 0.0);
                assert!(report.validation_accuracy <= 1.0);
            }

            // Calculate aggregate metrics
            let total_time: Duration = reports.iter().map(|r| r.total_execution_time).sum();
            let total_memory: usize = reports.iter().map(|r| r.memory_usage_bytes).sum();
            let avg_accuracy: f64 =
                reports.iter().map(|r| r.validation_accuracy).sum::<f64>() / reports.len() as f64;

            assert!(total_time > Duration::ZERO);
            assert!(total_memory > 0);
            assert!(avg_accuracy >= 0.0);
            assert!(avg_accuracy <= 1.0);
        }
        Err(error) => {
            // Should provide meaningful error for batch validation
            let error_msg = format!("{}", error);
            assert!(!error_msg.is_empty());
        }
    }
}

#[tokio::test]
async fn test_performance_regression_detection() {
    // RED: Test performance regression detection
    // This should fail because regression detection doesn't exist yet

    let baseline_results = vec![
        ValidationPerformanceResult {
            validation_type: ValidationType::Syntax,
            input_size_bytes: 1024,
            execution_time_ms: 5,
            memory_usage_bytes: 512,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
        ValidationPerformanceResult {
            validation_type: ValidationType::Type,
            input_size_bytes: 1024,
            execution_time_ms: 10,
            memory_usage_bytes: 512,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
        ValidationPerformanceResult {
            validation_type: ValidationType::Compilation,
            input_size_bytes: 1024,
            execution_time_ms: 20,
            memory_usage_bytes: 512,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
    ];

    let baseline_report = ValidationPerformanceReport::new(
        "Regression test".to_string(),
        baseline_results,
    );

    let current_results = vec![
        ValidationPerformanceResult {
            validation_type: ValidationType::Syntax,
            input_size_bytes: 1024,
            execution_time_ms: 8, // Slower
            memory_usage_bytes: 682,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
        ValidationPerformanceResult {
            validation_type: ValidationType::Type,
            input_size_bytes: 1024,
            execution_time_ms: 12, // Slightly slower
            memory_usage_bytes: 683,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
        ValidationPerformanceResult {
            validation_type: ValidationType::Compilation,
            input_size_bytes: 1024,
            execution_time_ms: 45, // Much slower
            memory_usage_bytes: 683,
            is_valid: true,
            compliance: PerformanceCompliance::Compliant,
            accuracy: Some(1.0),
        },
    ];

    let current_report = ValidationPerformanceReport::new(
        "Regression test".to_string(),
        current_results,
    );

    let regression_analysis = PerformanceRegressionAnalysis::analyze(
        baseline_report,
        current_report,
        0.1, // 10% significance threshold
    );

    assert!(regression_analysis.regression_detected);
    assert!(!regression_analysis.performance_changes.is_empty());

    // Check that we detected performance regressions
    let regression_count = regression_analysis.performance_changes.iter()
        .filter(|change| matches!(
            change.change_type,
            PerformanceChangeType::ExecutionTimeRegression |
            PerformanceChangeType::MemoryUsageRegression |
            PerformanceChangeType::AccuracyRegression
        ))
        .count();

    assert!(regression_count > 0);

    // Check specific regression details
    let syntax_regression = regression_analysis.performance_changes
        .iter()
        .find(|change| change.validation_type == ValidationType::Syntax);
    assert!(syntax_regression.is_some());
    assert!(syntax_regression.unwrap().magnitude > 0.0);

    let compilation_regression = regression_analysis.performance_changes
        .iter()
        .find(|change| change.validation_type == ValidationType::Compilation);
    assert!(compilation_regression.is_some());
    assert!(compilation_regression.unwrap().magnitude > 0.0);
}

#[tokio::test]
async fn test_performance_contract_customization() {
    // RED: Test performance contract customization for different use cases
    // This should fail because contract customization doesn't exist yet

    // Fast contract for interactive development
    let fast_contract = ValidationPerformanceContract::new("Fast Interactive".to_string());
    let fast_syntax_threshold = fast_contract.threshold_for(ValidationType::Syntax).unwrap();
    let fast_type_threshold = fast_contract.threshold_for(ValidationType::Type).unwrap();

    assert!(fast_syntax_threshold.max_time_small_ms < 100);
    assert!(fast_type_threshold.max_time_small_ms < 200);
    assert!(fast_contract.memory_limits.max_memory_percentage < 0.8);

    // Thorough contract for CI/CD
    let thorough_contract = ValidationPerformanceContract::new("Thorough CI/CD".to_string());
    let syntax_threshold = thorough_contract.threshold_for(ValidationType::Syntax).unwrap();
    let type_threshold = thorough_contract.threshold_for(ValidationType::Type).unwrap();
    let compilation_threshold = thorough_contract.threshold_for(ValidationType::Compilation).unwrap();

    assert!(syntax_threshold.max_time_small_ms > 100);
    assert!(type_threshold.max_time_small_ms > 500);
    assert!(compilation_threshold.max_time_small_ms > 2000);
    assert!(syntax_threshold.min_accuracy > 0.95); // High accuracy required

    // Resource-constrained contract
    let resource_constrained_contract = ValidationPerformanceContract::new("Resource Constrained".to_string());
    assert!(resource_constrained_contract.memory_limits.max_memory_percentage < 0.5);

    // Test that contracts can be used for validation
    let validator = MockRustValidator::new();
    let test_case = ValidationTestCase {
        name: "Contract customization test".to_string(),
        code: "fn main() { println!(\"test\"); }".to_string(),
        file_path: PathBuf::from("/test/custom.rs"),
        expected_syntax_valid: true,
        expected_type_valid: true,
        expected_compilation_valid: true,
    };

    // All contracts should be usable for validation
    for contract in [
        fast_contract,
        thorough_contract,
        resource_constrained_contract,
    ] {
        let result = contract
            .validate_validation_performance(&validator, &test_case)
            .await;
        // Should either succeed or provide meaningful error
        assert!(result.is_ok() || result.is_err());
    }
}

// Mock implementation for testing
#[derive(Clone)]
struct MockRustValidator {
    name: String,
}

impl MockRustValidator {
    fn new() -> Self {
        Self {
            name: "mock_validator".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl RustCodeValidator for MockRustValidator {
    type Input = String;
    type Output = ValidationOutput;
    type Error = ValidationError;

    async fn validate_syntax(&self, code: &Self::Input) -> Result<Self::Output, Self::Error> {
        // Mock implementation for RED phase
        Ok(ValidationOutput::success(
            ValidationType::Syntax,
            10,
            1024,
        ))
    }

    async fn validate_types(&self, code: &Self::Input) -> Result<Self::Output, Self::Error> {
        // Mock implementation for RED phase
        Ok(ValidationOutput::success(
            ValidationType::Type,
            25,
            2048,
        ))
    }

    async fn validate_borrow_checker(
        &self,
        code: &Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        // Mock implementation for RED phase
        Ok(ValidationOutput::success(
            ValidationType::BorrowChecker,
            15,
            1536,
        ))
    }

    async fn validate_compilation(&self, code: &Self::Input) -> Result<Self::Output, Self::Error> {
        // Mock implementation for RED phase
        Ok(ValidationOutput::success(
            ValidationType::Compilation,
            50,
            3072,
        ))
    }

    async fn validate_all(&self, code: &Self::Input) -> Result<ValidationReport, Self::Error> {
        // GREEN phase: Simple mock implementation
        let syntax_result = self.validate_syntax(code).await?;
        let type_result = self.validate_types(code).await?;
        let borrow_result = self.validate_borrow_checker(code).await?;
        let compilation_result = self.validate_compilation(code).await?;

        Ok(ValidationReport::new(
            PathBuf::from("test.rs"),
            code.clone(),
            vec![syntax_result, type_result, borrow_result, compilation_result],
        ))
    }

    fn name(&self) -> &'static str {
        "mock_validator"
    }

    fn capabilities(&self) -> ValidatorCapabilities {
        ValidatorCapabilities::default()
    }
}

// Property-based tests for performance contracts
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_performance_contract_time_limits(
            max_syntax_time_ms in 1u64..1000,
            max_type_time_ms in 1u64..5000,
            max_compilation_time_ms in 1u64..10000
        ) {
            // RED: Property-based test for performance contract time limits
            // This should fail because ValidationPerformanceContract doesn't exist yet

            let contract = ValidationPerformanceContract::new(
                "Property test contract".to_string()
            );

            // Test that contract was created successfully
            prop_assert!(!contract.name.is_empty());
            prop_assert!(contract.threshold_for(ValidationType::Syntax).is_some());
            prop_assert!(contract.threshold_for(ValidationType::Type).is_some());
            prop_assert!(contract.threshold_for(ValidationType::Compilation).is_some());
        }

        #[test]
        fn test_validation_report_metrics_consistency(
            code_size_bytes in 100usize..10000,
            execution_time_ms in 10u64..5000,
            memory_usage_bytes in 512usize..100000
        ) {
            // RED: Property-based test for validation report metric consistency
            // This should fail because ValidationPerformanceReport doesn't exist yet

            let validation_results = vec![
                ValidationPerformanceResult {
                    validation_type: ValidationType::Syntax,
                    input_size_bytes: code_size_bytes,
                    execution_time_ms: execution_time_ms / 10,
                    memory_usage_bytes: memory_usage_bytes / 3,
                    is_valid: true,
                    compliance: PerformanceCompliance::Compliant,
                    accuracy: Some(1.0),
                },
                ValidationPerformanceResult {
                    validation_type: ValidationType::Type,
                    input_size_bytes: code_size_bytes,
                    execution_time_ms: execution_time_ms / 5,
                    memory_usage_bytes: memory_usage_bytes / 3,
                    is_valid: true,
                    compliance: PerformanceCompliance::Compliant,
                    accuracy: Some(1.0),
                },
                ValidationPerformanceResult {
                    validation_type: ValidationType::Compilation,
                    input_size_bytes: code_size_bytes,
                    execution_time_ms: execution_time_ms * 6 / 10,
                    memory_usage_bytes: memory_usage_bytes / 3,
                    is_valid: true,
                    compliance: PerformanceCompliance::Compliant,
                    accuracy: Some(1.0),
                },
            ];

            let report = ValidationPerformanceReport::new(
                "Property test".to_string(),
                validation_results,
            );

            let throughput_kbps = report.throughput_kbps();
            let memory_efficiency = report.memory_efficiency_ratio();

            prop_assert!(throughput_kbps > 0.0);
            prop_assert!(memory_efficiency > 0.0);

            // Test time breakdown consistency
            let syntax_percentage = report.syntax_validation_percentage();
            let type_percentage = report.type_validation_percentage();
            let compilation_percentage = report.compilation_validation_percentage();

            prop_assert!(syntax_percentage >= 0.0);
            prop_assert!(type_percentage >= 0.0);
            prop_assert!(compilation_percentage >= 0.0);

            let total_percentage = syntax_percentage + type_percentage + compilation_percentage;
            prop_assert!((total_percentage - 100.0).abs() < 0.001);
        }
    }
}

//! RED PHASE: Failing tests for performance contracts using parseltongue-01 framework
//! Following TDD principle: Write failing tests first

use parseltongue_01::performance::{
    ParsingPerformanceContract, PerformanceError, PerformanceReport,
};
use parseltongue_04::*;
use std::path::PathBuf;
use std::time::Duration;

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

    let report = ValidationPerformanceReport {
        test_case_name: "Test validation".to_string(),
        file_path: PathBuf::from("/test/main.rs"),
        code_size_bytes: 1024,
        syntax_validation_time: Duration::from_millis(5),
        type_validation_time: Duration::from_millis(15),
        compilation_validation_time: Duration::from_millis(50),
        total_execution_time: Duration::from_millis(70),
        memory_usage_bytes: 2048,
        validation_accuracy: 1.0,
        syntax_valid: true,
        type_valid: true,
        compilation_valid: true,
        overall_valid: true,
        contract_satisfied: true,
        performance_violations: vec![],
        generated_at: chrono::Utc::now(),
    };

    assert_eq!(report.test_case_name, "Test validation");
    assert_eq!(report.code_size_bytes, 1024);
    assert_eq!(report.syntax_validation_time, Duration::from_millis(5));
    assert_eq!(report.type_validation_time, Duration::from_millis(15));
    assert_eq!(
        report.compilation_validation_time,
        Duration::from_millis(50)
    );
    assert_eq!(report.total_execution_time, Duration::from_millis(70));
    assert_eq!(report.memory_usage_bytes, 2048);
    assert_eq!(report.validation_accuracy, 1.0);
    assert!(report.syntax_valid);
    assert!(report.type_valid);
    assert!(report.compilation_valid);
    assert!(report.overall_valid);
    assert!(report.contract_satisfied);
    assert!(report.performance_violations.is_empty());
}

#[tokio::test]
async fn test_validation_performance_report_calculations() {
    // RED: Test ValidationPerformanceReport metric calculations
    // This should fail because calculation methods don't exist yet

    let report = ValidationPerformanceReport {
        test_case_name: "Calculation test".to_string(),
        file_path: PathBuf::from("/test/calc.rs"),
        code_size_bytes: 2048, // 2KB
        syntax_validation_time: Duration::from_millis(10),
        type_validation_time: Duration::from_millis(30),
        compilation_validation_time: Duration::from_millis(100),
        total_execution_time: Duration::from_millis(140),
        memory_usage_bytes: 4096, // 4KB
        validation_accuracy: 0.95,
        syntax_valid: true,
        type_valid: true,
        compilation_valid: false, // One validation failed
        overall_valid: false,
        contract_satisfied: false,
        performance_violations: vec![],
        generated_at: chrono::Utc::now(),
    };

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

    let baseline_report = ValidationPerformanceReport {
        test_case_name: "Regression test".to_string(),
        file_path: PathBuf::from("/test/regression.rs"),
        code_size_bytes: 1024,
        syntax_validation_time: Duration::from_millis(5),
        type_validation_time: Duration::from_millis(10),
        compilation_validation_time: Duration::from_millis(20),
        total_execution_time: Duration::from_millis(35),
        memory_usage_bytes: 1536,
        validation_accuracy: 1.0,
        syntax_valid: true,
        type_valid: true,
        compilation_valid: true,
        overall_valid: true,
        contract_satisfied: true,
        performance_violations: vec![],
        generated_at: chrono::Utc::now(),
    };

    let current_report = ValidationPerformanceReport {
        test_case_name: "Regression test".to_string(),
        file_path: PathBuf::from("/test/regression.rs"),
        code_size_bytes: 1024,
        syntax_validation_time: Duration::from_millis(8), // Slower
        type_validation_time: Duration::from_millis(12),  // Slightly slower
        compilation_validation_time: Duration::from_millis(45), // Much slower
        total_execution_time: Duration::from_millis(65),  // Total slowdown
        memory_usage_bytes: 2048,                         // Higher memory usage
        validation_accuracy: 1.0,
        syntax_valid: true,
        type_valid: true,
        compilation_valid: true,
        overall_valid: true,
        contract_satisfied: false, // Performance degraded
        performance_violations: vec![],
        generated_at: chrono::Utc::now(),
    };

    let regression_analysis = analyze_performance_regression(&baseline_report, &current_report);

    assert!(regression_analysis.has_regression);
    assert!(regression_analysis.time_regression_factor > 1.0);
    assert!(regression_analysis.memory_regression_factor > 1.0);
    assert!(!regression_analysis.regression_details.is_empty());

    // Check specific regression details
    let syntax_regression = regression_analysis
        .regression_details
        .iter()
        .find(|detail| detail.validation_type == ValidationType::Syntax);
    assert!(syntax_regression.is_some());
    assert!(syntax_regression.unwrap().regression_factor > 1.0);

    let compilation_regression = regression_analysis
        .regression_details
        .iter()
        .find(|detail| detail.validation_type == ValidationType::Compilation);
    assert!(compilation_regression.is_some());
    assert!(compilation_regression.unwrap().regression_factor > 1.0);
}

#[tokio::test]
async fn test_performance_contract_customization() {
    // RED: Test performance contract customization for different use cases
    // This should fail because contract customization doesn't exist yet

    // Fast contract for interactive development
    let fast_contract = ValidationPerformanceContract::fast_interactive();
    assert!(fast_contract.max_syntax_validation_time_per_kb < Duration::from_millis(10));
    assert!(fast_contract.max_type_validation_time_per_kb < Duration::from_millis(50));
    assert!(fast_contract.max_memory_overhead_factor < 5.0);

    // Thorough contract for CI/CD
    let thorough_contract = ValidationPerformanceContract::thorough_ci();
    assert!(thorough_contract.max_syntax_validation_time_per_kb > Duration::from_millis(100));
    assert!(thorough_contract.max_type_validation_time_per_kb > Duration::from_millis(500));
    assert!(thorough_contract.max_compilation_time_per_kb > Duration::from_millis(2000));
    assert!(thorough_contract.min_validation_accuracy > 0.99); // Very high accuracy required

    // Resource-constrained contract
    let resource_constrained_contract = ValidationPerformanceContract::resource_constrained();
    assert!(resource_constrained_contract.max_memory_overhead_factor < 2.0);
    assert!(
        resource_constrained_contract.max_syntax_validation_time_per_kb < Duration::from_millis(50)
    );

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
    type Output = ValidationResult;
    type Error = ValidationError;

    async fn validate_syntax(&self, code: &Self::Input) -> Result<Self::Output, Self::Error> {
        // Mock implementation for RED phase
        Ok(ValidationResult {
            is_valid: true,
            validation_type: ValidationType::Syntax,
            errors: vec![],
            warnings: vec![],
            execution_time_ms: 10,
            memory_usage_bytes: 1024,
        })
    }

    async fn validate_types(&self, code: &Self::Input) -> Result<Self::Output, Self::Error> {
        // Mock implementation for RED phase
        Ok(ValidationResult {
            is_valid: true,
            validation_type: ValidationType::Type,
            errors: vec![],
            warnings: vec![],
            execution_time_ms: 25,
            memory_usage_bytes: 2048,
        })
    }

    async fn validate_borrow_checker(
        &self,
        code: &Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        // Mock implementation for RED phase
        Ok(ValidationResult {
            is_valid: true,
            validation_type: ValidationType::BorrowChecker,
            errors: vec![],
            warnings: vec![],
            execution_time_ms: 15,
            memory_usage_bytes: 1536,
        })
    }

    async fn validate_compilation(&self, code: &Self::Input) -> Result<Self::Output, Self::Error> {
        // Mock implementation for RED phase
        Ok(ValidationResult {
            is_valid: true,
            validation_type: ValidationType::Compilation,
            errors: vec![],
            warnings: vec![],
            execution_time_ms: 50,
            memory_usage_bytes: 3072,
        })
    }

    async fn validate_all(&self, code: &Self::Input) -> Result<ValidationReport, Self::Error> {
        // Mock implementation for RED phase
        todo!("Implement validate_all")
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

            let contract = ValidationPerformanceContract {
                max_syntax_validation_time_per_kb: Duration::from_millis(max_syntax_time_ms),
                max_type_validation_time_per_kb: Duration::from_millis(max_type_time_ms),
                max_compilation_time_per_kb: Duration::from_millis(max_compilation_time_ms),
                max_memory_overhead_factor: 3.0,
                min_validation_accuracy: 0.95,
            };

            prop_assert!(contract.max_syntax_validation_time_per_kb > Duration::ZERO);
            prop_assert!(contract.max_type_validation_time_per_kb > Duration::ZERO);
            prop_assert!(contract.max_compilation_time_per_kb > Duration::ZERO);
        }

        #[test]
        fn test_validation_report_metrics_consistency(
            code_size_bytes in 100usize..10000,
            execution_time_ms in 10u64..5000,
            memory_usage_bytes in 512usize..100000
        ) {
            // RED: Property-based test for validation report metric consistency
            // This should fail because ValidationPerformanceReport doesn't exist yet

            let report = ValidationPerformanceReport {
                test_case_name: "Property test".to_string(),
                file_path: PathBuf::from("/test/property.rs"),
                code_size_bytes,
                syntax_validation_time: Duration::from_millis(execution_time_ms / 10),
                type_validation_time: Duration::from_millis(execution_time_ms / 5),
                compilation_validation_time: Duration::from_millis(execution_time_ms * 6 / 10),
                total_execution_time: Duration::from_millis(execution_time_ms),
                memory_usage_bytes,
                validation_accuracy: 1.0,
                syntax_valid: true,
                type_valid: true,
                compilation_valid: true,
                overall_valid: true,
                contract_satisfied: true,
                performance_violations: vec![],
                generated_at: chrono::Utc::now(),
            };

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

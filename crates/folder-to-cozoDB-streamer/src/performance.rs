//! Performance contracts with automated validation
//! Following Steering Principle #5: Performance Claims Must Be Test-Validated

use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Performance contract for parsing operations
#[derive(Debug, Clone)]
pub struct ParsingPerformanceContract {
    pub max_parse_time_per_mb: Duration,
    pub max_memory_overhead_factor: f64,
    pub min_confidence_threshold: f64,
}

impl Default for ParsingPerformanceContract {
    fn default() -> Self {
        Self {
            max_parse_time_per_mb: Duration::from_millis(100), // 100ms per MB
            max_memory_overhead_factor: 3.0,                   // 3x input size max
            min_confidence_threshold: 0.8,                     // 80% confidence minimum
        }
    }
}

impl ParsingPerformanceContract {
    /// Validate parsing performance against contract
    pub async fn validate_parsing_performance<P>(
        &self,
        parser: &P,
        input: &P::Input,
        input_size_bytes: usize,
    ) -> Result<PerformanceReport, PerformanceError>
    where
        P: crate::traits::UniversalParser,
    {
        // Measure parsing time
        let start_time = Instant::now();
        let parse_future = parser.parse(input);

        let result = timeout(Duration::from_secs(30), parse_future)
            .await
            .map_err(|_| PerformanceError::Timeout {
                duration: Duration::from_secs(30),
            })?;

        let parse_duration = start_time.elapsed();
        let parsed_output = result
            .map_err(|e| PerformanceError::ParseFailure(format!("Parsing failed: {:?}", e)))?;

        // Validate time contract
        let expected_max_time = if input_size_bytes > 0 {
            let mb_size = input_size_bytes as u64 / 1_048_576; // Convert to MB
            let mb_multiplier = std::cmp::max(mb_size, 1);
            self.max_parse_time_per_mb * mb_multiplier as u32
        } else {
            self.max_parse_time_per_mb
        };

        if parse_duration > expected_max_time {
            return Err(PerformanceError::TimeContractViolation {
                actual: parse_duration,
                expected: expected_max_time,
                input_size: input_size_bytes,
            });
        }

        // Validate memory contract (approximate)
        let estimated_memory = parser.estimate_memory_usage(input_size_bytes);
        let max_allowed_memory =
            (input_size_bytes as f64 * self.max_memory_overhead_factor) as usize;
        if estimated_memory > max_allowed_memory {
            return Err(PerformanceError::MemoryContractViolation {
                estimated: estimated_memory,
                max_allowed: max_allowed_memory,
                input_size: input_size_bytes,
            });
        }

        Ok(PerformanceReport {
            parse_duration,
            estimated_memory,
            input_size_bytes,
            contract_satisfied: true,
            output_complexity: self.estimate_output_complexity(&parsed_output),
        })
    }

    fn estimate_output_complexity<T>(&self, _output: &T) -> ComplexityMetrics {
        // This would be implemented based on actual output type
        ComplexityMetrics {
            node_count: 0,
            edge_count: 0,
            depth: 0,
        }
    }
}

/// Performance report with detailed metrics
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub parse_duration: Duration,
    pub estimated_memory: usize,
    pub input_size_bytes: usize,
    pub contract_satisfied: bool,
    pub output_complexity: ComplexityMetrics,
}

impl PerformanceReport {
    /// Calculate parsing throughput in MB/s
    pub fn throughput_mbps(&self) -> f64 {
        if self.parse_duration.as_secs_f64() > 0.0 {
            (self.input_size_bytes as f64) / (1_048_576.0 * self.parse_duration.as_secs_f64())
        } else {
            0.0
        }
    }

    /// Calculate memory efficiency ratio
    pub fn memory_efficiency_ratio(&self) -> f64 {
        if self.input_size_bytes > 0 {
            self.estimated_memory as f64 / self.input_size_bytes as f64
        } else {
            0.0
        }
    }
}

/// Complexity metrics for parsed output
#[derive(Debug, Clone, Default)]
pub struct ComplexityMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub depth: usize,
}

/// Performance error types
#[derive(Debug, thiserror::Error)]
pub enum PerformanceError {
    #[error("Time contract violated: actual {actual:?} > expected {expected:?} for input size {input_size}")]
    TimeContractViolation {
        actual: Duration,
        expected: Duration,
        input_size: usize,
    },

    #[error("Memory contract violated: estimated {estimated} > max_allowed {max_allowed} for input size {input_size}")]
    MemoryContractViolation {
        estimated: usize,
        max_allowed: usize,
        input_size: usize,
    },

    #[error("Parsing failed during performance test: {0}")]
    ParseFailure(String),

    #[error("Performance test timeout after {duration:?}")]
    Timeout { duration: Duration },

    #[error("Invalid input parameters: {0}")]
    InvalidInput(String),
}

/// Stream processing performance contract
#[derive(Debug, Clone)]
pub struct StreamPerformanceContract {
    pub max_latency_per_item: Duration,
    pub max_memory_overhead_factor: f64,
    pub min_throughput_items_per_second: f64,
}

impl Default for StreamPerformanceContract {
    fn default() -> Self {
        Self {
            max_latency_per_item: Duration::from_millis(1), // 1ms per item
            max_memory_overhead_factor: 2.0,                // 2x input size max
            min_throughput_items_per_second: 1000.0,        // 1000 items/sec minimum
        }
    }
}

impl StreamPerformanceContract {
    /// Validate stream processing performance
    pub async fn validate_stream_performance<T, P>(
        &self,
        processor: &P,
        test_items: Vec<T>,
    ) -> Result<StreamPerformanceReport, PerformanceError>
    where
        T: Clone + Send + Sync + 'static,
        P: crate::traits::StreamProcessor<T>,
    {
        let start_time = Instant::now();
        let input_stream = crate::streaming::BoundedStream::new(test_items.len());

        // Process stream
        let result = processor.process_stream(input_stream).await;
        let processing_duration = start_time.elapsed();

        let _output_stream = result.map_err(|e| {
            PerformanceError::ParseFailure(format!("Stream processing failed: {:?}", e))
        })?;

        // Calculate metrics
        let items_processed = test_items.len();
        let throughput = items_processed as f64 / processing_duration.as_secs_f64();
        let avg_latency_per_item = processing_duration / items_processed as u32;

        // Validate throughput
        if throughput < self.min_throughput_items_per_second {
            return Err(PerformanceError::TimeContractViolation {
                actual: processing_duration,
                expected: Duration::from_secs_f64(
                    items_processed as f64 / self.min_throughput_items_per_second,
                ),
                input_size: items_processed,
            });
        }

        // Validate latency
        if avg_latency_per_item > self.max_latency_per_item {
            return Err(PerformanceError::TimeContractViolation {
                actual: avg_latency_per_item,
                expected: self.max_latency_per_item,
                input_size: items_processed,
            });
        }

        Ok(StreamPerformanceReport {
            processing_duration,
            items_processed,
            throughput_items_per_second: throughput,
            avg_latency_per_item,
            memory_efficiency_ratio: self.estimate_memory_ratio(&test_items),
        })
    }

    fn estimate_memory_ratio<T>(&self, items: &[T]) -> f64 {
        // Rough estimate based on item count
        if items.is_empty() {
            0.0
        } else {
            (std::mem::size_of_val(items) as f64 * self.max_memory_overhead_factor)
                / (std::mem::size_of::<T>() as f64 * items.len() as f64)
        }
    }
}

/// Stream processing performance report
#[derive(Debug, Clone)]
pub struct StreamPerformanceReport {
    pub processing_duration: Duration,
    pub items_processed: usize,
    pub throughput_items_per_second: f64,
    pub avg_latency_per_item: Duration,
    pub memory_efficiency_ratio: f64,
}

/// Performance validation utilities
pub struct PerformanceValidator;

impl PerformanceValidator {
    /// Validate that a component meets all required performance contracts
    pub async fn validate_component<P>(
        component: &P,
        test_cases: Vec<TestCase>,
    ) -> Result<Vec<PerformanceReport>, PerformanceError>
    where
        P: crate::traits::UniversalParser<Input = String>,
    {
        let mut reports = Vec::new();

        for test_case in test_cases {
            let contract = ParsingPerformanceContract::default();
            let report = contract
                .validate_parsing_performance(
                    component,
                    &test_case.input,
                    test_case.input_size_bytes,
                )
                .await?;

            reports.push(report);
        }

        Ok(reports)
    }
}

/// Test case for performance validation
#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub input: String,
    pub input_size_bytes: usize,
    pub expected_complexity: Option<ComplexityMetrics>,
}

impl TestCase {
    pub fn new(name: String, input: String) -> Self {
        let input_size_bytes = input.len();
        Self {
            name,
            input,
            input_size_bytes,
            expected_complexity: None,
        }
    }
}

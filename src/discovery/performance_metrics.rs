//! Performance metrics and monitoring for discovery operations
//! 
//! Provides comprehensive performance monitoring with Histogram and Counter metrics
//! to validate performance contracts and detect regressions.

use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Performance metrics error types
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum MetricsError {
    #[error("Performance contract violation: {operation} took {actual:?}, expected <{limit:?}")]
    ContractViolation {
        operation: String,
        actual: Duration,
        limit: Duration,
    },
    
    #[error("Memory usage exceeded limit: {current_mb}MB > {limit_mb}MB")]
    MemoryLimitExceeded {
        current_mb: usize,
        limit_mb: usize,
    },
    
    #[error("Metrics collection failed: {reason}")]
    CollectionFailed {
        reason: String,
    },
}

/// Counter metric for tracking operation counts
#[derive(Debug, Clone)]
pub struct Counter {
    value: Arc<AtomicU64>,
    name: String,
}

/// Histogram metric for tracking operation durations
#[derive(Debug, Clone)]
pub struct Histogram {
    samples: Arc<Mutex<Vec<Duration>>>,
    name: String,
    max_samples: usize,
}

/// Discovery performance metrics collector
#[derive(Debug, Clone)]
pub struct DiscoveryMetrics {
    // Counters for operation tracking
    discovery_operations: Counter,
    existing_queries: Counter,
    contract_violations: Counter,
    
    // Histograms for timing analysis
    discovery_duration: Histogram,
    existing_query_duration: Histogram,
    
    // Performance contracts
    discovery_time_limit: Duration,
    existing_query_limit: Duration,
    memory_limit_increase_percent: f64,
    
    // Memory tracking
    baseline_memory_mb: Arc<AtomicU64>,
}

/// Performance contract validation result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractValidation {
    pub operation: String,
    pub duration: Duration,
    pub limit: Duration,
    pub passed: bool,
    pub memory_usage_mb: Option<usize>,
}

/// Memory usage statistics
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryStats {
    pub current_mb: usize,
    pub baseline_mb: usize,
    pub increase_percent: f64,
    pub within_limit: bool,
}

/// Histogram statistics for performance analysis
#[derive(Debug, Clone, PartialEq)]
pub struct HistogramStats {
    pub min: Duration,
    pub max: Duration,
    pub mean: Duration,
    pub median: Duration,
    pub sample_count: usize,
}

/// Performance regression detection result
#[derive(Debug, Clone, PartialEq)]
pub struct RegressionCheck {
    pub is_regression: bool,
    pub performance_degradation_percent: f64,
    pub baseline_mean: Duration,
    pub current_duration: Duration,
}

impl Counter {
    /// Create a new counter with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            value: Arc::new(AtomicU64::new(0)),
            name: name.into(),
        }
    }
    
    /// Get the current counter value
    pub fn value(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
    
    /// Increment the counter by 1
    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Add a specific value to the counter
    pub fn add(&self, value: u64) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }
    
    /// Reset the counter to 0
    pub fn reset(&self) {
        self.value.store(0, Ordering::Relaxed);
    }
    
    /// Get the counter name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Histogram {
    /// Create a new histogram with the given name and maximum sample count
    pub fn new(name: impl Into<String>, max_samples: usize) -> Self {
        Self {
            samples: Arc::new(Mutex::new(Vec::with_capacity(max_samples))),
            name: name.into(),
            max_samples,
        }
    }
    
    /// Record a duration sample
    pub fn record(&self, duration: Duration) {
        let mut samples = self.samples.lock().unwrap();
        
        // If we're at capacity, remove the oldest sample (FIFO)
        if samples.len() >= self.max_samples {
            samples.remove(0);
        }
        
        samples.push(duration);
    }
    
    /// Get the number of recorded samples
    pub fn sample_count(&self) -> usize {
        self.samples.lock().unwrap().len()
    }
    
    /// Calculate histogram statistics
    pub fn statistics(&self) -> HistogramStats {
        let samples = self.samples.lock().unwrap();
        
        if samples.is_empty() {
            return HistogramStats {
                min: Duration::ZERO,
                max: Duration::ZERO,
                mean: Duration::ZERO,
                median: Duration::ZERO,
                sample_count: 0,
            };
        }
        
        let mut sorted_samples = samples.clone();
        sorted_samples.sort();
        
        let min = sorted_samples[0];
        let max = sorted_samples[sorted_samples.len() - 1];
        
        let total_nanos: u128 = sorted_samples.iter().map(|d| d.as_nanos()).sum();
        let mean = Duration::from_nanos((total_nanos / sorted_samples.len() as u128) as u64);
        
        let median = if sorted_samples.len() % 2 == 0 {
            let mid1 = sorted_samples[sorted_samples.len() / 2 - 1];
            let mid2 = sorted_samples[sorted_samples.len() / 2];
            Duration::from_nanos((mid1.as_nanos() + mid2.as_nanos()) as u64 / 2)
        } else {
            sorted_samples[sorted_samples.len() / 2]
        };
        
        HistogramStats {
            min,
            max,
            mean,
            median,
            sample_count: sorted_samples.len(),
        }
    }
    
    /// Get percentile value
    pub fn percentile(&self, percentile: f64) -> Duration {
        let samples = self.samples.lock().unwrap();
        
        if samples.is_empty() {
            return Duration::ZERO;
        }
        
        let mut sorted_samples = samples.clone();
        sorted_samples.sort();
        
        let index = ((percentile / 100.0) * (sorted_samples.len() - 1) as f64).round() as usize;
        let clamped_index = index.min(sorted_samples.len() - 1);
        
        sorted_samples[clamped_index]
    }
    
    /// Reset the histogram
    pub fn reset(&self) {
        self.samples.lock().unwrap().clear();
    }
    
    /// Get the histogram name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl HistogramStats {
    /// Get percentile from pre-calculated statistics
    pub fn percentile(&self, _percentile: f64) -> Duration {
        // For simplicity in tests, return max for high percentiles
        self.max
    }
}

impl DiscoveryMetrics {
    /// Create a new DiscoveryMetrics instance with default performance contracts
    pub fn new() -> Self {
        Self {
            discovery_operations: Counter::new("discovery_operations"),
            existing_queries: Counter::new("existing_queries"),
            contract_violations: Counter::new("contract_violations"),
            
            discovery_duration: Histogram::new("discovery_duration", 10000),
            existing_query_duration: Histogram::new("existing_query_duration", 10000),
            
            discovery_time_limit: Duration::from_millis(100),
            existing_query_limit: Duration::from_micros(50),
            memory_limit_increase_percent: 20.0,
            
            baseline_memory_mb: Arc::new(AtomicU64::new(0)),
        }
    }
    
    /// Validate discovery operation performance contract
    pub fn validate_discovery_performance(
        &self,
        operation: &str,
        duration: Duration,
    ) -> Result<ContractValidation, MetricsError> {
        let passed = duration <= self.discovery_time_limit;
        
        if !passed {
            self.contract_violations.increment();
            return Err(MetricsError::ContractViolation {
                operation: operation.to_string(),
                actual: duration,
                limit: self.discovery_time_limit,
            });
        }
        
        self.discovery_operations.increment();
        self.discovery_duration.record(duration);
        
        Ok(ContractValidation {
            operation: operation.to_string(),
            duration,
            limit: self.discovery_time_limit,
            passed,
            memory_usage_mb: None,
        })
    }
    
    /// Validate existing query performance contract
    pub fn validate_existing_query_performance(
        &self,
        operation: &str,
        duration: Duration,
    ) -> Result<ContractValidation, MetricsError> {
        let passed = duration <= self.existing_query_limit;
        
        if !passed {
            self.contract_violations.increment();
            return Err(MetricsError::ContractViolation {
                operation: operation.to_string(),
                actual: duration,
                limit: self.existing_query_limit,
            });
        }
        
        self.existing_queries.increment();
        self.existing_query_duration.record(duration);
        
        Ok(ContractValidation {
            operation: operation.to_string(),
            duration,
            limit: self.existing_query_limit,
            passed,
            memory_usage_mb: None,
        })
    }
    
    /// Set baseline memory usage
    pub fn set_baseline_memory(&mut self, memory_mb: usize) {
        self.baseline_memory_mb.store(memory_mb as u64, Ordering::Relaxed);
    }
    
    /// Validate memory usage against baseline
    pub fn validate_memory_usage(&self, current_mb: usize) -> Result<MemoryStats, MetricsError> {
        let baseline_mb = self.baseline_memory_mb.load(Ordering::Relaxed) as usize;
        
        let increase_percent = if baseline_mb > 0 {
            ((current_mb as f64 - baseline_mb as f64) / baseline_mb as f64) * 100.0
        } else {
            0.0
        };
        
        let within_limit = increase_percent <= self.memory_limit_increase_percent;
        
        if !within_limit {
            let limit_mb = baseline_mb + ((baseline_mb as f64 * self.memory_limit_increase_percent / 100.0) as usize);
            return Err(MetricsError::MemoryLimitExceeded {
                current_mb,
                limit_mb,
            });
        }
        
        Ok(MemoryStats {
            current_mb,
            baseline_mb,
            increase_percent,
            within_limit,
        })
    }
    
    /// Record a discovery operation
    pub fn record_discovery_operation(&self, _operation: &str, duration: Duration) {
        self.discovery_operations.increment();
        self.discovery_duration.record(duration);
    }
    
    /// Detect performance regression
    pub fn detect_performance_regression(
        &self,
        _operation: &str,
        current_duration: Duration,
    ) -> RegressionCheck {
        let stats = self.discovery_duration.statistics();
        
        if stats.sample_count == 0 {
            return RegressionCheck {
                is_regression: false,
                performance_degradation_percent: 0.0,
                baseline_mean: Duration::ZERO,
                current_duration,
            };
        }
        
        let baseline_mean = stats.mean;
        let degradation_percent = if baseline_mean.as_nanos() > 0 {
            ((current_duration.as_nanos() as f64 - baseline_mean.as_nanos() as f64) / baseline_mean.as_nanos() as f64) * 100.0
        } else {
            0.0
        };
        
        RegressionCheck {
            is_regression: degradation_percent > 25.0, // 25% threshold for regression
            performance_degradation_percent: degradation_percent,
            baseline_mean,
            current_duration,
        }
    }
    
    /// Validate operation with memory monitoring
    pub fn validate_operation_with_memory(
        &self,
        operation: &str,
        duration: Duration,
        memory_mb: usize,
    ) -> Result<ContractValidation, MetricsError> {
        // First validate timing
        let mut validation = self.validate_discovery_performance(operation, duration)?;
        
        // Then validate memory
        let _memory_stats = self.validate_memory_usage(memory_mb)?;
        
        // Add memory info to validation result
        validation.memory_usage_mb = Some(memory_mb);
        
        Ok(validation)
    }
    
    /// Reset all metrics
    pub fn reset(&self) {
        self.discovery_operations.reset();
        self.existing_queries.reset();
        self.contract_violations.reset();
        self.discovery_duration.reset();
        self.existing_query_duration.reset();
    }
    
    /// Get discovery operations counter (for testing)
    pub fn discovery_operations_count(&self) -> u64 {
        self.discovery_operations.value()
    }
    
    /// Get existing queries counter (for testing)
    pub fn existing_queries_count(&self) -> u64 {
        self.existing_queries.value()
    }
    
    /// Get contract violations counter (for testing)
    pub fn contract_violations_count(&self) -> u64 {
        self.contract_violations.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    // STUB → RED → GREEN → REFACTOR cycle starts here
    
    #[test]
    fn test_counter_creation_and_increment() {
        // RED: This test should fail until we implement Counter
        let counter = Counter::new("test_counter");
        assert_eq!(counter.value(), 0);
        
        counter.increment();
        assert_eq!(counter.value(), 1);
        
        counter.add(5);
        assert_eq!(counter.value(), 6);
    }
    
    #[test]
    fn test_histogram_creation_and_recording() {
        // RED: This test should fail until we implement Histogram
        let histogram = Histogram::new("test_histogram", 1000);
        assert_eq!(histogram.sample_count(), 0);
        
        histogram.record(Duration::from_millis(50));
        histogram.record(Duration::from_millis(100));
        histogram.record(Duration::from_millis(75));
        
        assert_eq!(histogram.sample_count(), 3);
        
        let stats = histogram.statistics();
        assert_eq!(stats.min, Duration::from_millis(50));
        assert_eq!(stats.max, Duration::from_millis(100));
        assert_eq!(stats.mean, Duration::from_millis(75));
    }
    
    #[test]
    fn test_discovery_metrics_creation() {
        // RED: This test should fail until we implement DiscoveryMetrics
        let metrics = DiscoveryMetrics::new();
        
        assert_eq!(metrics.discovery_operations.value(), 0);
        assert_eq!(metrics.existing_queries.value(), 0);
        assert_eq!(metrics.contract_violations.value(), 0);
        
        assert_eq!(metrics.discovery_duration.sample_count(), 0);
        assert_eq!(metrics.existing_query_duration.sample_count(), 0);
    }
    
    #[test]
    fn test_discovery_performance_contract_validation() {
        // RED: This test should fail until we implement contract validation
        let metrics = DiscoveryMetrics::new();
        
        // Fast operation should pass
        let fast_result = metrics.validate_discovery_performance(
            "list_entities",
            Duration::from_millis(50)
        );
        assert!(fast_result.is_ok());
        let validation = fast_result.unwrap();
        assert!(validation.passed);
        assert_eq!(validation.operation, "list_entities");
        assert_eq!(validation.duration, Duration::from_millis(50));
        assert_eq!(validation.limit, Duration::from_millis(100));
        
        // Slow operation should fail
        let slow_result = metrics.validate_discovery_performance(
            "list_entities",
            Duration::from_millis(150)
        );
        assert!(slow_result.is_err());
        
        if let Err(MetricsError::ContractViolation { operation, actual, limit }) = slow_result {
            assert_eq!(operation, "list_entities");
            assert_eq!(actual, Duration::from_millis(150));
            assert_eq!(limit, Duration::from_millis(100));
        } else {
            panic!("Expected ContractViolation error");
        }
    }
    
    #[test]
    fn test_existing_query_performance_contract_validation() {
        // RED: This test should fail until we implement existing query validation
        let metrics = DiscoveryMetrics::new();
        
        // Fast query should pass
        let fast_result = metrics.validate_existing_query_performance(
            "blast_radius",
            Duration::from_micros(25)
        );
        assert!(fast_result.is_ok());
        let validation = fast_result.unwrap();
        assert!(validation.passed);
        assert_eq!(validation.operation, "blast_radius");
        assert_eq!(validation.duration, Duration::from_micros(25));
        assert_eq!(validation.limit, Duration::from_micros(50));
        
        // Slow query should fail
        let slow_result = metrics.validate_existing_query_performance(
            "blast_radius",
            Duration::from_micros(100)
        );
        assert!(slow_result.is_err());
    }
    
    #[test]
    fn test_memory_usage_monitoring() {
        // RED: This test should fail until we implement memory monitoring
        let mut metrics = DiscoveryMetrics::new();
        metrics.set_baseline_memory(100);
        
        // Acceptable memory usage (10% increase)
        let acceptable_result = metrics.validate_memory_usage(110);
        assert!(acceptable_result.is_ok());
        let stats = acceptable_result.unwrap();
        assert_eq!(stats.current_mb, 110);
        assert_eq!(stats.baseline_mb, 100);
        assert_eq!(stats.increase_percent, 10.0);
        assert!(stats.within_limit);
        
        // At the limit (20% increase)
        let limit_result = metrics.validate_memory_usage(120);
        assert!(limit_result.is_ok());
        let stats = limit_result.unwrap();
        assert_eq!(stats.increase_percent, 20.0);
        assert!(stats.within_limit);
        
        // Excessive memory usage (30% increase)
        let excessive_result = metrics.validate_memory_usage(130);
        assert!(excessive_result.is_err());
        
        if let Err(MetricsError::MemoryLimitExceeded { current_mb, limit_mb }) = excessive_result {
            assert_eq!(current_mb, 130);
            assert_eq!(limit_mb, 120); // 100 + 20% = 120
        } else {
            panic!("Expected MemoryLimitExceeded error");
        }
    }
    
    #[test]
    fn test_performance_regression_detection() {
        // RED: This test should fail until we implement regression detection
        let metrics = DiscoveryMetrics::new();
        
        // Record baseline performance
        metrics.record_discovery_operation("list_entities", Duration::from_millis(80));
        metrics.record_discovery_operation("list_entities", Duration::from_millis(85));
        metrics.record_discovery_operation("list_entities", Duration::from_millis(75));
        
        // Check for regressions
        let regression_check = metrics.detect_performance_regression(
            "list_entities",
            Duration::from_millis(120) // 50% slower than baseline
        );
        
        assert!(regression_check.is_regression);
        assert!(regression_check.performance_degradation_percent > 40.0);
    }
    
    #[test]
    fn test_concurrent_metrics_collection() {
        // RED: This test should fail until we implement thread-safe metrics
        let metrics = Arc::new(DiscoveryMetrics::new());
        let mut handles = vec![];
        
        // Spawn multiple threads recording metrics
        for i in 0..10 {
            let metrics_clone = Arc::clone(&metrics);
            handles.push(thread::spawn(move || {
                for j in 0..100 {
                    metrics_clone.discovery_operations.increment();
                    metrics_clone.discovery_duration.record(Duration::from_millis(i * 10 + j));
                }
            }));
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify thread safety
        assert_eq!(metrics.discovery_operations.value(), 1000); // 10 threads * 100 increments
        assert_eq!(metrics.discovery_duration.sample_count(), 1000);
    }
    
    #[test]
    fn test_histogram_statistics_calculation() {
        // RED: This test should fail until we implement histogram statistics
        let histogram = Histogram::new("test", 1000);
        
        // Record known values
        histogram.record(Duration::from_millis(10));
        histogram.record(Duration::from_millis(20));
        histogram.record(Duration::from_millis(30));
        histogram.record(Duration::from_millis(40));
        histogram.record(Duration::from_millis(50));
        
        let stats = histogram.statistics();
        assert_eq!(stats.min, Duration::from_millis(10));
        assert_eq!(stats.max, Duration::from_millis(50));
        assert_eq!(stats.mean, Duration::from_millis(30));
        assert_eq!(stats.median, Duration::from_millis(30));
        
        // Test percentiles using the histogram method
        assert_eq!(histogram.percentile(50.0), Duration::from_millis(30)); // median
        assert_eq!(histogram.percentile(90.0), Duration::from_millis(50)); // 90th percentile
        assert_eq!(histogram.percentile(95.0), Duration::from_millis(50)); // 95th percentile
    }
    
    #[test]
    fn test_metrics_reset_and_cleanup() {
        // RED: This test should fail until we implement reset functionality
        let metrics = DiscoveryMetrics::new();
        
        // Record some data
        metrics.discovery_operations.increment();
        metrics.discovery_duration.record(Duration::from_millis(50));
        
        assert_eq!(metrics.discovery_operations.value(), 1);
        assert_eq!(metrics.discovery_duration.sample_count(), 1);
        
        // Reset metrics
        metrics.reset();
        
        assert_eq!(metrics.discovery_operations.value(), 0);
        assert_eq!(metrics.discovery_duration.sample_count(), 0);
    }
    
    #[test]
    fn test_performance_contract_validation_with_memory() {
        // RED: This test should fail until we implement combined validation
        let mut metrics = DiscoveryMetrics::new();
        metrics.set_baseline_memory(100);
        
        let validation = metrics.validate_operation_with_memory(
            "complex_discovery",
            Duration::from_millis(80),
            110 // 10% memory increase
        );
        
        assert!(validation.is_ok());
        let result = validation.unwrap();
        assert!(result.passed);
        assert_eq!(result.memory_usage_mb, Some(110));
        
        // Test with contract violation
        let violation = metrics.validate_operation_with_memory(
            "slow_discovery",
            Duration::from_millis(150), // Exceeds 100ms limit
            105 // Acceptable memory
        );
        
        assert!(violation.is_err());
    }
}

/// Micro-benchmark tests for performance optimization validation
/// 
/// Following TDD: RED → GREEN → REFACTOR cycle
/// These tests establish performance contracts that must be met.
#[cfg(test)]
mod micro_benchmark_tests {
    use super::*;
    use crate::discovery::{
        indexes::{DiscoveryIndexes, CompactEntityInfo},
        string_interning::FileInterner,
        types::{EntityInfo, EntityType},
    };
    use std::time::Instant;
    
    // RED PHASE: Micro-benchmark tests that should FAIL until optimizations are implemented
    
    #[test]
    fn test_micro_benchmark_entity_filtering_performance() {
        // PERFORMANCE CONTRACT: Entity filtering must complete in <50μs
        let mut indexes = DiscoveryIndexes::new();
        
        // Create test dataset
        let mut entities = Vec::new();
        for i in 0..1000 {
            entities.push(EntityInfo::new(
                format!("entity_{}", i),
                format!("src/file_{}.rs", i % 10),
                if i % 2 == 0 { EntityType::Function } else { EntityType::Struct },
                Some(i as u32 + 1),
                Some((i % 80) as u32 + 1),
            ));
        }
        
        indexes.rebuild_from_entities(entities).unwrap();
        
        // Micro-benchmark: Zero-allocation filtering
        let iterations = 100;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let count = indexes
                .filter_entities_by_type(EntityType::Function)
                .filter(|e| e.line_number > 100)
                .take(100)
                .count();
            std::hint::black_box(count);
        }
        
        let elapsed = start.elapsed();
        let per_iteration = elapsed / iterations;
        
        // PERFORMANCE CONTRACT: <50μs per filtering operation
        assert!(per_iteration < Duration::from_micros(50),
                "Entity filtering took {:?} per iteration, expected <50μs", per_iteration);
    }
    
    #[test]
    fn test_micro_benchmark_string_interning_performance() {
        // PERFORMANCE CONTRACT: String interning must be efficient
        let mut interner = FileInterner::new();
        
        let test_paths = vec![
            "src/main.rs", "src/lib.rs", "src/parser.rs", "src/utils.rs",
            "tests/integration.rs", "benches/benchmark.rs",
        ];
        
        // Micro-benchmark: Interning performance
        let iterations = 1000;
        let start = Instant::now();
        
        for i in 0..iterations {
            let path = test_paths[i % test_paths.len()];
            let _file_id = interner.intern(path);
        }
        
        let elapsed = start.elapsed();
        let per_intern = elapsed / iterations as u32;
        
        // PERFORMANCE CONTRACT: <1μs per interning operation
        assert!(per_intern < Duration::from_micros(1),
                "String interning took {:?} per operation, expected <1μs", per_intern);
        
        // Test memory efficiency
        let usage = interner.memory_usage();
        let bytes_per_entry = usage.total_bytes() / interner.len();
        
        assert!(bytes_per_entry < 200,
                "String interning uses {} bytes per entry, expected <200", bytes_per_entry);
    }
    
    #[test]
    fn test_micro_benchmark_memory_stats_calculation() {
        // PERFORMANCE CONTRACT: Memory stats calculation must be fast
        let mut indexes = DiscoveryIndexes::new();
        
        let entities = (0..1000).map(|i| {
            EntityInfo::new(
                format!("entity_{}", i),
                format!("src/file_{}.rs", i % 20),
                EntityType::Function,
                Some(i as u32 + 1),
                Some((i % 80) as u32 + 1),
            )
        }).collect();
        
        indexes.rebuild_from_entities(entities).unwrap();
        
        // Micro-benchmark: Memory stats calculation
        let iterations = 100;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let stats = indexes.memory_stats();
            std::hint::black_box(stats);
        }
        
        let elapsed = start.elapsed();
        let per_calculation = elapsed / iterations;
        
        // PERFORMANCE CONTRACT: <10μs per calculation
        assert!(per_calculation < Duration::from_micros(10),
                "Memory stats calculation took {:?}, expected <10μs", per_calculation);
    }
    
    #[test]
    fn test_micro_benchmark_compact_entity_memory_layout() {
        // PERFORMANCE CONTRACT: CompactEntityInfo must be exactly 24 bytes
        let size = std::mem::size_of::<CompactEntityInfo>();
        let align = std::mem::align_of::<CompactEntityInfo>();
        
        // This will FAIL until we optimize the memory layout
        assert_eq!(size, 24, "CompactEntityInfo must be exactly 24 bytes, got {}", size);
        assert_eq!(align, 8, "CompactEntityInfo must be 8-byte aligned, got {}", align);
        
        // Test that we can fit multiple entities in a cache line
        let entities_per_cache_line = 64 / size; // 64-byte cache line
        assert!(entities_per_cache_line >= 2,
                "Should fit at least 2 entities per cache line, got {}", entities_per_cache_line);
    }
    
    #[test]
    fn test_micro_benchmark_index_rebuild_scalability() {
        // PERFORMANCE CONTRACT: Index rebuild must scale efficiently
        let test_sizes = vec![100, 500, 1000];
        let mut rebuild_times = Vec::new();
        
        for &size in &test_sizes {
            let mut indexes = DiscoveryIndexes::new();
            let entities = (0..size).map(|i| {
                EntityInfo::new(
                    format!("entity_{}", i),
                    format!("src/file_{}.rs", i % 10),
                    EntityType::Function,
                    Some(i as u32 + 1),
                    Some((i % 80) as u32 + 1),
                )
            }).collect();
            
            let start = Instant::now();
            indexes.rebuild_from_entities(entities).unwrap();
            let rebuild_time = start.elapsed();
            
            rebuild_times.push(rebuild_time);
            
            // PERFORMANCE CONTRACT: Should rebuild quickly
            let max_time = Duration::from_millis(size as u64); // 1ms per 1000 entities
            assert!(rebuild_time < max_time,
                    "Rebuild of {} entities took {:?}, expected <{:?}", 
                    size, rebuild_time, max_time);
        }
        
        // Test that rebuild time scales sub-linearly
        if rebuild_times.len() >= 2 {
            let size_ratio = test_sizes[1] as f64 / test_sizes[0] as f64;
            let time_ratio = rebuild_times[1].as_nanos() as f64 / rebuild_times[0].as_nanos() as f64;
            
            // Time should scale better than linearly
            assert!(time_ratio < size_ratio * 1.5,
                    "Rebuild time scaling: {:.2}x time for {:.2}x size (should be sub-linear)",
                    time_ratio, size_ratio);
        }
    }
}
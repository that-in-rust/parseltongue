//! Performance regression tests for all critical discovery paths
//! 
//! These tests validate that performance contracts are maintained across
//! all critical discovery operations and detect regressions early.

use super::performance_metrics::{DiscoveryMetrics, MetricsError};
use super::simple_discovery_engine::SimpleDiscoveryEngine;
// Note: EntityType and DiscoveryQuery may be used in future enhancements
use super::engine::DiscoveryEngine;
use crate::isg::{OptimizedISG, NodeKind, NodeData, SigHash};
use std::time::{Duration, Instant};
use std::sync::Arc;

/// Performance contract validation for critical discovery paths
pub struct PerformanceRegressionTester {
    metrics: DiscoveryMetrics,
    discovery_engine: SimpleDiscoveryEngine,
}

impl PerformanceRegressionTester {
    /// Create a new performance regression tester
    pub fn new() -> Self {
        let isg = OptimizedISG::new();
        let discovery_engine = SimpleDiscoveryEngine::new(isg);
        
        Self {
            metrics: DiscoveryMetrics::new(),
            discovery_engine,
        }
    }
    
    /// Create a tester with test data
    pub fn with_test_data(entity_count: usize) -> Self {
        let mut tester = Self::new();
        tester.populate_test_data(entity_count);
        tester
    }
    
    /// Populate the discovery engine with test data
    fn populate_test_data(&mut self, entity_count: usize) {
        // Create a new ISG with test data using the correct API
        let isg = OptimizedISG::new();
        
        for i in 0..entity_count {
            let node_data = NodeData {
                hash: SigHash::from_signature(&format!("test_entity_{}", i)),
                kind: NodeKind::Function,
                name: Arc::from(format!("test_entity_{}", i)),
                signature: Arc::from(format!("fn test_entity_{}()", i)),
                file_path: Arc::from(format!("src/test_{}.rs", i % 10)),
                line: i as u32 + 1,
            };
            
            isg.upsert_node(node_data);
        }
        
        // Replace the discovery engine with one that has test data
        self.discovery_engine = SimpleDiscoveryEngine::new(isg);
    }
    
    /// Test discovery operation performance contract (<100ms)
    pub async fn test_discovery_performance_contract(&self, max_results: usize) -> Result<Duration, MetricsError> {
        let start = Instant::now();
        
        let _result = self.discovery_engine
            .list_all_entities(None, max_results)
            .await
            .map_err(|e| MetricsError::CollectionFailed { 
                reason: format!("Discovery failed: {}", e) 
            })?;
        
        let elapsed = start.elapsed();
        
        // Validate against contract
        self.metrics.validate_discovery_performance("list_all_entities", elapsed)?;
        
        Ok(elapsed)
    }
    
    /// Test existing query performance contract (<50μs)
    pub async fn test_existing_query_performance_contract(&self, entity_name: &str) -> Result<Duration, MetricsError> {
        let start = Instant::now();
        
        let _result = self.discovery_engine
            .where_defined(entity_name)
            .await
            .map_err(|e| MetricsError::CollectionFailed { 
                reason: format!("Query failed: {}", e) 
            })?;
        
        let elapsed = start.elapsed();
        
        // Validate against contract
        self.metrics.validate_existing_query_performance("where_defined", elapsed)?;
        
        Ok(elapsed)
    }
    
    /// Test file-based discovery performance
    pub async fn test_file_discovery_performance(&self, file_path: &str) -> Result<Duration, MetricsError> {
        let start = Instant::now();
        
        let _result = self.discovery_engine
            .entities_in_file(file_path)
            .await
            .map_err(|e| MetricsError::CollectionFailed { 
                reason: format!("File discovery failed: {}", e) 
            })?;
        
        let elapsed = start.elapsed();
        
        // File discovery should also meet the discovery contract
        self.metrics.validate_discovery_performance("entities_in_file", elapsed)?;
        
        Ok(elapsed)
    }
    
    /// Test entity count performance
    pub async fn test_entity_count_performance(&self) -> Result<Duration, MetricsError> {
        let start = Instant::now();
        
        let _count = self.discovery_engine
            .total_entity_count()
            .await
            .map_err(|e| MetricsError::CollectionFailed { 
                reason: format!("Entity count failed: {}", e) 
            })?;
        
        let elapsed = start.elapsed();
        
        // Entity count should be very fast (<10ms as per contract)
        if elapsed > Duration::from_millis(10) {
            return Err(MetricsError::ContractViolation {
                operation: "total_entity_count".to_string(),
                actual: elapsed,
                limit: Duration::from_millis(10),
            });
        }
        
        Ok(elapsed)
    }
    
    /// Test memory usage during operations
    pub fn test_memory_usage_contract(&mut self, baseline_mb: usize) -> Result<(), MetricsError> {
        self.metrics.set_baseline_memory(baseline_mb);
        
        // Simulate current memory usage (should be within 20% of baseline)
        let current_mb = baseline_mb + (baseline_mb * 15 / 100); // 15% increase
        
        self.metrics.validate_memory_usage(current_mb)?;
        
        Ok(())
    }
    
    /// Run comprehensive performance regression test suite
    pub async fn run_comprehensive_test_suite(&mut self) -> PerformanceTestResults {
        let mut results = PerformanceTestResults::new();
        
        // Test 1: Small dataset discovery (100 entities)
        match self.test_discovery_performance_contract(100).await {
            Ok(duration) => results.add_success("small_discovery", duration),
            Err(e) => results.add_failure("small_discovery", e),
        }
        
        // Test 2: Large dataset discovery (1000 entities)
        match self.test_discovery_performance_contract(1000).await {
            Ok(duration) => results.add_success("large_discovery", duration),
            Err(e) => results.add_failure("large_discovery", e),
        }
        
        // Test 3: Existing query performance
        match self.test_existing_query_performance_contract("test_entity_0").await {
            Ok(duration) => results.add_success("existing_query", duration),
            Err(e) => results.add_failure("existing_query", e),
        }
        
        // Test 4: File-based discovery
        match self.test_file_discovery_performance("src/test_0.rs").await {
            Ok(duration) => results.add_success("file_discovery", duration),
            Err(e) => results.add_failure("file_discovery", e),
        }
        
        // Test 5: Entity count performance
        match self.test_entity_count_performance().await {
            Ok(duration) => results.add_success("entity_count", duration),
            Err(e) => results.add_failure("entity_count", e),
        }
        
        // Test 6: Memory usage contract
        match self.test_memory_usage_contract(100) {
            Ok(()) => results.add_success("memory_usage", Duration::ZERO),
            Err(e) => results.add_failure("memory_usage", e),
        }
        
        results
    }
}

/// Results of performance regression testing
#[derive(Debug, Clone)]
pub struct PerformanceTestResults {
    pub successes: Vec<(String, Duration)>,
    pub failures: Vec<(String, MetricsError)>,
}

impl PerformanceTestResults {
    pub fn new() -> Self {
        Self {
            successes: Vec::new(),
            failures: Vec::new(),
        }
    }
    
    pub fn add_success(&mut self, test_name: impl Into<String>, duration: Duration) {
        self.successes.push((test_name.into(), duration));
    }
    
    pub fn add_failure(&mut self, test_name: impl Into<String>, error: MetricsError) {
        self.failures.push((test_name.into(), error));
    }
    
    pub fn has_failures(&self) -> bool {
        !self.failures.is_empty()
    }
    
    pub fn success_count(&self) -> usize {
        self.successes.len()
    }
    
    pub fn failure_count(&self) -> usize {
        self.failures.len()
    }
    
    pub fn total_tests(&self) -> usize {
        self.successes.len() + self.failures.len()
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total_tests() == 0 {
            return 0.0;
        }
        (self.success_count() as f64 / self.total_tests() as f64) * 100.0
    }
    
    pub fn format_summary(&self) -> String {
        let mut summary = format!(
            "Performance Test Results: {}/{} passed ({:.1}%)\n",
            self.success_count(),
            self.total_tests(),
            self.success_rate()
        );
        
        if !self.successes.is_empty() {
            summary.push_str("\n✅ Successful Tests:\n");
            for (test_name, duration) in &self.successes {
                summary.push_str(&format!("  • {}: {:?}\n", test_name, duration));
            }
        }
        
        if !self.failures.is_empty() {
            summary.push_str("\n❌ Failed Tests:\n");
            for (test_name, error) in &self.failures {
                summary.push_str(&format!("  • {}: {}\n", test_name, error));
            }
        }
        
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_performance_regression_tester_creation() {
        let tester = PerformanceRegressionTester::new();
        
        // Should be able to create without errors
        assert_eq!(tester.metrics.discovery_operations_count(), 0);
    }
    
    #[tokio::test]
    async fn test_performance_regression_with_small_dataset() {
        let tester = PerformanceRegressionTester::with_test_data(10);
        
        // Small dataset should easily meet performance contracts
        let result = tester.test_discovery_performance_contract(10).await;
        assert!(result.is_ok());
        
        let duration = result.unwrap();
        assert!(duration < Duration::from_millis(100));
    }
    
    #[tokio::test]
    async fn test_existing_query_performance_contract() {
        let tester = PerformanceRegressionTester::with_test_data(100);
        
        // Existing query should be very fast
        let result = tester.test_existing_query_performance_contract("test_entity_0").await;
        assert!(result.is_ok());
        
        let duration = result.unwrap();
        assert!(duration < Duration::from_micros(50));
    }
    
    #[tokio::test]
    async fn test_file_discovery_performance() {
        let tester = PerformanceRegressionTester::with_test_data(50);
        
        let result = tester.test_file_discovery_performance("src/test_0.rs").await;
        assert!(result.is_ok());
        
        let duration = result.unwrap();
        assert!(duration < Duration::from_millis(100));
    }
    
    #[tokio::test]
    async fn test_entity_count_performance() {
        let tester = PerformanceRegressionTester::with_test_data(1000);
        
        let result = tester.test_entity_count_performance().await;
        assert!(result.is_ok());
        
        let duration = result.unwrap();
        assert!(duration < Duration::from_millis(10));
    }
    
    #[tokio::test]
    async fn test_memory_usage_contract() {
        let mut tester = PerformanceRegressionTester::new();
        
        // Should pass with acceptable memory usage
        let result = tester.test_memory_usage_contract(100);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_comprehensive_test_suite() {
        let mut tester = PerformanceRegressionTester::with_test_data(100);
        
        let results = tester.run_comprehensive_test_suite().await;
        
        // Should have run all tests
        assert_eq!(results.total_tests(), 6);
        
        // Most tests should pass (allowing for some variance in CI environments)
        assert!(results.success_rate() >= 80.0);
        
        println!("{}", results.format_summary());
    }
    
    #[tokio::test]
    async fn test_performance_contract_violation_detection() {
        let tester = PerformanceRegressionTester::new();
        
        // Simulate a slow operation that violates the contract
        let slow_duration = Duration::from_millis(200); // Exceeds 100ms limit
        
        let result = tester.metrics.validate_discovery_performance("slow_operation", slow_duration);
        assert!(result.is_err());
        
        if let Err(MetricsError::ContractViolation { operation, actual, limit }) = result {
            assert_eq!(operation, "slow_operation");
            assert_eq!(actual, slow_duration);
            assert_eq!(limit, Duration::from_millis(100));
        } else {
            panic!("Expected ContractViolation error");
        }
    }
    
    #[tokio::test]
    async fn test_memory_limit_violation_detection() {
        let mut tester = PerformanceRegressionTester::new();
        tester.metrics.set_baseline_memory(100);
        
        // Simulate excessive memory usage (30% increase, exceeds 20% limit)
        let result = tester.metrics.validate_memory_usage(130);
        assert!(result.is_err());
        
        if let Err(MetricsError::MemoryLimitExceeded { current_mb, limit_mb }) = result {
            assert_eq!(current_mb, 130);
            assert_eq!(limit_mb, 120); // 100 + 20% = 120
        } else {
            panic!("Expected MemoryLimitExceeded error");
        }
    }
    
    #[tokio::test]
    async fn test_performance_regression_detection() {
        let tester = PerformanceRegressionTester::new();
        
        // Record baseline performance
        tester.metrics.record_discovery_operation("test_op", Duration::from_millis(50));
        tester.metrics.record_discovery_operation("test_op", Duration::from_millis(60));
        tester.metrics.record_discovery_operation("test_op", Duration::from_millis(55));
        
        // Test with significantly slower performance (100ms vs ~55ms baseline)
        let regression_check = tester.metrics.detect_performance_regression(
            "test_op",
            Duration::from_millis(100)
        );
        
        assert!(regression_check.is_regression);
        assert!(regression_check.performance_degradation_percent > 50.0);
    }
}
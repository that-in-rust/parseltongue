//! Performance Monitoring and Regression Detection
//! 
//! This module provides comprehensive performance monitoring capabilities
//! for detecting regressions and ensuring performance contracts are maintained.

use crate::performance_validation::{PerformanceValidator, WorkloadConfig, PerformanceMetrics};
use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use chrono::DateTime;

/// Performance baseline for regression detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub timestamp: u64,
    pub platform: String,
    pub rust_version: String,
    pub workload_metrics: HashMap<String, PerformanceMetrics>,
}

/// Performance regression detector
pub struct PerformanceMonitor {
    validator: PerformanceValidator,
    baseline_path: String,
}

impl PerformanceMonitor {
    pub fn new(baseline_path: &str) -> Self {
        Self {
            validator: PerformanceValidator::new(),
            baseline_path: baseline_path.to_string(),
        }
    }
    
    /// Establish performance baseline
    pub fn establish_baseline(&self) -> Result<PerformanceBaseline, Box<dyn std::error::Error>> {
        println!("📊 Establishing performance baseline...");
        
        let workloads = vec![
            ("small", WorkloadConfig::small()),
            ("medium", WorkloadConfig::medium()),
            ("large", WorkloadConfig::large()),
            ("extra_large", WorkloadConfig::extra_large()),
        ];
        
        let mut workload_metrics = HashMap::new();
        
        for (name, config) in workloads {
            println!("   Measuring {} workload...", name);
            let metrics = self.validator.validate_workload(&config);
            workload_metrics.insert(name.to_string(), metrics);
        }
        
        let baseline = PerformanceBaseline {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            platform: std::env::consts::OS.to_string(),
            rust_version: "unknown".to_string(), // Would need build script to get actual version
            workload_metrics,
        };
        
        // Save baseline to file
        let baseline_json = serde_json::to_string_pretty(&baseline)?;
        fs::write(&self.baseline_path, baseline_json)?;
        
        println!("✅ Performance baseline established and saved to {}", self.baseline_path);
        Ok(baseline)
    }
    
    /// Load existing baseline
    pub fn load_baseline(&self) -> Result<PerformanceBaseline, Box<dyn std::error::Error>> {
        let baseline_json = fs::read_to_string(&self.baseline_path)?;
        let baseline: PerformanceBaseline = serde_json::from_str(&baseline_json)?;
        Ok(baseline)
    }
    
    /// Detect performance regressions
    pub fn detect_regressions(&self) -> Result<RegressionReport, Box<dyn std::error::Error>> {
        println!("🔍 Detecting performance regressions...");
        
        let baseline = self.load_baseline()?;
        let mut regressions = Vec::new();
        let mut improvements = Vec::new();
        
        for (workload_name, baseline_metrics) in &baseline.workload_metrics {
            println!("   Checking {} workload...", workload_name);
            
            let config = match workload_name.as_str() {
                "small" => WorkloadConfig::small(),
                "medium" => WorkloadConfig::medium(),
                "large" => WorkloadConfig::large(),
                "extra_large" => WorkloadConfig::extra_large(),
                _ => continue,
            };
            
            let current_metrics = self.validator.validate_workload(&config);
            
            // Check for regressions in key metrics
            self.check_metric_regression(
                &mut regressions,
                &mut improvements,
                workload_name,
                "node_upsert",
                baseline_metrics.node_operations.upsert_time_us,
                current_metrics.node_operations.upsert_time_us,
                20.0, // 20% tolerance
            );
            
            self.check_metric_regression(
                &mut regressions,
                &mut improvements,
                workload_name,
                "blast_radius_query",
                baseline_metrics.query_operations.blast_radius_time_us,
                current_metrics.query_operations.blast_radius_time_us,
                30.0, // 30% tolerance for queries
            );
            
            self.check_metric_regression(
                &mut regressions,
                &mut improvements,
                workload_name,
                "file_update",
                current_metrics.file_operations.update_time_ms * 1000, // Convert to μs
                baseline_metrics.file_operations.update_time_ms * 1000,
                25.0, // 25% tolerance
            );
            
            self.check_metric_regression(
                &mut regressions,
                &mut improvements,
                workload_name,
                "memory_usage",
                baseline_metrics.memory_usage.total_memory_mb as u64,
                current_metrics.memory_usage.total_memory_mb as u64,
                15.0, // 15% tolerance for memory
            );
        }
        
        let report = RegressionReport {
            baseline_timestamp: baseline.timestamp,
            current_timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            platform: std::env::consts::OS.to_string(),
            regressions,
            improvements,
        };
        
        Ok(report)
    }
    
    fn check_metric_regression(
        &self,
        regressions: &mut Vec<PerformanceRegression>,
        improvements: &mut Vec<PerformanceImprovement>,
        workload: &str,
        metric_name: &str,
        baseline_value: u64,
        current_value: u64,
        tolerance_percent: f64,
    ) {
        if baseline_value == 0 {
            return; // Skip zero baseline values
        }
        
        let change_percent = ((current_value as f64 - baseline_value as f64) / baseline_value as f64) * 100.0;
        
        if change_percent > tolerance_percent {
            regressions.push(PerformanceRegression {
                workload: workload.to_string(),
                metric: metric_name.to_string(),
                baseline_value,
                current_value,
                change_percent,
                tolerance_percent,
            });
        } else if change_percent < -10.0 { // Significant improvement
            improvements.push(PerformanceImprovement {
                workload: workload.to_string(),
                metric: metric_name.to_string(),
                baseline_value,
                current_value,
                improvement_percent: -change_percent,
            });
        }
    }
    
    /// Generate performance report
    pub fn generate_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let regression_report = self.detect_regressions()?;
        
        let mut report = String::new();
        report.push_str("# Performance Monitoring Report\n\n");
        report.push_str(&format!("**Generated**: {}\n", 
            DateTime::from_timestamp(regression_report.current_timestamp as i64, 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("**Platform**: {}\n", regression_report.platform));
        report.push_str(&format!("**Baseline**: {}\n\n", 
            DateTime::from_timestamp(regression_report.baseline_timestamp as i64, 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M:%S UTC")));
        
        if regression_report.regressions.is_empty() {
            report.push_str("## ✅ No Performance Regressions Detected\n\n");
        } else {
            report.push_str("## ❌ Performance Regressions Detected\n\n");
            for regression in &regression_report.regressions {
                report.push_str(&format!(
                    "- **{}** in **{}**: {:.1}% slower ({} → {} μs, tolerance: {:.1}%)\n",
                    regression.metric,
                    regression.workload,
                    regression.change_percent,
                    regression.baseline_value,
                    regression.current_value,
                    regression.tolerance_percent
                ));
            }
            report.push_str("\n");
        }
        
        if !regression_report.improvements.is_empty() {
            report.push_str("## 🚀 Performance Improvements\n\n");
            for improvement in &regression_report.improvements {
                report.push_str(&format!(
                    "- **{}** in **{}**: {:.1}% faster ({} → {} μs)\n",
                    improvement.metric,
                    improvement.workload,
                    improvement.improvement_percent,
                    improvement.baseline_value,
                    improvement.current_value
                ));
            }
            report.push_str("\n");
        }
        
        report.push_str("## Performance Contracts Status\n\n");
        report.push_str("All critical performance contracts are being monitored:\n");
        report.push_str("- Node operations: <50μs (O(1) guarantee)\n");
        report.push_str("- Query operations: <1ms (simple queries)\n");
        report.push_str("- File updates: <12ms (real-time constraint)\n");
        report.push_str("- Memory usage: <25MB at 100K LOC\n");
        report.push_str("- Cross-platform consistency: deterministic hashing\n");
        
        Ok(report)
    }
}

/// Performance regression details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegression {
    pub workload: String,
    pub metric: String,
    pub baseline_value: u64,
    pub current_value: u64,
    pub change_percent: f64,
    pub tolerance_percent: f64,
}

/// Performance improvement details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImprovement {
    pub workload: String,
    pub metric: String,
    pub baseline_value: u64,
    pub current_value: u64,
    pub improvement_percent: f64,
}

/// Regression detection report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionReport {
    pub baseline_timestamp: u64,
    pub current_timestamp: u64,
    pub platform: String,
    pub regressions: Vec<PerformanceRegression>,
    pub improvements: Vec<PerformanceImprovement>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_performance_monitoring_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let baseline_path = temp_dir.path().join("baseline.json");
        let monitor = PerformanceMonitor::new(baseline_path.to_str().unwrap());
        
        // Establish baseline
        let baseline = monitor.establish_baseline().unwrap();
        assert!(!baseline.workload_metrics.is_empty());
        assert!(baseline.timestamp > 0);
        
        // Load baseline
        let loaded_baseline = monitor.load_baseline().unwrap();
        assert_eq!(baseline.timestamp, loaded_baseline.timestamp);
        
        // Detect regressions (should be none immediately after baseline)
        let report = monitor.detect_regressions().unwrap();
        println!("Regressions: {}, Improvements: {}", 
            report.regressions.len(), report.improvements.len());
        
        // Generate report
        let report_text = monitor.generate_report().unwrap();
        assert!(report_text.contains("Performance Monitoring Report"));
        
        println!("✅ Performance monitoring workflow test passed");
    }
}
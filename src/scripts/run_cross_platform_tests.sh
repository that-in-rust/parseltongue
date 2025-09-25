#!/bin/bash
# Cross-platform consistency test runner for CI/CD environments
# 
# This script runs comprehensive cross-platform tests and generates reports
# suitable for validating identical behavior across Linux, macOS, and Windows

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPORT_DIR="target/cross_platform_reports"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
PLATFORM="${OSTYPE:-unknown}"
ARCH=$(uname -m 2>/dev/null || echo "unknown")

echo -e "${BLUE}🚀 Parseltongue Cross-Platform Test Runner${NC}"
echo -e "${BLUE}===========================================${NC}"
echo "Platform: $PLATFORM"
echo "Architecture: $ARCH"
echo "Timestamp: $TIMESTAMP"
echo ""

# Create report directory
mkdir -p "$REPORT_DIR"

# Function to run tests and capture output
run_test_suite() {
    local test_name=$1
    local test_command=$2
    local report_file="$REPORT_DIR/${test_name}_${PLATFORM}_${TIMESTAMP}.log"
    
    echo -e "${YELLOW}📋 Running $test_name tests...${NC}"
    
    if $test_command > "$report_file" 2>&1; then
        echo -e "${GREEN}✅ $test_name tests PASSED${NC}"
        return 0
    else
        echo -e "${RED}❌ $test_name tests FAILED${NC}"
        echo "   Report saved to: $report_file"
        return 1
    fi
}

# Function to extract test metrics
extract_metrics() {
    local log_file=$1
    local metrics_file="${log_file%.log}_metrics.json"
    
    # Extract basic metrics (this is a simplified version)
    local total_tests=$(grep -c "test result:" "$log_file" 2>/dev/null || echo "0")
    local passed_tests=$(grep "passed" "$log_file" | tail -1 | sed -n 's/.*\([0-9]\+\) passed.*/\1/p' || echo "0")
    local failed_tests=$(grep "failed" "$log_file" | tail -1 | sed -n 's/.*\([0-9]\+\) failed.*/\1/p' || echo "0")
    
    # Create simple JSON metrics
    cat > "$metrics_file" << EOF
{
    "platform": "$PLATFORM",
    "architecture": "$ARCH",
    "timestamp": "$TIMESTAMP",
    "total_tests": $total_tests,
    "passed_tests": $passed_tests,
    "failed_tests": $failed_tests,
    "success_rate": $(echo "scale=2; $passed_tests * 100 / ($passed_tests + $failed_tests)" | bc -l 2>/dev/null || echo "0")
}
EOF
}

# Main test execution
echo -e "${BLUE}🔍 Running Cross-Platform Test Suite${NC}"
echo ""

# Track overall results
TOTAL_SUITES=0
PASSED_SUITES=0
FAILED_SUITES=0

# Test 1: Core cross-platform integration tests
TOTAL_SUITES=$((TOTAL_SUITES + 1))
if run_test_suite "cross_platform_integration" "cargo test --test cross_platform_integration"; then
    PASSED_SUITES=$((PASSED_SUITES + 1))
    extract_metrics "$REPORT_DIR/cross_platform_integration_${PLATFORM}_${TIMESTAMP}.log"
else
    FAILED_SUITES=$((FAILED_SUITES + 1))
fi

# Test 2: Platform reference data validation
TOTAL_SUITES=$((TOTAL_SUITES + 1))
if run_test_suite "platform_reference_data" "cargo test --test platform_reference_data"; then
    PASSED_SUITES=$((PASSED_SUITES + 1))
    extract_metrics "$REPORT_DIR/platform_reference_data_${PLATFORM}_${TIMESTAMP}.log"
else
    FAILED_SUITES=$((FAILED_SUITES + 1))
fi

# Test 3: Cross-platform runner comprehensive tests
TOTAL_SUITES=$((TOTAL_SUITES + 1))
if run_test_suite "cross_platform_runner" "cargo test --test cross_platform_runner"; then
    PASSED_SUITES=$((PASSED_SUITES + 1))
    extract_metrics "$REPORT_DIR/cross_platform_runner_${PLATFORM}_${TIMESTAMP}.log"
else
    FAILED_SUITES=$((FAILED_SUITES + 1))
fi

# Test 4: Existing performance validation cross-platform tests
TOTAL_SUITES=$((TOTAL_SUITES + 1))
if run_test_suite "performance_validation_cross_platform" "cargo test cross_platform --lib"; then
    PASSED_SUITES=$((PASSED_SUITES + 1))
    extract_metrics "$REPORT_DIR/performance_validation_cross_platform_${PLATFORM}_${TIMESTAMP}.log"
else
    FAILED_SUITES=$((FAILED_SUITES + 1))
fi

# Test 5: SigHash consistency validation
TOTAL_SUITES=$((TOTAL_SUITES + 1))
if run_test_suite "sighash_consistency" "cargo test sighash"; then
    PASSED_SUITES=$((PASSED_SUITES + 1))
    extract_metrics "$REPORT_DIR/sighash_consistency_${PLATFORM}_${TIMESTAMP}.log"
else
    FAILED_SUITES=$((FAILED_SUITES + 1))
fi

echo ""
echo -e "${BLUE}📊 Cross-Platform Test Summary${NC}"
echo -e "${BLUE}==============================${NC}"
echo "Platform: $PLATFORM ($ARCH)"
echo "Total test suites: $TOTAL_SUITES"
echo "Passed suites: $PASSED_SUITES"
echo "Failed suites: $FAILED_SUITES"

# Calculate success rate
if [ $TOTAL_SUITES -gt 0 ]; then
    SUCCESS_RATE=$(echo "scale=1; $PASSED_SUITES * 100 / $TOTAL_SUITES" | bc -l 2>/dev/null || echo "0")
    echo "Success rate: ${SUCCESS_RATE}%"
else
    SUCCESS_RATE=0
    echo "Success rate: 0%"
fi

echo "Reports saved to: $REPORT_DIR"

# Generate overall summary report
SUMMARY_FILE="$REPORT_DIR/cross_platform_summary_${PLATFORM}_${TIMESTAMP}.json"
cat > "$SUMMARY_FILE" << EOF
{
    "platform": "$PLATFORM",
    "architecture": "$ARCH",
    "timestamp": "$TIMESTAMP",
    "test_summary": {
        "total_suites": $TOTAL_SUITES,
        "passed_suites": $PASSED_SUITES,
        "failed_suites": $FAILED_SUITES,
        "success_rate": $SUCCESS_RATE
    },
    "test_suites": [
        "cross_platform_integration",
        "platform_reference_data", 
        "cross_platform_runner",
        "performance_validation_cross_platform",
        "sighash_consistency"
    ],
    "report_directory": "$REPORT_DIR"
}
EOF

echo ""
echo -e "${BLUE}📄 Summary Report${NC}"
echo "Summary saved to: $SUMMARY_FILE"

# Final result
echo ""
if [ $FAILED_SUITES -eq 0 ]; then
    echo -e "${GREEN}✅ All cross-platform tests PASSED!${NC}"
    echo -e "${GREEN}   Platform $PLATFORM is fully compatible${NC}"
    exit 0
else
    echo -e "${RED}❌ $FAILED_SUITES test suite(s) FAILED${NC}"
    echo -e "${RED}   Cross-platform compatibility issues detected${NC}"
    echo -e "${YELLOW}   Check individual test reports for details${NC}"
    exit 1
fi
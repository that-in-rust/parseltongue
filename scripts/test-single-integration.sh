#!/bin/bash
# test-single-integration.sh
# Run a single integration test with detailed output and progress tracking

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Default test name if not provided
TEST_NAME=${1:-"test_tool2_simulation_output_parsing"}

print_status $BLUE "🔴 Testing Integration: $TEST_NAME"
print_status $YELLOW "================================"

# Check if test exists
if ! cargo test --test tool2_integration_tests --list 2>/dev/null | grep -q "$TEST_NAME"; then
    print_status $RED "❌ Test '$TEST_NAME' not found!"
    echo "Available integration tests:"
    cargo test --test tool2_integration_tests --list 2>/dev/null | grep "test_" || echo "No tests found"
    exit 1
fi

# Run the specific test with full output
print_status $BLUE "🧪 Running test: $TEST_NAME"
echo ""

if cargo test --test tool2_integration_tests --exact "$TEST_NAME" -- --nocapture; then
    print_status $GREEN "✅ Test PASSED: $TEST_NAME"
    TEST_STATUS="PASSED"
else
    print_status $RED "❌ Test FAILED: $TEST_NAME"
    TEST_STATUS="FAILED"
fi

echo ""
print_status $YELLOW "📊 Integration Test Summary:"
echo "================================"

# Count total integration tests
TOTAL_TESTS=$(cargo test --test tool2_integration_tests 2>&1 | grep -c "test .*" | head -1 || echo "0")
PASSING_TESTS=$(cargo test --test tool2_integration_tests 2>&1 | grep -c "test .* ok" || echo "0")
FAILING_TESTS=$(cargo test --test tool2_integration_tests 2>&1 | grep -c "test .* FAILED" || echo "0")

print_status $BLUE "Total Integration Tests: $TOTAL_TESTS"
print_status $GREEN "Passing: $PASSING_TESTS"
print_status $RED "Failing: $FAILING_TESTS"

if [ "$PASSING_TESTS" -gt 0 ]; then
    PROGRESS=$((PASSING_TESTS * 100 / TOTAL_TESTS))
    print_status $YELLOW "Progress: $PROGRESS% ($PASSING_TESTS/$TOTAL_TESTS passing)"
fi

echo ""
print_status $YELLOW "🏗️  Overall Project Test Status:"
echo "================================"

# Check overall project health
OVERALL_OUTPUT=$(cargo test --all 2>&1 || true)
OVERALL_RESULT=$(echo "$OVERALL_OUTPUT" | grep "test result:" | tail -1)

if [ -n "$OVERALL_RESULT" ]; then
    print_status $BLUE "Overall: $OVERALL_RESULT"
else
    print_status $RED "❌ Unable to determine overall test status"
fi

# Check for warnings
print_status $YELLOW "⚠️  Compilation Warnings Check:"
echo "================================"

if cargo check --all 2>&1 | grep -q "warning:"; then
    print_status $YELLOW "⚠️  Warnings detected:"
    cargo check --all 2>&1 | grep "warning:" | head -5
else
    print_status $GREEN "✅ No compilation warnings"
fi

# Provide next steps based on test status
echo ""
print_status $BLUE "📋 Next Steps:"
echo "================================"

if [ "$TEST_STATUS" = "PASSED" ]; then
    print_status $GREEN "✅ Test passed! Ready for next implementation."
    echo "Consider running:"
    echo "  ./scripts/quality-gate.sh"
    echo "  git add . && git commit -m \"feat: implement $TEST_NAME\""
else
    print_status $YELLOW "🔧 Test failed. Implementation needed."
    echo "Check the test requirements in:"
    echo "  - .steeringDocs/S07-TDD-Workflow-Structure.md"
    echo "  - .prdArchDocs/P07-Phase3A-Integration-Resolution.md"
    echo ""
    echo "To run with backtrace:"
    echo "  RUST_BACKTRACE=1 cargo test --test tool2_integration_tests --exact \"$TEST_NAME\""
fi

echo ""
print_status $BLUE "📈 Progress Monitoring:"
echo "================================"
echo "Run progress monitor: ./scripts/progress-monitor.sh"
echo "Run quality gates: ./scripts/quality-gate.sh"
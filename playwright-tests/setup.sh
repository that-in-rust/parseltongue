#!/bin/bash

# Playwright Test Environment Setup Script
# Sets up the complete testing environment for Parseltongue WASM visualizations

set -e  # Exit on any error

echo "🎭 Setting up Playwright Test Environment for Parseltongue"
echo "=============================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    print_error "Please run this script from the playwright-tests directory"
    exit 1
fi

print_status "Setting up Playwright test environment..."

# Check Node.js version
print_status "Checking Node.js version..."
if ! command -v node &> /dev/null; then
    print_error "Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi

NODE_VERSION=$(node --version | cut -d'v' -f2)
NODE_MAJOR=$(echo $NODE_VERSION | cut -d'.' -f1)

if [ "$NODE_MAJOR" -lt 18 ]; then
    print_error "Node.js 18+ is required. Found version: $NODE_VERSION"
    exit 1
fi

print_success "Node.js $NODE_VERSION is compatible"

# Install npm dependencies
print_status "Installing npm dependencies..."
npm install

# Install Playwright browsers
print_status "Installing Playwright browsers..."
npm run install

# Create necessary directories
print_status "Creating test directories..."
mkdir -p test-output
mkdir -p test-results
mkdir -p test-data

# Check if debug_output exists in parent directory
if [ ! -d "../debug_output" ]; then
    print_warning "No debug_output directory found in parent folder"
    print_status "Generating test HTML files..."

    # Build the project first
    print_status "Building Parseltongue project..."
    cd ..
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cannot find Cargo.toml. Are you in the right project directory?"
        exit 1
    fi

    cargo build --release

    # Generate test files
    if [ -d "src" ]; then
        print_status "Generating test HTML from src/ directory..."
        ./target/release/parseltong ingest src/
    elif [ -d "examples" ]; then
        print_status "Generating test HTML from examples/ directory..."
        ./target/release/parseltong ingest examples/
    else
        print_warning "No source directories found. Creating minimal test files..."
        mkdir -p test-data/test-rs
        echo 'fn test_function() { println!("test"); }' > test-data/test-rs/main.rs
        ./target/release/parseltong ingest test-data/
    fi

    cd playwright-tests
else
    print_success "Found debug_output directory"
fi

# Copy existing HTML files to test-output
print_status "Setting up test files..."
if [ -d "../debug_output" ]; then
    cp ../debug_output/*.html test-output/ 2>/dev/null || true
    FILE_COUNT=$(ls test-output/*.html 2>/dev/null | wc -l)
    if [ "$FILE_COUNT" -gt 0 ]; then
        print_success "Copied $FILE_COUNT HTML files to test-output/"
    else
        print_warning "No HTML files found in debug_output"
    fi
fi

# Create minimal test file if none exist
if [ ! -f "test-output/minimal-test.html" ]; then
    print_status "Creating minimal test HTML file..."
    cat > test-output/minimal-test.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test Visualization</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .controls { margin: 20px 0; }
        canvas { border: 1px solid #ccc; }
        .stats { margin: 10px 0; }
    </style>
</head>
<body>
    <h1>Parseltongue WASM Visualization</h1>
    <div class="controls">
        <button onclick="zoomIn()">🔍 Zoom In</button>
        <button onclick="zoomOut()">🔍 Zoom Out</button>
        <button onclick="resetZoom()">🔄 Reset</button>
        <select id="layoutSelect" onchange="changeLayout()">
            <option value="breadthfirst" selected>Breadth-First</option>
            <option value="forcedirected">Force-Directed</option>
            <option value="hierarchical">Hierarchical</option>
            <option value="circular">Circular</option>
        </select>
    </div>
    <div class="stats">
        <span id="nodeCount">Nodes: 5</span> |
        <span id="edgeCount">Edges: 4</span> |
        <span id="renderTime">Render: 10ms</span>
    </div>
    <canvas id="canvas" width="800" height="600"></canvas>
    <script>
        let graphData = {
            "nodes": [
                {"id": "1", "name": "test1", "node_type": "function"},
                {"id": "2", "name": "test2", "node_type": "struct"},
                {"id": "3", "name": "test3", "node_type": "trait"},
                {"id": "4", "name": "test4", "node_type": "impl"},
                {"id": "5", "name": "test5", "node_type": "function"}
            ],
            "edges": [
                {"source": "1", "target": "2"},
                {"source": "2", "target": "3"},
                {"source": "3", "target": "4"},
                {"source": "4", "target": "5"}
            ]
        };

        function renderGraph() {
            console.log('Rendering graph with', graphData.nodes.length, 'nodes');
            const canvas = document.getElementById('canvas');
            const ctx = canvas.getContext('2d');

            // Simple test rendering
            ctx.fillStyle = '#f0f0f0';
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            ctx.fillStyle = '#667eea';
            ctx.font = '16px Arial';
            ctx.fillText('Test Visualization - ' + graphData.nodes.length + ' nodes', 50, 50);
        }

        function zoomIn() { console.log('Zoom in'); renderGraph(); }
        function zoomOut() { console.log('Zoom out'); renderGraph(); }
        function resetZoom() { console.log('Reset zoom'); renderGraph(); }
        function changeLayout() { console.log('Change layout'); renderGraph(); }

        // Initialize on load
        window.addEventListener('load', () => {
            renderGraph();
            document.getElementById('renderTime').textContent = 'Render: 15ms';
        });
    </script>
</body>
</html>
EOF
    print_success "Created minimal test HTML file"
fi

# Test the setup
print_status "Running a quick test to verify setup..."
if command -v npx &> /dev/null; then
    npx playwright test --list > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        print_success "Playwright setup verified successfully"
    else
        print_error "Playwright setup verification failed"
        exit 1
    fi
else
    print_error "npx command not found. Please ensure Node.js is properly installed."
    exit 1
fi

echo ""
print_success "🎉 Playwright test environment setup complete!"
echo ""
echo "Next steps:"
echo "1. Run tests: npm test"
echo "2. Run tests with UI: npm run test:ui"
echo "3. Run tests in browser: npm run test:headed"
echo "4. View test reports: npm run report"
echo ""
echo "Test files location: test-output/"
echo "Results location: test-results/"
echo ""
echo "🚀 Happy testing!"
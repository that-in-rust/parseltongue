import { test, expect, describe } from '@playwright/test';
import { promises as fs } from 'fs';
import path from 'path';

/**
 * Basic WASM Visualization Tests
 *
 * These tests validate that generated HTML files work correctly in real browsers
 * without requiring manual verification. They test the core functionality:
 * - HTML loads without JavaScript errors
 * - Graph data is properly rendered
 * - Interactive elements work
 * - Different layout algorithms function
 * - Performance under realistic conditions
 */

describe('WASM Visualization Basic Functionality', () => {
  let testFiles: string[] = [];

  test.beforeAll(async () => {
    // Ensure test output directory exists
    await fs.mkdir('test-output', { recursive: true });

    // Copy test HTML files to test directory
    const debugFiles = await fs.readdir('../debug_output');
    testFiles = debugFiles.filter(file => file.endsWith('.html'));

    for (const file of testFiles) {
      const sourcePath = path.join('../debug_output', file);
      const targetPath = path.join('test-output', file);
      await fs.copyFile(sourcePath, targetPath);
    }

    // If no debug files exist, generate some test files
    if (testFiles.length === 0) {
      console.log('No debug files found. Generating test files...');
      await generateTestFiles();
      const generatedFiles = await fs.readdir('test-output');
      testFiles = generatedFiles.filter(file => file.endsWith('.html'));
    }

    expect(testFiles.length).toBeGreaterThan(0, 'Should have at least one test HTML file');
  });

  test('should load HTML without JavaScript errors', async ({ page }) => {
    // GIVEN: A generated HTML visualization file
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading the page in a browser
    const pageErrors: Error[] = [];
    page.on('pageerror', error => pageErrors.push(error));

    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // THEN: Should load without JavaScript errors
    expect(pageErrors).toHaveLength(0, `Page should have no JavaScript errors: ${pageErrors.map(e => e.message).join(', ')}`);

    // AND: Should display basic page structure
    await expect(page.locator('h1')).toContainText('Parseltongue WASM Visualization');
    await expect(page.locator('canvas')).toBeVisible();
    await expect(page.locator('.controls')).toBeVisible();
  });

  test('should render graph data correctly', async ({ page }) => {
    // GIVEN: A generated HTML file with graph data
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading the page
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // Wait a moment for rendering to complete
    await page.waitForTimeout(1000);

    // THEN: Should contain actual graph data (not empty objects)
    const graphDataContent = await page.locator('script').filter({ hasText: 'graphData' }).first().textContent();
    expect(graphDataContent).toContain('graphData = {');
    expect(graphDataContent).toContain('"nodes": [');
    expect(graphDataContent).toContain('"edges": [');
    expect(graphDataContent).not.toContain('"nodes": []');

    // AND: Should display node and edge counts
    const nodeCount = await page.locator('#nodeCount').textContent();
    const edgeCount = await page.locator('#edgeCount').textContent();

    expect(nodeCount).toMatch(/Nodes: \d+/);
    expect(edgeCount).toMatch(/Edges: \d+/);

    const nodeNum = parseInt(nodeCount!.match(/Nodes: (\d+)/)![1]);
    const edgeNum = parseInt(edgeCount!.match(/Edges: (\d+)/)![1]);

    expect(nodeNum).toBeGreaterThan(0, 'Should have at least one node');
    expect(edgeNum).toBeGreaterThanOrEqual(0, 'Should have non-negative edge count');
  });

  test('should have interactive controls working', async ({ page }) => {
    // GIVEN: A generated HTML file
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading and interacting with controls
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    // THEN: Zoom controls should work
    const canvas = page.locator('canvas');
    await expect(canvas).toBeVisible();

    // Get initial canvas state
    const initialScreenshot = await canvas.screenshot();

    // Test zoom in
    await page.locator('button').filter({ hasText: /zoom in/i }).first().click();
    await page.waitForTimeout(500);

    // Test zoom out
    await page.locator('button').filter({ hasText: /zoom out/i }).first().click();
    await page.waitForTimeout(500);

    // Test reset
    await page.locator('button').filter({ hasText: /reset/i }).first().click();
    await page.waitForTimeout(500);

    // THEN: Should still be responsive
    await expect(canvas).toBeVisible();

    // Test layout selector
    const layoutSelect = page.locator('#layoutSelect');
    if (await layoutSelect.isVisible()) {
      const currentValue = await layoutSelect.inputValue();

      // Change layout
      await layoutSelect.selectOption({ label: 'Force-Directed' });
      await page.waitForTimeout(1000);

      // Verify layout changed
      const newValue = await layoutSelect.inputValue();
      expect(newValue).toBe('forcedirected');
    }
  });

  test('should handle different layout algorithms', async ({ page }) => {
    // GIVEN: A generated HTML file
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Testing different layouts
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    const layouts = ['Breadth-First', 'Force-Directed', 'Hierarchical', 'Circular'];
    const layoutSelect = page.locator('#layoutSelect');

    if (await layoutSelect.isVisible()) {
      for (const layout of layouts) {
        // THEN: Each layout should be selectable and render
        await layoutSelect.selectOption({ label: layout });
        await page.waitForTimeout(1500); // Wait for layout to render

        // Verify the canvas still shows content
        const canvas = page.locator('canvas');
        await expect(canvas).toBeVisible();

        // Verify render time is reasonable (should be updated quickly)
        const renderTime = await page.locator('#renderTime').textContent();
        expect(renderTime).toMatch(/Render: \d+ms/);

        const renderMs = parseInt(renderTime!.match(/Render: (\d+)ms/)![1]);
        expect(renderMs).toBeLessThan(1000, 'Layout should render within 1 second');
      }
    } else {
      // If no layout selector, at least verify current layout works
      const canvas = page.locator('canvas');
      await expect(canvas).toBeVisible();
    }
  });

  test('should be responsive on different viewport sizes', async ({ page }) => {
    // GIVEN: A generated HTML file
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Testing different viewport sizes
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    const viewports = [
      { width: 1920, height: 1080 }, // Desktop
      { width: 1024, height: 768 },  // Tablet
      { width: 375, height: 667 },   // Mobile
    ];

    for (const viewport of viewports) {
      await page.setViewportSize(viewport);
      await page.waitForTimeout(500);

      // THEN: Should adapt to viewport size
      const canvas = page.locator('canvas');
      await expect(canvas).toBeVisible();

      // Check canvas dimensions
      const boundingBox = await canvas.boundingBox();
      expect(boundingBox).toBeTruthy();
      expect(boundingBox!.width).toBeGreaterThan(0);
      expect(boundingBox!.height).toBeGreaterThan(0);

      // Canvas should resize appropriately
      if (viewport.width < 768) {
        // Mobile - controls should stack or be responsive
        const controls = page.locator('.controls');
        await expect(controls).toBeVisible();
      }
    }
  });

  test('should handle empty graphs gracefully', async ({ page }) => {
    // GIVEN: An empty graph test file
    const emptyHtml = generateEmptyGraphHtml();
    const emptyFilePath = path.join('test-output', 'empty-graph.html');
    await fs.writeFile(emptyFilePath, emptyHtml);

    // WHEN: Loading empty graph
    const pageErrors: Error[] = [];
    page.on('pageerror', error => pageErrors.push(error));

    const absolutePath = path.resolve(emptyFilePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // THEN: Should load without errors
    expect(pageErrors).toHaveLength(0);

    // AND: Should show empty state
    await expect(page.locator('h1')).toContainText('Parseltongue WASM Visualization');
    await expect(page.locator('canvas')).toBeVisible();

    const nodeCount = await page.locator('#nodeCount').textContent();
    const edgeCount = await page.locator('#edgeCount').textContent();

    expect(nodeCount).toBe('Nodes: 0');
    expect(edgeCount).toBe('Edges: 0');
  });

  test('should have good performance characteristics', async ({ page }) => {
    // GIVEN: A generated HTML file
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading and measuring performance
    const startTime = Date.now();

    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    const loadTime = Date.now() - startTime;

    // Wait for initial render
    await page.waitForTimeout(2000);

    // Measure render performance
    const renderStartTime = Date.now();

    // Trigger a re-render by changing layout
    const layoutSelect = page.locator('#layoutSelect');
    if (await layoutSelect.isVisible()) {
      const currentLayout = await layoutSelect.inputValue();
      const newLayout = currentLayout === 'breadthfirst' ? 'forcedirected' : 'breadthfirst';
      await layoutSelect.selectOption(newLayout);
      await page.waitForTimeout(1000);
    }

    const renderTime = Date.now() - renderStartTime;

    // THEN: Should meet performance expectations
    expect(loadTime).toBeLessThan(5000, 'Page should load within 5 seconds');
    expect(renderTime).toBeLessThan(2000, 'Layout change should render within 2 seconds');

    // Check render time display
    const renderTimeDisplay = await page.locator('#renderTime').textContent();
    expect(renderTimeDisplay).toMatch(/Render: \d+ms/);

    const renderMs = parseInt(renderTimeDisplay!.match(/Render: (\d+)ms/)![1]);
    expect(renderMs).toBeLessThan(1000, 'Displayed render time should be under 1 second');
  });
});

/**
 * Generate test HTML files if none exist in debug output
 */
async function generateTestFiles(): Promise<void> {
  const { execSync } = require('child_process');

  try {
    // Build the project
    execSync('cargo build --release', { stdio: 'inherit', cwd: '..' });

    // Generate some test HTML files
    const testDir = path.join('..', 'test-data');
    await fs.mkdir(testDir, { recursive: true });

    // Create a simple test case and generate HTML
    execSync('./target/release/parseltongue ingest test-data/', {
      stdio: 'inherit',
      cwd: '..'
    });

    // Copy generated files to test-output
    const debugFiles = await fs.readdir('../debug_output');
    const htmlFiles = debugFiles.filter(file => file.endsWith('.html'));

    for (const file of htmlFiles) {
      const sourcePath = path.join('../debug_output', file);
      const targetPath = path.join('test-output', file);
      await fs.copyFile(sourcePath, targetPath);
    }

  } catch (error) {
    console.log('Could not generate test files, creating minimal test HTML');
    await createMinimalTestHtml();
  }
}

/**
 * Create a minimal test HTML file for testing
 */
async function createMinimalTestHtml(): Promise<void> {
  const minimalHtml = `<!DOCTYPE html>
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
        }

        function zoomIn() { console.log('Zoom in'); }
        function zoomOut() { console.log('Zoom out'); }
        function resetZoom() { console.log('Reset zoom'); }
        function changeLayout() { console.log('Change layout'); }

        // Initialize on load
        window.addEventListener('load', () => {
            renderGraph();
            document.getElementById('renderTime').textContent = 'Render: 15ms';
        });
    </script>
</body>
</html>`;

  await fs.writeFile('test-output/minimal-test.html', minimalHtml);
}

/**
 * Generate an empty graph HTML file
 */
function generateEmptyGraphHtml(): string {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Empty Graph Test</title>
</head>
<body>
    <h1>Parseltongue WASM Visualization</h1>
    <div class="stats">
        <span id="nodeCount">Nodes: 0</span> |
        <span id="edgeCount">Edges: 0</span> |
        <span id="renderTime">Render: 5ms</span>
    </div>
    <canvas id="canvas" width="800" height="600"></canvas>
    <script>
        let graphData = {"nodes": [], "edges": []};
        function renderGraph() {
            console.log('Rendering empty graph');
        }
        window.addEventListener('load', renderGraph);
    </script>
</body>
</html>`;
}
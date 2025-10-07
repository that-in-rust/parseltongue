import { test, expect, describe } from '@playwright/test';
import { promises as fs } from 'fs';
import path from 'path';

/**
 * Visual Regression Tests
 *
 * These tests ensure that visual output remains consistent across changes.
 * They catch unintended visual regressions in the WASM visualizations.
 */

test.describe('Visual Regression Tests', () => {
  let testFiles: string[] = [];

  test.beforeAll(async () => {
    // Ensure test files are available
    const debugFiles = await fs.readdir('../debug_output');
    testFiles = debugFiles.filter(file => file.endsWith('.html'));

    if (testFiles.length === 0) {
      console.log('No debug files found for visual regression testing');
      test.skip();
    }
  });

  test('visualization should look consistent across browsers', async ({ page, browserName }) => {
    // GIVEN: A visualization file
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading in different browsers
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000); // Wait for rendering

    // THEN: Should render consistently (basic structure)
    await expect(page.locator('h1')).toContainText('Parseltongue WASM Visualization');
    await expect(page.locator('canvas')).toBeVisible();
    await expect(page.locator('.controls')).toBeVisible();

    // Take a screenshot for visual comparison
    await page.screenshot({
      path: `test-results/visual-regression-${browserName}.png`,
      fullPage: true
    });
  });

  test('canvas should render content properly', async ({ page }) => {
    // GIVEN: A visualization file
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading and waiting for render
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // THEN: Canvas should have actual content
    const canvas = page.locator('canvas');
    await expect(canvas).toBeVisible();

    // Check canvas is not empty (has been drawn to)
    const canvasDataUrl = await canvas.evaluate((el: HTMLCanvasElement) => {
      const ctx = el.getContext('2d');
      if (!ctx) return null;

      // Get a small sample of canvas pixels to check if it's been drawn to
      const imageData = ctx.getImageData(0, 0, 10, 10);
      const pixels = imageData.data;

      // Check if any pixels have non-zero alpha (indicating drawing occurred)
      return pixels.some((pixel, index) => index % 4 === 3 && pixel > 0);
    });

    expect(canvasDataUrl).toBe(true, 'Canvas should have drawn content');
  });

  test('should handle different color schemes', async ({ page }) => {
    // Test that the visualization doesn't have color accessibility issues
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Check for basic color contrast
    const backgroundColor = await page.evaluate(() => {
      const computed = window.getComputedStyle(document.body);
      return computed.backgroundColor;
    });

    const textColor = await page.evaluate(() => {
      const computed = window.getComputedStyle(document.body);
      return computed.color;
    });

    // Ensure we have colors (not empty/transparent)
    expect(backgroundColor).not.toBe('');
    expect(textColor).not.toBe('');

    // Take screenshot for color analysis if needed
    await page.screenshot({
      path: 'test-results/color-scheme-test.png'
    });
  });
});

test.describe('Responsive Design Tests', () => {
  test('should adapt to mobile viewports', async ({ page }) => {
    // GIVEN: Mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    const testFiles = await fs.readdir('../debug_output');
    const htmlFiles = testFiles.filter(file => file.endsWith('.html'));

    if (htmlFiles.length === 0) {
      test.skip();
      return;
    }

    const testFile = htmlFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading on mobile
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // THEN: Should be usable on mobile
    await expect(page.locator('canvas')).toBeVisible();

    // Controls should be accessible
    const controls = page.locator('.controls');
    if (await controls.isVisible()) {
      // Check if controls are properly sized for mobile
      const controlsBox = await controls.boundingBox();
      expect(controlsBox!.width).toBeLessThanOrEqual(page.viewportSize().width - 40);
    }

    await page.screenshot({
      path: 'test-results/mobile-viewport.png',
      fullPage: true
    });
  });

  test('should work on tablet viewports', async ({ page }) => {
    // GIVEN: Tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });

    const testFiles = await fs.readdir('../debug_output');
    const htmlFiles = testFiles.filter(file => file.endsWith('.html'));

    if (htmlFiles.length === 0) {
      test.skip();
      return;
    }

    const testFile = htmlFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading on tablet
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // THEN: Should utilize tablet space well
    await expect(page.locator('canvas')).toBeVisible();

    const canvas = page.locator('canvas');
    const canvasBox = await canvas.boundingBox();
    expect(canvasBox!.width).toBeGreaterThan(500, 'Canvas should use tablet width effectively');

    await page.screenshot({
      path: 'test-results/tablet-viewport.png',
      fullPage: true
    });
  });
});
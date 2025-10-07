import { test, expect, describe } from '@playwright/test';
import { promises as fs } from 'fs';
import path from 'path';

/**
 * Accessibility Tests
 *
 * These tests ensure that the WASM visualizations are accessible to users
 * with disabilities, following WCAG guidelines and best practices.
 */

describe('Accessibility Tests', () => {
  let testFiles: string[] = [];

  test.beforeAll(async () => {
    const debugFiles = await fs.readdir('../debug_output');
    testFiles = debugFiles.filter(file => file.endsWith('.html'));

    if (testFiles.length === 0) {
      console.log('No debug files found for accessibility testing');
      test.skip();
    }
  });

  test('should have proper semantic HTML structure', async ({ page }) => {
    // GIVEN: A generated HTML visualization
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading the page
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // THEN: Should have proper semantic structure
    await expect(page.locator('h1')).toBeVisible();
    await expect(page.locator('main, [role="main"]')).toHaveCount(1);
    await expect(page.locator('nav, [role="navigation"]')).toHaveCount({ gte: 0 }); // navigation is optional
    await expect(page.locator('canvas')).toHaveAttribute('aria-label');
  });

  test('should have accessible controls', async ({ page }) => {
    // GIVEN: A visualization with interactive controls
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading the page
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // THEN: Controls should be accessible
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();

    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);

      // Each button should have accessible text or aria-label
      const buttonText = await button.textContent();
      const ariaLabel = await button.getAttribute('aria-label');

      expect(buttonText || ariaLabel).toBeTruthy();

      // Buttons should be keyboard accessible
      await button.focus();
      expect(await button.isFocused()).toBeTruthy();
    }

    // Test select elements
    const selects = page.locator('select');
    const selectCount = await selects.count();

    for (let i = 0; i < selectCount; i++) {
      const select = selects.nth(i);
      await select.focus();
      expect(await select.isFocused()).toBeTruthy();

      // Should have accessible label
      const ariaLabel = await select.getAttribute('aria-label');
      const labelId = await select.getAttribute('aria-labelledby');

      expect(ariaLabel || labelId || await select.getAttribute('title')).toBeTruthy();
    }
  });

  test('should have proper color contrast', async ({ page }) => {
    // GIVEN: A loaded visualization
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading the page
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // THEN: Should have readable color contrast
    // This is a basic check - full color contrast testing would require more sophisticated tools
    const bodyStyles = await page.evaluate(() => {
      const computed = window.getComputedStyle(document.body);
      return {
        backgroundColor: computed.backgroundColor,
        color: computed.color,
        fontSize: computed.fontSize
      };
    });

    // Check that colors are not transparent or very light
    expect(bodyStyles.backgroundColor).not.toBe('rgba(0, 0, 0, 0)');
    expect(bodyStyles.color).not.toBe('rgba(0, 0, 0, 0)');

    // Check font size is reasonable
    const fontSize = parseFloat(bodyStyles.fontSize);
    expect(fontSize).toBeGreaterThanOrEqual(12); // Minimum readable font size
  });

  test('should be keyboard navigable', async ({ page }) => {
    // GIVEN: A loaded visualization
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Using keyboard navigation
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // Test Tab navigation
    await page.keyboard.press('Tab');

    // Should focus on first interactive element
    const focusedElement = await page.locator(':focus');
    expect(await focusedElement.count()).toBeGreaterThan(0);

    // Continue tabbing through controls
    const interactiveElements = await page.locator('button, select, input, a, [tabindex]:not([tabindex="-1"])').count();

    let tabCount = 0;
    let previousFocused = '';

    while (tabCount < interactiveElements && tabCount < 20) { // Prevent infinite loop
      await page.keyboard.press('Tab');
      await page.waitForTimeout(100);

      const currentFocused = await page.evaluate(() => {
        const focused = document.activeElement;
        return focused ? focused.tagName + (focused.id ? '#' + focused.id : '') : '';
      });

      if (currentFocused === previousFocused) {
        break; // We've cycled back
      }

      previousFocused = currentFocused;
      tabCount++;
    }

    // Should be able to navigate through multiple elements
    expect(tabCount).toBeGreaterThan(1);
  });

  test('should have proper ARIA labels', async ({ page }) => {
    // GIVEN: A loaded visualization
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading the page
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // THEN: Canvas should have proper labeling
    const canvas = page.locator('canvas');

    // Canvas should have aria-label, aria-labelledby, or title
    const hasAriaLabel = await canvas.getAttribute('aria-label');
    const hasAriaLabelledBy = await canvas.getAttribute('aria-labelledby');
    const hasTitle = await canvas.getAttribute('title');

    expect(hasAriaLabel || hasAriaLabelledBy || hasTitle).toBeTruthy();

    // Check for live regions for dynamic content updates
    const liveRegions = page.locator('[aria-live], [aria-atomic]');
    const hasLiveRegions = await liveRegions.count();

    // Stats should be announced when they change
    const nodeCount = page.locator('#nodeCount');
    if (await nodeCount.isVisible()) {
      // Ideally, this should have aria-live or be in a live region
      const nodeCountAria = await nodeCount.getAttribute('aria-live');
      const parentLiveRegion = await nodeCount.locator('xpath=ancestor::*[@aria-live]').count();

      expect(nodeCountAria || parentLiveRegion).toBeTruthy();
    }
  });

  test('should have proper focus management', async ({ page }) => {
    // GIVEN: A loaded visualization
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading and interacting with the page
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // Check initial focus
    const initialFocus = await page.evaluate(() => document.activeElement?.tagName);
    expect(initialFocus).toBe('BODY'); // Should start on body

    // Test focus on controls
    const firstButton = page.locator('button').first();
    if (await firstButton.count() > 0) {
      await firstButton.focus();
      expect(await firstButton.isFocused()).toBeTruthy();
    }

    // Test focus trap prevention (focus shouldn't be trapped in bad places)
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Shift+Tab');
    await page.keyboard.press('Shift+Tab');

    // Should still be able to navigate freely
    const currentFocus = await page.evaluate(() => document.activeElement?.tagName);
    expect(['BUTTON', 'SELECT', 'BODY', 'CANVAS']).toContain(currentFocus);
  });

  test('should have sufficient touch targets', async ({ page }) => {
    // GIVEN: Mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading on mobile
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // THEN: Touch targets should be sufficiently large
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();

    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);
      const boundingBox = await button.boundingBox();

      if (boundingBox) {
        // Minimum touch target size is 44x44 points (approximately 44x44 CSS pixels)
        expect(boundingBox.width).toBeGreaterThanOrEqual(44);
        expect(boundingBox.height).toBeGreaterThanOrEqual(44);
      }
    }

    // Check spacing between interactive elements
    const controls = page.locator('.controls button, .controls select');
    const controlCount = await controls.count();

    for (let i = 0; i < controlCount - 1; i++) {
      const control1 = controls.nth(i);
      const control2 = controls.nth(i + 1);

      const box1 = await control1.boundingBox();
      const box2 = await control2.boundingBox();

      if (box1 && box2) {
        // Check horizontal spacing
        const horizontalDistance = Math.abs(box1.x + box1.width - box2.x);
        expect(horizontalDistance).toBeGreaterThanOrEqual(8); // Minimum 8px spacing
      }
    }
  });

  test('should support screen readers', async ({ page }) => {
    // GIVEN: A loaded visualization
    const testFile = testFiles[0];
    const filePath = path.join('test-output', testFile);

    // WHEN: Loading with screen reader considerations
    const absolutePath = path.resolve(filePath);
    await page.goto(`file://${absolutePath}`);
    await page.waitForLoadState('networkidle');

    // THEN: Should have proper structure for screen readers
    const documentStructure = await page.evaluate(() => {
      const structure = {
        hasTitle: document.title !== '',
        hasHeading: !!document.querySelector('h1, h2, h3, h4, h5, h6'),
        hasMain: !!document.querySelector('main, [role="main"]'),
        hasLandmarks: !!document.querySelector('header, footer, nav, section, [role]'),
        hasLanguage: !!document.documentElement.getAttribute('lang')
      };

      // Check for skip links
      const skipLinks = document.querySelectorAll('a[href^="#"]');
      structure.hasSkipLinks = skipLinks.length > 0;

      return structure;
    });

    expect(documentStructure.hasTitle).toBeTruthy();
    expect(documentStructure.hasHeading).toBeTruthy();
    expect(documentStructure.hasLanguage).toBeTruthy();

    // Check for alternative text on meaningful images (if any)
    const images = page.locator('img');
    const imageCount = await images.count();

    for (let i = 0; i < imageCount; i++) {
      const image = images.nth(i);
      const alt = await image.getAttribute('alt');
      const role = await image.getAttribute('role');

      // Decorative images should have empty alt or role="none"
      // Meaningful images should have descriptive alt
      if (role !== 'none') {
        expect(alt !== null).toBeTruthy();
      }
    }
  });
});
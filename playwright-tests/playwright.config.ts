import { defineConfig, devices } from '@playwright/test';

/**
 * Playwright Configuration for Parseltongue WASM Visualization Tests
 *
 * Following industry best practices for browser automation testing:
 * - Cross-browser testing (Chromium, Firefox, WebKit)
 * - Mobile viewport testing
 * - Headless and headed modes
 * - Screenshot and video recording for debugging
 * - Reporting and trace files for CI/CD integration
 */
export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [
    ['html'],
    ['json', { outputFile: 'test-results/results.json' }],
    ['junit', { outputFile: 'test-results/junit.xml' }]
  ],
  use: {
    baseURL: 'file://./test-output',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },

  projects: [
    // Desktop browsers
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },

    // Mobile viewports
    {
      name: 'Mobile Chrome',
      use: { ...devices['Pixel 5'] },
    },
    {
      name: 'Mobile Safari',
      use: { ...devices['iPhone 12'] },
    },

    // Tablet viewports
    {
      name: 'Tablet',
      use: { ...devices['iPad Pro'] },
    },
  ],

  // Note: webServer disabled for file:// protocol testing
  // webServer: {
  //   command: 'python -m http.server 8080 --directory test-output',
  //   port: 8080,
  //   reuseExistingServer: !process.env.CI,
  // },
});
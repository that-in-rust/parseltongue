# Parseltongue Playwright Tests

Browser automation testing for WASM visualization HTML output.

## Purpose

Validates that generated HTML visualizations work correctly across browsers without manual verification.

## Test Coverage

### Files Tested
- `basic-visualization.spec.ts` - Core functionality (JavaScript errors, rendering, controls)
- `visual-regression.spec.ts` - Visual consistency across browsers
- `accessibility.spec.ts` - WCAG compliance and keyboard navigation

### Browsers
- Chromium (Chrome/Edge)
- Firefox
- WebKit (Safari)
- Mobile (Pixel 5, iPhone 12)
- Tablet (iPad Pro)

## Setup

```bash
npm install
npm run install  # Install Playwright browsers
```

## Running Tests

```bash
npm test                    # Run all tests
npm run test:headed         # With browser UI
npm run test:debug          # Step-by-step debugging
npm run test:ui            # Visual test runner
npm run report             # View HTML report
```

### Specific Tests
```bash
npx playwright test basic-visualization
npx playwright test accessibility
npx playwright test --project chromium
```

## Reports

- Screenshots/videos on failures
- HTML report: `playwright-report/index.html`
- JUnit XML: `test-results/junit.xml` (CI/CD)

## Debugging

### Common Issues

**No test files**: Generate HTML first
```bash
cd .. && cargo build --release && ./target/release/parseltongue ingest src/
```

**Timeout errors**: Increase timeout or use headed mode
```bash
npx playwright test --timeout 60000
npm run test:headed
```

**Canvas not rendering**: Check WASM initialization timing

### Debug Commands
```bash
npm run test:debug  # Step-by-step
npm run test:ui     # Visual runner
```

## Test Data

Tests require HTML files in `test-output/`:
- Generated from `../debug_output/visualization.html`
- Auto-copied from debug directory
- Minimal test files created if none exist

## Notes

- Tests validate HTML output from Rust WASM visualization generation
- No web server required (uses `file://` protocol)
- Screenshots/videos captured on failures
- CI/CD integration via JUnit XML output
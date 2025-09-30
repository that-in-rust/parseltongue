# Build Requirements and Setup Instructions

## Overview

Parseltongue is a discovery-first architectural intelligence tool for Rust codebases. This document provides comprehensive build requirements and setup instructions for developers.

## System Requirements

### Minimum Requirements
- **Rust**: 1.70.0 or later (2021 edition)
- **Operating System**: Linux, macOS, or Windows
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Disk Space**: 2GB free space for build artifacts

### Recommended Development Environment
- **Rust**: Latest stable version
- **IDE**: VS Code with rust-analyzer extension
- **Git**: 2.30 or later
- **Terminal**: Modern terminal with Unicode support

## Installation

### 1. Install Rust

If you don't have Rust installed, install it via rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Verify Installation

```bash
rustc --version
cargo --version
```

### 3. Install Required Components

```bash
rustup component add rustfmt clippy
```

## Building the Project

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd parseltongue

# Build the project
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

### Development Build

```bash
# Clean build from scratch
cargo clean
cargo build

# Build with all features
cargo build --all-features

# Build in release mode
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests only
cargo test --test integration_tests
```

## Code Quality Checks

### Formatting

```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all
```

### Linting

```bash
# Run clippy with all features
cargo clippy --all-targets --all-features

# Run clippy with strict warnings
cargo clippy --all-targets --all-features -- -D warnings

# Run clippy for specific target
cargo clippy --bin parseltongue_20250924231324
```

### Documentation

```bash
# Build documentation
cargo doc --all-features

# Build and open documentation
cargo doc --all-features --open

# Test documentation examples
cargo test --doc
```

## Feature Flags

The project supports the following feature flags:

- **default**: Standard functionality (no additional features)
- **experimental**: Experimental features (disabled by default)

### Building with Features

```bash
# Build with experimental features
cargo build --features experimental

# Build with all features
cargo build --all-features

# Test with specific features
cargo test --features experimental
```

## Performance Validation

### Benchmarks

```bash
# Run performance benchmarks (if available)
cargo bench

# Run specific benchmark
cargo bench benchmark_name
```

### Memory Usage

```bash
# Check for memory leaks with valgrind (Linux)
valgrind --tool=memcheck --leak-check=full cargo test

# Profile memory usage
cargo build --release
valgrind --tool=massif target/release/parseltongue_20250924231324
```

## Troubleshooting

### Common Build Issues

#### 1. Compilation Errors

```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update
```

#### 2. Test Failures

```bash
# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run tests in single thread
cargo test -- --test-threads=1
```

#### 3. Clippy Warnings

```bash
# Fix automatically fixable issues
cargo clippy --fix --all-targets --all-features

# Allow specific warnings (temporary)
cargo clippy --all-targets --all-features -- -A clippy::warning_name
```

### Platform-Specific Issues

#### Windows

- Ensure you have the Microsoft C++ Build Tools installed
- Use PowerShell or Command Prompt, not Git Bash for some operations

#### macOS

- Install Xcode Command Line Tools: `xcode-select --install`
- Ensure you have the latest macOS SDK

#### Linux

- Install build essentials: `sudo apt-get install build-essential`
- Ensure you have pkg-config: `sudo apt-get install pkg-config`

## Continuous Integration

The project uses GitHub Actions for CI/CD with comprehensive quality checks. The CI pipeline is designed to catch issues early and ensure code quality across all platforms.

### CI Pipeline Overview

The CI workflow (`.github/workflows/ci.yml`) includes multiple jobs:

#### 1. **Test Suite Job**
- **Code Formatting Check**: Ensures consistent code style with `cargo fmt`
- **Clippy Linting**: Catches common mistakes and suggests improvements
- **Build Verification**: Confirms the project builds successfully with all features
- **Test Execution**: Runs the complete test suite including unit and integration tests
- **Documentation Tests**: Validates all code examples in documentation

#### 2. **Cross-Platform Build Matrix**
- **Multi-OS Testing**: Validates builds on Linux, macOS, and Windows
- **Rust Version Compatibility**: Tests with stable Rust toolchain
- **Feature Compatibility**: Ensures all features work across platforms

#### 3. **Security Audit Job**
- **Dependency Scanning**: Checks for known vulnerabilities using `cargo audit`
- **Security Best Practices**: Validates secure coding patterns

### CI Configuration Details

```yaml
# Triggers
on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

# Key Features
- Caching for faster builds
- Parallel execution across platforms
- Comprehensive error reporting
- Security vulnerability scanning
```

### Quality Gates

The CI enforces these quality standards:

1. **Code Style**: All code must pass `cargo fmt --check`
2. **Linting**: All code must pass `cargo clippy` with warnings as errors
3. **Compilation**: Project must build successfully with all features
4. **Testing**: Core functionality tests must pass (some performance tests may be flaky)
5. **Documentation**: All doc tests must pass
6. **Security**: No high-severity vulnerabilities in dependencies

### Allowed Warnings

The CI configuration allows certain warnings that are part of the TDD process:

- `clippy::assertions-on-constants`: Used for TDD placeholder tests
- `clippy::redundant-pattern-matching`: Used in some test scenarios

### Performance Considerations

Some performance-sensitive tests may occasionally fail due to system load variations. The CI is configured to:

- Continue on test failures to avoid blocking development
- Report test results for investigation
- Focus on build and code quality validation

### Local CI Simulation

Run the same checks locally before pushing:

```bash
# Full CI check simulation
./scripts/ci-check.sh
```

Or manually:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-features
cargo test --all-features
cargo test --doc --all-features
```

## Development Workflow

### 1. Before Starting Work

```bash
git pull origin main
cargo update
cargo test
```

### 2. During Development

```bash
# Frequent checks
cargo check
cargo test

# Before committing
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

### 3. Before Pushing

```bash
# Final validation
cargo clean
cargo build --all-features
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

## Performance Considerations

### Build Performance

- Use `cargo check` for fast syntax checking during development
- Use `sccache` for distributed compilation caching
- Consider using `cargo-watch` for automatic rebuilds during development

### Runtime Performance

- Always use `--release` flag for production builds
- Profile with `cargo flamegraph` or `perf` for performance analysis
- Use `cargo bloat` to analyze binary size

## Contributing

### Code Standards

1. **Formatting**: Use `cargo fmt` with default settings
2. **Linting**: All clippy warnings must be addressed
3. **Testing**: Maintain >90% test coverage
4. **Documentation**: All public APIs must be documented
5. **Performance**: Performance-critical code must include benchmarks

### Commit Guidelines

1. Run full test suite before committing
2. Use conventional commit messages
3. Keep commits atomic and focused
4. Include tests for new functionality

## Support

For build issues or questions:

1. Check this documentation first
2. Search existing GitHub issues
3. Create a new issue with:
   - Rust version (`rustc --version`)
   - Operating system and version
   - Complete error output
   - Steps to reproduce

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Clippy Documentation](https://rust-lang.github.io/rust-clippy/)
- [rustfmt Configuration](https://rust-lang.github.io/rustfmt/)
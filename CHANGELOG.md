# Changelog

All notable changes to Parseltongue will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive CI/CD pipeline with GitHub Actions
- Multi-platform build verification (Linux, macOS, Windows)
- Security audit integration with cargo-audit
- Local CI validation script for pre-push checks
- Comprehensive BUILD.md documentation
- Pull request template for consistent contributions

### Fixed
- All compilation errors across 50+ source files
- Dead code and unused import cleanup
- Code formatting and linting compliance
- Test infrastructure and integration test issues

### Changed
- Enhanced README.md with CI badges and build status
- Improved error handling patterns across codebase
- Standardized code formatting with cargo fmt

## [0.1.0] - 2024-09-30

### Added
- Initial release of Parseltongue architectural intelligence tool
- Core discovery engine with ISG (Interface Signature Graph) construction
- Rust codebase parsing and analysis capabilities
- CLI interface for codebase exploration
- Performance monitoring and validation framework
- Experimental feature flag support

### Technical Milestones
- **S03-fix-bloat**: Complete codebase cleanup and CI/CD implementation
  - ✅ Fixed all compilation failures
  - ✅ Implemented comprehensive CI pipeline
  - ✅ Added build documentation and developer workflows
  - ✅ Established code quality standards and automation

### Architecture
- Discovery-first approach with proven OptimizedISG architecture
- Layered Rust architecture (L1→L2→L3) following TDD principles
- Dependency injection for testability
- RAII resource management
- Structured error handling with thiserror/anyhow

### Performance
- Sub-500μs query execution for architectural intelligence
- Memory-efficient string interning system
- Concurrent discovery engine with async/await patterns
- Optimized data structures for large codebase analysis

### Developer Experience
- One-command codebase onboarding
- Rich context generation for LLM integration
- Kiro steering integration for AI-assisted development
- Copy-paste ready templates and configurations
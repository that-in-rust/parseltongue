# UJ-030: Cargo-Native Architectural Analysis

## Overview
**Persona**: Individual Developer, Team Lead  
**Workflow Type**: Development, Architecture Analysis  
**Source**: DTNote02.md - Cargo Subcommand Integration  
**Strategic Theme**: Zero-Friction Developer Experience, Symbiotic Tool Ecosystem Integration

## Current Pain Points
- Architectural analysis tools require separate installation and learning curves
- No integration with standard Rust development workflow
- Context switching between `cargo` commands and external tools
- High friction for adoption in existing development teams
- CI/CD pipelines need custom scripting for architectural checks

## Proposed Solution
Expose Parseltongue functionality as native `cargo` subcommands by creating a `cargo-parseltongue` binary in the user's `$PATH`:

**Core Commands**:
- `cargo parseltongue blast-radius <symbol>`: Impact analysis for code changes
- `cargo parseltongue find-cycles`: Detect circular dependencies
- `cargo parseltongue check-arch`: Comprehensive architectural health check for CI
- `cargo parseltongue generate-context <entity>`: LLM-ready factual summaries
- `cargo parseltongue onboard`: Initialize analysis for new projects

**Integration Benefits**:
- Automatic discovery via Cargo's built-in mechanism
- Listed under `cargo --list` with other standard commands
- Standard exit codes for CI/CD integration
- Familiar command patterns reduce learning curve

## Success Metrics
- **Adoption Rate**: Measured via telemetry and package download statistics
- **Workflow Integration**: Time from installation to first productive use
- **CI/CD Usage**: Number of pipelines using `cargo parseltongue check-arch`
- **Developer Satisfaction**: Reduced friction compared to standalone tools

## Integration Requirements
- **Binary Naming**: `cargo-parseltongue` executable in `$PATH`
- **Command Discovery**: Automatic listing under `cargo --list`
- **Exit Codes**: Standard 0/1 exit codes for CI pipeline integration
- **Configuration**: Support for `.parseltongue.toml` project configuration files
- **Documentation**: Integrated help via `cargo parseltongue --help`

## Expected Outcomes
- Architectural analysis becomes part of standard Rust development workflow
- Teams adopt architectural quality gates without additional tooling overhead
- CI/CD pipelines gain simple, reliable architectural checks
- Reduced context switching improves developer flow state
- Lower barrier to entry increases tool adoption across Rust ecosystem

## Implementation Notes
- Follow Cargo's established patterns for subcommand discovery
- Provide comprehensive help text and examples
- Support both interactive and non-interactive (CI) usage modes
- Maintain compatibility with existing Parseltongue CLI interface
- Include progress indicators for long-running operations

## Cross-References
- **Technical Insight**: TI-025 Smart Grep Pipeline Architecture
- **Strategic Theme**: ST-022 Zero-Friction Developer Experience
- **Related Journey**: UJ-031 Git-Integrated Architectural Guardians
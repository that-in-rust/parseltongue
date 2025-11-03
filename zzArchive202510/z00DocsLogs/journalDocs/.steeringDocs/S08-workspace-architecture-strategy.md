# Workspace Architecture Strategy: Monorepo with Extractable Crates

**Status:** Active Architecture Strategy
**Version:** 1.0
**Date:** 2025-10-26
**Type:** Structural Guidelines

---

## Executive Summary

This document establishes **Option A — Monorepo, extractable crates** as the official workspace architecture strategy for Parseltongue. This approach provides the lowest day 1 development overhead while maintaining complete future-proof flexibility for extracting crates into independent repositories when needed.

**Key Benefits:**
- ✅ **Cheapest day 1** - Simple monorepo development
- ✅ **Future-proof** - Crates can become independent repos
- ✅ **No coordination overhead** - Single repo for daily work
- ✅ **Independent publishing** - Each crate can be published separately
- ✅ **Complete history** - Preserved when extracting crates

---

## Workspace Structure

### Required Directory Layout
```
parseltongue/
├── Cargo.toml                 # [workspace] configuration only
├── Cargo.lock
├── README.md                   # Workspace level documentation
├── LICENSE                     # Workspace level license
├── .github/workflows/         # Workspace level CI
└── crates/
    ├── parseltongue-01/        # Core functionality
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── LICENSE
    │   ├── CHANGELOG.md
    │   ├── .cargo_vcs_info.json
    │   └── src/
    ├── parseltongue-02/        # Tree-sitter integration
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── LICENSE
    │   ├── CHANGELOG.md
    │   ├── .cargo_vcs_info.json
    │   └── src/
    ├── parseltongue-03/        # Database operations
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── LICENSE
    │   ├── CHANGELOG.md
    │   ├── .cargo_vcs_info.json
    │   └── src/
    ├── parseltongue-04/        # Graph algorithms
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── LICENSE
    │   ├── CHANGELOG.md
    │   ├── .cargo_vcs_info.json
    │   └── src/
    ├── parseltongue-05/        # Visualization tools
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── LICENSE
    │   ├── CHANGELOG.md
    │   ├── .cargo_vcs_info.json
    │   └── src/
    └── parseltongue-06/        # CLI interface
        ├── Cargo.toml
        ├── README.md
        ├── LICENSE
        ├── CHANGELOG.md
        ├── .cargo_vcs_info.json
        └── src/
```

### Workspace Root Cargo.toml
```toml
[workspace]
members = [
    "crates/*"
]
resolver = "2"

[workspace.package]
version = "0.7.0"
edition = "2021"
authors = ["Parseltongue Team"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/that-in-rust/parseltongue"
homepage = "https://github.com/that-in-rust/parseltongue"
rust-version = "1.70"

[workspace.dependencies]
# Core dependencies
anyhow = "1.0"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }

# CLI dependencies
clap = { version = "4.0", features = ["derive"] }
console = "0.15"
indicatif = "0.17"

# Parsing dependencies
tree-sitter = "0.20"
tree-sitter-rust = "0.20"

# Storage dependencies
cozo = "0.7"

# Export dependencies
wasm-bindgen = "0.2"

# Development dependencies
criterion = "0.5"
proptest = "1.0"
tempfile = "3.0"

# Export format dependencies
mermaid = "0.1"
graphviz = "0.2"
```

---

## Repo-Ready Crate Rules

### Rule 1: Self-Contained Root
**Every crate MUST be a complete, publishable package:**

**Required files in each crate directory:**
- `Cargo.toml` - Complete package configuration
- `README.md` - Crate-specific documentation
- `LICENSE` - License file (can be symlink to workspace root)
- `CHANGELOG.md` - Version history
- `.cargo_vcs_info.json` - VCS information for publishing
- `src/` - Source code directory

**Example crate Cargo.toml (01):**
```toml
[package]
name = "parseltongue-01"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
rust-version.workspace = true
description = "Core functionality for Parseltongue code analysis"
keywords = ["code-analysis", "core", "parseltongue"]
categories = ["development-tools"]

[dependencies]
anyhow.workspace = true
thiserror.workspace = true
serde.workspace = true
serde_json.workspace = true

[dev-dependencies]
criterion.workspace = true
proptest.workspace = true
tempfile.workspace = true

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

**Example crate Cargo.toml (02):**
```toml
[package]
name = "parseltongue-02"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
rust-version.workspace = true
description = "Tree-sitter integration for Parseltongue"
keywords = ["tree-sitter", "parsing", "parseltongue"]
categories = ["development-tools", "parsing"]

[dependencies]
anyhow.workspace = true
thiserror.workspace = true
serde.workspace = true
serde_json.workspace = true
tree-sitter.workspace = true
tree-sitter-rust.workspace = true

[dev-dependencies]
criterion.workspace = true
proptest.workspace = true
tempfile.workspace = true

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

### Rule 2: No Lateral Path Dependencies
**Inter-crate dependencies MUST use workspace versioning:**

**✅ CORRECT:**
```toml
[dependencies]
parseltongue-01 = { workspace = true }
parseltongue-02 = { workspace = true }
```

**❌ INCORRECT:**
```toml
[dependencies]
parseltongue-01 = { path = "../01" }
parseltongue-02 = { path = "../02" }
```

**Development Configuration:**
In workspace root `Cargo.toml` or `.cargo/config.toml`:
```toml
[patch.crates-io]
parseltongue-01 = { path = "crates/parseltongue-01" }
parseltongue-02 = { path = "crates/parseltongue-02" }
parseltongue-03 = { path = "crates/parseltongue-03" }
parseltongue-04 = { path = "crates/parseltongue-04" }
parseltongue-05 = { path = "crates/parseltongue-05" }
parseltongue-06 = { path = "crates/parseltongue-06" }
```

**Publication Process:**
1. Comment out `[patch.crates-io]` section
2. Run `cargo publish -p parseltongue-01`
3. Test with fresh workspace: `cargo new --bin test && cd test && cargo add parseltongue-01`

### Rule 3: CI Per Crate
**Each crate MUST have independent CI testing:**

**CI Workflow Location:** `/crates/<crate-number>/.github/workflows/ci.yml`

**Example CI Workflow (for crate 02):**
```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
    paths:
      - 'crates/02/**'
      - 'Cargo.lock'
      - 'Cargo.toml'
  pull_request:
    branches: [ main ]
    paths:
      - 'crates/02/**'
      - 'Cargo.lock'
      - 'Cargo.toml'

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust:
          - stable
          - beta
          - nightly

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: dtolnay/rust-toolchain@master
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt clippy

    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry/index
          ~/.cargo/registry/cache
          ~/.cargo/git/db
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

    - name: Test package
      run: cargo test --workspace --package parseltongue-02

    - name: Check formatting
      run: cargo fmt --package parseltongue-02 -- --check

    - name: Clippy
      run: cargo clippy --package parseltongue-02 -- -D warnings

  coverage:
    name: Coverage
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: llvm-tools-preview

    - name: Install cargo-llvm-cov
      run: cargo install cargo-llvm-cov

    - name: Generate coverage report
      run: cargo llvm-cov --workspace --package parseltongue-02 --lcov --output-path lcov.info

    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        files: lcov.info
        flags: unittests
        name: codecov-umbrella
```

### Rule 4: History Split Ready
**Workspace structure MUST support clean history extraction:**

**Git History Requirements:**
- Each crate's development history should be contained within `/crates/<crate>/`
- Use conventional commits for easy filtering
- Avoid cross-crate commits when possible

**Extraction Tools:**
- **git subtree**: `git subtree split --prefix=crates/parser -b parser-history`
- **git filter-repo**: More powerful for complex extractions
- **git-filter-branch**: Legacy option

**Extraction Process (when ready):**
```bash
# Create extract branch with just crate 02 history
git subtree split --prefix=crates/parseltongue-02 -b extract-02

# Create new repository
mkdir parseltongue-02
cd parseltongue-02
git init
git pull ../ extract-02 main

# Push to new GitHub repository
git remote add origin git@github.com:your-org/parseltongue-02.git
git push -u origin main

# Update workspace to use published version
# Remove patch.crates-io entry
# Update dependency to use published version
```

---

## Development Guidelines

### Adding New Crates

1. **Create Crate Structure:**
   ```bash
   mkdir crates/07  # Next available number
   cd crates/07
   cargo init --lib
   ```

2. **Add Required Files:**
   ```bash
   # Add README.md, LICENSE, CHANGELOG.md, .cargo_vcs_info.json
   touch README.md LICENSE CHANGELOG.md .cargo_vcs_info.json
   ```

3. **Configure Cargo.toml:**
   - Use `name = "parseltongue-07"`
   - Use `version.workspace = true`
   - Use `authors.workspace = true`
   - Use appropriate workspace dependencies

4. **Add to Workspace:**
   - Add `"crates/07"` to workspace members
   - Add `parseltongue-07 = { path = "crates/07" }` to patch section
   - Create CI workflow in `/crates/07/.github/workflows/ci.yml`

5. **Update Documentation:**
   - Add crate description to this architecture document
   - Update workspace README if needed

### Inter-Crate Dependencies

**Dependency Declaration:**
```toml
[dependencies]
# Core dependencies (always allowed)
parseltongue-01 = { workspace = true }

# Optional dependencies with features
parseltongue-03 = { workspace = true, optional = true }
```

**Feature Flags:**
```toml
[features]
default = ["storage"]
storage = ["dep:parseltongue-03"]
```

### Publishing Process

**Pre-Publish Checklist:**
- [ ] All tests pass: `cargo test --workspace --package parseltongue-XX`
- [ ] Documentation builds: `cargo doc --package parseltongue-XX`
- [ ] No patch dependencies in final build
- [ ] CHANGELOG.md updated for version
- [ ] README.md includes usage examples
- [ ] License file present

**Publish Commands:**
```bash
# Dry run
cargo publish --dry-run --package parseltongue-XX

# Actual publish
cargo publish --package parseltongue-XX

# Verify publish
cargo add --dev parseltongue-XX  # In test project
```

---

## Extraction Readiness Checklist

For each crate, verify extraction readiness:

### Self-Contained
- [ ] Complete package metadata
- [ ] Standalone README.md
- [ ] License file
- [ ] CHANGELOG.md
- [ ] .cargo_vcs_info.json

### Dependencies
- [ ] No path dependencies
- [ ] All dependencies use workspace = true
- [ ] Patch configuration only in workspace root
- [ ] Can build independently: `cargo build --package <crate>`

### CI/CD
- [ ] Individual CI workflow
- [ ] Tests pass in isolation
- [ ] Coverage reporting
- [ ] Documentation builds

### History
- [ ] Commits focused on crate when possible
- [ ] Clean extraction with git subtree possible
- [ ] No cross-crate file dependencies

### Publishing
- [ ] Can publish independently
- [ ] Documentation complete
- [ ] Examples working
- [ ] Version management clear

---

## Migration Plan

### Phase 1: Setup Workspace Structure
1. Create workspace Cargo.toml
2. Create crate directories
3. Set up basic crate structure
4. Configure inter-crate dependencies

### Phase 2: Implement Core Crates
1. Implement parseltongue-01 (Core functionality)
2. Implement parseltongue-02 (Tree-sitter integration)
3. Implement parseltongue-03 (Database operations)
4. Implement parseltongue-04 (Graph algorithms)
5. Implement parseltongue-05 (Visualization tools)
6. Implement parseltongue-06 (CLI interface)

### Phase 3: Add CI/CD
1. Create individual CI workflows
2. Set up workspace-level CI
3. Configure publishing workflows
4. Add coverage reporting

### Phase 4: Documentation
1. Update crate READMEs
2. Create API documentation
3. Update workspace documentation
4. Create extraction guides

### Phase 5: Validation
1. Test independent builds
2. Validate publishing process
3. Test extraction procedures
4. Validate CI/CD pipelines

---

## Success Metrics

### Development Efficiency
- [ ] Single command builds workspace
- [ ] Independent crate development possible
- [ ] No cross-crate coordination overhead

### Publishing Success
- [ ] Each crate can be published independently
- [ ] No publishing conflicts
- [ ] Semantic versioning works correctly

### Extraction Success
- [ ] Clean history extraction possible
- [ ] No cross-crate dependencies in extracted repos
- [ ] Independent maintenance feasible

---

## Examples and Templates

### Crate Template Structure
```
crates/XX/
├── .cargo_vcs_info.json
├── CHANGELOG.md
├── Cargo.toml
├── LICENSE -> ../../LICENSE
├── README.md
├── .github/
│   └── workflows/
│       └── ci.yml
└── src/
    └── lib.rs
```

### Workspace Development Config
`.cargo/config.toml`:
```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "/path/to/vendor"

[patch.crates-io]
parseltongue-01 = { path = "crates/parseltongue-01" }
parseltongue-02 = { path = "crates/parseltongue-02" }
parseltongue-03 = { path = "crates/parseltongue-03" }
parseltongue-04 = { path = "crates/parseltongue-04" }
parseltongue-05 = { path = "crates/parseltongue-05" }
parseltongue-06 = { path = "crates/parseltongue-06" }
```

---

## Conclusion

This workspace architecture strategy provides the optimal balance between development simplicity and future flexibility. By following these guidelines, the Parseltongue project can maintain efficient monorepo development while preserving the ability to extract crates into independent repositories when the need arises.

**Key Success Factors:**
- Consistent application of repo-ready crate rules
- Discipline in dependency management
- Comprehensive CI/CD per crate
- Clean separation of concerns

When followed correctly, this strategy enables seamless transitions from monorepo development to independent crate maintenance with minimal friction and maximum flexibility.

---

*This document should be reviewed quarterly and updated as the project evolves. All new crates must follow these guidelines from day one.*
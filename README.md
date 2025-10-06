# 🐍 Parseltongue - Rust Architectural Intelligence Daemon

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A high-performance Rust-only architectural intelligence daemon that analyzes codebases and creates Interface Signature Graphs (ISG) for LLM consumption and visualization.

## ✨ Features

- **🔍 Smart Code Analysis**: Extracts functions, structs, traits, and their relationships from Rust code
- **⚡ Lightning Fast**: <500μs query performance, <5s ingestion for 2.1MB codebases
- **📊 Interactive Visualization**: WASM-powered interactive graphs with multiple layout algorithms
- **🔄 Real-time Monitoring**: Live file watching with <12ms update latency
- **🤖 LLM-Ready**: Optimized JSON output for AI-assisted development
- **📈 Mermaid Export**: GitHub-compatible diagram generation

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/that-in-rust/parseltongue.git
cd parseltongue

# Build in release mode
cargo build --release

# Verify installation
./target/release/parseltongue --help
```

### Basic Usage

```bash
# Analyze a Rust codebase
./target/release/parseltongue ingest examples/sample-rust-code.txt

# Query the architecture
./target/release/parseltongue query what-implements Clone
./target/release/parseltongue query blast-radius MyStruct --format json

# Generate visualizations
./target/release/parseltongue export              # Mermaid diagram for GitHub
./target/release/parseltongue export-wasm         # Interactive WASM visualization

# Monitor live changes
./target/release/parseltongue daemon --watch ./src
```

## 📋 Commands Overview

| Command | Description | Example |
|---------|-------------|---------|
| `ingest` | Process code dumps with FILE: markers | `ingest codebase.txt` |
| `query` | Execute graph queries | `query what-implements Trait` |
| `export` | Generate Mermaid diagrams | `export --output my-diagram` |
| `export-wasm` | Create interactive WASM visualizations | `export-wasm --layout forcedirected` |
| `daemon` | Real-time file monitoring | `daemon --watch ./src` |
| `generate-context` | Create LLM context | `generate-context MyFunction` |
| `debug` | Debug and visualization utilities | `debug --sample` |

## 🎯 Query Types

- **what-implements**: Find all implementors of a trait
- **blast-radius**: Analyze impact of changing an entity
- **find-cycles**: Detect circular dependencies
- **who-calls**: Find all functions that call a target
- **get-called-functions**: Find all functions called by a target
- **execution-path**: Find path between two functions

## 📊 Visualization Options

### Mermaid Export (GitHub-compatible)
```bash
./target/release/parseltongue export --output architecture.md
```
Creates GitHub-compatible Mermaid diagrams perfect for documentation.

### WASM Export (Interactive)
```bash
./target/release/parseltongue export-wasm --layout forcedirected --output viz/
```
Generates interactive HTML visualizations with:
- Multiple layout algorithms (breadth-first, force-directed, hierarchical, circular)
- Zoom, pan, and interaction controls
- Real-time performance metrics
- Node coloring by type (functions, structs, traits, impls)

## 🔧 Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   CLI Layer     │───▶│  Daemon Core     │───▶│  ISG Engine     │
│  (cli.rs)       │    │  (daemon.rs)     │    │  (isg.rs)       │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │                        │
                                ▼                        ▼
                       ┌──────────────────┐    ┌─────────────────┐
                       │ Query Processing │    │ Graph Storage   │
                       │                  │    │ (petgraph +     │
                       │                  │    │  parking_lot)   │
                       └──────────────────┘    └─────────────────┘
```

## 📈 Performance

- **Ingestion**: <5s for 2.1MB codebases (54K lines)
- **Queries**: <500μs for simple queries, <1ms for complex analysis
- **Memory**: ~15-25MB for small codebases, ~50-100MB for large ones
- **Real-time**: <12ms file update latency in daemon mode

## 🎨 Layout Algorithms

| Algorithm | Best For | Performance | Visual Quality |
|-----------|-----------|-------------|----------------|
| **breadth-first** | Quick overview | ⚡ Fast | ✅ Good |
| **force-directed** | Aesthetics | 🐢 Slow | ✨ Excellent |
| **hierarchical** | DAG structures | 🚀 Medium | ✅ Very Good |
| **circular** | Small graphs | ⚡ Fast | ✅ Good |

## 🛠️ Development

### Running Tests
```bash
cargo test
```

### WASM Development
```bash
# Build WASM modules
wasm-pack build --target web --out-dir pkg

# Run local development server
python3 -m http.server 8080
```

### Code Quality
```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Check for security issues
cargo audit
```

## 📚 Documentation

- **[Onboarding Guide](docs/ONBOARDING_GUIDE.md)** - Complete getting started guide
- **[CLI Implementation](docs/CLI_IMPLEMENTATION_SUMMARY.md)** - Technical implementation details
- **[ISG Explained](docs/ISG_EXPLAINED.md)** - Understanding Interface Signature Graphs
- **[Mermaid Reference](docs/mermaid-reference.md)** - Diagram generation guidelines

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow **Test-Driven Development** (STUB → RED → GREEN → REFACTOR)
- Use **Mermaid diagrams only** for all visualizations (per steering docs)
- Maintain **performance contracts** - all new features must meet timing constraints
- Write **comprehensive tests** with >90% coverage
- Use **structured error handling** with `thiserror` and `anyhow`

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📊 Case Studies: Real-World Performance

### 🚀 **Case Study 1: Tokio Async Runtime Analysis**
- **Scale**: 717 Rust files, 2,571 architectural entities
- **Performance**: 0.25s ingestion (20x faster than requirement)
- **Output**: Interactive visualization with 2,571 nodes, 136 relationships
- **Details**: [Complete analysis](docs/CASE_STUDIES.md#-case-study-1-tokio-async-runtime-analysis)

### 🔄 **Case Study 2: Live Codebase Delta Detection**
- **Scale**: 11 Rust files, 2,641 entities
- **Delta Detection**: 2,639 → 2,641 nodes (+2 change detected)
- **Performance**: <0.01s real-time analysis
- **Output**: Interactive hierarchical visualization
- **Details**: [Complete analysis](docs/CASE_STUDIES.md#-case-study-2-live-codebase-delta-analysis)

### 📈 **Proven Performance**
| Metric | Requirement | Achieved | Status |
|--------|-------------|----------|---------|
| Ingestion | <5s | 0.01-0.25s | ✅ 20-250x faster |
| Queries | <500μs | ~10μs | ✅ 50x faster |
| Memory | <100MB | ~15MB | ✅ 6.7x under |

### 🎯 **Live Demonstrations**
- **[Tokio WASM Visualization](tokio-wasm-viz/visualization.html)** - Interactive 2,571-node graph
- **[Parseltongue Self-Analysis](parseltongue-live-viz/visualization.html)** - Hierarchical architecture view

## 🔗 Related Projects

- **[Tokio Analysis](analysis/tokio-hierarchical/)** - Comprehensive case study on Tokio async runtime
- **[Case Studies Documentation](docs/CASE_STUDIES.md)** - Complete analysis with performance metrics
- **[Mermaid Export](src/mermaid_export.rs)** - GitHub-compatible diagram generation
- **[WASM Renderer](src/wasm_renderer.rs)** - Interactive visualization system

## 🙏 Acknowledgments

Built with Rust and following the architectural principles outlined in our [steering documents](steeringDocs/).

---

**Parseltongue** - Speak the language of Rust architecture 🐍
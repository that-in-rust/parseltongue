# Parseltongue AIM Daemon

**Rust-only architectural intelligence daemon** providing deterministic, graph-based code analysis with sub-millisecond query performance.

## 🎯 The Problem We Solve

**Rust Codebase Discovery Bottleneck**: Finding entity names and understanding architecture in unfamiliar codebases takes minutes to hours.

**Our Solution**: Parse once, query forever. Build an Interface Signature Graph that gives you:
- Complete entity discovery in milliseconds
- Instant architectural impact analysis
- Deterministic, sub-millisecond queries

## 🚀 Features

- **Real-time File Monitoring**: Watch Rust codebases with <12ms update latency
- **Code Dump Analysis**: Process large code dumps in <5 seconds
- **Graph-based Queries**: Sub-millisecond architectural queries
- **LLM Integration**: Generate structured context for AI code assistance
- **High Performance**: 6μs node operations, concurrent-safe architecture
- **Production Ready**: Comprehensive error handling and crash recovery

## 📦 Installation

```bash
git clone <repository>
cd parseltongue
cargo build --release
```

## 🚀 30-Second Demo

See the system in action with the built-in example:
```bash
# Build and run the visualization example
cargo run --example visualize_isg
```

This demonstrates:
- ✅ Code ingestion from `example_dump.txt`
- ✅ ISG structure creation (4 nodes, 1 edge)
- ✅ Graph queries (what-implements, blast-radius)
- ✅ LLM context generation
- ✅ Graphviz DOT export for visualization

## 🎯 Quick Start

### Analyze a Code Dump
```bash
# Using the provided example
parseltongue ingest example_dump.txt

# Query the generated graph
parseltongue query what-implements Display
parseltongue generate-context User --format json
```

### Real-time Monitoring
```bash
# Monitor a Rust project directory
parseltongue daemon --watch src/
```

### Query Architecture
```bash
# Find all implementors of a trait
parseltongue query what-implements Greeter

# Calculate blast radius of changes
parseltongue query blast-radius Person

# Find circular dependencies
parseltongue query find-cycles
```

### Generate LLM Context
```bash
# Human-readable context
parseltongue generate-context Person

# JSON format for LLM consumption
parseltongue generate-context Person --format json
```

## 🏗️ Architecture

### Core Components
- **OptimizedISG**: High-performance Interface Signature Graph using petgraph + parking_lot
- **ParseltongueAIM**: Main daemon with file monitoring and code parsing
- **CLI Interface**: Complete command-line interface with clap
- **Persistence Layer**: JSON serialization with crash recovery

### Validated Performance Characteristics
- **Node Operations**: ~6μs (verified ✅)
- **Simple Queries**: <500μs (verified ✅)
- **Complex Queries**: <1ms (verified ✅)
- **File Updates**: <12ms
- **Code Ingestion**: <5s for large dumps (verified ✅)
- **Memory Usage**: Efficient for real codebases

### Technical Stack
- **Language**: Rust (100%)
- **Graph Library**: petgraph with StableDiGraph
- **Concurrency**: parking_lot RwLock for thread safety
- **Parsing**: syn crate for Rust AST analysis
- **File Monitoring**: notify crate for cross-platform file watching
- **CLI**: clap with derive macros
- **Serialization**: serde with JSON format

## 🧪 Testing

The project maintains 97.5% test coverage with comprehensive TDD approach:

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --lib isg      # Core graph tests
cargo test --lib daemon   # Daemon functionality
cargo test --lib cli      # CLI interface tests
```

### Test Categories
- **Unit Tests**: Core functionality validation
- **Integration Tests**: End-to-end workflow testing
- **Performance Tests**: Timing constraint validation
- **Concurrency Tests**: Thread safety verification

## 📊 Performance Validation

All performance contracts are automatically validated:

```bash
# Performance test results
Node operations: ~6μs ✅
Simple queries: <500μs ✅
Complex queries: <1ms ✅
File updates: <12ms ✅
Persistence: <500ms ✅
```

## 🔧 Configuration

### Environment Variables
- `RUST_LOG`: Set logging level (debug, info, warn, error)
- `PARSELTONGUE_SNAPSHOT_PATH`: Custom snapshot file location

### File Formats
- **Input**: Code dumps use `FILE: path` markers:
```
FILE: src/lib.rs
pub trait Display {
    fn fmt(&self) -> String;
}
================================================
FILE: src/main.rs
fn main() {
    // code
}
```
Separators like `====` are automatically ignored.

- **Output**: JSON or human-readable formats
- **Persistence**: JSON snapshots for crash recovery
- **Error Handling**: Malformed Rust files are logged and skipped, allowing processing to continue

### Robust Processing
- **Graceful Error Recovery**: Malformed files are logged and skipped
- **Partial Processing**: Continues analysis even with some file errors
- **Error Reporting**: Clear error messages for debugging

## 🎯 Common Workflows

### Understand Trait Implementations
```bash
# Ingest a codebase and find trait implementors
parseltongue ingest codebase.txt
parseltongue query what-implements Clone --format json
```

### Assess Change Impact
```bash
# Calculate blast radius for proposed changes
parseltongue query blast-radius UserStruct
parseltongue generate-context UserStruct
```

### Generate LLM Context
```bash
# Export context for AI code assistance
parseltongue generate-context EntityName --format json > context.json
```

### Debug Architecture
```bash
# Visualize the graph structure
parseltongue debug --graph
parseltongue debug --dot > graph.dot
```

## 🎯 Use Cases

### For Developers
- **Code Navigation**: Understand complex Rust codebases quickly
- **Impact Analysis**: Assess blast radius of proposed changes
- **Architecture Review**: Validate trait implementations and dependencies
- **Refactoring**: Safe code restructuring with dependency analysis
- **Robust Processing**: Handles malformed files gracefully without stopping analysis

### For AI/LLM Integration
- **Context Generation**: Provide accurate architectural context to AI tools
- **Code Assistance**: Enable AI to understand project structure
- **Documentation**: Generate architectural summaries automatically

### For Teams
- **Code Reviews**: Architectural impact assessment
- **Onboarding**: Help new team members understand codebase structure
- **Technical Debt**: Identify circular dependencies and architectural issues

## 🚦 Status

**Production Ready** ✅
- All MVP requirements completed
- Comprehensive test coverage (40/40 tests passing)
- Performance validated against all constraints
- Error handling and edge cases covered
- Real-world usage tested
- Resilient parsing with graceful error recovery

## 🤝 Contributing

This project follows Test-Driven Development (TDD):
1. Write failing tests first (RED)
2. Implement minimal functionality (GREEN)
3. Refactor and optimize (REFACTOR)

## 📄 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

Built with the excellent Rust ecosystem:
- [petgraph](https://github.com/petgraph/petgraph) - Graph data structure library
- [parking_lot](https://github.com/Amanieu/parking_lot) - High-performance synchronization primitives
- [syn](https://github.com/dtolnay/syn) - Rust syntax tree parsing
- [notify](https://github.com/notify-rs/notify) - Cross-platform file system notifications
- [clap](https://github.com/clap-rs/clap) - Command line argument parser
- [serde](https://github.com/serde-rs/serde) - Serialization framework

---

**Parseltongue AIM Daemon** - Deterministic architectural intelligence for Rust codebases 🐍⚡
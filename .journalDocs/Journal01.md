# Journal01: Parseltongue CLI Interface Design

**Date:** 2025-10-26
**Focus:** CPU-only Code Graph Builder CLI Design
**Status:** Design Documentation Phase

---

## 1. Core Product Vision (UltraThink Analysis)

### **Strategic Overview**
Building an **automated code understanding platform** that reads, understands, and safely modifies software systems through semantic interface analysis - completely CPU-based with no LLM dependencies.

### **Problem Domain**
Software systems are complex and hard to modify safely. Current tools work at line-level syntax, not semantic understanding. We need **structural comprehension** to enable trustworthy automated changes.

### **Key Innovation: Interface-Centric Analysis**
Unlike AST parsers, we focus on **semantic interfaces** - the contracts that matter for system behavior. This enables:
- **Impact Analysis**: Understand what breaks when interfaces change
- **Safe Transformations**: Modify implementation while preserving contracts
- **Test Coverage Mapping**: Link tests to specific interface requirements

### **Technical Approach**
- **Phase 1**: Semantic Extraction (SystemGate + ISG Builder)
- **Phase 2**: Database Persistence (CozoDB with RocksDB)
- **Phase 3**: Query Interface (Multiple query types, CLI-friendly)

---

## 2. CLI Interface Design

### **Global Structure**
```bash
parseltongue [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

### **Global Options**
```bash
--db <path>          # Database directory (default: ./isg_db)
--verbose, -v        # Increase output verbosity
--quiet, -q          # Minimal output only
--help, -h           # Show help
--version            # Show version info
```

### **Core Commands**

#### **2.1 System Validation (SystemGate)**
```bash
parseltongue check [OPTIONS]
```

**Purpose:** Validate system capabilities before building ISG

**Options:**
- `--json` - Output system specs as JSON
- `--benchmark` - Run performance benchmarks
- `--detailed` - Show full system analysis

**Expected Outputs:**
- Architecture compatibility (Apple Silicon/Intel/Unsupported)
- Memory validation (≥9GB required, ≥16GB recommended)
- Disk space validation (≥10GB free)
- Performance tier (high/medium/unsupported)
- Block reasons with specific remediation advice

#### **2.2 Build ISG (Main Workhorse)**
```bash
parseltongue build [OPTIONS] [REPO_PATH]
```

**Purpose:** Parse Rust repository and build Interface Signature Graph

**Arguments:**
- `REPO_PATH` - Repository root directory (default: current directory)

**Options:**
```bash
--include-code      # Store full code snippets (increases DB size)
--batch-size <n>    # DB batch size (default: 500)
--workers <n>       # Parallel parse workers (default: CPU cores)
--exclude <pat>     # Exclude patterns (can repeat)
--include <pat>     # Include patterns (default: **/*.rs)
--force             # Rebuild even if DB exists
--no-gitignore      # Don't respect .gitignore
--stats             # Show detailed parsing statistics
```

**Usage Examples:**
```bash
# Basic build on current directory
parseltongue build

# Build with code snippets, custom repo
parseltongue build --include-code ../my-rust-project

# Build with custom settings
parseltongue build --batch-size 1000 --workers 8 --exclude "**/tests/**" ./src

# Force rebuild with detailed stats
parseltongue build --force --stats ./my-project
```

#### **2.3 Query Interface**
```bash
parseltongue query <QUERY_TYPE> [OPTIONS]
```

**Query Types:**

**Prefix Search:**
```bash
parseltongue query prefix --prefix "src/utils" --limit 20
```

**Interface by Exact Key:**
```bash
parseltongue interface <isgl1_key>
```

**Relationship Search:**
```bash
parseltongue query related --to <isgl1_key> --type defines|calls
```

**Interface Type Listing:**
```bash
parseltongue query type --kind struct|trait|function|impl
```

**Full-text Search:**
```bash
parseltongue query search --text "async fn" --in-tests
```

**Query Options:**
```bash
--limit <n>         # Max results (default: 20)
--offset <n>        # Pagination offset
--format <fmt>      # Output: table|json|csv (default: table)
--in-tests          # Include test implementations
--code-only         # Only show interfaces with stored code
--relationships     # Include relationships in output
```

#### **2.4 Database Management**
```bash
parseltongue db <SUBCOMMAND>
```

**Subcommands:**
```bash
parseltongue db info      # Show DB statistics
parseltongue db optimize  # Optimize database
parseltongue db export    # Export to JSON
parseltongue db import    # Import from JSON
parseltongue db reset     # Delete database
```

#### **2.5 Interactive Mode**
```bash
parseltongue shell [OPTIONS]
```

**Options:**
- `--db <path>` - Use specific database
- `--history` - Enable command history

**Shell Commands:**
```
> query prefix --prefix src
> interface src-main-main.rs-MyStruct::new
> db info
> exit
```

---

## 3. Technical Implementation Details

### **Exit Codes**
- `0` - Success
- `1` - General error
- `2` - System incompatible
- `3` - Database error
- `4` - Parse error

### **Performance Considerations**
- **System validation** should complete in <2 seconds
- **Build operations** use batch processing for scalability
- **Query operations** support pagination and result limiting
- **Worker counts** adapt to CPU core availability
- **Batch sizes** configurable based on available memory

### **Error Handling Strategy**
- **Graceful degradation** for unsupported architectures
- **Resource awareness** - adjust behavior based on available RAM/disk
- **Clear error messages** with specific remediation suggestions
- **Progress reporting** for long-running operations

### **Integration Points**
- **Tree-sitter** for robust Rust parsing
- **CozoDB** for graph storage and querying
- **Ignore crate** for proper .gitignore handling
- **Sysinfo** for system capability detection

---

## 4. Usage Examples

### **Daily Workflow (Developer)**
```bash
# Quick system check before starting
parseltongue check

# Build current project with code snippets
parseltongue build --include-code ./my-rust-project

# Query specific interface
parseltongue interface src-model-user.rs-User::new

# Find all structs in utils package
parseltongue query type --kind struct --prefix "src/utils"

# Quick search for async functions
parseltongue query search --text "async fn" --limit 10
```

### **Power User Workflow (Architect/Lead)**
```bash
# Detailed build with full stats and optimization
parseltongue build --stats --workers 12 --force ./large-project

# Export entire ISG for external analysis
parseltongue db export --output isg-backup.json

# Complex relationship analysis
parseltongue query related --to "src-core-service.rs-Service::process" --type calls

# Find all test implementations for specific interfaces
parseltongue query prefix --prefix "src/models" --in-tests --format json

# Database optimization and maintenance
parseltongue db optimize
parseltongue db info
```

### **CI/CD Integration**
```bash
# Automated system validation
parseltongue check --json > system-report.json

# Build with minimal output for scripts
parseltongue build --quiet --batch-size 1000 ./src

# Query for change impact analysis
parseltongue query related --to "src-api-routes.rs-Router::new" --format json > impact.json
```

---

## 5. Design Principles

### **Progressive Disclosure**
- **Simple defaults** work out of the box
- **Advanced options** available when needed
- **Consistent patterns** across all commands

### **Fast Feedback Loops**
- **System check** completes in <2 seconds
- **Query operations** return results quickly
- **Progress indicators** for long operations

### **Batch-Friendly Scripting**
- **JSON output** available for all commands
- **Parseable exit codes** for automation
- **Quiet mode** for reduced output in scripts

### **Resource Awareness**
- **Auto-detect** CPU cores for worker count
- **Adaptive batch sizes** based on available memory
- **Graceful handling** of resource constraints

### **Error Clarity**
- **Specific error messages** with remediation steps
- **System requirements** clearly communicated
- **Recovery suggestions** for common failure modes

---

## 6. Architecture Decisions Rationale

### **Why CLI-First Design**
- **Developer workflow integration** - fits naturally into existing toolchains
- **Automation friendly** - easy to integrate into CI/CD pipelines
- **Low overhead** - no GUI dependencies, faster execution
- **Remote server usage** - SSH friendly, works in headless environments

### **Why Multiple Query Types**
- **Different use cases** require different access patterns
- **Exploration vs targeted lookup** - prefix search vs exact key
- **Relationship analysis** - critical for impact assessment
- **Text search** - useful for finding specific patterns

### **Why Interactive Mode**
- **Exploration workflow** - iterative query refinement
- **Learning curve reduction** - discoverable interface
- **Rapid prototyping** - test queries before scripting

### **Why Database Management Commands**
- **Data portability** - export/import for offline analysis
- **Performance tuning** - optimization for large codebases
- **Maintenance operations** - keep database healthy

---

## 7. Future Extensibility Considerations

### **Language Support Expansion**
- **Command structure** accommodates multiple language parsers
- **Database schema** designed for language-agnostic storage
- **Query interface** abstracted across different language types

### **Advanced Analysis Features**
- **Metrics collection** - complexity, coupling, cohesion analysis
- **Visualization integration** - export to graph analysis tools
- **Historical tracking** - track changes over time

### **Performance Optimization**
- **Caching strategies** - query result caching
- **Incremental updates** - only process changed files
- **Distributed processing** - handle very large codebases

---

## 8. Success Metrics

### **Performance Targets**
- **System validation**: <2 seconds
- **Small repo build** (<1000 files): <30 seconds
- **Large repo build** (>10k files): <5 minutes
- **Query response**: <1 second for typical queries

### **Usability Targets**
- **Command discovery**: intuitive help system
- **Error recovery**: clear guidance for issues
- **Learning curve**: productive within 15 minutes

### **Reliability Targets**
- **Parse success rate**: >99% on valid Rust code
- **Database corruption**: zero tolerance
- **Memory usage**: efficient handling of large codebases

---

---

## 9. Production Repository Analysis (GitHub: that-in-rust/parseltongue)

### **9.1 Performance Requirements Reality Check**

**Production Performance Targets (Much Stricter Than Assumed):**
- **File monitoring**: <12ms update latency
- **Code dump processing**: <5 seconds for 2.1MB code
- **Node operations**: 6μs (microseconds!)
- **Query performance**: Sub-millisecond architectural queries
- **Blast radius calculation**: <1ms
- **Implementors lookup**: <500μs

**Our Original Targets (Need Revisiting):**
- System validation: <2 seconds (OK)
- Small repo build: <30 seconds (should be <5 seconds)
- Query response: <1 second (should be <1 millisecond!)

### **9.2 Architecture Comparison**

**Production Uses:**
- **`syn` crate** for Rust parsing (instead of Tree-sitter)
- **`StableDiGraph<NodeData, EdgeKind>`** from petgraph library
- **`FxHashMap`** for O(1) lookups
- **`Arc<str>`** for memory-efficient string interning
- **`RwLock`** for concurrent access
- **`SigHash(u64)`** for collision-free identifiers

**Our Design Uses:**
- Tree-sitter for parsing
- CozoDB for persistence
- ISGL1 key hierarchy
- Batch processing approach

### **9.3 Command Structure Differences**

**Production Commands:**
```bash
ingest           # Process code dumps with FILE: markers
daemon          # Real-time file monitoring
query           # WhatImplements, BlastRadius, FindCycles, etc.
generate-context # LLM context generation
export          # Mermaid diagram export
export-wasm     # WASM visualization export
debug           # Graph debugging and visualization
```

**Our Proposed Commands:**
```bash
check           # System validation
build           # Build ISG from repo
query           # Prefix, exact, relationship, type, search
db              # Database management
shell           # Interactive mode
```

### **9.4 Missing Features in Our Design**

**Critical Gaps Identified:**
1. **Real-time monitoring** - Daemon mode with file watching
2. **Visualization** - Mermaid and WASM export capabilities
3. **Advanced algorithms** - Cycle detection, execution paths, blast radius
4. **Context generation** - Built-in LLM context export
5. **Performance optimization** - Much more aggressive performance targets needed

**Potential Integration Opportunities:**
1. **Hybrid approach** - Combine Tree-sitter parsing with production graph algorithms
2. **Performance targets** - Adopt production-level performance requirements
3. **Visualization pipeline** - Add export commands for graph visualization
4. **Real-time capabilities** - Consider daemon mode for development workflows

### **9.5 Design Implications**

**Performance Requirements Adjustment:**
- Target <5 second build times for typical repos
- Sub-millisecond query performance for interactive use
- <12ms file update processing for real-time mode

**Architecture Considerations:**
- Consider `syn` crate for more Rust-specific parsing
- Investigate petgraph for high-performance graph operations
- Implement string interning for memory efficiency
- Add concurrent access patterns for multi-threaded operations

**Feature Prioritization:**
1. Core parsing and graph building (already planned)
2. High-performance query algorithms (need upgrade)
3. Visualization export capabilities (new requirement)
4. Real-time monitoring (stretch goal)

---

*This journal documents the ideation process for Parseltongue's CLI interface design, incorporating insights from the production repository. All design decisions, trade-offs, and rationale are preserved here for future reference and implementation guidance.*
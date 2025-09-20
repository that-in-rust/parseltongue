# Sig Graph Ideas: Signature Stub 3x3 Graph Extractor

## Harry Potter Themed Name Options

1. **Marauder's Map**
   - "I solemnly swear that I am up to no good"
   - Reveals hidden magical pathways (code relationships) that are normally invisible
   - Shows who goes where, what connects to what
   - Magical reveal of the castle's secret passages

2. **Hogwarts Express**
   - The magical train that connects all parts of the wizarding world
   - Connects Platform 9¾ to Hogwarts (connects all parts of the codebase)
   - Represents the journey through different magical realms (frontend/backend/database)
   - Reliable transportation between magical worlds

3. **Pensieve**
   - The magical basin that stores memories and allows you to view relationships
   - Extract memories (code signatures) and view their connections
   - Allows analysis of complex relationships and patterns
   - Magical tool for understanding complex information structures

**Selected Recommendation: Pensieve** - Best represents the analytical, relationship-mapping nature of the tool while maintaining the magical theme of discovering hidden connections.

## User Journey: Developer Experience

### Step 1: Installation & Setup
```bash
# Developer installs the tool
cargo install pensieve

# Or downloads pre-built binary
wget https://github.com/your-org/pensieve/releases/latest/pensieve-linux
chmod +x pensieve-linux
mv pensieve-linux /usr/local/bin/pensieve
```

### Step 2: First Run - Discovery Phase
```bash
# Developer navigates to their full-stack project
cd my-fullstack-app

# Initial scan to discover what's there
pensieve scan
```

**What the developer sees:**
```
🔍 Pensieve v0.1.0
Scanning project structure...
├── Found: Rust backend (src/, Cargo.toml)
├── Found: TypeScript frontend (src/components/, package.json)
├── Found: SQL schemas (migrations/, *.sql)
├── Found: API specs (openapi.yaml, *.graphql)
└── Found: Configuration files (config/, *.env.example)

📊 Project Analysis:
  • Total files: 247
  • Rust files: 89
  • TS/JS files: 112
  • SQL files: 18
  • Config files: 28

💡 Recommendation: This looks like a Rust+React full-stack application
   Run 'pensieve extract' to build the 3x3 signature graph
```

### Step 3: Graph Extraction
```bash
# Developer runs the full extraction
pensieve extract --output ./siggraph.jsonl
```

**What the developer sees:**
```
🏗️  Building 3x3 Signature Graph...
🔧 Parsing Rust source files... (89 files)
📝 Extracting function signatures... (234 functions)
🏷️  Identifying type definitions... (67 types)
🔌 Analyzing trait implementations... (45 traits)
🔗 Discovering relationships...
   • Calls relationships: 412 found
   • Implements relationships: 89 found
   • Interacts relationships: 156 found
🔐 Generating SigHash IDs... (BLAKE3 hashing)
💾 Exporting to JSONL format...
✅ Extraction complete! Generated siggraph.jsonl (2.3MB)

📈 Graph Statistics:
  • Total nodes: 435
    - Functions: 234
    - Types: 67
    - Traits: 45
    - Cross-stack: 89
  • Total edges: 657
    - Calls: 412
    - Implements: 89
    - Interacts: 156
  • Compression ratio: 98.7% (247 files → 2.3MB)
```

### Step 4: Interactive Analysis
```bash
# Developer explores the extracted graph
pensieve query --interactive
```

**Interactive session experience:**
```
🔍 Pensieve Query Mode (type 'help' for commands)

pensieve> who-calls AuthService::login
📡 Functions calling AuthService::login:
   • Routes::auth_login (HTTP handler)
   • WebSocketHandler::authenticate (WebSocket auth)
   • TestHelpers::create_test_session (Test utility)

pensieve> blast-radius User::id --depth 2
💥 Blast radius for User::id (depth 2):
   🎯 Direct impact (2 nodes):
      • UserService::get_by_id
      • UserRepository::find_by_id
   📊 Secondary impact (8 nodes):
      • Routes::user_profile (via UserService)
      • CacheService::user_data (via UserService)
      • AuditService::log_user_access (via UserRepository)
      • ... (5 more)

pensieve> what-implements IDataStore
🔌 Implementations of IDataStore:
   • DatabaseStore: SQLite implementation
   • CacheStore: Redis-backed cache
   • MockStore: Test double

pensieve> find-cycles --in auth_module
🔄 Cycle detection in auth_module:
   ⚠️  Found 1 potential cycle:
      AuthService → SessionManager → TokenValidator → AuthService
   💡 Recommendation: Consider breaking this cycle with dependency injection
```

### Step 5: Architecture Visualization
```bash
# Generate visualizations
pensieve visualize --format mermaid --output architecture.md
```

### Step 6: LLM Integration Prep
```bash
# Prepare interface stubs for LLM code generation
pensieve export-stubs --target rust --output interfaces.rs
pensieve export-stubs --target typescript --output interfaces.ts
```

## Functional Journey: What the Program Does

### Phase 1: Codebase Analysis
1. **File Discovery**: Scans directory structure, identifies file types, maps project topology
2. **Language-Specific Parsing**: Uses syn for Rust, swc for TypeScript, regex for SQL/config

### Phase 2: Graph Construction
3. **Node Generation**: Creates graph nodes for Functions, Types, Traits with SigHash IDs
4. **Edge Discovery**: Finds Calls, Implements, Interacts relationships

### Phase 3: Query Processing
5. **Graph Indexing**: Loads JSONL into in-memory SQLite, creates indexes
6. **Query Execution**: Runs SQL queries with BFS for blast radius analysis

### Phase 4: Output Generation
7. **Signature Hashing**: Generates BLAKE3-based SigHash IDs
8. **Interface Stub Generation**: Converts graph nodes to language-specific interfaces

### Phase 5: Analysis & Reporting
9. **Consistency Checking**: Detects circular dependencies, validates contracts
10. **Metrics Generation**: Calculates complexity metrics, identifies hotspots

## Technical Specifications

### Core Features
- Multi-language Support: Rust, TypeScript, SQL, API specs
- Graph Operations: who-calls, who-implements, blast-radius, cycle detection
- Export Formats: JSONL, SQLite, Mermaid, Interface stubs
- Performance: <50ms for 10k nodes, 98%+ compression ratio

### Architecture
- Parser Layer: Language-specific AST parsers
- Graph Engine: petgraph for in-memory operations
- Query Engine: SQLite with custom functions
- Export Layer: Multiple output formats
- CLI Interface: clap-based command structure

### Data Model
```rust
enum NodeKind { Function, Type, Trait }
enum EdgeKind { Calls, Implements, Interacts }

struct GraphNode {
    id: SigHash,           // BLAKE3 signature hash
    kind: NodeKind,
    signature: String,     // Normalized signature
    location: FileLocation,
    metadata: HashMap<String, String>,
}
```

## Success Metrics

### Technical Metrics
- **Performance**: <50ms to extract 10k nodes
- **Accuracy**: >95% signature extraction accuracy
- **Compression**: 98%+ reduction from source to graph
- **Memory**: <1GB RAM for large codebases

### User Experience Metrics
- **Onboarding**: <5 minutes from install to first insights
- **Query Response**: <100ms for complex graph queries
- **Output Quality**: Actionable insights for 90% of queries

## Development Roadmap

### MVP (Weeks 1-4)
- Core Rust parser (syn crate)
- Basic graph construction (petgraph)
- SigHash generation (BLAKE3)
- JSONL export format
- Basic query interface

### V1.0 (Weeks 5-8)
- TypeScript/JavaScript parser
- SQLite integration for complex queries
- Blast radius analysis
- Mermaid visualization export
- Interface stub generation

### V1.5 (Weeks 9-12)
- SQL schema extraction
- API spec parsing (OpenAPI, GraphQL)
- Cross-stack relationship detection
- Interactive query mode
- Cycle detection algorithms

### V2.0 (Weeks 13-16)
- LLM integration (prepared prompts)
- Architecture recommendations
- Performance optimization
- CI/CD integration
- Web interface (optional)

## Target Audience

### Primary Users
- Software Architects: Understanding system design and dependencies
- Senior Developers: Code analysis and refactoring decisions
- Tech Leads: Architecture reviews and team guidance

### Secondary Users
- DevOps Engineers: Understanding deployment dependencies
- QA Engineers: Test coverage analysis
- Product Managers: Feature impact assessment

## Key Differentiators

### Advantages Over Existing Tools
- **Interface-First**: Focuses on signatures rather than implementation
- **Multi-Language**: Full-stack analysis in single tool
- **LLM-Ready**: Optimized for AI-assisted development
- **Performance**: Faster and more efficient than traditional static analysis

### Unique Selling Points
- **3x3 Graph Model**: Simplified yet comprehensive relationship mapping
- **SigHash System**: Stable, collision-resistant identifiers
- **Anti-Coordination**: Aligns with modern architectural principles
- **Compression**: Dramatic reduction in analysis complexity

## Complete Command Reference

```bash
# Installation
cargo install pensieve

# Discovery
pensieve scan
pensieve analyze --verbose

# Extraction
pensieve extract --output ./siggraph.jsonl
pensieve extract --format sqlite --output ./siggraph.db

# Querying
pensieve query "who-calls AuthService::login"
pensieve query "blast-radius User::id --depth 3"
pensieve query "what-implements IDataStore"
pensieve query "find-cycles --in auth_module"
pensieve query --interactive

# Visualization
pensieve visualize --format mermaid --output architecture.md
pensieve visualize --format dot --output graph.dot

# Export
pensieve export-stubs --target rust --output interfaces.rs
pensieve export-stubs --target typescript --output interfaces.ts
pensieve export --format jsonl --output siggraph.jsonl

# Analysis
pensieve check --strict --fail-on-cycles
pensieve metrics --complexity --hotspots
pensieve suggest --refactoring opportunities
```

## Integration Examples

### CI/CD Pipeline
```yaml
# .github/workflows/pensieve-analysis.yml
name: Architecture Analysis
on: [push, pull_request]
jobs:
  pensieve:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Pensieve
        run: cargo install pensieve
      - name: Extract Graph
        run: pensieve extract
      - name: Check for Issues
        run: pensieve check --strict --fail-on-cycles
      - name: Generate Report
        run: pensieve metrics > ARCHITECTURE_REPORT.md
```

### LLM Integration
```bash
# Generate LLM-ready context
pensieve extract --format jsonl | llm-analyze

# Generate interface stubs for AI-assisted coding
pensieve export-stubs --target rust | copilot-chat
```

---

*Document Status: Ideas and Planning*
*Created: 2025-01-18*
*Next Steps: Begin MVP implementation with Rust parser*
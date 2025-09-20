# Parseltongue

Rust-based architectural intelligence system for real-time codebase analysis.

## CLI Interface ✅ IMPLEMENTED

Complete command-line interface with performance monitoring and JSON/human output formats.

### Commands

#### Code Dump Ingestion
```bash
parseltongue ingest <file>
```
Process code dumps with FILE: markers, extracting Rust interface signatures.
- **Performance**: <5s for 2.1MB dumps (monitored and reported)
- **Output**: Files processed, nodes created, execution time

#### Daemon Mode
```bash
parseltongue daemon --watch <directory>
```
Start real-time file monitoring for .rs files with <12ms update latency.

#### Query Operations
```bash
# Find trait implementors
parseltongue query what-implements <trait-name> [--format json]

# Calculate blast radius
parseltongue query blast-radius <entity-name> [--format json]

# Find circular dependencies
parseltongue query find-cycles <entity-name> [--format json]
```
- **Performance**: <500μs simple queries, <1ms complex queries (monitored)
- **Output**: Human-readable or JSON format for LLM consumption

#### LLM Context Generation
```bash
parseltongue generate-context <entity-name> [--format json]
```
Generate comprehensive context with 2-hop dependency analysis for LLM consumption.

### Output Formats
- **Human**: Readable terminal output with performance metrics
- **JSON**: Structured data for LLM integration with execution metadata

## Core Features

### AIM Daemon
Real-time code analysis with Interface Signature Graph (ISG) construction.

**Key Capabilities**:
- **File Monitoring**: <12ms update latency for .rs files
- **Code Dump Ingestion**: <5s processing for 2.1MB dumps with FILE: markers
- **ISG Persistence**: Save/load snapshots with <500ms performance target
- **Entity Lookup**: Find functions, structs, traits by name
- **Dependency Analysis**: Get callers and dependencies for any entity

**Performance Constraints**:
- File updates: <12ms (monitored and reported)
- Snapshot operations: <500ms (monitored and reported)
- Query performance: <500μs simple, <1ms complex (monitored and reported)
- Memory efficient: Arc<RwLock<HashMap<SigHash, Node>>>

### ISG Snapshot System ✅ IMPLEMENTED
Persistent storage of Interface Signature Graph state.

**Features**:
- **JSON Serialization**: Human-readable snapshot format
- **Metadata Tracking**: Version, timestamp, node/edge counts
- **Performance Monitoring**: Automatic constraint validation
- **Graceful Recovery**: Missing snapshots handled transparently
- **Atomic Operations**: Safe concurrent access during save/load

**API**:
```rust
// Save current ISG state to file
daemon.save_snapshot(&path)?;

// Load ISG state from file (handles missing files gracefully)
daemon.load_snapshot(&path)?;
```

## Architecture

### Performance-First Design
All operations include automatic performance monitoring with constraint validation:

- **Ingestion**: Reports timing and warns if >5s for 2.1MB dumps
- **Queries**: Reports execution time and warns if >500μs (simple) or >1ms (complex)
- **File Updates**: Monitors <12ms constraint for live file monitoring
- **Snapshots**: Validates <500ms constraint for save/load operations

### Error Handling
- **Structured Errors**: Complete error hierarchies with context
- **Graceful Degradation**: Performance warnings don't fail operations
- **LLM Integration**: JSON output includes error context for debugging

### Thread Safety
- **Concurrent Access**: Arc<RwLock<>> for safe multi-threaded ISG access
- **Lock Optimization**: Minimal lock duration with early release patterns
- **Resource Management**: RAII cleanup for all file and network resources

## Hook Automation System

### Unified Progress Tracker
Automatically tracks all development activity with intelligent git integration.

**Triggers**: Any file save (excludes `.git/` folder)  
**Actions**:
- Repository snapshots for all changes
- Session context updates
- Git commits only for `.kiro/` directory changes

**Status**: ✅ Active on v01 branch

### Usage
The hook runs automatically on file saves. Manual trigger available via Agent Hooks panel in Kiro IDE.

**Key Files**:
- `.kiro/hooks/unified-progress-tracker.kiro.hook` - Hook configuration
- `.kiro/unified-progress-tracker.sh` - Automation script
- `_refDocs/SESSION_CONTEXT.md` - Current session state

## Installation & Usage

### Building from Source
```bash
cargo build --release
```

### Example Workflow
```bash
# 1. Ingest a code dump
./target/release/parseltongue ingest code_dump.txt

# 2. Query for trait implementors
./target/release/parseltongue query what-implements Iterator --format json

# 3. Generate LLM context for a function
./target/release/parseltongue generate-context my_function

# 4. Start daemon for live monitoring
./target/release/parseltongue daemon --watch ./src
```

## Development

**Branch**: v01  
**Focus**: Rust-only architectural intelligence  
**Constraints**: <12ms updates, LLM-terminal integration  
**Status**: CLI interface complete, daemon core implemented

### Implementation Status
- ✅ CLI command parsing and execution
- ✅ Code dump ingestion with performance monitoring
- ✅ ISG snapshot system with persistence
- ✅ Query operations with timing constraints
- ✅ LLM context generation
- ✅ File monitoring daemon
- ✅ Performance constraint validation

See `.kiro/steering/` for detailed development guidelines.
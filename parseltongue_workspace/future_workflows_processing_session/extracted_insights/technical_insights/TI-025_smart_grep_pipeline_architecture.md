# TI-025: Smart Grep Pipeline Architecture

## Overview
**Description**: Two-stage search pipeline combining ripgrep's textual speed with Parseltongue's semantic accuracy for zero-hallucination code search  
**Source**: DTNote02.md - Smart Grep Pipeline Architecture  
**Strategic Value**: Enables precise, fast code search that provides verified context for LLM integration

## Architecture Design

### Pipeline Stages
1. **Stage 1 - Textual Filtering**: Use `ripgrep` for fast, broad textual search across codebase
2. **Stage 2 - Semantic Validation**: Pipe results through Parseltongue's `what-implements` query for AST-based verification
3. **Stage 3 - Context Generation**: Format results as structured, verifiable context for downstream tools

### Implementation Architectures

**On-Demand Ingestion (Stateless)**:
```bash
rg 'impl MyTrait for' | xargs parseltongue ingest | parseltongue query what-implements MyTrait
```
- Ideal for CI jobs and batch processing
- No persistent state required
- Higher latency but guaranteed accuracy

**Real-Time Daemon (Stateful)**:
```bash
parseltongue daemon --watch ./src &
parseltongue query what-implements MyTrait
```
- Persistent graph maintained in memory
- Sub-second query response times
- Incremental updates with <12ms latency

## Technology Stack
- **Text Search**: ripgrep for initial filtering
- **Semantic Analysis**: Parseltongue AST-based validation
- **Communication**: Pipe-based workflow or JSON-RPC for daemon mode
- **Caching**: JSON snapshot system for daemon persistence
- **Integration**: Cargo subcommand interface

## Performance Requirements
- **Precision**: 93% improvement over text-only search through false positive elimination
- **Recall**: High accuracy including macro-generated code missed by textual search
- **Latency**: Sub-second queries in daemon mode, seconds for on-demand analysis
- **Memory**: <25MB footprint for 100K+ LOC codebases
- **Scalability**: Support for multi-million LOC monorepos with RocksDB persistence

## Integration Patterns
- **Command Line**: `cargo parseltongue grep <pattern>` for seamless workflow integration
- **IDE Integration**: JSON-RPC communication with LSP extensions
- **CI/CD**: Stateless mode for build pipeline integration
- **LLM Context**: Structured output for RAG pipeline consumption

## Security Considerations
- **Sandboxed Analysis**: No code execution, only AST parsing and analysis
- **Input Validation**: Sanitize search patterns and file paths
- **Resource Limits**: Bounded memory and CPU usage for daemon mode
- **Access Control**: Respect file system permissions and .gitignore patterns

## Implementation Details

### Caching Strategy
- **In-Memory**: Complete graph held in daemon process memory
- **Persistence**: JSON snapshot system for quick daemon restarts
- **Invalidation**: File system watching with incremental updates
- **Cold Start**: Full re-ingestion when cache is stale or corrupted

### Error Handling
- **Graceful Degradation**: Fall back to text-only search when semantic analysis fails
- **Clear Messaging**: Informative error messages for debugging and user guidance
- **Retry Logic**: Automatic retry for transient failures in daemon communication
- **Logging**: Comprehensive logging for performance analysis and debugging

## Linked User Journeys
- **UJ-029**: Smart Grep Semantic Search Enhancement
- **UJ-033**: Zero-Hallucination LLM Context Generation

## Cross-References
- **Strategic Theme**: ST-022 Zero-Friction Developer Experience
- **Related Insight**: TI-026 LSP Sidecar Architecture
- **Performance Metrics**: Measured via OpenTelemetry framework (TI-030)
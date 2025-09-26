# TI-026: LSP Sidecar Architecture

## Overview
**Description**: Persistent daemon process providing architectural intelligence via custom LSP extensions without interfering with rust-analyzer  
**Source**: DTNote02.md - IDE Experience Leap via Sidecar LSP Extension  
**Strategic Value**: Enables 8x faster architectural navigation while maintaining IDE performance and compatibility

## Architecture Design

### Core Components
1. **Parseltongue Daemon**: Persistent process maintaining always-current codebase graph
2. **IDE Extension**: Lightweight VS Code/Neovim extension handling communication
3. **Multiplexer Pattern**: Routes standard LSP requests to rust-analyzer, custom queries to Parseltongue
4. **Custom LSP Protocol**: Extensions using `$/parseltongue/` prefix for architectural queries

### Communication Flow
```
IDE Client
    ├── Standard LSP Requests → rust-analyzer
    └── Custom Requests ($/parseltongue/*) → Parseltongue Sidecar
```

### Process Architecture
- **Daemon Process**: `parseltongue daemon --watch ./src` runs independently
- **Extension Process**: Lightweight mediator in IDE process space
- **IPC Mechanism**: JSON-RPC over named pipes or TCP sockets
- **Lifecycle Management**: Automatic daemon restart and health monitoring

## Technology Stack
- **Communication Protocol**: JSON-RPC 2.0 with custom LSP extensions
- **Process Management**: Cross-platform daemon management with auto-restart
- **Graph Storage**: In-memory graph with incremental updates
- **File Watching**: Platform-native file system event monitoring
- **IDE Integration**: VS Code Extension API, Neovim Lua plugins

## Performance Requirements
- **Update Latency**: Sub-12ms incremental graph updates on file changes
- **Query Response**: <100ms for complex architectural queries
- **Memory Footprint**: <25MB for 100K+ LOC codebases
- **CPU Usage**: <5% background CPU usage during idle periods
- **Startup Time**: <2 seconds for daemon initialization

## Integration Patterns

### Custom LSP Messages
```json
{
  "method": "$/parseltongue/blastRadius",
  "params": {
    "symbol": "MyStruct::method",
    "includeTests": true
  }
}
```

### IDE Features Enabled
- **Go to Concrete Implementation**: Cross-crate trait implementation discovery
- **Show Blast Radius**: On-hover impact analysis with visual overlays
- **Find Dependency Cycles**: Real-time circular dependency detection
- **Interactive Visualization**: Embedded webview with dependency graphs

## Security Considerations
- **Process Isolation**: Sidecar runs in separate process space from IDE
- **Resource Limits**: Bounded memory and CPU usage with monitoring
- **File Access**: Respects workspace boundaries and .gitignore patterns
- **Network Security**: Local-only communication, no external network access

## Implementation Details

### Conflict Resolution
- **LSP Namespace**: Use `$/parseltongue/` prefix to avoid conflicts with standard LSP
- **Feature Overlap**: Complement rust-analyzer rather than replace functionality
- **Error Handling**: Graceful fallback when sidecar unavailable
- **Configuration**: Per-project settings for sidecar behavior

### Performance Optimization
- **Incremental Updates**: Only reprocess changed files and their dependencies
- **Lazy Loading**: Load graph sections on-demand for large codebases
- **Caching**: Persistent cache for expensive analysis results
- **Batching**: Batch multiple file changes for efficient processing

### Cross-Platform Support
- **Process Management**: Platform-specific daemon lifecycle management
- **File Watching**: Native file system event APIs (inotify, FSEvents, ReadDirectoryChangesW)
- **IPC**: Cross-platform communication mechanisms
- **IDE Compatibility**: Support for multiple editors and platforms

## Linked User Journeys
- **UJ-032**: IDE Sidecar Performance Enhancement
- **UJ-029**: Smart Grep Semantic Search Enhancement

## Cross-References
- **Strategic Theme**: ST-024 Performance-First Architecture Culture
- **Related Insight**: TI-025 Smart Grep Pipeline Architecture
- **Visualization**: Advanced WebGL console features for graph display
# Parseltongue

A high-performance, async Rust tool for analyzing and processing software projects, starting with efficient ZIP archive handling.

## Vision

Parseltongue aims to be your AI-ready code analysis companion. It processes software projects into an optimized format that both preserves critical metadata and enables efficient LLM-based analysis for:
- Code understanding and debugging
- Dependency analysis
- Security scanning
- Architecture insights

## Current Phase (01): Foundation

Building robust core infrastructure with:

### ZIP Processing
- Zero-copy streaming (no extraction)
- Concurrent processing with backpressure
- Adaptive buffering
- Multi-encoding support (UTF-8, Windows-1252)

### Storage Engine
- Embedded sled database
- Async transactions
- Connection pooling
- Optimized indexing

### Production Infrastructure
- Async/await (Tokio)
- Structured logging
- Performance metrics
- Resource management
- Graceful shutdown

## Usage

### Command Line


# Parseltongue - Phase 01

A high-performance, async Rust tool for processing ZIP archives into an embedded database.

## Vision

Parseltongue aims to be a comprehensive suite of tools for code analysis, dependency tracking, and security scanning of software projects. The name reflects its ability to "speak" to and understand various code formats, much like Harry Potter's parseltongue ability.

## Current Implementation: Phase 01 

Phase 01 focuses on efficient ZIP file processing and database storage with robust error handling and metrics collection.

### Core Features

- **Command Line Interface**  ```bash
  parseltongue [OPTIONS] --input-zip <ZIP_PATH> --output-dir <OUTPUT_DIR>

  Required:
    -i, --input-zip <PATH>    Source ZIP file path
    -o, --output-dir <PATH>   Base output directory

  Options:
    -w, --workers <NUM>       Worker threads (default: CPU cores)
    -b, --buffer-size <SIZE>  Buffer size in bytes (default: adaptive)
    -s, --shutdown <SECS>     Shutdown timeout in seconds (default: 30)
    -v, --verbose            Enable verbose logging
    -h, --help              Show this help message  ```

- **Output Structure**  ```
  output_dir/
  └── {project-name}-{YYYYMMDDHHMMSS}/  # e.g., my-project-20240125143022/
      ├── db/                           # sled database files
      │   ├── entries/                  # File contents and metadata
      │   └── indices/                  # Search and lookup indices
      ├── logs/                         # Operation logs
      │   ├── processing.log            # Main process log
      │   └── error.log                 # Error tracking
      └── metrics/                      # Runtime metrics
          ├── tokio-console.log         # Async runtime metrics
          └── task-metrics.json         # Performance data  ```

### Technical Features

#### ZIP Processing
- Stream-based processing without extraction
- Concurrent entry processing with backpressure
- Adaptive buffer sizing
- CRC validation
- Multiple encoding support (UTF-8, Windows-1252)

#### Database Storage
- Embedded sled database with timestamped directories
- Key-value storage of file paths and contents
- Connection pooling and batched writes
- Async transaction management
- Priority-based scheduling

#### Production Infrastructure
- Structured logging with trace contexts
- Performance metrics collection
- Circuit breakers for failing operations
- Graceful degradation under load
- Clean shutdown handling

### Progress Tracking
- Real-time progress visualization:
  - Files processed count
  - Processing speed
  - Estimated time remaining
  - Current operation status
- Detailed logs in `logs/processing.log`
- Performance metrics in `metrics/`

### Status & Roadmap

Phase 01 establishes core infrastructure. Future phases will introduce:
- Code analysis capabilities
- LLM-optimized output formats
- Dependency tracking
- Security scanning features
- Architecture analysis tools

### Contributing

This is an active project under development. See CONTRIBUTING.md for guidelines.

### Dependencies
- **Core**: tokio, sled, zip
- **Utilities**: clap, anyhow, tracing
- **Metrics**: metrics, tokio-console
- **Encoding**: encoding_rs

### Non-Goals for Phase 01
- ZIP extraction functionality
- Code analysis features
- Summary generation
- Dependency analysis
- Security scanning
- Distributed processing
- Persistent metrics storage
- Runtime reconfiguration

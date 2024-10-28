# Parseltongue - Phase 01

A high-performance, async Rust tool for processing ZIP archives into an embedded database.

## Vision

Parseltongue aims to be a comprehensive suite of tools for code analysis, dependency tracking, and security scanning of software projects. The name reflects its ability to "speak" to and understand various code formats, much like Harry Potter's parseltongue ability.

## Current Implementation: Phase 01 

Phase 01 focuses on efficient ZIP file processing and database storage with robust error handling and metrics collection.

### Core Features

- **Command Line Interface**
  - Required: Input ZIP path and output directory
  - Optional: Verbosity, workers, buffer size, shutdown timeout
  - Progress visualization

- **ZIP Processing**
  - Stream-based processing without extraction
  - Concurrent entry processing with backpressure
  - Adaptive buffer sizing
  - CRC validation
  - Multiple encoding support (UTF-8, Windows-1252)

- **Database Storage**
  - Embedded sled database with timestamped directories
  - Key-value storage of file paths and contents
  - Connection pooling and batched writes
  - Async transaction management
  - Priority-based scheduling

- **Production Infrastructure**
  - Structured logging with trace contexts
  - Performance metrics collection
  - Circuit breakers for failing operations
  - Graceful degradation under load
  - Clean shutdown handling

### Usage

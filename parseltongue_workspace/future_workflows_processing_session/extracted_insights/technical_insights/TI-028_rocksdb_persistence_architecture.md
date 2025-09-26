# TI-028: RocksDB Persistence Architecture

## Overview
**Description**: On-disk storage system using RocksDB for multi-million LOC monorepo analysis with predictable performance characteristics  
**Source**: DTNote02.md - Scaling to Monorepos & Multi-Repo Analysis  
**Strategic Value**: Enables analysis of enterprise-scale codebases with 16-minute cold-start for 3M LOC and efficient incremental updates

## Architecture Design

### Storage Schema Design
**Key-Value Structure** (inspired by NebulaGraph):
```
Vertex Keys: "v:{vertex_id}"
Edge Keys: "e:{src_id}:{edge_type}:{dst_id}"
Index Keys: "i:{index_type}:{value}:{vertex_id}"
Metadata Keys: "m:{metadata_type}:{key}"
```

### Dual-Edge Representation
- **Forward Edges**: Enable efficient traversal from source to destination
- **Reverse Edges**: Support reverse traversal and bidirectional queries
- **Edge Types**: Calls, Uses, Implements, Contains, Depends relationships
- **Composite Keys**: Enable range scans and efficient lookups

### Data Serialization
- **Format**: `rkyv` zero-copy serialization for maximum performance
- **Compression**: Zstandard (ZSTD) with per-level compression in RocksDB
- **Schema Evolution**: Versioned serialization format for backward compatibility
- **Batch Operations**: Efficient bulk loading and updates

## Technology Stack
- **Database**: RocksDB LSM-tree storage engine
- **Serialization**: rkyv for zero-copy deserialization
- **Compression**: ZSTD for space efficiency
- **Indexing**: Custom composite key design for graph queries
- **Caching**: RocksDB block cache and OS page cache integration

## Performance Requirements
- **Cold Start**: 16-minute initialization for 3M LOC codebases
- **Query Latency**: Sub-millisecond for single-hop queries, <100ms for complex traversals
- **Memory Usage**: Configurable cache sizes, efficient memory utilization
- **Disk Usage**: Compressed storage with predictable space requirements
- **Concurrent Access**: Multi-reader support with write serialization

## Integration Patterns

### Cache Management
- **Multi-Level Caching**: RocksDB block cache, application-level graph cache
- **Cache Keys**: Composite keys based on `(crate_name, version, source_checksum, enabled_features)`
- **Invalidation Strategy**: File system watching with incremental updates
- **Warm-up**: Background cache population for frequently accessed data

### Workspace Handling
- **External Crates**: Shallow AST stubs via `rustdoc --output-format json`
- **Dependency Versioning**: Cargo.lock as source of truth for version resolution
- **Feature Flags**: Support for conditional compilation and feature-dependent analysis
- **Cross-Repository**: Hash-based sharding for multi-repo analysis

## Security Considerations
- **Data Integrity**: Checksums and validation for all stored data
- **Access Control**: File system permissions and workspace boundaries
- **Concurrent Safety**: ACID properties for graph modifications
- **Backup Strategy**: Incremental backup support for large datasets

## Implementation Details

### Sharding Strategy
- **Hash-Based Partitioning**: Distribute graph nodes across multiple RocksDB instances
- **Cross-Shard Queries**: Efficient query planning for multi-shard traversals
- **Load Balancing**: Even distribution of data and query load
- **Fault Tolerance**: Graceful handling of shard unavailability

### Schema Evolution
- **Version Management**: Schema version tracking in metadata keys
- **Migration Support**: Automated migration between schema versions
- **Backward Compatibility**: Support for reading older data formats
- **Validation**: Schema validation during read and write operations

### Performance Optimization
- **Bloom Filters**: Reduce disk I/O for non-existent key lookups
- **Compaction Strategy**: Optimized LSM-tree compaction for graph workloads
- **Batch Processing**: Efficient bulk operations for large updates
- **Memory Management**: Careful memory allocation and deallocation patterns

### Monitoring and Diagnostics
- **Metrics Collection**: RocksDB internal metrics and custom application metrics
- **Performance Profiling**: Query performance analysis and optimization
- **Health Checks**: Database integrity validation and corruption detection
- **Debugging Tools**: Graph visualization and query execution analysis

## Linked User Journeys
- **Multiple**: Scaling foundation for all user journeys in large codebases
- **Enterprise Workflows**: Monorepo analysis and cross-repository queries

## Cross-References
- **Strategic Theme**: ST-024 Performance-First Architecture Culture
- **Related Insight**: TI-026 LSP Sidecar Architecture (for caching integration)
- **Scalability**: Supports advanced visualization and plugin architectures
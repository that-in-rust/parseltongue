# CodeGraphContext.json Schema Specification

## Overview

CodeGraphContext.json serves as the critical bridge between CozoDB and the LLM, providing optimized context that stays under the 100k token limit while excluding Current_Code to prevent bloat. This specification defines the exact JSON structure, size optimization rules, and token counting methodology.

## Schema Definition

### Root Structure

```json
{
  "version": "1.0",
  "generated_at": "2025-10-29T16:00:00Z",
  "token_count": 48732,
  "size_status": "optimal",
  "query_context": {
    "base_entities": ["src/models/user.rs-models-User"],
    "hop_depth": 2,
    "change_type": "edit"
  },
  "entities": [
    {
      "isgl1_key": "src/models/user.rs-models-User",
      "interface_signature": { /* Interface details */ },
      "tdd_classification": { /* TDD metadata */ },
      "lsp_metadata": { /* LSP enhanced data */ },
      "relevance_score": 1.0,
      "dependency_level": 0
    }
  ],
  "relationships": [
    {
      "dependent": "src/models/user.rs-models-User",
      "dependency": "src/database.rs-database-Database",
      "relationship_type": "implements",
      "strength": 0.9
    }
  ],
  "optimization_info": {
    "excluded_entities": [],
    "truncation_applied": false,
    "prioritization_strategy": "blast_radius"
  }
}
```

### Entity Schema

```json
{
  "isgl1_key": "src/models/user.rs-models-User",
  "interface_signature": {
    "entity_type": "struct",
    "name": "User",
    "visibility": "public",
    "file_path": "src/models/user.rs",
    "line_range": {
      "start": 15,
      "end": 42
    },
    "signature": {
      "fields": [
        {
          "name": "id",
          "type": "Uuid",
          "visibility": "public"
        },
        {
          "name": "name",
          "type": "String",
          "visibility": "public"
        },
        {
          "name": "email",
          "type": "String",
          "visibility": "private"
        }
      ],
      "generics": [],
      "traits": ["Clone", "Debug", "Serialize"]
    },
    "module_path": ["models"],
    "documentation": "Represents a user in the system with basic profile information."
  },
  "tdd_classification": {
    "testability": "high",
    "complexity": "low",
    "dependencies": 3,
    "test_coverage_estimate": 0.85,
    "critical_path": false,
    "change_risk": "low"
  },
  "lsp_metadata": {
    "type_information": {
      "resolved_types": {
        "id": "uuid::Uuid",
        "name": "alloc::string::String",
        "email": "alloc::string::String"
      },
      "trait_impls": {
        "inferred": ["Clone", "Debug"],
        "explicit": ["serde::Serialize"]
      }
    },
    "usage_analysis": {
      "references": 12,
      "call_sites": ["src/handlers/user_handler.rs", "src/services/auth.rs"],
      "imports": ["use uuid::Uuid;"]
    },
    "semantic_tokens": {
      "type": "struct",
      "modifier": ["public", "declaration"]
    }
  },
  "relevance_score": 1.0,
  "dependency_level": 0
}
```

### Function Entity Schema

```json
{
  "isgl1_key": "src/models/user.rs-models-new_user",
  "interface_signature": {
    "entity_type": "function",
    "name": "new_user",
    "visibility": "public",
    "file_path": "src/models/user.rs",
    "line_range": {
      "start": 44,
      "end": 52
    },
    "signature": {
      "parameters": [
        {
          "name": "name",
          "type": "String",
          "pattern": "owned"
        },
        {
          "name": "email",
          "type": "String",
          "pattern": "owned"
        }
      ],
      "return_type": "Result<Self, UserError>",
      "async": false,
      "unsafe": false,
      "const": false
    },
    "module_path": ["models"],
    "documentation": "Creates a new user instance with validation."
  },
  "tdd_classification": {
    "testability": "high",
    "complexity": "medium",
    "dependencies": 1,
    "test_coverage_estimate": 0.90,
    "critical_path": true,
    "change_risk": "medium"
  },
  "lsp_metadata": {
    "type_information": {
      "resolved_types": {
        "name": "alloc::string::String",
        "email": "alloc::string::String",
        "return": "Result<models::User, models::UserError>"
      }
    },
    "usage_analysis": {
      "references": 8,
      "call_sites": ["src/handlers/user_handler.rs", "tests/user_tests.rs"]
    }
  },
  "relevance_score": 0.9,
  "dependency_level": 1
}
```

### Relationship Schema

```json
{
  "dependent": "src/models/user.rs-models-User",
  "dependency": "src/traits/database.rs-traits-Database",
  "relationship_type": "implements",
  "strength": 0.9,
  "context": {
    "implementation_location": "src/models/user.rs:25-35",
    "trait_methods": ["save", "find_by_id", "delete"]
  }
}
```

## Size Optimization Strategy

### Token Counting Methodology

**Base Token Estimates**:
- **JSON Structure Overhead**: ~5 tokens per entity
- **Field Names**: ~1-2 tokens per field name
- **String Content**: ~1.3 tokens per character (approximate)
- **Numbers**: ~1 token per number
- **Booleans**: ~1 token per boolean

**Entity Token Budget Allocation**:
```
Total Budget: 100,000 tokens
├── JSON Structure: 5,000 tokens (5%)
├── Entity Data: 80,000 tokens (80%)
├── Relationships: 10,000 tokens (10%)
├── Metadata: 5,000 tokens (5%)
```

### Content Prioritization

**Priority 1 (Always Included)**:
- Base entities specified in query
- Direct dependencies (1-hop)
- Critical interface changes
- Test interface modifications

**Priority 2 (Space Permitting)**:
- 2-hop dependencies
- Related non-test interfaces
- Supporting type definitions
- LSP metadata for Rust entities

**Priority 3 (Excluded if Needed)**:
- 3+ hop dependencies
- Generic type implementations
- Documentation strings
- Low-relevance metadata

### Dynamic Truncation

When token limit is exceeded:

1. **Remove Priority 3**: Drop low-relevance entities
2. **Compress Signatures**: Simplify complex type signatures
3. **Truncate Documentation**: Remove or shorten doc strings
4. **Limit Relationships**: Keep only strongest relationships

**Truncation Example**:
```json
// Before Truncation (120 tokens)
"documentation": "Creates a new user instance with email validation and duplicate checking. Returns UserError if email is invalid or user already exists."

// After Truncation (15 tokens)
"documentation": "Creates new user with validation."
```

## Query Context Specification

### Base Query Parameters

```json
{
  "query_context": {
    "base_entities": ["src/models/user.rs-models-User"],
    "hop_depth": 2,
    "change_type": "edit",
    "focus_areas": ["test_interfaces", "non_test_interfaces"],
    "size_limit": 100000,
    "optimization_strategy": "blast_radius"
  }
}
```

### Change Types

- **"edit"**: Modifying existing entity
- **"create"**: Creating new entity
- **"delete"**: Deleting existing entity
- **"refactor"**: Large-scale refactoring operation

### Hop Depth Configuration

- **0**: Only base entities
- **1**: Direct dependencies only
- **2**: Dependencies of dependencies (default for most operations)
- **3**: Deep analysis (used for complex refactorings)

## Relevance Scoring Algorithm

### Base Scoring

```rust
fn calculate_relevance_score(entity: &Entity, query: &Query) -> f64 {
    let mut score = 0.0;

    // Base entity: 1.0
    if query.base_entities.contains(&entity.isgl1_key) {
        score += 1.0;
    }

    // Direct dependency: 0.8
    if is_direct_dependency(entity, &query.base_entities) {
        score += 0.8;
    }

    // 2-hop dependency: 0.6
    if is_n_hop_dependency(entity, &query.base_entities, 2) {
        score += 0.6;
    }

    // Test interface: +0.2
    if is_test_interface(entity) {
        score += 0.2;
    }

    // Critical path: +0.3
    if is_critical_path(entity) {
        score += 0.3;
    }

    score.min(1.0)
}
```

### Dependency Level Calculation

```rust
fn calculate_dependency_level(entity: &Entity, base_entities: &[String]) -> u32 {
    if base_entities.contains(&entity.isgl1_key) {
        return 0; // Base entity
    }

    if is_direct_dependency(entity, base_entities) {
        return 1; // Direct dependency
    }

    // Calculate minimum hop distance
    for n in 2..=query.max_hop_depth {
        if is_n_hop_dependency(entity, base_entities, n) {
            return n;
        }
    }

    query.max_hop_depth + 1 // Beyond requested depth
}
```

## Performance Targets

### Generation Performance

- **Small Projects** (< 100 entities): <100ms
- **Medium Projects** (100-1000 entities): <300ms
- **Large Projects** (1000+ entities): <500ms

### Size Targets

- **Average Context Size**: 50,000-80,000 tokens
- **Maximum Context Size**: 100,000 tokens (hard limit)
- **Minimum Useful Context**: 5,000 tokens

### Quality Metrics

- **Relevance Coverage**: >90% of relevant entities included
- **Completeness**: All direct dependencies included
- **Accuracy**: Correct dependency relationships maintained

## Error Handling

### Size Limit Errors

```json
{
  "error": "context_size_exceeded",
  "message": "Context generation exceeded 100,000 token limit",
  "attempted_size": 125000,
  "suggestion": "Reduce hop_depth or limit query scope"
}
```

### Entity Not Found Errors

```json
{
  "error": "entity_not_found",
  "message": "Base entity not found in codebase",
  "missing_entities": ["src/models/user.rs-models-NonExistent"],
  "suggestion": "Check ISGL1 key format and entity existence"
}
```

### Dependency Graph Errors

```json
{
  "error": "dependency_cycle_detected",
  "message": "Circular dependency detected in relationship graph",
  "cycle_path": ["A", "B", "C", "A"],
  "suggestion": "Resolve circular dependencies before context generation"
}
```

## Caching Strategy

### Context Cache Key

```rust
fn generate_cache_key(query: &Query) -> String {
    format!(
        "ctx_{}_{}_{}_{}",
        hash_entities(&query.base_entities),
        query.hop_depth,
        query.change_type,
        query.size_limit
    )
}
```

### Cache Validation

Context cache is valid when:
- No files have been modified since generation
- Database schema hasn't changed
- Query parameters match exactly

## MVP Constraints

Following ultra-minimalist principles for MVP (~10 users):

1. **Simple JSON Structure**: Basic nested objects, no complex schema validation
2. **Fixed Token Limits**: Hard 100k token limit, no dynamic scaling
3. **Manual Truncation**: Simple size-based truncation, no intelligent prioritization
4. **Basic Relevance Scoring**: Simple hop-distance based scoring
5. **No Compression**: No compression or binary format optimization

This specification provides a robust foundation for LLM context generation while maintaining simplicity and reliability for the MVP target audience.
# CozoDB Schema Specification

## Overview

This document defines the exact CozoDB schema for the Parseltongue MVP implementation, following the ultra-minimalist principles established in the PRD documents.

## Core Tables

### 1. CodeGraph Table

**Purpose**: Primary storage for all code entities with temporal versioning capabilities.

**Schema**:
```datalog
/**
 * CodeGraph: Main table storing all code entities with temporal versioning
 * Primary Key: ISGL1_key
 * Temporal State: (current_ind, future_ind, Future_Action)
 */
? {
    // Primary identifier following ISGL1 format: filepath-filename-InterfaceName
    ISGL1_key: String,

    // Code content (stored as text for simplicity in MVP)
    Current_Code: String? = null,        // null for new entities
    Future_Code: String? = null,         // null for unchanged entities

    // Interface signature as structured JSON
    interface_signature: Json = null,

    // TDD classification for context optimization
    TDD_Classification: Json = null,

    // LSP metadata (Rust-only enhancement)
    lsp_meta_data: Json = null,

    // Temporal versioning flags
    current_ind: Bool = true,            // exists in current state
    future_ind: Bool = true,             // exists in future state
    Future_Action: String? = null,       // "Create", "Edit", "Delete", or null

    // Metadata
    file_path: String,                   // full file path
    language: String,                    // programming language
    last_modified: String,               // ISO 8601 timestamp
    entity_type: String,                 // "function", "struct", "trait", "impl", etc.
}
```

**Constraints**:
- `ISGL1_key` must be unique across all entities
- `current_ind` and `future_ind` cannot both be false
- `Future_Action` must be null when both indicators are true (no change)
- `Current_Code` must be non-null when `current_ind` is true
- `Future_Code` must be non-null when `future_ind` is true and `Future_Action` is "Edit"

### 2. Dependencies Table

**Purpose**: Store dependency relationships between code entities for hopping analysis and blast radius calculations.

**Schema**:
```datalog
/**
 * Dependencies: Relationships between code entities
 * Enables 1-hop, 2-hop, and N-hop analysis for context optimization
 */
? {
    // Dependency relationship
    dependent_ISGL1: String,            // entity that depends on dependency
    dependency_ISGL1: String,           // entity being depended upon

    // Relationship metadata
    relationship_type: String,          // "imports", "calls", "implements", "inherits", etc.
    strength: Float = 1.0,              // relationship strength for prioritization

    // Temporal versioning
    current_ind: Bool = true,           // exists in current state
    future_ind: Bool = true,            // exists in future state
    Future_Action: String? = null,      // "Create", "Delete", or null

    // Metadata
    file_context: String,               // where this dependency was found
    line_number: Int? = null,           // line number of dependency (if available)
}
```

**Constraints**:
- Both `dependent_ISGL1` and `dependency_ISGL1` must reference existing entities in CodeGraph
- `relationship_type` must be from predefined set of valid types
- `strength` must be between 0.0 and 1.0

### 3. Context_Optimization Table

**Purpose**: Cache computed context optimization results for performance.

**Schema**:
```datalog
/**
 * Context_Optimization: Pre-computed context for common queries
 * Enables sub-500ms context generation performance target
 */
? {
    // Query identifier
    query_hash: String,                 // hash of the query parameters

    // Cached context data
    included_entities: Json,            // array of ISGL1 keys included
    excluded_entities: Json,            // array of ISGL1 keys excluded due to size limits
    token_count: Int,                   // total token count of this context

    // Metadata
    created_at: String,                 // ISO 8601 timestamp
    hit_count: Int = 0,                 // number of times this cache was used
    last_used: String? = null,          // last usage timestamp
}
```

## Temporal Versioning Logic

### State Transitions

The temporal versioning system uses three boolean flags to track entity state transitions:

| current_ind | future_ind | Future_Action | Meaning | Action Required |
|-------------|------------|---------------|---------|-----------------|
| 1 | 1 | null | **Exists → Continues** | No action needed |
| 1 | 0 | "Delete" | **Exists → Delete** | Mark for deletion |
| 0 | 1 | "Create" | **Create → Exists** | Create new entity |
| 1 | 1 | "Edit" | **Exists → Modified** | Update with Future_Code |

### Transition Rules

1. **Create Operation**:
   ```
   current_ind: 0, future_ind: 1, Future_Action: "Create"
   Current_Code: null, Future_Code: <new_code>
   ```

2. **Edit Operation**:
   ```
   current_ind: 1, future_ind: 1, Future_Action: "Edit"
   Current_Code: <old_code>, Future_Code: <new_code>
   ```

3. **Delete Operation**:
   ```
   current_ind: 1, future_ind: 0, Future_Action: "Delete"
   Current_Code: <existing_code>, Future_Code: null
   ```

4. **No Change**:
   ```
   current_ind: 1, future_ind: 1, Future_Action: null
   Current_Code: <existing_code>, Future_Code: null
   ```

## Query Patterns

### 1. Temporal State Query

**Purpose**: Get all entities that will change in the future state.

```datalog
?changed_entities[ISGL1_key, Current_Code, Future_Code, Future_Action] :=
    *CodeGraph{ISGL1_key, Current_Code, Future_Code, future_ind, Future_Action},
    future_ind == true,
    Future_Action != null
```

### 2. Dependency Hopping Query

**Purpose**: Get N-hop dependencies for a given entity.

```datalog
?hop_1[entity, dependency] :=
    *Dependencies{dependent_ISGL1: entity, dependency_ISGL1: dependency, current_ind: true}

?hop_2[entity, hop2_dependency] :=
    ?hop_1[entity, intermediate],
    *Dependencies{dependent_ISGL1: intermediate, dependency_ISGL1: hop2_dependency, current_ind: true}

?hop_n[entity, n_dependency, distance] <~ n:1..3 :=
    ?hop_1[entity, direct_dependency],
    distance = 1,
    n_dependency = direct_dependency;

    ?hop_n[entity, intermediate, n],
    *Dependencies{dependent_ISGL1: intermediate, dependency_ISGL1: n_dependency, current_ind: true},
    distance = n + 1
```

### 3. Context Generation Query

**Purpose**: Generate optimized context for LLM reasoning with size limits.

```datalog
?context_entities[ISGL1_key, interface_signature, TDD_Classification, lsp_meta_data] :=
    *CodeGraph{ISGL1_key, interface_signature, TDD_Classification, lsp_meta_data, current_ind: true},
    // Additional filtering logic for size optimization
    // Exclude Current_Code to prevent context bloat
```

## Performance Optimizations

### Indexes

1. **Primary Index**: `ISGL1_key` on CodeGraph table
2. **Temporal Index**: `(current_ind, future_ind, Future_Action)` on CodeGraph table
3. **Dependency Index**: `dependent_ISGL1` on Dependencies table
4. **Query Cache Index**: `query_hash` on Context_Optimization table

### Data Types

- **String**: Used for all text data in MVP for simplicity
- **Json**: Structured metadata stored as JSON for flexibility
- **Bool**: Temporal indicators
- **Float**: Relationship strengths
- **Int**: Counts and line numbers

### Storage Strategy

- **SQLite Backend**: Using `storage-sqlite` feature for simplicity and reliability
- **Single File Database**: All data stored in single `.cozo` file
- **ACID Compliance**: Ensures data consistency during operations

## Error Handling

### Constraint Violations

1. **Unique Key Violation**: Duplicate ISGL1_key - reject with clear error message
2. **Invalid Temporal State**: Invalid combination of current_ind, future_ind, Future_Action
3. **Missing Dependencies**: References to non-existent entities
4. **Context Size Overflow**: Generated context exceeds token limits

### Recovery Strategies

1. **Transaction Rollback**: Rollback entire operation on constraint violation
2. **Partial State Reset**: Use Tool 6 to reset to clean state if corruption detected
3. **Data Validation**: Run consistency checks after major operations

## MVP Constraints

Following ultra-minimalist principles for MVP (~10 users):

1. **Single Database File**: No complex database clustering or replication
2. **Simple Schema**: No advanced features like triggers or stored procedures
3. **Manual Backups**: No automated backup system (users can copy .cozo file)
4. **Basic Constraints**: Essential constraints only, no complex validation rules
5. **Direct Query Access**: No query abstraction layer, use CozoDB queries directly

This schema provides the foundation for the complete Parseltongue workflow while maintaining simplicity and reliability for the MVP target audience.
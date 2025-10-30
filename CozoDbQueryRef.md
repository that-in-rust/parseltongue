# CozoDB Query Reference for Parseltongue

This document provides reference queries for the Parseltongue reasoning LLM to extract relevant code context from the CozoDB database.

## ISGL1 Key Format (Updated 2025-10-30)

**Format**: `{language}:{type}:{name}:{sanitized_path}:{start_line}-{end_line}`

**Examples:**
- `rust:fn:calculate_sum:src_lib_rs:42-56` - Function in Rust
- `rust:struct:Calculator:src_types_rs:10-25` - Struct in Rust
- `python:fn:process_data:lib_utils_py:100-120` - Function in Python

**Rationale:** This format provides language-specific filtering, entity type identification, unique naming with file context, line ranges for multi-version tracking, and URL-safe characters.

## Database Schema

### Primary Relation: `CodeGraph`
```
isgl1_key (Primary Key) | current_code | future_code | interface_signature | lsp_metadata | tdd_classification | current_ind | future_ind | future_action
```

**Field Descriptions:**
- `isgl1_key`: ISGL1 identifier (format above)
- `current_code`: Current implementation code
- `future_code`: Proposed future implementation (empty initially)
- `interface_signature`: Function/method signature information (JSON)
- `lsp_metadata`: Language Server Protocol metadata from rust-analyzer (JSON, optional)
- `tdd_classification`: Object with `entity_class` ("TEST" | "CODE"), testability, complexity, dependencies, etc.
- `current_ind`: Boolean - entity exists in current state
- `future_ind`: Boolean - entity will exist in future state
- `future_action`: "Create" | "Edit" | "Delete" | null

## Common Query Patterns

### 1. Basic Interface Retrieval
```datalog
# Get all current code for a specific interface
?[isgl1_key, current_code, interface_signature] :=
    *CodeGraph{isgl1_key, current_code, interface_signature}

# Filter by specific function name
?[isgl1_key, current_code, interface_signature] :=
    *CodeGraph{isgl1_key, current_code, interface_signature},
    isgl1_key ~ "rust:fn:calculate_sum:.*"
```

### 2. Module/File Level Queries
```datalog
# Get all interfaces from a specific file (note: paths use _ not /)
?[isgl1_key, current_code, interface_signature] :=
    *CodeGraph{isgl1_key, current_code, interface_signature},
    isgl1_key ~ "rust:.*:.*:src_database_.*"

# Get all functions from src/main.rs
?[isgl1_key, interface_signature] :=
    *CodeGraph{isgl1_key, interface_signature},
    isgl1_key ~ "rust:fn:.*:src_main_rs:.*"
```

### 3. Test-related Queries (Using entity_class)
```datalog
# Get all test functions
?[isgl1_key, interface_signature, tdd_classification] :=
    *CodeGraph{isgl1_key, interface_signature, tdd_classification},
    tdd_classification.entity_class == "TEST"

# Get all production code (non-tests)
?[isgl1_key, interface_signature, tdd_classification] :=
    *CodeGraph{isgl1_key, interface_signature, tdd_classification},
    tdd_classification.entity_class == "CODE"
```

### 4. Signature-based Queries
```cozo
# Find all methods with specific parameter patterns
?[ISGL1, Current_Code, interface_signature] <-
  code_chunks {ISGL1, Current_Code, interface_signature},
  contains(interface_signature, "timeout:")
```

### 5. Complex Multi-pattern Queries
```cozo
# Get all database-related code and their tests
?[ISGL1, Current_Code, interface_signature, TDD_Classification] <-
  code_chunks {ISGL1, Current_Code, interface_signature, TDD_Classification},
  (contains(ISGL1, "database") OR contains(interface_signature, "Database"))
```

## Query Templates for Common Change Requests

### Adding Parameters to Interfaces
```cozo
# Find all database connection methods
?[ISGL1, Current_Code, interface_signature] <-
  code_chunks {ISGL1, Current_Code, interface_signature},
  (contains(interface_signature, "connect") OR contains(ISGL1, "Connection"))
```

### Converting Sync to Async
```cozo
# Find all synchronous database operations
?[ISGL1, Current_Code, interface_signature] <-
  code_chunks {ISGL1, Current_Code, interface_signature},
  (contains(interface_signature, "-> Result<") AND NOT contains(interface_signature, "async"))
```

### Adding Error Handling
```cozo
# Find methods without explicit error handling
?[ISGL1, Current_Code, interface_signature] <-
  code_chunks {ISGL1, Current_Code, interface_signature},
  NOT contains(interface_signature, "Result<")
```

### Finding Test Coverage
```cozo
# Get all test implementations
?[ISGL1, Current_Code, TDD_Classification] <-
  code_chunks {ISGL1, Current_Code, TDD_Classification},
  TDD_Classification = "TEST_IMPLEMENTATION"
```

## Query Best Practices

### 1. Use Specific Patterns
- Target exact interface names when possible
- Use file path prefixes for module-specific queries
- Leverage `contains()` for pattern matching

### 2. Include Relevant Metadata
- Always request `interface_signature` for understanding APIs
- Include `lsp_meta_data` for type information
- Consider `TDD_Classification` for test-related queries

### 3. Optimize Query Scope
- Start with broader queries, then refine
- Use file path filtering to reduce result sets
- Combine multiple conditions with `AND`/`OR`

### 4. Handle Large Results
- Use `LIMIT` for exploratory queries
- Break complex queries into multiple simpler ones
- Consider pagination for very large codebases

## Integration with Parseltongue Workflow

When the reasoning LLM processes a change request:

1. **Analyze the Request**: Identify what types of code/context are needed
2. **Construct Query**: Use the patterns above to create appropriate CozoDB queries
3. **Execute via Tool 2**: Pass the query to `cozo-to-context-writer`
4. **Receive Context**: Process the `CodeGraphContext.json` output
5. **Plan Changes**: Use the context to generate change specifications

## Example LLM Query Generation

**User Request**: "Add timeout parameter to all database connection methods"

**LLM-generated Query**:
```cozo
?[ISGL1, Current_Code, interface_signature] <-
  code_chunks {ISGL1, Current_Code, interface_signature},
  (contains(interface_signature, "connect") OR contains(ISGL1, "Connection") OR contains(ISGL1, "database"))
```

**Result Processing**: The LLM would receive all connection-related interfaces and analyze their signatures to plan the timeout parameter addition.

---

*This reference serves as a guide for the Parseltongue reasoning LLM to construct effective CozoDB queries for various code modification scenarios.*
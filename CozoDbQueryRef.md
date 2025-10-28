# CozoDB Query Reference for Parseltongue

This document provides reference queries for the Parseltongue reasoning LLM to extract relevant code context from the CozoDB database.

## Database Schema

### Primary Relation: `code_chunks`
```
ISGL1 (Primary Key) | Current_Code | Future_Code | interface_signature | lsp_meta_data | TDD_Classification | current_id | future_id
```

**Field Descriptions:**
- `ISGL1`: Interface Signature Graph Level 1 identifier (format: `filepath-filename-InterfaceName`)
- `Current_Code`: Current implementation code
- `Future_Code`: Proposed future implementation (empty initially)
- `interface_signature`: Function/method signature information
- `lsp_meta_data`: Language Server Protocol metadata (types, dependencies)
- `TDD_Classification`: `TEST_IMPLEMENTATION` or `CODE_IMPLEMENTATION`
- `current_id`: Current state identifier
- `future_id`: Future state identifier

## Common Query Patterns

### 1. Basic Interface Retrieval
```cozo
# Get all current code for a specific interface
?[ISGL1, Current_Code, interface_signature] <-
  code_chunks {ISGL1, Current_Code, interface_signature}

# Filter by specific interface name pattern
?[ISGL1, Current_Code, interface_signature] <-
  code_chunks {ISGL1, Current_Code, interface_signature},
  contains(ISGL1, "DatabaseConnection")
```

### 2. Module/File Level Queries
```cozo
# Get all interfaces from a specific file
?[ISGL1, Current_Code, interface_signature] <-
  code_chunks {ISGL1, Current_Code, interface_signature},
  contains(ISGL1, "src/database/")

# Get all interfaces from a specific module
?[ISGL1, Current_Code, interface_signature] <-
  code_chunks {ISGL1, Current_Code, interface_signature},
  contains(ISGL1, "mod-auth")
```

### 3. Relationship-based Queries
```cozo
# Find all interfaces that depend on a specific type
?[ISGL1, Current_Code, lsp_meta_data] <-
  code_chunks {ISGL1, Current_Code, lsp_meta_data},
  contains(lsp_meta_data, "AuthResult")

# Get test implementations for specific code
?[ISGL1, Current_Code, TDD_Classification] <-
  code_chunks {ISGL1, Current_Code, TDD_Classification},
  TDD_Classification = "TEST_IMPLEMENTATION"
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
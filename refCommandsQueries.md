# Parseltongue Commands & Queries Reference

**Purpose**: Comprehensive reference for all 6 parseltongue tools, CozoDB query patterns, and command-line interfaces.

**Target Audience**: Developers, LLMs, automation scripts, and CI/CD pipelines.

**Last Updated**: 2025-11-01 (Version 0.7.0)

---

## Table of Contents

1. [Tool Overview](#tool-overview)
2. [Tool 1: folder-to-cozodb-streamer](#tool-1-folder-to-cozodb-streamer)
3. [Tool 2: llm-to-cozodb-writer](#tool-2-llm-to-cozodb-writer)
4. [Tool 3: llm-cozodb-to-context-writer](#tool-3-llm-cozodb-to-context-writer)
5. [Tool 4: rust-preflight-code-simulator](#tool-4-rust-preflight-code-simulator)
6. [Tool 5: llm-cozodb-to-diff-writer](#tool-5-llm-cozodb-to-diff-writer)
7. [Tool 6: cozodb-make-future-code-current](#tool-6-cozodb-make-future-code-current)
8. [CozoDB Query Patterns](#cozodb-query-patterns)
9. [Database Schema](#database-schema)
10. [Common Workflows](#common-workflows)
11. [Performance & Optimization](#performance--optimization)
12. [Troubleshooting](#troubleshooting)

---

## Tool Overview

Parseltongue consists of 6 ultra-minimalist Rust tools that form a complete code analysis and modification pipeline:

| Tool | Binary Name | Purpose | Phase | Input | Output |
|------|-------------|---------|-------|-------|--------|
| **1** | `folder-to-cozodb-streamer` | Index codebase | Analysis | Source files | CozoDB entities |
| **2** | `llm-to-cozodb-writer` | LLM reasoning → DB | Specification | LLM reasoning | Temporal flags |
| **3** | `llm-cozodb-to-context-writer` | DB → LLM context | Optimization | CozoDB query | JSON context |
| **4** | `rust-preflight-code-simulator` | Pre-flight validation | Validation | future_code | Validation report |
| **5** | `llm-cozodb-to-diff-writer` | Generate file diffs | Writing | CozoDB state | CodeDiff.json |
| **6** | `cozodb-make-future-code-current` | State reset | Cleanup | CozoDB + files | Fresh state |

**Database Backend**: CozoDB with RocksDB or in-memory storage
**Query Language**: Datalog (CozoDB native query language)
**Data Model**: Temporal versioning with `current_ind`, `future_ind`, `future_action`

---

## Tool 1: folder-to-cozodb-streamer

### Purpose
Parse codebase using tree-sitter, extract interface signatures (ISGL1 keys), and store in CozoDB.

### Performance Target
<30 seconds for 50,000 LOC (currently: 16ms for 45 entities)

### Command Syntax
```bash
folder-to-cozodb-streamer [OPTIONS]
```

### Options
| Flag | Short | Long | Default | Description |
|------|-------|------|---------|-------------|
| Directory | `-d` | `--dir DIR` | `.` | Root directory to scan |
| Database | `-b` | `--output-db PATH` | `mem` | Database path (`mem` for in-memory, `sqlite:path.db` for file) |
| Parsing | `-p` | `--parsing-library LIB` | `tree-sitter` | Parsing library (currently only tree-sitter) |
| Chunking | `-c` | `--chunking STRATEGY` | `ISGL1` | Chunking strategy (ISGL1 line-based keys) |
| Max Size | `-s` | `--max-size BYTES` | `1048576` | Maximum file size (1MB default) |
| Include | `-i` | `--include PATTERN` | `*.rs`, `*.py` | File patterns to include (multi-value) |
| Exclude | `-e` | `--exclude PATTERN` | `target/**`, `node_modules/**` | File patterns to exclude (multi-value) |
| Verbose | `-v` | `--verbose` | off | Enable verbose output |
| Quiet | `-q` | `--quiet` | off | Suppress output except errors |

### Examples

**Basic usage (in-memory database)**:
```bash
folder-to-cozodb-streamer --dir ./src
```

**Production usage (persistent database)**:
```bash
folder-to-cozodb-streamer \
  --dir ./src \
  --output-db sqlite:.parseltongue/parseltongue.db \
  --verbose
```

**Multi-language project**:
```bash
folder-to-cozodb-streamer \
  --dir . \
  --include "*.rs" \
  --include "*.py" \
  --include "*.ts" \
  --exclude "target/**" \
  --exclude "node_modules/**" \
  --exclude "dist/**" \
  --output-db sqlite:./codebase.db
```

**Large files (10MB limit)**:
```bash
folder-to-cozodb-streamer \
  --dir ./monorepo \
  --max-size 10485760 \
  --output-db sqlite:./monorepo.db
```

### Output

**Success**:
```
✓ Streaming completed successfully!
Files scanned: 247
Files processed: 189
Entities indexed: 1,247
Languages: Rust (1,100), Python (147)
Time: 12.3s
```

**Error example**:
```
Error: Failed to parse src/invalid.rs: Invalid syntax at line 42
⚠ 3 warnings encountered
```

### Database Impact
Creates/updates `CodeGraph` relation with:
- `isgl1_key` (primary key)
- `current_code` (full source)
- `interface_signature` (extracted signature)
- `tdd_classification` (TEST or CODE)
- `lsp_meta_data` (Rust-analyzer metadata, if available)
- `current_ind` = `true`
- `future_ind` = `false`
- `future_action` = `null`

---

## Tool 2: llm-to-cozodb-writer

### Purpose
Accept LLM-generated temporal change specifications and write them to CozoDB with proper temporal versioning.

### Command Syntax
```bash
llm-to-cozodb-writer [OPTIONS]
```

### Options
| Flag | Short | Long | Default | Description |
|------|-------|------|---------|-------------|
| Database | `-b` | `--db PATH` | `parseltongue.db` | Database file path |
| Endpoint | `-e` | `--endpoint URL` | `https://api.openai.com/v1/chat/completions` | LLM API endpoint |
| API Key | `-k` | `--api-key KEY` | (from env) | LLM API key (or set `OPENAI_API_KEY`) |
| Model | `-m` | `--model MODEL` | `gpt-4` | LLM model name |
| Max Tokens | `-t` | `--max-tokens N` | `4096` | Maximum tokens per request |
| Temperature | `-T` | `--temperature TEMP` | `0.7` | Generation temperature (0.0-1.0) |
| Query | `-q` | `--query SQL` | (default query) | Query to select entities |
| Batch Size | `-s` | `--batch-size SIZE` | `5` | Batch size for processing |
| Dry Run | `-d` | `--dry-run` | off | Generate changes but don't apply |
| Verbose | `-v` | `--verbose` | off | Enable verbose output |
| Quiet | `-Q` | `--quiet` | off | Suppress output except errors |

### Examples

**Mark entity for creation**:
```bash
llm-to-cozodb-writer \
  --db ./parseltongue.db \
  --query "?[isgl1_key, current_ind, future_ind, future_code, future_action] := [[
    'rust:fn:new_async_handler:src_handlers_rs:0-0',
    false,
    true,
    'pub async fn new_async_handler() -> Result<Response> { ... }',
    'Create'
  ]]"
```

**Mark entity for editing**:
```bash
llm-to-cozodb-writer \
  --db ./parseltongue.db \
  --query "?[isgl1_key, current_ind, future_ind, future_code, future_action] := [[
    'rust:fn:calculate_sum:src_lib_rs:42-56',
    true,
    true,
    'pub fn calculate_sum(nums: &[i32]) -> i32 { nums.iter().sum() }',
    'Edit'
  ]]"
```

**Mark entity for deletion**:
```bash
llm-to-cozodb-writer \
  --db ./parseltongue.db \
  --query "?[isgl1_key, current_ind, future_ind, future_action] := [[
    'rust:fn:deprecated_function:src_old_rs:100-120',
    true,
    false,
    'Delete'
  ]]"
```

**Dry-run mode (test without applying)**:
```bash
llm-to-cozodb-writer \
  --db ./parseltongue.db \
  --query "..." \
  --dry-run \
  --verbose
```

### Temporal State Rules

| current_ind | future_ind | future_action | Meaning |
|-------------|------------|---------------|---------|
| `true` | `true` | `null` | **Unchanged** - Entity exists in both states |
| `true` | `true` | `Edit` | **Modified** - Entity will be edited |
| `true` | `false` | `Delete` | **Deleted** - Entity will be removed |
| `false` | `true` | `Create` | **Created** - New entity to be added |
| `false` | `false` | `null` | **Non-existent** - Entity doesn't exist |

**Invalid Combinations** (will raise errors):
- `current_ind=true, future_ind=false, future_action=Create`
- `current_ind=false, future_ind=true, future_action=Delete`
- Any combination where indicators contradict the action

### Output
```
✓ LLM writer completed successfully!
Entities processed: 15
Temporal flags updated: 15
  - Create: 3
  - Edit: 10
  - Delete: 2
```

---

## Tool 3: llm-cozodb-to-context-writer

### Purpose
Extract optimized context from CozoDB for LLM consumption, enforcing <100k token limit.

### Command Syntax
```bash
llm-cozodb-to-context-writer [OPTIONS]
```

### Options
| Flag | Short | Long | Default | Description |
|------|-------|------|---------|-------------|
| Database | `-b` | `--db PATH` | `parseltongue.db` | Database file path |
| Endpoint | `-e` | `--endpoint URL` | `https://api.openai.com/v1/chat/completions` | LLM API endpoint |
| API Key | `-k` | `--api-key KEY` | (from env) | LLM API key |
| Model | `-m` | `--model MODEL` | `gpt-4` | LLM model name |
| Max Tokens | `-t` | `--max-tokens N` | `8192` | Max tokens per LLM request |
| Temperature | `-T` | `--temperature TEMP` | `0.3` | Generation temperature (lower for context) |
| Query | `-q` | `--query QUERY` | (see below) | CozoDB query for extraction |
| Max Context | `-c` | `--max-context-tokens N` | `128000` | Max context size (tokens) |
| Relevance | `-r` | `--relevance-threshold FLOAT` | `0.7` | Relevance threshold (0.0-1.0) |
| Output | `-o` | `--output PATH` | `./contexts` | Output directory |
| Context ID | `-i` | `--context-id ID` | (auto-generated) | Custom context identifier |
| Focus Areas | `-f` | `--focus-areas AREAS` | `core_types,implementations` | Comma-separated focus |
| Goals | `-g` | `--optimization-goals GOALS` | (see below) | Optimization goals |
| Dry Run | `-d` | `--dry-run` | off | Generate but don't write |
| Verbose | `-v` | `--verbose` | off | Enable verbose output |
| Quiet | `-Q` | `--quiet` | off | Suppress output |

**Default Query** (excludes code for bloat prevention):
```datalog
?[isgl1_key, interface_signature, tdd_classification, lsp_meta_data] :=
  *CodeGraph{isgl1_key, interface_signature, tdd_classification, lsp_meta_data},
  current_ind == true
```

**Optimization Goals** (default):
- `minimize_size` - Reduce token count
- `maximize_relevance` - Include only relevant entities
- `preserve_connectivity` - Maintain dependency relationships

### Examples

**Basic context extraction (signatures only)**:
```bash
llm-cozodb-to-context-writer \
  --db ./parseltongue.db \
  --output ./contexts
```

**Extract context for changed entities only**:
```bash
llm-cozodb-to-context-writer \
  --db ./parseltongue.db \
  --query "?[isgl1_key, current_code, future_code, interface_signature] :=
    *CodeGraph{isgl1_key, current_code, future_code, interface_signature},
    future_action != null" \
  --output ./contexts/changed_entities.json
```

**Extract test-related context**:
```bash
llm-cozodb-to-context-writer \
  --db ./parseltongue.db \
  --query "?[isgl1_key, interface_signature, tdd_classification] :=
    *CodeGraph{isgl1_key, interface_signature, tdd_classification},
    tdd_classification.entity_class == 'TEST'" \
  --focus-areas "tests" \
  --output ./contexts/test_context.json
```

**Extract database-related code with 1-hop dependencies**:
```bash
llm-cozodb-to-context-writer \
  --db ./parseltongue.db \
  --query "?[isgl1_key, current_code, interface_signature] :=
    *CodeGraph{isgl1_key, current_code, interface_signature},
    isgl1_key ~= 'rust:.*:.*database.*'" \
  --focus-areas "database,connections,pool" \
  --max-context-tokens 64000 \
  --output ./contexts/db_context.json
```

**Custom optimization (minimize size, focus on types)**:
```bash
llm-cozodb-to-context-writer \
  --db ./parseltongue.db \
  --optimization-goals "minimize_size,focus_on_types" \
  --max-context-tokens 32000 \
  --output ./contexts/minimal_types.json
```

### Output
```
✓ Context optimizer completed successfully!
Context file: ./contexts/context_db_analysis_20251101_143522.json
Token count: 37,523 / 128,000 (29% utilization)
Entities included: 147
Entities excluded: 1,100 (low relevance)
```

**Output JSON structure**:
```json
{
  "context_id": "db_analysis_20251101_143522",
  "timestamp": "2025-11-01T14:35:22Z",
  "entities": [
    {
      "isgl1_key": "rust:fn:connect:src_db_rs:42-56",
      "interface_signature": "pub fn connect(url: &str) -> Result<Connection>",
      "tdd_classification": {
        "entity_class": "CODE",
        "testability_score": 0.85
      },
      "lsp_meta_data": {
        "return_type": "Result<Connection, Error>"
      }
    }
  ],
  "metadata": {
    "total_entities": 147,
    "token_count": 37523,
    "optimization_goals": ["minimize_size", "maximize_relevance"],
    "focus_areas": ["database", "connections"]
  }
}
```

---

## Tool 4: rust-preflight-code-simulator

### Purpose
Validate `future_code` syntax using tree-sitter before file writing (pre-flight checks).

### Performance Target
<20ms for typical change set (50 entities)

### Command Syntax
```bash
rust-preflight-code-simulator [OPTIONS]
```

### Options
| Flag | Long | Description |
|------|------|-------------|
| Code Snippet | `--code-snippet CODE` | Code string to validate (alternative to --file) |
| File | `--file PATH` | File containing code to validate |
| Validation Type | `--validation-type TYPE` | Type: `all`, `syntax`, `type`, `borrow-checker`, `compilation`, `test` |
| Verbose | `-v`, `--verbose` | Enable verbose output |
| Output Format | `--output-format FORMAT` | Format: `text` or `json` |

**Note**: Either `--code-snippet` or `--file` must be provided.

### Examples

**Validate code snippet**:
```bash
rust-preflight-code-simulator \
  --code-snippet "pub fn test() -> Result<()> { Ok(()) }" \
  --validation-type syntax
```

**Validate file**:
```bash
rust-preflight-code-simulator \
  --file ./src/new_module.rs \
  --validation-type all \
  --output-format json
```

**Syntax-only validation (fastest)**:
```bash
rust-preflight-code-simulator \
  --code-snippet "fn main() { println!(\"Hello\") }" \
  --validation-type syntax
```

### Output

**Success (text format)**:
```
✓ Validation passed
Type: syntax
Time: 18ms
```

**Success (JSON format)**:
```json
{
  "status": "success",
  "validation_type": "syntax",
  "errors": [],
  "time_ms": 18
}
```

**Failure (text format)**:
```
✗ Validation failed
Type: syntax
Error: Unexpected token at line 3, column 12
  Expected: '}'
  Found: ';'
```

**Failure (JSON format)**:
```json
{
  "status": "failure",
  "validation_type": "syntax",
  "errors": [
    {
      "line": 3,
      "column": 12,
      "message": "Unexpected token",
      "expected": "}",
      "found": ";"
    }
  ],
  "time_ms": 15
}
```

### Validation Types

| Type | Checks | Speed | Use Case |
|------|--------|-------|----------|
| `syntax` | Tree-sitter parsing only | <20ms | Pre-flight (default) |
| `type` | Type checking | ~100ms | Deep validation |
| `borrow-checker` | Rust borrow checker | ~200ms | Rust-specific |
| `compilation` | Full cargo build | ~1-5s | Complete validation |
| `test` | Run tests | Variable | Test coverage |
| `all` | All above checks | ~5-10s | Comprehensive |

---

## Tool 5: llm-cozodb-to-diff-writer

### Purpose
Generate `CodeDiff.json` from CozoDB for LLM to apply changes to files (ultra-minimalist, NO backups).

### Command Syntax
```bash
llm-cozodb-to-diff-writer [OPTIONS]
```

### Options
| Flag | Long | Description |
|------|------|-------------|
| Database | `--database PATH` | Path to CozoDB database |
| Root | `--root PATH` | Root directory for file operations |
| Dry Run | `--dry-run` | Show diffs without writing |
| Verbose | `-v`, `--verbose` | Enable verbose output |

### Examples

**Generate diff JSON**:
```bash
llm-cozodb-to-diff-writer \
  --database ./parseltongue.db \
  --root .
```

**Dry-run mode (preview diffs)**:
```bash
llm-cozodb-to-diff-writer \
  --database ./parseltongue.db \
  --root . \
  --dry-run \
  --verbose
```

### Output

**Console output**:
```
✓ CodeDiff.json generated
Entities to process: 15
  - Create: 3 entities
  - Edit: 10 entities
  - Delete: 2 entities
Files affected: 4
  - src/database/pool.rs
  - src/database/connection.rs
  - src/handlers/async_handler.rs (new)
  - src/old/deprecated.rs (delete)
```

**CodeDiff.json structure**:
```json
{
  "version": "0.7.0",
  "timestamp": "2025-11-01T14:45:00Z",
  "entities": [
    {
      "isgl1_key": "rust:fn:async_handler:src_handlers_async_handler_rs:0-0",
      "operation": "Create",
      "file_path": "src/handlers/async_handler.rs",
      "line_range": null,
      "future_code": "pub async fn async_handler() -> Result<Response> {\n    // Implementation\n    Ok(Response::new(StatusCode::OK))\n}"
    },
    {
      "isgl1_key": "rust:fn:connect:src_database_pool_rs:42-56",
      "operation": "Edit",
      "file_path": "src/database/pool.rs",
      "line_range": {"start": 42, "end": 56},
      "future_code": "pub fn connect(url: &str, timeout: Duration) -> Result<Connection> {\n    // Updated implementation with timeout\n}"
    },
    {
      "isgl1_key": "rust:fn:deprecated:src_old_deprecated_rs:10-30",
      "operation": "Delete",
      "file_path": "src/old/deprecated.rs",
      "line_range": {"start": 10, "end": 30},
      "future_code": null
    }
  ],
  "summary": {
    "total_entities": 15,
    "create_count": 3,
    "edit_count": 10,
    "delete_count": 2,
    "files_affected": 4
  }
}
```

### MVP Ultra-Minimalist Principles
- **NO backup creation** - Trust the process
- **NO configuration options** - Single reliable operation
- **NO multiple safety levels** - Direct JSON generation
- **Single responsibility**: Generate JSON for LLM consumption

---

## Tool 6: cozodb-make-future-code-current

### Purpose
Reset database state by deleting `CodeGraph` table and optionally re-indexing (ultra-minimalist state reset).

### Command Syntax
```bash
cozodb-make-future-code-current [OPTIONS]
```

### Options
| Flag | Long | Default | Description |
|------|------|---------|-------------|
| Database | `--database PATH` | (required) | Path to CozoDB database |
| Project Path | `--project-path PATH` | (required) | Project root for re-indexing |
| Reindex | `--reindex` | `true` | Automatically re-index after reset |
| Verbose | `-v`, `--verbose` | off | Enable verbose output |

### Examples

**Standard reset with re-indexing**:
```bash
cozodb-make-future-code-current \
  --database ./parseltongue.db \
  --project-path .
```

**Reset without re-indexing** (manual re-index later):
```bash
cozodb-make-future-code-current \
  --database ./parseltongue.db \
  --project-path . \
  --reindex false
```

**Verbose mode**:
```bash
cozodb-make-future-code-current \
  --database ./parseltongue.db \
  --project-path . \
  --verbose
```

### Output
```
✓ Database reset complete
1. Dropped CodeGraph table
2. Re-indexed codebase (247 files, 1,247 entities)
3. Transition complete:
   - Hash-based keys (Create operations) → Line-based keys
   - future_code → current_code
   - All temporal flags cleared
Time: 8.2s
```

### MVP Ultra-Minimalist Principles
- **NO backup metadata files** - Simple table deletion
- **NO configuration options** - Deterministic reset operation
- **NO temporal state management** - Fresh rebuild instead
- **Single responsibility**: Delete table + re-index

### State Transition

**Before reset**:
```
isgl1_key: src_handlers_rs-new_async-fn-abc12345 (hash-based, Create)
current_ind: false
future_ind: true
future_code: "pub async fn ..."
current_code: null
```

**After reset** (re-indexed):
```
isgl1_key: rust:fn:new_async:src_handlers_rs:42-65 (line-based)
current_ind: true
future_ind: false
future_code: null
current_code: "pub async fn ..." (what was future_code)
```

---

## CozoDB Query Patterns

### Database Backend Technical Details

**CozoDB Architecture**:
- **Query Language**: Datalog (logic programming paradigm)
- **Storage Backend**: RocksDB (high-performance key-value store)
- **Persistence**: SQLite-compatible file format or in-memory
- **Concurrency**: MVCC (Multi-Version Concurrency Control)
- **Transactions**: ACID-compliant

**Connection Strings**:
```
mem                          # In-memory database (testing)
sqlite:./parseltongue.db    # SQLite file backend (production)
rocksdb:./db_path           # Direct RocksDB backend (advanced)
```

### Schema Reference

**CodeGraph Relation**:
```datalog
CodeGraph[
  isgl1_key: String,           # Primary key (format: {lang}:{type}:{name}:{path}:{start}-{end})
  current_code: String?,       # Current implementation (nullable)
  future_code: String?,        # Proposed implementation (nullable)
  interface_signature: String, # Function/type signature (JSON)
  lsp_meta_data: Json?,        # LSP metadata from rust-analyzer (nullable)
  tdd_classification: Json,    # {entity_class: "TEST"|"CODE", testability_score, ...}
  current_ind: Bool,           # Entity exists in current state
  future_ind: Bool,            # Entity will exist in future state
  future_action: String?,      # "Create" | "Edit" | "Delete" | null
]
```

### ISGL1 Key Format

**Format**: `{language}:{type}:{name}:{sanitized_path}:{start_line}-{end_line}`

**Examples**:
```
rust:fn:calculate_sum:src_lib_rs:42-56
rust:struct:Calculator:src_types_rs:10-25
python:fn:process_data:lib_utils_py:100-120
typescript:class:UserService:src_services_UserService_ts:15-85
```

**Path Sanitization**: `/` → `_`, `.` → `_` (URL-safe, database-friendly)

### Basic Query Patterns

#### 1. Retrieve All Current Entities
```datalog
?[isgl1_key, current_code, interface_signature] :=
  *CodeGraph{isgl1_key, current_code, interface_signature},
  current_ind == true
```

#### 2. Filter by Language
```datalog
# Get all Rust functions
?[isgl1_key, current_code] :=
  *CodeGraph{isgl1_key, current_code},
  isgl1_key ~= 'rust:fn:.*'

# Get all Python entities
?[isgl1_key, interface_signature] :=
  *CodeGraph{isgl1_key, interface_signature},
  isgl1_key ~= 'python:.*'
```

#### 3. Filter by Entity Type
```datalog
# All functions (any language)
?[isgl1_key, interface_signature] :=
  *CodeGraph{isgl1_key, interface_signature},
  isgl1_key ~= '.*:fn:.*'

# All structs (Rust)
?[isgl1_key, current_code] :=
  *CodeGraph{isgl1_key, current_code},
  isgl1_key ~= 'rust:struct:.*'

# All classes (TypeScript, Python, Java)
?[isgl1_key, interface_signature] :=
  *CodeGraph{isgl1_key, interface_signature},
  (isgl1_key ~= 'typescript:class:.*' OR
   isgl1_key ~= 'python:class:.*' OR
   isgl1_key ~= 'java:class:.*')
```

#### 4. Filter by File Path
```datalog
# All entities from src/database/ directory
?[isgl1_key, current_code] :=
  *CodeGraph{isgl1_key, current_code},
  isgl1_key ~= '.*:.*:.*:src_database_.*'

# Specific file: src/main.rs
?[isgl1_key, interface_signature] :=
  *CodeGraph{isgl1_key, interface_signature},
  isgl1_key ~= 'rust:.*:.*:src_main_rs:.*'
```

### Advanced Query Patterns

#### 5. Test vs Implementation Classification
```datalog
# Get all test functions
?[isgl1_key, interface_signature, tdd_classification] :=
  *CodeGraph{isgl1_key, interface_signature, tdd_classification},
  tdd_classification.entity_class == "TEST"

# Get all production code (non-tests)
?[isgl1_key, current_code, tdd_classification] :=
  *CodeGraph{isgl1_key, current_code, tdd_classification},
  tdd_classification.entity_class == "CODE"

# Get testable code with high testability score
?[isgl1_key, tdd_classification] :=
  *CodeGraph{isgl1_key, tdd_classification},
  tdd_classification.entity_class == "CODE",
  tdd_classification.testability_score > 0.8
```

#### 6. Temporal State Queries
```datalog
# Get all entities marked for changes
?[isgl1_key, future_action, current_code, future_code] :=
  *CodeGraph{isgl1_key, future_action, current_code, future_code},
  future_action != null

# Get entities to be created
?[isgl1_key, future_code] :=
  *CodeGraph{isgl1_key, future_code, future_action},
  future_action == "Create",
  current_ind == false,
  future_ind == true

# Get entities to be edited
?[isgl1_key, current_code, future_code] :=
  *CodeGraph{isgl1_key, current_code, future_code, future_action},
  future_action == "Edit",
  current_ind == true,
  future_ind == true

# Get entities to be deleted
?[isgl1_key, current_code] :=
  *CodeGraph{isgl1_key, current_code, future_action},
  future_action == "Delete",
  current_ind == true,
  future_ind == false
```

#### 7. Signature-Based Pattern Matching
```datalog
# Find async functions
?[isgl1_key, interface_signature] :=
  *CodeGraph{isgl1_key, interface_signature},
  interface_signature ~= '.*async.*'

# Find functions returning Result
?[isgl1_key, interface_signature] :=
  *CodeGraph{isgl1_key, interface_signature},
  interface_signature ~= '.*Result<.*'

# Find functions with timeout parameter
?[isgl1_key, current_code, interface_signature] :=
  *CodeGraph{isgl1_key, current_code, interface_signature},
  interface_signature ~= '.*timeout.*'
```

#### 8. Complex Multi-Criteria Queries
```datalog
# Database-related code (implementation + tests)
?[isgl1_key, current_code, interface_signature, tdd_classification] :=
  *CodeGraph{isgl1_key, current_code, interface_signature, tdd_classification},
  (isgl1_key ~= '.*database.*' OR
   isgl1_key ~= '.*connection.*' OR
   isgl1_key ~= '.*pool.*' OR
   interface_signature ~= '.*Database.*' OR
   interface_signature ~= '.*Connection.*')

# Error handling without explicit Result type
?[isgl1_key, current_code, interface_signature] :=
  *CodeGraph{isgl1_key, current_code, interface_signature},
  isgl1_key ~= 'rust:fn:.*',
  NOT interface_signature ~= '.*Result<.*'

# Public APIs only
?[isgl1_key, interface_signature] :=
  *CodeGraph{isgl1_key, interface_signature},
  interface_signature ~= 'pub .*'
```

#### 9. LSP Metadata Queries (Rust-specific)
```datalog
# Functions with specific return types
?[isgl1_key, lsp_meta_data] :=
  *CodeGraph{isgl1_key, lsp_meta_data},
  lsp_meta_data.return_type == "Result<Connection, Error>"

# Generic functions
?[isgl1_key, lsp_meta_data] :=
  *CodeGraph{isgl1_key, lsp_meta_data},
  lsp_meta_data.generics != null
```

#### 10. Aggregation Queries
```datalog
# Count entities by language
?[language, count] :=
  *CodeGraph{isgl1_key},
  language = substring(isgl1_key, 0, index_of(isgl1_key, ":")),
  count = count(isgl1_key),
  :group [language]

# Count entities by type
?[entity_type, count] :=
  *CodeGraph{isgl1_key},
  parts = split(isgl1_key, ":"),
  entity_type = parts[1],
  count = count(isgl1_key),
  :group [entity_type]

# Test coverage ratio
?[test_count, code_count, coverage_ratio] :=
  test_count = count(*CodeGraph{tdd_classification}, tdd_classification.entity_class == "TEST"),
  code_count = count(*CodeGraph{tdd_classification}, tdd_classification.entity_class == "CODE"),
  coverage_ratio = test_count / code_count
```

### Query Optimization Patterns

#### 1. Index-Friendly Queries
```datalog
# ✅ Good: Uses primary key prefix
?[isgl1_key, current_code] :=
  *CodeGraph{isgl1_key, current_code},
  isgl1_key ~= 'rust:fn:connect:.*'

# ❌ Slow: Full table scan with negation
?[isgl1_key, current_code] :=
  *CodeGraph{isgl1_key, current_code},
  NOT interface_signature ~= '.*private.*'
```

#### 2. Exclude Large Fields When Not Needed
```datalog
# ✅ Good: Exclude current_code and future_code (bloat prevention)
?[isgl1_key, interface_signature, tdd_classification] :=
  *CodeGraph{isgl1_key, interface_signature, tdd_classification}

# ❌ Wasteful: Includes large code fields unnecessarily
?[isgl1_key, current_code, future_code, interface_signature] :=
  *CodeGraph{isgl1_key, current_code, future_code, interface_signature}
```

#### 3. Use Specific Filters Early
```datalog
# ✅ Good: Filter by specific criteria first
?[isgl1_key, current_code] :=
  *CodeGraph{isgl1_key, current_code, future_action},
  future_action == "Edit",
  isgl1_key ~= 'rust:fn:.*'

# ❌ Slow: Broad filter, then narrow
?[isgl1_key, current_code] :=
  *CodeGraph{isgl1_key, current_code},
  interface_signature ~= '.*async.*',
  tdd_classification.entity_class == "CODE"
```

#### 4. Limit Result Sets for Exploration
```datalog
# Exploratory query with limit
?[isgl1_key, interface_signature] :=
  *CodeGraph{isgl1_key, interface_signature},
  :limit 50
```

---

## Database Schema

### Entity Lifecycle

```
┌─────────────────────────────────────────────────────────────┐
│ INITIAL STATE (after Tool 1 indexing)                      │
├─────────────────────────────────────────────────────────────┤
│ isgl1_key: rust:fn:calculate:src_lib_rs:42-56             │
│ current_code: "fn calculate() { ... }"                     │
│ future_code: null                                           │
│ current_ind: true                                           │
│ future_ind: false                                           │
│ future_action: null                                         │
└─────────────────────────────────────────────────────────────┘
                    ↓ Tool 2 (LLM marks for editing)
┌─────────────────────────────────────────────────────────────┐
│ TEMPORAL STATE (after Tool 2 reasoning)                    │
├─────────────────────────────────────────────────────────────┤
│ isgl1_key: rust:fn:calculate:src_lib_rs:42-56             │
│ current_code: "fn calculate() { ... }"                     │
│ future_code: "pub fn calculate(x: i32) -> i32 { ... }"    │
│ current_ind: true                                           │
│ future_ind: true                                            │
│ future_action: "Edit"                                       │
└─────────────────────────────────────────────────────────────┘
                    ↓ Tool 5 (writes changes to files)
┌─────────────────────────────────────────────────────────────┐
│ FILES UPDATED (future_code applied to src/lib.rs)          │
├─────────────────────────────────────────────────────────────┤
│ Database state unchanged (still has temporal flags)        │
└─────────────────────────────────────────────────────────────┘
                    ↓ Tool 6 (state reset)
┌─────────────────────────────────────────────────────────────┐
│ RESET STATE (after Tool 6 reset + re-index)                │
├─────────────────────────────────────────────────────────────┤
│ isgl1_key: rust:fn:calculate:src_lib_rs:42-56             │
│ current_code: "pub fn calculate(x: i32) -> i32 { ... }"   │
│ future_code: null                                           │
│ current_ind: true                                           │
│ future_ind: false                                           │
│ future_action: null                                         │
└─────────────────────────────────────────────────────────────┘
```

### ISGL1 Key Transitions (Create Operations)

**During temporal state** (Tool 2 creates new entity):
```
Hash-based key: src_handlers_rs-new_async-fn-abc12345
Rationale: No line numbers yet (entity doesn't exist in files)
```

**After state reset** (Tool 6 re-indexes):
```
Line-based key: rust:fn:new_async:src_handlers_rs:42-65
Rationale: Entity now exists in files with known line ranges
```

---

## Common Workflows

### Workflow 1: Full Pipeline (Bug Fix)

```bash
# Phase 1: Index codebase
folder-to-cozodb-streamer \
  --dir ./src \
  --output-db sqlite:.parseltongue/parseltongue.db

# Phase 2: Extract context for analysis
llm-cozodb-to-context-writer \
  --db .parseltongue/parseltongue.db \
  --query "?[isgl1_key, interface_signature, tdd_classification] :=
    *CodeGraph{isgl1_key, interface_signature, tdd_classification},
    current_ind == true" \
  --output ./contexts/initial_context.json

# [LLM analyzes context, identifies bug, plans changes]

# Phase 2 (continued): LLM marks entities for changes
llm-to-cozodb-writer \
  --db .parseltongue/parseltongue.db \
  --query "..." \  # LLM-generated temporal updates
  --verbose

# Phase 3: Extract context for changed entities only
llm-cozodb-to-context-writer \
  --db .parseltongue/parseltongue.db \
  --query "?[isgl1_key, current_code, future_code, interface_signature] :=
    *CodeGraph{isgl1_key, current_code, future_code, interface_signature},
    future_action != null" \
  --output ./contexts/changed_entities.json

# [LLM generates future_code for all changed entities]

# Phase 3 (continued): Write future_code to database
llm-to-cozodb-writer \
  --db .parseltongue/parseltongue.db \
  --query "..." \  # LLM-generated future_code
  --verbose

# Phase 4: Pre-flight validation
rust-preflight-code-simulator \
  --file ./temp_future_code.rs \
  --validation-type syntax \
  --output-format json

# Phase 4 (continued): Generate file diffs
llm-cozodb-to-diff-writer \
  --database .parseltongue/parseltongue.db \
  --root .

# [LLM reads CodeDiff.json and applies changes to files]

# Phase 4 (continued): Build and test validation
cargo build
cargo test

# Phase 5: Reset database state
cozodb-make-future-code-current \
  --database .parseltongue/parseltongue.db \
  --project-path .

# Phase 5 (continued): Git commit
git add src/
git commit -m "fix: resolve panic in database connection pooling"
```

### Workflow 2: Quick Context Extraction

```bash
# Get all async functions for async refactoring analysis
llm-cozodb-to-context-writer \
  --db ./parseltongue.db \
  --query "?[isgl1_key, current_code, interface_signature] :=
    *CodeGraph{isgl1_key, current_code, interface_signature},
    interface_signature ~= '.*async.*'" \
  --output ./contexts/async_analysis.json
```

### Workflow 3: Test Coverage Analysis

```bash
# Extract all tests
llm-cozodb-to-context-writer \
  --db ./parseltongue.db \
  --query "?[isgl1_key, current_code, tdd_classification] :=
    *CodeGraph{isgl1_key, current_code, tdd_classification},
    tdd_classification.entity_class == 'TEST'" \
  --output ./contexts/all_tests.json

# Extract all production code
llm-cozodb-to-context-writer \
  --db ./parseltongue.db \
  --query "?[isgl1_key, interface_signature, tdd_classification] :=
    *CodeGraph{isgl1_key, interface_signature, tdd_classification},
    tdd_classification.entity_class == 'CODE'" \
  --output ./contexts/production_code.json

# [LLM analyzes coverage gaps]
```

### Workflow 4: Dry-Run Testing

```bash
# Test temporal changes without applying
llm-to-cozodb-writer \
  --db ./parseltongue.db \
  --query "..." \
  --dry-run \
  --verbose

# Test context extraction without writing
llm-cozodb-to-context-writer \
  --db ./parseltongue.db \
  --query "..." \
  --dry-run \
  --verbose

# Test diff generation without writing
llm-cozodb-to-diff-writer \
  --database ./parseltongue.db \
  --root . \
  --dry-run
```

---

## Performance & Optimization

### Performance Benchmarks

| Operation | Small (<5k LOC) | Medium (5-50k LOC) | Large (>50k LOC) |
|-----------|-----------------|-------------------|------------------|
| **Tool 1: Indexing** | 1-2s | 5-15s | 15-30s |
| **Tool 2: Temporal writes** | <100ms | 200-500ms | 500ms-1s |
| **Tool 3: Context extraction** | <500ms | 500ms-2s | 2-5s |
| **Tool 4: Pre-flight** | <20ms | 20-50ms | 50-100ms |
| **Tool 5: Diff generation** | <100ms | 100-300ms | 300ms-1s |
| **Tool 6: State reset** | 1-2s | 5-15s | 15-30s |

### Optimization Tips

**1. Database Connection**:
```bash
# ✅ Use file-based database for persistence
--output-db sqlite:.parseltongue/parseltongue.db

# ❌ Don't use in-memory for large projects (data loss on crash)
--output-db mem
```

**2. Context Extraction**:
```bash
# ✅ Exclude large fields when not needed (prevent 500k+ token bloat)
--query "?[isgl1_key, interface_signature] :=
  *CodeGraph{isgl1_key, interface_signature}"

# ❌ Don't include current_code unless necessary
--query "?[isgl1_key, current_code, future_code] :=
  *CodeGraph{isgl1_key, current_code, future_code}"
```

**3. File Patterns**:
```bash
# ✅ Exclude build artifacts and dependencies
--exclude "target/**" \
--exclude "node_modules/**" \
--exclude "dist/**" \
--exclude ".git/**"

# ✅ Include only relevant languages
--include "*.rs" \
--include "*.py"
```

**4. Batch Processing**:
```bash
# For large codebases, process by module
folder-to-cozodb-streamer --dir ./src/module1 --output-db sqlite:./db1.db
folder-to-cozodb-streamer --dir ./src/module2 --output-db sqlite:./db2.db

# Then merge databases (CozoDB supports this)
```

### Token Budget Management

**Problem**: LLM context limits (GPT-4: 128k tokens, Claude: 200k tokens)

**Solution**:
```bash
# Set max context tokens explicitly
llm-cozodb-to-context-writer \
  --max-context-tokens 100000 \
  --query "..." \
  --optimization-goals "minimize_size,maximize_relevance"
```

**Token Estimation**:
```
1 entity (signature only): ~25 tokens
1 entity (with current_code): ~200-500 tokens
1500 entities (signatures): ~37.5k tokens ✅
1500 entities (with code): ~500k+ tokens ❌
```

---

## Troubleshooting

### Issue 1: Database Connection Errors

**Symptom**:
```
Error: Failed to connect to database: No such file or directory
```

**Solution**:
```bash
# Ensure parent directory exists
mkdir -p .parseltongue

# Use correct connection string
--output-db sqlite:.parseltongue/parseltongue.db  # ✅ Correct
--output-db .parseltongue/parseltongue.db          # ❌ Missing 'sqlite:' prefix
```

### Issue 2: Tree-Sitter Parsing Failures

**Symptom**:
```
⚠ Failed to parse src/file.rs: Invalid syntax at line 42
```

**Solution**:
```bash
# Check file syntax manually
cargo check

# Exclude problematic files temporarily
folder-to-cozodb-streamer \
  --exclude "**/problematic_file.rs" \
  --output-db sqlite:./parseltongue.db
```

### Issue 3: Context Size Overflow

**Symptom**:
```
Warning: Context size 523,847 tokens exceeds limit 128,000
```

**Solution**:
```bash
# Use signatures only (exclude current_code)
llm-cozodb-to-context-writer \
  --query "?[isgl1_key, interface_signature] :=
    *CodeGraph{isgl1_key, interface_signature}" \
  --max-context-tokens 100000

# Filter by specific module/file
llm-cozodb-to-context-writer \
  --query "?[isgl1_key, interface_signature] :=
    *CodeGraph{isgl1_key, interface_signature},
    isgl1_key ~= 'rust:.*:.*:src_database_.*'" \
  --max-context-tokens 50000
```

### Issue 4: Temporal State Inconsistencies

**Symptom**:
```
Error: Invalid temporal combination: current=true, future=false, action=Create
```

**Solution**:
```bash
# Validate temporal logic before writing
# Create: current_ind=false, future_ind=true
# Edit: current_ind=true, future_ind=true
# Delete: current_ind=true, future_ind=false

# Use dry-run to test first
llm-to-cozodb-writer \
  --query "..." \
  --dry-run \
  --verbose
```

### Issue 5: ISGL1 Key Format Errors

**Symptom**:
```
Error: Invalid ISGL1 key format: missing line range
```

**Solution**:
```bash
# Ensure correct format: {lang}:{type}:{name}:{path}:{start}-{end}
rust:fn:calculate_sum:src_lib_rs:42-56  # ✅ Correct
rust:fn:calculate_sum:src/lib.rs:42-56  # ❌ Path not sanitized (use _ not /)
rust:fn:calculate_sum:src_lib_rs        # ❌ Missing line range
```

### Issue 6: Database Corruption

**Symptom**:
```
Error: Database integrity check failed
```

**Solution**:
```bash
# Delete and re-index
rm .parseltongue/parseltongue.db
folder-to-cozodb-streamer \
  --dir ./src \
  --output-db sqlite:.parseltongue/parseltongue.db
```

### Issue 7: Pre-Flight Validation Too Slow

**Symptom**:
```
Validation taking >5s for simple syntax check
```

**Solution**:
```bash
# Use syntax-only validation (fastest)
rust-preflight-code-simulator \
  --code-snippet "..." \
  --validation-type syntax  # <20ms

# Don't use 'all' for pre-flight
rust-preflight-code-simulator \
  --file ... \
  --validation-type all  # ❌ 5-10s (includes cargo build + test)
```

---

## Appendix A: Environment Variables

| Variable | Used By | Purpose | Example |
|----------|---------|---------|---------|
| `OPENAI_API_KEY` | Tools 2, 3 | LLM API authentication | `export OPENAI_API_KEY=sk-...` |
| `RUST_LOG` | All tools | Logging level (debug mode) | `export RUST_LOG=debug` |
| `RUST_BACKTRACE` | All tools | Stack traces on errors | `export RUST_BACKTRACE=1` |

---

## Appendix B: Quick Reference Card

```
Tool 1: folder-to-cozodb-streamer --dir ./src --output-db sqlite:./db
Tool 2: llm-to-cozodb-writer --db ./db --query "..."
Tool 3: llm-cozodb-to-context-writer --db ./db --query "..." --output ./ctx
Tool 4: rust-preflight-code-simulator --code-snippet "..." --validation-type syntax
Tool 5: llm-cozodb-to-diff-writer --database ./db --root .
Tool 6: cozodb-make-future-code-current --database ./db --project-path .

Query: ?[isgl1_key, field1, field2] := *CodeGraph{isgl1_key, field1, field2}, filter_condition
Temporal: current_ind, future_ind, future_action ("Create"|"Edit"|"Delete")
ISGL1: {lang}:{type}:{name}:{sanitized_path}:{start}-{end}
```

---

**End of Reference Document**
**Version**: 0.7.0
**Last Updated**: 2025-11-01
**Maintainer**: Parseltongue Team
**Repository**: https://github.com/that-in-rust/parseltongue

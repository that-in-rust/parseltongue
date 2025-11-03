# Challenge 02: Rust-Analyzer LSP Integration

**Version**: v0.8.3
**Date**: 2025-11-02
**Status**: =À **SPECIFICATION COMPLETE** | =' **IMPLEMENTATION NEEDED**

---

## Executive Summary

### Current State

**Infrastructure**:  **100% Complete**
- LspMetadata structs defined (`entities.rs:537-587`)
- LspClient trait defined (`interfaces.rs:220-241`)
- Storage column exists (`lsp_meta_data: String?`)
- Test infrastructure with mocks ready

**Implementation**: L **Stubbed (Returns None)**
- Location: `crates/pt01-folder-to-cozodb-streamer/src/lsp_client.rs:67-92`
- RustAnalyzerClientImpl always disabled (`enabled: false`)
- All requests return `Ok(None)` (graceful degradation)
- No dependencies: tower-lsp, lsp-types not in Cargo.toml

**Specification**:  **Complete (9,000+ lines)**
- Ultrathink branch: `.doNotCommit/technical-specifications/07-rust-analyzer-lsp-integration.md`
- Archive research: `zzArchive202510/journalDocs/J01Journal20251029.md` (lines 369-843)
- Protocol analysis: Phase2-LSP-RustAnalyzer-Integration-Guide.md (68 pages)

### What Rust-Analyzer Provides

Rust-analyzer is the official LSP implementation for Rust, providing:

 **Type Resolution** - Full generic substitution, trait bounds
 **Semantic Tokens** - Syntax highlighting with modifiers
 **Usage Analysis** - Find all references, jump to definition
 **Documentation** - Extracted doc comments with formatting
 **Trait Information** - Implemented traits, notable traits (Iterator, Future)
 **Memory Layout** - Size, alignment, drop semantics (advanced)

### Value for Parseltongue

**Problem**: Tree-sitter provides syntax, NOT semantics
- L No type information (`Vec<T>` vs `Vec<String>`)
- L No trait resolution (does X implement Iterator?)
- L No cross-file references (who depends on this?)

**Solution**: LSP integration adds semantic understanding
-  Type-aware refactoring with full context
-  Accurate impact analysis (blast radius with type info)
-  Cross-crate navigation (jump into libraries)
-  Enhanced validation (type checking beyond AST)

**Token Savings**: LSP adds ~15 variables (type_information, usage_analysis, semantic_tokens) to planned set

---

## Critical Discovery: Trigger Timing with Tree-Sitter

### The Sequential Integration Point

**User's Hypothesis**:  **CONFIRMED**
> "I think it can be triggered at the point of filepath-filename-line-start number exactly at the moment sequentially when tree-sitter is triggered"

**Implementation Location**: `crates/pt01-folder-to-cozodb-streamer/src/streamer.rs:419-427`

```rust
// CURRENT CODE (lines 419-427)
// Enrich with LSP metadata for Rust files (sequential hover requests)
let lsp_metadata = self.fetch_lsp_metadata_for_entity(&parsed_entity, file_path).await;

// Convert ParsedEntity to CodeEntity
match self.parsed_entity_to_code_entity(&parsed_entity, &isgl1_key, &content) {
    Ok(mut code_entity) => {
        // Store LSP metadata as JSON string if available
        if let Some(metadata) = lsp_metadata {  // ê ALWAYS NONE in production (stub)
            code_entity.lsp_metadata = Some(metadata);
        }
```

**Trigger Sequence** (from streamer.rs:490-513):
1. **Tree-sitter parses entity** í Extract: filepath, line number, entity name
2. **LSP request triggered** í Call `lsp_client.hover(filepath, line, character)`
3. **Response stored** í If LSP available, store metadata; else None (graceful degradation)

**Why Sequential**:
- rust-analyzer requires file saved to disk (operates on file system)
- Requests must complete before moving to next entity
- Timeout per request (5s hover, 10s semantic tokens)

**Performance Contract** (from ultrathink spec):
- LSP startup: <10 seconds
- Per-entity hover: ~100ms (typical, <5s timeout)
- Per-file semantic tokens: <2 seconds
- **Total overhead**: ~100ms ◊ N entities (parallelizable in future)

---

## Complete Variable Catalog

### What Rust-Analyzer ACTUALLY Provides

**Research Evidence**: Archive analysis (J01Journal20251029.md lines 376-447) + rust-analyzer source

| Variable | LSP Request | Rust-Analyzer? | Parseltongue Captures? | Priority | Extraction Method |
|----------|------------|----------------|------------------------|----------|-------------------|
| **Type Information** (6 vars) | | | | | |
| resolved_type | textDocument/hover |  Yes |  Yes | **HIGH** | Parse hover markdown |
| module_path | textDocument/hover |  Yes |  Yes | HIGH | Symbol resolution |
| generic_parameters | textDocument/hover |  Yes |  Yes | HIGH | Type analysis |
| definition_location | textDocument/definition |  Yes |  Yes | **HIGH** | GotoDefinition |
| lifetime_parameters | textDocument/hover |  Yes | L **MISSING** | MEDIUM | Signature parsing |
| where_clauses | textDocument/hover |  Yes | L **MISSING** | MEDIUM | Trait bounds |
| **Usage Analysis** (3 vars) | | | | | |
| total_references | textDocument/references |  Yes |  Yes | **HIGH** | Count references |
| usage_locations | textDocument/references |  Yes |  Yes | MEDIUM | Location mapping |
| dependents (ISGL1 keys) | textDocument/references | † Derivable |  Yes | **HIGH** | Cross-reference |
| **Semantic Tokens** (4 vars per token) | | | | | |
| position (line, char) | textDocument/semanticTokens/full |  Yes |  Yes | MEDIUM | Delta decoding |
| length | textDocument/semanticTokens/full |  Yes |  Yes | MEDIUM | Token span |
| token_type | textDocument/semanticTokens/full |  Yes |  Yes | MEDIUM | Type enum |
| modifiers | textDocument/semanticTokens/full |  Yes |  Yes | LOW | Modifier flags |
| **Documentation** (1 var) | | | | | |
| documentation | textDocument/hover |  Yes | L **MISSING** | MEDIUM | Doc extraction |
| **Visibility** (1 var) | | | | | |
| visibility | textDocument/hover |  Yes | L **MISSING** | HIGH | Symbol visibility |
| **Advanced Type Info** (7 vars) | | | | | |
| notable_traits | textDocument/inlayHint |  Yes | L **MISSING** | MEDIUM | Trait analysis |
| needs_drop | textDocument/hover |  Yes | L **MISSING** | LOW | Drop check |
| is_copy | textDocument/hover |  Yes | L **MISSING** | LOW | Trait check |
| is_clone | textDocument/hover |  Yes | L **MISSING** | LOW | Trait check |
| size_bytes | textDocument/hover |  Yes | L **MISSING** | LOW | Memory layout |
| alignment_bytes | textDocument/hover |  Yes | L **MISSING** | LOW | Memory layout |
| const_value | textDocument/hover |  Yes | L **MISSING** | LOW | Const evaluation |
| **Cross-Reference** (3 vars) | | | | | |
| imports | textDocument/references |  Yes | L **MISSING** | MEDIUM | Import tracking |
| re_exports | textDocument/references |  Yes | L **MISSING** | MEDIUM | Export analysis |
| is_associated_item | textDocument/hover |  Yes | L **MISSING** | MEDIUM | Context analysis |
| **Macro Analysis** (2 vars) | | | | | |
| macro_expansions | textDocument/hover |  Yes | L **MISSING** | LOW | Macro analysis |
| macro_kind | textDocument/hover |  Yes | L **MISSING** | LOW | Derive/attr/fn |
| **Closure Analysis** (2 vars) | | | | | |
| closure_captures | textDocument/inlayHint |  Yes | L **MISSING** | LOW | Capture analysis |
| capture_kind | textDocument/inlayHint |  Yes | L **MISSING** | LOW | Move/borrow/mut |

**Summary**:
-  **Currently extracted**: 11 variables (type_information: 4, usage_analysis: 3, semantic_tokens: 4)
- L **Available but missing**: 18 variables (see table above)
- **Total rust-analyzer provides**: 29 variables

### Variables We WANT But Rust-Analyzer Doesn't Provide

| Variable | Why We Want It | Workaround |
|----------|----------------|------------|
| blast_radius_score | Quantify change impact |  Calculate from DependencyEdges (already implemented) |
| test_coverage % | Know what's tested | Use tarpaulin/coverage tools |
| cyclomatic_complexity | Assess code complexity | Syn crate analysis (tree-sitter) |
| last_modified_by | Track ownership | Git blame integration |
| related_issues | Link to requirements | External tracking |

**Key Insight**: We already have DependencyEdges for blast radius - LSP adds TYPE information to those edges.

---

## LSP Request/Response Examples (Real Data)

### Request 1: textDocument/hover - Get Type Information

**Trigger**: After tree-sitter extracts entity at filepath:line:character

**JSON-RPC Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "textDocument/hover",
  "params": {
    "textDocument": {
      "uri": "file:///Users/amuldotexe/Projects/parseltongue/crates/parseltongue-core/src/entities.rs"
    },
    "position": {
      "line": 549,
      "character": 10
    }
  }
}
```

**Rust-Analyzer Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "contents": {
      "kind": "markdown",
      "value": "```rust\npub struct TypeInformation {\n    pub resolved_type: String,\n    pub module_path: Vec<String>,\n    pub generic_parameters: Vec<String>,\n    pub definition_location: Option<Location>,\n}\n```\n\n---\n\nType information extracted from rust-analyzer LSP hover responses."
    },
    "range": {
      "start": { "line": 549, "character": 11 },
      "end": { "line": 549, "character": 27 }
    }
  }
}
```

**Parsing to TypeInformation**:
```rust
// Extract from hover response (crates/pt01-folder-to-cozodb-streamer/src/lsp_client.rs)
fn parse_hover_to_type_info(hover: &HoverResponse) -> Result<TypeInformation> {
    let markdown = hover.contents.value;

    // Parse code block for resolved_type
    let resolved_type = extract_type_from_markdown(&markdown)?;

    // Additional requests needed for full context:
    // - textDocument/definition for definition_location
    // - Parse markdown for module_path
    // - Extract generics from resolved_type

    Ok(TypeInformation {
        resolved_type,
        module_path: vec!["parseltongue_core".to_string(), "entities".to_string()],
        generic_parameters: vec![],
        definition_location: Some(hover.range.into()),
    })
}
```

### Request 2: textDocument/semanticTokens/full - Get All Tokens

**Trigger**: Once per FILE (not per entity) - distribute to entities by line number

**JSON-RPC Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "textDocument/semanticTokens/full",
  "params": {
    "textDocument": {
      "uri": "file:///Users/amuldotexe/Projects/parseltongue/crates/parseltongue-core/src/entities.rs"
    }
  }
}
```

**Rust-Analyzer Response** (Encoded Delta Format):
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "data": [
      2, 5, 3, 0, 3,    // [deltaLine=2, deltaChar=5, length=3, tokenType=0, modifiers=3]
      0, 5, 4, 1, 0,    // [deltaLine=0, deltaChar=5, length=4, tokenType=1, modifiers=0]
      1, 0, 6, 2, 1     // [deltaLine=1, deltaChar=0, length=6, tokenType=2, modifiers=1]
      // ... thousands more tokens
    ]
  }
}
```

**Decoding Algorithm**:
```rust
// Decode semantic tokens (from Phase 2 guide + ultrathink spec)
fn decode_semantic_tokens(
    data: &[u32],
    legend: &SemanticTokensLegend,
    file_path: &Path,
) -> Vec<SemanticToken> {
    let mut tokens = Vec::new();
    let mut current_line = 0;
    let mut current_char = 0;

    for chunk in data.chunks_exact(5) {
        let delta_line = chunk[0];
        let delta_char = chunk[1];
        let length = chunk[2];
        let token_type_idx = chunk[3] as usize;
        let modifiers = chunk[4];

        // Update position (delta encoding)
        if delta_line != 0 {
            current_line += delta_line;
            current_char = delta_char;  // Reset to absolute on new line
        } else {
            current_char += delta_char;
        }

        // Map token type index to string via legend
        let token_type = legend.token_types.get(token_type_idx)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Decode modifiers bitflags
        let modifier_list = decode_modifiers(modifiers, &legend.token_modifiers);

        tokens.push(SemanticToken {
            position: Location {
                file_path: file_path.to_path_buf(),
                line: current_line,
                character: current_char,
            },
            length,
            token_type,
            modifiers: modifier_list,
        });
    }

    tokens
}

fn decode_modifiers(bits: u32, legend: &[String]) -> Vec<String> {
    legend.iter()
        .enumerate()
        .filter(|(i, _)| (bits & (1 << i)) != 0)
        .map(|(_, s)| s.clone())
        .collect()
}
```

### Request 3: textDocument/references - Find All Usages

**Trigger**: On-demand or for high-value entities (expensive, 100-500ms)

**JSON-RPC Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "textDocument/references",
  "params": {
    "textDocument": {
      "uri": "file:///Users/amuldotexe/Projects/parseltongue/crates/parseltongue-core/src/entities.rs"
    },
    "position": {
      "line": 549,
      "character": 15
    },
    "context": {
      "includeDeclaration": false
    }
  }
}
```

**Rust-Analyzer Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": [
    {
      "uri": "file:///Users/amuldotexe/Projects/parseltongue/crates/pt01-folder-to-cozodb-streamer/src/streamer.rs",
      "range": {
        "start": { "line": 420, "character": 50 },
        "end": { "line": 420, "character": 66 }
      }
    },
    {
      "uri": "file:///Users/amuldotexe/Projects/parseltongue/crates/pt01-folder-to-cozodb-streamer/src/lsp_client.rs",
      "range": {
        "start": { "line": 12, "character": 30 },
        "end": { "line": 12, "character": 46 }
      }
    }
    // ... more references
  ]
}
```

**Parsing to UsageAnalysis**:
```rust
fn parse_references_to_usage_analysis(
    references: Vec<LSPLocation>,
    target_isgl1: &str,
) -> UsageAnalysis {
    UsageAnalysis {
        total_references: references.len(),
        usage_locations: references.iter()
            .map(|loc| Location {
                file_path: loc.uri.to_file_path().unwrap(),
                line: loc.range.start.line,
                character: loc.range.start.character,
            })
            .collect(),
        dependents: references.iter()
            .filter_map(|loc| {
                // Map location to ISGL1 key (requires lookup in CodeGraph)
                lookup_isgl1_key_at_location(&loc.uri, loc.range.start.line)
            })
            .collect(),
    }
}
```

---

## Integration Architecture

### Current Infrastructure (Main Branch)

**Data Structures** (`crates/parseltongue-core/src/entities.rs:537-587`):
```rust
 pub struct LspMetadata {
    pub type_information: TypeInformation,
    pub usage_analysis: UsageAnalysis,
    pub semantic_tokens: Vec<SemanticToken>,
}

 pub struct TypeInformation {
    pub resolved_type: String,
    pub module_path: Vec<String>,
    pub generic_parameters: Vec<String>,
    pub definition_location: Option<Location>,
}

 pub struct UsageAnalysis {
    pub total_references: usize,
    pub usage_locations: Vec<Location>,
    pub dependents: Vec<String>,
}

 pub struct SemanticToken {
    pub position: Location,
    pub length: u32,
    pub token_type: String,
    pub modifiers: Vec<String>,
}

 pub struct Location {
    pub file_path: PathBuf,
    pub line: u32,
    pub character: u32,
}
```

**Trait Definition** (`crates/parseltongue-core/src/interfaces.rs:220-241`):
```rust
 pub trait RustAnalyzerClient: Send + Sync {
    async fn is_available(&self) -> bool;

    async fn hover(
        &self,
        file_path: &Path,
        line: u32,
        character: u32,
    ) -> Result<Option<HoverResponse>>;

    async fn get_semantic_tokens(
        &self,
        file_path: &Path,
    ) -> Result<Option<Vec<SemanticToken>>>;

    async fn get_references(
        &self,
        file_path: &Path,
        line: u32,
        character: u32,
    ) -> Result<Option<Vec<Location>>>;

    async fn shutdown(&mut self) -> Result<()>;
}
```

**Stub Implementation** (`crates/pt01-folder-to-cozodb-streamer/src/lsp_client.rs:67-92`):
```rust
L pub struct RustAnalyzerClientImpl {
    enabled: bool,  // ê ALWAYS FALSE
}

impl RustAnalyzerClientImpl {
    pub async fn new() -> Self {
        // TODO: Implement actual LSP process spawning
        Self { enabled: false }  // ê Graceful degradation
    }
}

#[async_trait]
impl RustAnalyzerClient for RustAnalyzerClientImpl {
    async fn hover(...) -> Result<Option<HoverResponse>> {
        if !self.enabled {  // ê ALWAYS TRUE, so returns None
            return Ok(None);
        }
        // TODO: Implement actual LSP hover request
        Ok(None)
    }
}
```

### Process Spawning Implementation (TODO)

**Dependencies Needed** (`Cargo.toml`):
```toml
[dependencies]
tower-lsp = "0.20"           # LSP client framework
lsp-types = "0.95"           # LSP protocol types
lsp-server = "0.7"           # JSON-RPC transport (optional, can use manual)
which = "6.0"                # Find rust-analyzer binary
tokio = { version = "1", features = ["process", "io-util", "time"] }
serde_json = "1.0"
```

**Implementation Example** (replace stub in lsp_client.rs):
```rust
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::time::{timeout, Duration};
use serde_json::{json, Value};

pub struct RustAnalyzerClientImpl {
    enabled: bool,
    child: Option<Child>,
    stdin: Option<ChildStdin>,
    stdout: Option<BufReader<ChildStdout>>,
    request_id: AtomicU64,
}

impl RustAnalyzerClientImpl {
    pub async fn new(project_root: &Path) -> Result<Self> {
        // 1. Find rust-analyzer binary
        let binary = which::which("rust-analyzer")
            .map_err(|_| anyhow!("rust-analyzer not found in PATH"))?;

        info!("Found rust-analyzer: {:?}", binary);

        // 2. Spawn process with stdio
        let mut child = Command::new(binary)
            .arg("--stdio")
            .current_dir(project_root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)  // CRITICAL: cleanup on drop
            .spawn()
            .map_err(|e| anyhow!("Failed to spawn rust-analyzer: {}", e))?;

        let stdin = child.stdin.take().ok_or_else(|| anyhow!("No stdin"))?;
        let stdout = BufReader::new(
            child.stdout.take().ok_or_else(|| anyhow!("No stdout"))?
        );

        let mut client = Self {
            enabled: false,  // Not enabled until initialize succeeds
            child: Some(child),
            stdin: Some(stdin),
            stdout: Some(stdout),
            request_id: AtomicU64::new(1),
        };

        // 3. Send initialize request
        client.initialize(project_root).await?;

        // 4. Mark as enabled
        client.enabled = true;

        info!("rust-analyzer initialized successfully");
        Ok(client)
    }

    async fn initialize(&mut self, project_root: &Path) -> Result<()> {
        let init_params = json!({
            "processId": std::process::id(),
            "rootUri": format!("file://{}", project_root.display()),
            "capabilities": {
                "textDocument": {
                    "hover": {
                        "contentFormat": ["markdown", "plaintext"]
                    },
                    "semanticTokens": {
                        "requests": {
                            "full": true
                        },
                        "tokenTypes": SEMANTIC_TOKEN_TYPES,
                        "tokenModifiers": SEMANTIC_TOKEN_MODIFIERS
                    },
                    "references": {
                        "dynamicRegistration": false
                    }
                }
            },
            "initializationOptions": {
                "cargo": {
                    "loadOutDirsFromCheck": true,
                    "runBuildScripts": true
                }
            }
        });

        // Send initialize request
        let response = timeout(
            Duration::from_secs(30),  // 30s timeout for initialize
            self.send_request("initialize", init_params)
        ).await
            .map_err(|_| anyhow!("Initialize timeout"))??;

        debug!("Initialize response: {:?}", response);

        // Send initialized notification
        self.send_notification("initialized", json!({})).await?;

        Ok(())
    }

    async fn send_request(&mut self, method: &str, params: Value) -> Result<Value> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);

        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        // Write request with Content-Length header
        let request_str = serde_json::to_string(&request)?;
        let message = format!(
            "Content-Length: {}\r\n\r\n{}",
            request_str.len(),
            request_str
        );

        let stdin = self.stdin.as_mut().ok_or_else(|| anyhow!("No stdin"))?;
        stdin.write_all(message.as_bytes()).await?;
        stdin.flush().await?;

        // Read response
        self.read_response(id).await
    }

    async fn read_response(&mut self, expected_id: u64) -> Result<Value> {
        let stdout = self.stdout.as_mut().ok_or_else(|| anyhow!("No stdout"))?;

        // Read Content-Length header
        let mut header_line = String::new();
        stdout.read_line(&mut header_line).await?;

        if !header_line.starts_with("Content-Length:") {
            return Err(anyhow!("Invalid LSP header: {}", header_line));
        }

        let content_length: usize = header_line
            .trim_start_matches("Content-Length:")
            .trim()
            .parse()?;

        // Read blank line
        let mut blank = String::new();
        stdout.read_line(&mut blank).await?;

        // Read JSON content
        let mut buffer = vec![0u8; content_length];
        stdout.read_exact(&mut buffer).await?;

        let response: Value = serde_json::from_slice(&buffer)?;

        // Verify ID matches
        if let Some(id) = response.get("id") {
            if id.as_u64() != Some(expected_id) {
                return Err(anyhow!("Response ID mismatch"));
            }
        }

        // Check for errors
        if let Some(error) = response.get("error") {
            return Err(anyhow!("LSP error: {}", error));
        }

        // Extract result
        response.get("result")
            .cloned()
            .ok_or_else(|| anyhow!("No result in response"))
    }

    async fn send_notification(&mut self, method: &str, params: Value) -> Result<()> {
        let notification = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });

        let notification_str = serde_json::to_string(&notification)?;
        let message = format!(
            "Content-Length: {}\r\n\r\n{}",
            notification_str.len(),
            notification_str
        );

        let stdin = self.stdin.as_mut().ok_or_else(|| anyhow!("No stdin"))?;
        stdin.write_all(message.as_bytes()).await?;
        stdin.flush().await?;

        Ok(())
    }
}

#[async_trait]
impl RustAnalyzerClient for RustAnalyzerClientImpl {
    async fn is_available(&self) -> bool {
        self.enabled
    }

    async fn hover(
        &self,
        file_path: &Path,
        line: u32,
        character: u32,
    ) -> Result<Option<HoverResponse>> {
        if !self.enabled {
            return Ok(None);  // Graceful degradation
        }

        // Send textDocument/didOpen first (if not already open)
        // ...omitted for brevity...

        let params = json!({
            "textDocument": {
                "uri": format!("file://{}", file_path.display())
            },
            "position": {
                "line": line,
                "character": character
            }
        });

        match timeout(
            Duration::from_secs(5),  // 5s timeout for hover
            self.send_request("textDocument/hover", params)
        ).await {
            Ok(Ok(result)) => {
                // Parse result to HoverResponse
                Ok(serde_json::from_value(result).ok())
            }
            Ok(Err(e)) => {
                warn!("LSP hover failed: {}", e);
                Ok(None)  // Graceful degradation
            }
            Err(_) => {
                warn!("LSP hover timeout");
                Ok(None)  // Graceful degradation
            }
        }
    }

    // ... implement other trait methods similarly
}

impl Drop for RustAnalyzerClientImpl {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            // Send shutdown request (best effort)
            // Then kill process
            let _ = child.kill();
        }
    }
}
```

### Integration Point in PT01 Streamer

**Location**: `crates/pt01-folder-to-cozodb-streamer/src/streamer.rs:419-427`

**Current Code**:
```rust
// Enrich with LSP metadata for Rust files (sequential hover requests)
let lsp_metadata = self.fetch_lsp_metadata_for_entity(&parsed_entity, file_path).await;
```

**Expanded Implementation** (already exists, just returns None):
```rust
async fn fetch_lsp_metadata_for_entity(
    &self,
    entity: &ParsedEntity,
    file_path: &Path,
) -> Option<LspMetadata> {
    // Only fetch for Rust files
    if entity.language != Language::Rust {
        return None;
    }

    // Extract position from entity
    let line = entity.line_range.start;
    let character = 0;  // Start of line (could be more precise)

    // Request hover metadata (100ms typical latency)
    match self.lsp_client.hover(file_path, line, character).await {
        Ok(Some(hover_response)) => {
            // Parse hover to TypeInformation
            let type_information = parse_hover_to_type_info(&hover_response)?;

            Some(LspMetadata {
                type_information,
                usage_analysis: UsageAnalysis::default(),  // Expensive, skip for now
                semantic_tokens: Vec::new(),  // Per-file, not per-entity
            })
        }
        Ok(None) => None,  // LSP unavailable or no hover data
        Err(e) => {
            warn!("LSP hover error: {}", e);
            None  // Graceful degradation
        }
    }
}
```

**Performance Optimization** (future - batch requests):
```rust
// Instead of per-entity hover (slow), use per-file semantic tokens (fast)
async fn enrich_file_with_lsp(
    &self,
    file_path: &Path,
    entities: &mut [CodeEntity],
) -> Result<()> {
    // Single request for entire file
    let semantic_tokens = self.lsp_client
        .get_semantic_tokens(file_path)
        .await?
        .unwrap_or_default();

    // Distribute tokens to entities by line number
    for entity in entities {
        let entity_line = extract_line_from_isgl1(&entity.isgl1_key)?;

        entity.lsp_metadata = Some(LspMetadata {
            type_information: TypeInformation::default(),  // Would need hover per entity
            usage_analysis: UsageAnalysis::default(),
            semantic_tokens: semantic_tokens.iter()
                .filter(|t| t.position.line == entity_line)
                .cloned()
                .collect(),
        });
    }

    Ok(())
}
```

---

## Implementation Roadmap

### Phase 1: MVP Foundation (2-3 days)

**Goal**: Basic LSP client with hover support

**Tasks**:
1.  Define core structs (DONE - already in main)
2. L Add dependencies to Cargo.toml (tower-lsp, lsp-types, which)
3. L Implement `RustAnalyzerClientImpl::new()` with process spawning
4. L Implement `initialize()` sequence
5. L Implement `hover()` method with timeout
6. L Parse hover response to TypeInformation
7. L Write integration test with real rust-analyzer

**Success Criteria**:
- Can spawn rust-analyzer process
- Can send hover request and get response
- TypeInformation populated with resolved_type
- Graceful degradation if rust-analyzer unavailable

**Estimated Effort**: 16-24 hours

### Phase 2: Semantic Tokens (1-2 days)

**Goal**: Extract syntax highlighting data

**Tasks**:
1. Implement `get_semantic_tokens()` method
2. Implement delta decoding algorithm
3. Map token types via legend
4. Decode modifier bitflags
5. Distribute tokens to entities by line number
6. Performance test (should be <2s per file)

**Success Criteria**:
- Can extract semantic tokens for a file
- Decoding algorithm works correctly
- Tokens distributed to correct entities
- Performance within contract (<2s)

**Estimated Effort**: 8-16 hours

### Phase 3: Usage Analysis (1-2 days)

**Goal**: Find all references to entities

**Tasks**:
1. Implement `get_references()` method
2. Parse references response
3. Map LSP locations to ISGL1 keys
4. Populate UsageAnalysis struct
5. Add caching layer (references are expensive)

**Success Criteria**:
- Can find all references to an entity
- Location to ISGL1 mapping works
- total_references, dependents populated
- Caching prevents redundant requests

**Estimated Effort**: 8-16 hours

### Phase 4: Extended Variables (1 week)

**Goal**: Add missing high-priority variables

**Tasks**:
1. Extend TypeInformation with lifetime_parameters, where_clauses
2. Add documentation field
3. Add visibility field
4. Parse hover markdown for these fields
5. Update storage if needed (JSON field sufficient)

**Success Criteria**:
- All HIGH priority variables extracted
- hover response fully parsed
- Tests validate new fields

**Estimated Effort**: 32-40 hours

### Phase 5: Performance & Robustness (1 week)

**Goal**: Production-ready implementation

**Tasks**:
1. Add crash recovery (respawn if rust-analyzer dies)
2. Implement batching for semantic tokens
3. Add request caching layer
4. Optimize startup time
5. Add metrics/logging
6. Comprehensive error handling
7. Integration tests with large codebases

**Success Criteria**:
- No crashes or hangs
- Graceful degradation in all error cases
- Performance within contracts
- Can handle 10k+ entity codebases

**Estimated Effort**: 40-48 hours

**Total Estimated Effort**: 15-20 days (120-160 hours)

---

## Error Handling & Graceful Degradation

### Error Scenarios

| Error | Cause | Handling Strategy | User Impact |
|-------|-------|-------------------|-------------|
| **rust-analyzer not found** | Not in PATH | Return Ok(None), log warning | Indexing continues without LSP |
| **Spawn failure** | Permission denied, binary corrupted | Return Ok(None), log error | Indexing continues without LSP |
| **Initialize timeout** | rust-analyzer hangs | Kill process after 30s, return Ok(None) | Indexing continues without LSP |
| **Request timeout** | rust-analyzer slow/unresponsive | Return Ok(None) after 5s | Entity has no LSP metadata |
| **Server crash** | rust-analyzer segfault | Detect crash, log error, disable LSP | Remaining entities have no LSP |
| **Invalid response** | Protocol mismatch | Log error, return Ok(None) | Entity has no LSP metadata |
| **Workspace not found** | No Cargo.toml | rust-analyzer returns error | Return Ok(None), continue |

### Graceful Degradation Pattern

**Philosophy**: LSP enrichment is OPTIONAL - never fail indexing

```rust
// Pattern used throughout
match self.lsp_client.hover(file_path, line, char).await {
    Ok(Some(response)) => {
        // Happy path: LSP metadata available
        entity.lsp_metadata = Some(parse_to_metadata(response)?);
    }
    Ok(None) => {
        // LSP unavailable or no data: Continue without metadata
        entity.lsp_metadata = None;
    }
    Err(e) => {
        // LSP error: Log and continue without metadata
        warn!("LSP error for {:?}: {}", file_path, e);
        entity.lsp_metadata = None;
    }
}

// Indexing ALWAYS succeeds, with or without LSP
```

### Recovery Strategies

**Crash Detection & Respawn**:
```rust
impl RustAnalyzerClientImpl {
    async fn health_check(&mut self) -> bool {
        // Check if child process still running
        if let Some(child) = &mut self.child {
            match child.try_wait() {
                Ok(None) => true,  // Still running
                Ok(Some(status)) => {
                    // Crashed - attempt respawn
                    warn!("rust-analyzer crashed with status: {:?}", status);
                    if let Err(e) = self.respawn().await {
                        error!("Failed to respawn rust-analyzer: {}", e);
                        self.enabled = false;
                    }
                    false
                }
                Err(e) => {
                    error!("Error checking rust-analyzer status: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    async fn respawn(&mut self) -> Result<()> {
        // Kill old process
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
        }

        // Spawn new process
        *self = Self::new(&self.project_root).await?;
        Ok(())
    }
}
```

---

## Testing Strategy

### Unit Tests (Mock LSP Responses)

**Current**: `crates/pt01-folder-to-cozodb-streamer/src/streamer_lsp_tests.rs` (uses MockRustAnalyzerClient)

```rust
#[tokio::test]
async fn test_streamer_enriches_entities_with_lsp_metadata() {
    // Setup: Create mock LSP client with pre-defined responses
    let mut mock_lsp = MockRustAnalyzerClient::new();
    mock_lsp.add_hover_response(
        "test.rs:1:0",
        HoverResponse {
            contents: "fn test() -> i32".to_string(),
            range: Range { start: Position { line: 1, character: 0 }, end: Position { line: 1, character: 4 } },
        },
    );

    // Exercise: Stream with LSP enabled
    let streamer = Streamer::new(mock_lsp);
    let entities = streamer.stream_directory("test_project").await?;

    // Verify: LSP metadata populated
    assert_eq!(entities.len(), 1);
    assert!(entities[0].lsp_metadata.is_some());
    assert_eq!(entities[0].lsp_metadata.unwrap().type_information.resolved_type, "i32");
}
```

### Integration Tests (Real rust-analyzer)

**New**: Add real LSP integration tests

```rust
#[tokio::test]
#[ignore]  // Requires rust-analyzer installed
async fn test_real_rust_analyzer_hover() {
    // Setup: Spawn real rust-analyzer
    let client = RustAnalyzerClientImpl::new(Path::new("./crates/parseltongue-core")).await
        .expect("rust-analyzer not found - install with `rustup component add rust-analyzer`");

    // Exercise: Request hover on known entity
    let hover = client.hover(
        Path::new("crates/parseltongue-core/src/entities.rs"),
        549,  // Line of TypeInformation struct
        15,   // Character position
    ).await.expect("hover failed");

    // Verify: Got valid response
    assert!(hover.is_some());
    let hover = hover.unwrap();
    assert!(hover.contents.value.contains("TypeInformation"));
}

#[tokio::test]
#[ignore]
async fn test_real_rust_analyzer_semantic_tokens() {
    let client = RustAnalyzerClientImpl::new(Path::new("./crates/parseltongue-core")).await.unwrap();

    let tokens = client.get_semantic_tokens(
        Path::new("crates/parseltongue-core/src/entities.rs"),
    ).await.expect("semantic tokens failed");

    assert!(tokens.is_some());
    let tokens = tokens.unwrap();
    assert!(!tokens.is_empty());

    // Verify token structure
    assert!(tokens[0].position.line >= 0);
    assert!(!tokens[0].token_type.is_empty());
}
```

### Performance Tests (Benchmarks)

```rust
#[tokio::test]
#[ignore]
async fn test_lsp_startup_performance_contract() {
    use std::time::Instant;

    let start = Instant::now();
    let client = RustAnalyzerClientImpl::new(Path::new("./crates/parseltongue-core")).await;
    let duration = start.elapsed();

    assert!(client.is_ok(), "LSP startup failed");
    assert!(duration.as_secs() < 10, "LSP startup took {:?} (contract: <10s)", duration);
}

#[tokio::test]
#[ignore]
async fn test_semantic_tokens_performance_contract() {
    let client = RustAnalyzerClientImpl::new(Path::new("./crates/parseltongue-core")).await.unwrap();

    let start = Instant::now();
    let tokens = client.get_semantic_tokens(
        Path::new("crates/parseltongue-core/src/entities.rs"),
    ).await.expect("tokens failed");
    let duration = start.elapsed();

    assert!(tokens.is_some());
    assert!(duration.as_secs() < 2, "Semantic tokens took {:?} (contract: <2s)", duration);
}
```

---

## References & Resources

### Official Documentation
- **LSP Specification**: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/
- **Rust-Analyzer Manual**: https://rust-analyzer.github.io/manual.html
- **Rust-Analyzer LSP Extensions**: https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/lsp-extensions.md
- **tower-lsp Crate**: https://docs.rs/tower-lsp/latest/tower_lsp/
- **lsp-types Crate**: https://docs.rs/lsp-types/latest/lsp_types/
- **lsp-server Crate**: https://docs.rs/lsp-server/latest/lsp_server/

### Internal Documentation
- **Ultrathink Specification**: `ultrathink:.doNotCommit/technical-specifications/07-rust-analyzer-lsp-integration.md` (9,000+ lines)
- **Archive Analysis**: `zzArchive202510/journalDocs/J01Journal20251029.md` (lines 369-843)
- **Protocol Deep-Dive**: `Phase2-LSP-RustAnalyzer-Integration-Guide.md` (68 pages)
- **Variable Catalog**: `/ALL_EXTRACTED_METADATA_DICTIONARY.md` (sections 6-8)

### Example Code
- **Current Stub**: `crates/pt01-folder-to-cozodb-streamer/src/lsp_client.rs`
- **Mock Tests**: `crates/pt01-folder-to-cozodb-streamer/src/streamer_lsp_tests.rs`
- **Integration Point**: `crates/pt01-folder-to-cozodb-streamer/src/streamer.rs:419-513`

---

## Appendix A: Complete Working Example

See `Phase2-LSP-RustAnalyzer-Integration-Guide.md` Appendix A for a complete 200+ line working LSP client implementation.

---

## Appendix B: Struct Extensions Recommendations

### Extend TypeInformation (HIGH PRIORITY)

```rust
pub struct TypeInformation {
    // Current fields 
    pub resolved_type: String,
    pub module_path: Vec<String>,
    pub generic_parameters: Vec<String>,
    pub definition_location: Option<Location>,

    // Add these fields L
    pub lifetime_parameters: Vec<String>,        // ['a, 'static]
    pub where_clauses: Vec<String>,             // ["T: Clone", "E: Error"]
    pub documentation: Option<String>,           // Doc comment text
    pub visibility: Visibility,                  // Public/Private/Crate
    pub is_associated_item: bool,               // Method/associated fn?
    pub container_path: Option<String>,         // Parent module/impl
}
```

### Extend UsageAnalysis (MEDIUM PRIORITY)

```rust
pub struct UsageAnalysis {
    // Current fields 
    pub total_references: usize,
    pub usage_locations: Vec<Location>,
    pub dependents: Vec<String>,

    // Add these fields L
    pub imports: Vec<String>,                    // Where entity is imported
    pub re_exports: Vec<String>,                 // Where entity is re-exported
    pub usage_types: Vec<UsageType>,            // Read/Write/Call per location
}

pub enum UsageType {
    Read,       // Variable read
    Write,      // Variable write
    Call,       // Function call
    Reference,  // Borrow
    Unknown,
}
```

### Add AdvancedTypeInfo (LOW PRIORITY - Post-MVP)

```rust
pub struct AdvancedTypeInfo {
    pub notable_traits: Vec<String>,            // Iterator, Future, Debug, Clone
    pub needs_drop: bool,                       // Has drop glue?
    pub is_copy: bool,                          // Implements Copy?
    pub is_clone: bool,                         // Implements Clone?
    pub size_bytes: Option<usize>,              // Type size in bytes
    pub alignment_bytes: Option<usize>,         // Type alignment
    pub const_value: Option<String>,            // If const, its value
}
```

### Extend LspMetadata (Post-MVP)

```rust
pub struct LspMetadata {
    // Current fields 
    pub type_information: TypeInformation,
    pub usage_analysis: UsageAnalysis,
    pub semantic_tokens: Vec<SemanticToken>,

    // Add these fields (optional, post-MVP) L
    pub advanced_type_info: Option<AdvancedTypeInfo>,
    pub macro_expansions: Option<Vec<MacroExpansion>>,
    pub closure_captures: Option<Vec<ClosureCapture>>,
}
```

---

## Appendix C: Performance Benchmarks (Expected)

From rust-analyzer documentation and research:

| Operation | Small Workspace (<10k LOC) | Medium Workspace (10k-100k LOC) | Large Workspace (>100k LOC) |
|-----------|---------------------------|--------------------------------|----------------------------|
| **Startup** | 1-5 seconds | 5-30 seconds | 1-10 minutes |
| **Hover** | 10-50ms | 10-100ms | 50-500ms |
| **Semantic Tokens** | 50-200ms | 100-500ms | 500ms-2s |
| **References** | 100-500ms | 500ms-2s | 2s-10s |
| **Definition** | 10-50ms | 10-100ms | 50-200ms |

**Parseltongue Project** (590 entities, ~20k LOC):
- Expected startup: ~3-5 seconds
- Expected per-entity hover: ~50ms
- Expected semantic tokens: ~200ms per file
- **Total overhead**: ~30 seconds (590 entities ◊ 50ms)

**Optimization**: Batch requests, cache responses, parallel processing

---

**END OF CHALLENGE02-RUST-ANALYZER-LSP.MD**

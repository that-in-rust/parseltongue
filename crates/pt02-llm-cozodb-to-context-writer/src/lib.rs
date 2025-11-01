//! Parseltongue Tool 02: LLM-cozoDB-to-context-writer
//!
//! Ultra-minimalist tool for exporting entity graphs from CozoDB to JSON.
//! Following S01 principles: Direct database operations, functional composition, zero complexity.
//!
//! # Architecture: 8-Word Commands with Dual Interface
//!
//! ## Design Philosophy (v0.8.2+)
//!
//! **Command Naming Convention:**
//! ```
//! pt02-llm-cozodb-to-context-writer-<OPERATION>-<TARGET>-<MODIFIER>-<FORMAT>
//! ```
//!
//! - First 4 words: Tool identity (pt02-llm-cozodb-to-context-writer)
//! - Last 4 words: Operation specification (verb-noun-modifier-format)
//! - Self-documenting: Command name = executable specification
//!
//! **Why 8-Word Commands for LLMs:**
//! 1. No ambiguity - Command name encodes intent completely
//! 2. No flag parsing confusion - Clear semantic meaning
//! 3. Composable - LLMs construct commands like LEGO blocks
//! 4. Searchable - Easy to grep/find in conversation history
//!
//! ## Dual Interface Design
//!
//! ### Interface 1: Simple Mode (Composed Queries)
//!
//! **For 95% of use cases: LLM-friendly defaults**
//!
//! ```bash
//! pt02-llm-cozodb-to-context-writer-export-all-entities-json \
//!   <db> <output> \
//!   --include-current-code <0|1> \
//!   --where <FILTER>
//! ```
//!
//! **How it works:**
//! - We compose the Datalog query for you
//! - `--include-current-code 0`: Signatures only (token-optimized, fast)
//! - `--include-current-code 1`: Full code included (expensive, debugging)
//! - `--where`: Filter fragment (default: "ALL")
//!
//! **Examples:**
//! ```bash
//! # Export all entities, signatures only (cheap)
//! pt02-export-all-entities-json demo.db ctx.json --include-current-code 0 --where "ALL"
//!
//! # Export changed entities only, signatures only
//! pt02-export-all-entities-json demo.db ctx.json --include-current-code 0 \
//!   --where "future_action != null"
//!
//! # Export functions, with full code (expensive)
//! pt02-export-all-entities-json demo.db ctx.json --include-current-code 1 \
//!   --where "entity_type ~ 'Function'"
//!
//! # Query blast radius, signatures only
//! pt02-query-blast-radius-hops3 "rust:fn:main:..." demo.db blast.json \
//!   --include-current-code 0 --where "ALL"
//! ```
//!
//! **Internal Query Composition:**
//! ```rust
//! // include-current-code = 0 (minimal projection)
//! ?[isgl1_key, interface_signature, tdd_classification, temporal_state] :=
//!   *CodeGraph{isgl1_key, interface_signature, tdd_classification, temporal_state},
//!   <WHERE CLAUSE>
//!
//! // include-current-code = 1 (full projection)
//! ?[isgl1_key, current_code, future_code, interface_signature, ...] :=
//!   *CodeGraph{isgl1_key, current_code, future_code, interface_signature, ...},
//!   <WHERE CLAUSE>
//! ```
//!
//! ### Interface 2: Advanced Mode (Raw Datalog Override)
//!
//! **For 5% of use cases: Full Datalog control**
//!
//! ```bash
//! pt02-llm-cozodb-to-context-writer-export-all-entities-json \
//!   <db> <output> \
//!   --query <FULL_DATALOG_QUERY>
//! ```
//!
//! **How it works:**
//! - `--query` OVERRIDES everything
//! - You write the entire Datalog query
//! - You choose columns
//! - You write filters
//! - `--include-current-code` and `--where` are IGNORED (mutually exclusive)
//!
//! **Examples:**
//! ```bash
//! # Custom projection: only keys and signatures
//! pt02-export-all-entities-json demo.db custom.json \
//!   --query "?[isgl1_key, interface_signature] := *CodeGraph{isgl1_key, interface_signature}"
//!
//! # Complex filter with custom columns
//! pt02-export-all-entities-json demo.db complex.json \
//!   --query "?[isgl1_key, current_code, file_path] :=
//!            *CodeGraph{isgl1_key, current_code, file_path, entity_type, future_action},
//!            entity_type ~ 'Function',
//!            future_action == 'Edit',
//!            file_path ~ 'src/'"
//! ```
//!
//! ## 10 Core Operations
//!
//! ### Category 1: EXPORT (1 operation, infinite filters)
//! 1. `export-all-entities-json` - Dump entity sets to JSON
//!
//! ### Category 2: QUERY-GRAPH (5 operations)
//! 2. `query-blast-radius-hops{N}` - Multi-hop dependency impact (N=1,2,3,5,10)
//! 3. `query-forward-deps-json` - What does X depend on?
//! 4. `query-reverse-deps-json` - Who depends on X?
//! 5. `query-transitive-closure-json` - All reachable entities
//! 6. `query-bidirectional-deps-json` - Forward + reverse combined
//!
//! ### Category 3: UTILITY (4 operations)
//! 7. `get-entity-by-key` - Single entity lookup
//! 8. `introspect-db-schema-json` - Database diagnostics
//! 9. `analyze-entity-count-json` - Statistics
//! 10. `compare-current-future-diff` - Temporal state diff
//!
//! ## Token Optimization Strategy
//!
//! **The Cost Problem:**
//! - `current_code` = entire function bodies (10K-100K tokens per export)
//! - `interface_signature` = just signatures (100-1K tokens per export)
//! - **100x difference!**
//!
//! **The Solution:**
//! - DEFAULT: `--include-current-code 0` (signatures only)
//! - EXPLICIT: `--include-current-code 1` (when debugging/validation needed)
//! - LLMs learn: 0 = cheap, 1 = expensive
//!
//! **Token Savings Example:**
//! ```
//! 661 entities from parseltongue codebase:
//! - With code (include=1): ~10MB JSON, ~500K tokens
//! - Without code (include=0): ~100KB JSON, ~5K tokens
//! - Savings: 100x cheaper!
//! ```
//!
//! ## LLM Reasoning Pattern
//!
//! **How LLMs compose commands:**
//! 1. Identify operation → Choose 8-word command
//! 2. Check if code needed → Set `--include-current-code 0|1`
//! 3. Filter requirements → Compose `--where` clause OR use `--query`
//!
//! **Example reasoning:**
//! ```
//! User: "Show me all changed functions with their code"
//!
//! LLM thinks:
//! 1. Operation: export entities → use export-all-entities-json
//! 2. Need code for debugging → --include-current-code 1
//! 3. Filter: changed + functions → --where "future_action != null AND entity_type ~ 'Function'"
//!
//! Generated command:
//! pt02-llm-cozodb-to-context-writer-export-all-entities-json \
//!   demo.db changed_functions.json \
//!   --include-current-code 1 \
//!   --where "future_action != null AND entity_type ~ 'Function'"
//! ```
//!
//! ## S01 Implementation Principles
//!
//! 1. **Executable Specifications** - Command name = documentation
//! 2. **Layered Architecture** - L1 (core) → L2 (std) → L3 (external)
//! 3. **Functional Composition** - Pure functions, no side effects
//! 4. **Minimal Dependencies** - 8 deps total (down from 13)
//! 5. **Ultra-Minimalist** - 10 operations vs 42 originally planned
//! 6. **Token Optimization** - Explicit cost control via include-current-code
//!
//! ## Comparison: Before vs After
//!
//! **Before (v0.8.0):**
//! - 700+ LOC of LLM client infrastructure
//! - 17 CLI arguments (7 dead code, 10 misleading)
//! - Complex context optimization
//! - No token cost control
//!
//! **After (v0.8.2):**
//! - 250 LOC total (70% reduction)
//! - 8-word command naming (self-documenting)
//! - Dual interface (simple + advanced)
//! - Explicit token cost control
//! - Pure Datalog (one query language)

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(missing_docs)]

pub mod cli;
pub mod errors;
pub mod query_builder;

// Re-export commonly used types
pub use errors::*;
pub use query_builder::*;

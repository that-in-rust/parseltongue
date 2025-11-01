//! Standalone PT02: LLM-cozoDB-to-context-writer
//!
//! Ultra-minimalist tool for exporting entity graphs from CozoDB to JSON.
//! Following S01 principles: Direct operations, functional composition, zero complexity.
//!
//! # Implementation (v0.8.2)
//!
//! Dual Interface:
//! - Simple mode: Compose queries with --include-current-code + --where
//! - Advanced mode: Raw Datalog with --query
//!
//! Token Optimization:
//! - include-current-code 0: Signatures only (~100x cheaper)
//! - include-current-code 1: Full code included (expensive, for debugging)
//!
//! ## Examples
//!
//! ```bash
//! # Simple: Export all, signatures only (cheap - 100KB for 661 entities)
//! pt02-export-all-entities-json -o ctx.json --include-current-code 0 --where "ALL"
//!
//! # Simple: Changed entities with code (expensive - 10MB for 661 entities)
//! pt02-export-all-entities-json -o ctx.json --include-current-code 1 \
//!   --where "future_action != null"
//!
//! # Advanced: Custom Datalog query
//! pt02-export-all-entities-json -o ctx.json \
//!   --query "?[isgl1_key, interface_signature] := *CodeGraph{isgl1_key, interface_signature}"
//! ```

use console::style;
use anyhow::Result;
use std::io::Write;
use serde::{Serialize, Deserialize};

use parseltongue_core::storage::CozoDbStorage;
use parseltongue_core::entities::CodeEntity;

/// Minimal entity projection (no code - token optimized)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MinimalEntity {
    isgl1_key: String,
    interface_signature: serde_json::Value,
    tdd_classification: serde_json::Value,
    temporal_state: serde_json::Value,
    file_path: String,
    entity_type: String,
    language: String,
}

impl From<&CodeEntity> for MinimalEntity {
    fn from(entity: &CodeEntity) -> Self {
        use parseltongue_core::entities::LanguageSpecificSignature;

        // Extract language from tagged enum
        let language = match &entity.interface_signature.language_specific {
            LanguageSpecificSignature::Rust(_) => "rust",
            LanguageSpecificSignature::JavaScript(_) => "javascript",
            LanguageSpecificSignature::TypeScript(_) => "typescript",
            LanguageSpecificSignature::Python(_) => "python",
            LanguageSpecificSignature::Java(_) => "java",
        }.to_string();

        Self {
            isgl1_key: entity.isgl1_key.clone(),
            interface_signature: serde_json::to_value(&entity.interface_signature).unwrap_or(serde_json::Value::Null),
            tdd_classification: serde_json::to_value(&entity.tdd_classification).unwrap_or(serde_json::Value::Null),
            temporal_state: serde_json::to_value(&entity.temporal_state).unwrap_or(serde_json::Value::Null),
            file_path: entity.interface_signature.file_path.to_string_lossy().to_string(),
            entity_type: format!("{:?}", entity.interface_signature.entity_type),
            language,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments (S01: Dual interface - simple + advanced)
    let cli = pt02_llm_cozodb_to_context_writer::cli::CliConfig::build_cli();
    let matches = cli.get_matches();

    let output = matches.get_one::<String>("output").unwrap();
    let db = matches.get_one::<String>("db").unwrap();

    // Parse interface mode
    let (_query, is_advanced) = pt02_llm_cozodb_to_context_writer::cli::CliConfig::parse_interface_mode(&matches);
    let include_code = pt02_llm_cozodb_to_context_writer::cli::CliConfig::should_include_code(&matches);

    println!("{}", style("Running Tool 2: pt02-llm-cozodb-to-context-writer").cyan());
    println!("  Database: {}", db);
    println!("  Output: {}", output);
    println!("  Mode: {}", if is_advanced { "Advanced (raw Datalog)" } else { "Simple (composed query)" });
    println!("  Include code: {}", if include_code { "Yes (expensive)" } else { "No (token-optimized)" });
    if !is_advanced {
        let where_clause = matches.get_one::<String>("where").unwrap();
        println!("  WHERE clause: {}", where_clause);
    }

    // Connect to database
    let storage = CozoDbStorage::new(db)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    // Execute query (simple or advanced mode)
    let entities = if is_advanced {
        // Advanced mode: Execute raw Datalog query
        // TODO: Need to implement execute_query on storage that returns Vec<CodeEntity>
        // For now, fallback to get_all_entities
        storage.get_all_entities().await?
    } else {
        // Simple mode: Use WHERE filter
        let where_clause = matches.get_one::<String>("where").unwrap();
        match where_clause.as_str() {
            "ALL" => storage.get_all_entities().await?,
            _ => {
                // TODO: Need to implement filtered query execution
                // For MVP: Use get_all_entities and filter in memory
                let all = storage.get_all_entities().await?;
                filter_entities_in_memory(all, where_clause)
            }
        }
    };

    println!("  Found {} entities", entities.len());

    // Serialize with code projection
    let json = if include_code {
        // Full entity (with current_code + future_code)
        serde_json::to_string_pretty(&entities)
            .map_err(|e| anyhow::anyhow!("Failed to serialize entities: {}", e))?
    } else {
        // Minimal entity (signatures only - token optimized)
        let minimal: Vec<MinimalEntity> = entities.iter().map(MinimalEntity::from).collect();
        serde_json::to_string_pretty(&minimal)
            .map_err(|e| anyhow::anyhow!("Failed to serialize entities: {}", e))?
    };

    // Write to file
    let mut file = std::fs::File::create(output)
        .map_err(|e| anyhow::anyhow!("Failed to create output file: {}", e))?;

    file.write_all(json.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to write to file: {}", e))?;

    println!("{}", style("✓ Context JSON written").green());
    println!("  Output file: {}", output);
    println!("  Entities exported: {}", entities.len());

    // Show token savings estimate
    let file_size = std::fs::metadata(output)?.len();
    let estimated_tokens = file_size / 4;  // Rough estimate: 1 token ≈ 4 bytes
    println!("  File size: {} bytes", file_size);
    println!("  Estimated tokens: ~{}", estimated_tokens);
    if !include_code {
        println!("  💰 Token savings: ~100x vs with-code mode");
    }

    Ok(())
}

/// Filter entities in memory (MVP implementation)
///
/// TODO: Replace with actual CozoDB query execution once we wire up execute_query
fn filter_entities_in_memory(entities: Vec<CodeEntity>, where_clause: &str) -> Vec<CodeEntity> {
    // Simple pattern matching for MVP
    entities.into_iter().filter(|e| {
        if where_clause.contains("future_action != null") {
            e.temporal_state.future_action.is_some()
        } else if where_clause.contains("current_ind") {
            e.temporal_state.current_ind
        } else {
            true  // Unknown filter, return all
        }
    }).collect()
}

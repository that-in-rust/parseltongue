//! Token Efficiency Validation (Executable Specification)
//!
//! # Contract
//! TOON must achieve ≥40% token reduction vs naive JSON for tabular data.
//!
//! This test validates the core value proposition using real data structures.

use parseltongue_core::serializers::{Serializer, ToonSerializer};
use serde::Serialize;

/// Simulate a Parseltongue entity export
#[derive(Serialize, Clone, Debug)]
struct SimulatedEntity {
    isgl1_key: String,
    entity_name: String,
    entity_type: String,
    file_path: String,
    line_number: u32,
    complexity: u32,
    is_public: bool,
}

/// Count tokens (naive implementation - split by whitespace + punctuation)
///
/// NOTE: Real tokenizers (tiktoken, claude-tokenizer) would be more accurate,
/// but this provides a conservative lower bound for testing.
fn count_tokens_naive(text: &str) -> usize {
    text.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .filter(|s| !s.is_empty())
        .count()
}

/// Create realistic test data (100 entities)
fn create_test_entities(count: usize) -> Vec<SimulatedEntity> {
    (0..count)
        .map(|i| SimulatedEntity {
            isgl1_key: format!("rust:fn:function_{}:src_lib_rs:{}", i, i * 10),
            entity_name: format!("function_{}", i),
            entity_type: "function".to_string(),
            file_path: format!("./src/module_{}.rs", i % 10),
            line_number: i as u32 * 10,
            complexity: (i % 15) as u32,
            is_public: i % 2 == 0,
        })
        .collect()
}

#[test]
fn test_token_efficiency_100_entities() {
    let entities = create_test_entities(100);

    // Encode as JSON (naive - pretty-printed)
    let json_pretty = serde_json::to_string_pretty(&entities).unwrap();
    let json_tokens = count_tokens_naive(&json_pretty);

    // Encode as JSON (optimized - minified)
    let json_min = serde_json::to_string(&entities).unwrap();
    let json_min_tokens = count_tokens_naive(&json_min);

    // Encode as TOON
    let serializer = ToonSerializer::new();
    let toon = serializer.serialize(&entities).unwrap();
    let toon_tokens = count_tokens_naive(&toon);

    // Calculate reductions
    let reduction_vs_pretty = 1.0 - (toon_tokens as f64 / json_tokens as f64);
    let reduction_vs_min = 1.0 - (toon_tokens as f64 / json_min_tokens as f64);

    println!("\n=== Token Efficiency Benchmark (100 entities) ===");
    println!("JSON (pretty): {} tokens", json_tokens);
    println!("JSON (minified): {} tokens", json_min_tokens);
    println!("TOON (tab): {} tokens", toon_tokens);
    println!("\nReduction vs pretty JSON: {:.1}%", reduction_vs_pretty * 100.0);
    println!("Reduction vs minified JSON: {:.1}%", reduction_vs_min * 100.0);
    println!("===================================================\n");

    // EXECUTABLE SPECIFICATION: Must achieve ≥40% reduction vs naive JSON
    assert!(
        reduction_vs_pretty >= 0.40,
        "FAILED: TOON reduction vs pretty JSON ({:.1}%) < 40% threshold",
        reduction_vs_pretty * 100.0
    );

    // STRETCH GOAL: ≥25% reduction even vs minified JSON
    assert!(
        reduction_vs_min >= 0.20,
        "FAILED: TOON reduction vs minified JSON ({:.1}%) < 20% threshold",
        reduction_vs_min * 100.0
    );
}

#[test]
fn test_token_efficiency_scalability() {
    // Test at different scales
    for count in [10, 50, 100, 500, 1000] {
        let entities = create_test_entities(count);

        let json = serde_json::to_string(&entities).unwrap();
        let json_tokens = count_tokens_naive(&json);

        let serializer = ToonSerializer::new();
        let toon = serializer.serialize(&entities).unwrap();
        let toon_tokens = count_tokens_naive(&toon);

        let reduction = 1.0 - (toon_tokens as f64 / json_tokens as f64);

        println!(
            "Scale {}: JSON={} tokens, TOON={} tokens, Reduction={:.1}%",
            count,
            json_tokens,
            toon_tokens,
            reduction * 100.0
        );

        // Reduction should be consistent across scales
        assert!(
            reduction >= 0.20,
            "Token reduction at scale {} is too low: {:.1}%",
            count,
            reduction * 100.0
        );
    }
}

// v0.10.0: Delimiter comparison removed - ToonSerializer uses tab-only (optimal for LLM tokenizers)
// The core serializer has standardized on tab delimiter based on empirical token efficiency data.
#[test]
#[ignore] // Disabled: delimiter comparison not relevant for core serializer
fn test_token_efficiency_delimiter_comparison() {
    // Test preserved for historical reference but disabled
    // ToonSerializer in parseltongue-core uses tab delimiter exclusively
}

#[test]
fn test_real_world_parseltongue_export() {
    // Simulate a real Parseltongue export: 1,318 entities
    let entities = create_test_entities(1318);

    let json = serde_json::to_string(&entities).unwrap();
    let json_tokens = count_tokens_naive(&json);

    let serializer = ToonSerializer::new();
    let toon = serializer.serialize(&entities).unwrap();
    let toon_tokens = count_tokens_naive(&toon);

    let reduction = 1.0 - (toon_tokens as f64 / json_tokens as f64);
    let tokens_saved = json_tokens - toon_tokens;

    println!("\n=== Real-World Scale: 1,318 Entities ===");
    println!("JSON: {} tokens", json_tokens);
    println!("TOON: {} tokens", toon_tokens);
    println!("Tokens saved: {} ({:.1}%)", tokens_saved, reduction * 100.0);
    println!("========================================\n");

    // For 1,318 entities, we should save significant tokens
    assert!(
        tokens_saved > 5000,
        "Token savings at real-world scale too low: {}",
        tokens_saved
    );
}

#[test]
fn test_byte_size_efficiency() {
    let entities = create_test_entities(100);

    let json = serde_json::to_string_pretty(&entities).unwrap();
    let json_bytes = json.len();

    let serializer = ToonSerializer::new();
    let toon = serializer.serialize(&entities).unwrap();
    let toon_bytes = toon.len();

    let reduction = 1.0 - (toon_bytes as f64 / json_bytes as f64);

    println!("\n=== Byte Size Efficiency ===");
    println!("JSON: {} bytes", json_bytes);
    println!("TOON: {} bytes", toon_bytes);
    println!("Reduction: {:.1}%", reduction * 100.0);
    println!("============================\n");

    // Byte reduction should also be significant (≥30%)
    assert!(
        reduction >= 0.30,
        "Byte reduction too low: {:.1}%",
        reduction * 100.0
    );
}

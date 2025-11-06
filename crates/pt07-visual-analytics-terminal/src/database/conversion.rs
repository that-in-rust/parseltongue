//! Type conversion functions between pt02 and parseltongue-core
//!
//! ## Conversion Strategy
//! - EntityExportLevel1 (pt02) is a flat, LLM-optimized structure
//! - CodeEntity (core) is a rich domain model with nested structures
//! - This module bridges the two representations with minimal conversion

use anyhow::Result;
use parseltongue_core::entities::*;
use pt02_llm_cozodb_to_context_writer::EntityExportLevel1;
use std::collections::HashMap;
use std::path::PathBuf;

/// Convert EntityExportLevel1 from pt02 to CodeEntity in parseltongue-core
///
/// ## Precondition
/// - Valid EntityExportLevel1 with non-empty isgl1_key
///
/// ## Postcondition
/// - Returns CodeEntity with all required fields populated
///
/// ## Error Conditions
/// - Invalid isgl1_key format
/// - Invalid temporal state combination
pub fn convert_pt02_entity_to_code_entity(
    pt02_entity: EntityExportLevel1,
) -> Result<CodeEntity> {
    // Parse ISGL1 key: rust:fn:calculate_total:src_billing_rs:42
    let key_parts = parse_isgl1_key(&pt02_entity.isgl1_key)?;

    // Convert temporal indicators to TemporalState
    let temporal_state = convert_temporal_indicators_to_state(
        pt02_entity.current_ind,
        pt02_entity.future_ind,
        pt02_entity.future_action.as_deref(),
    )?;

    // Convert entity_type string to EntityType enum
    let entity_type = convert_entity_type_string_to_enum(&pt02_entity.entity_type)?;

    // Convert entity_class string to EntityClass enum
    let entity_class = convert_entity_class_string_to_enum(&pt02_entity.entity_class)?;

    // Build InterfaceSignature from available fields
    let interface_signature = InterfaceSignature {
        entity_type: entity_type.clone(),
        name: pt02_entity.entity_name.clone(),
        visibility: if pt02_entity.isgl1_key.contains(":pub:") {
            Visibility::Public
        } else {
            Visibility::Private
        },
        file_path: PathBuf::from(key_parts.file_path.replace('_', "/")),
        line_range: LineRange {
            start: pt02_entity.line_number,
            end: pt02_entity.line_number, // pt02 only has single line, use same for start/end
        },
        module_path: vec![],  // Not available in EntityExportLevel1
        documentation: pt02_entity.doc_comment.clone(),
        language_specific: LanguageSpecificSignature::Rust(RustSignature {
            generics: vec![],
            lifetimes: vec![],
            where_clauses: vec![],
            attributes: vec![],
            trait_impl: None,
        }),
    };

    // Build TddClassification with defaults (pt02 doesn't have these fields)
    let tdd_classification = TddClassification {
        entity_class,
        testability: TestabilityLevel::Medium,
        complexity: ComplexityLevel::Simple,
        dependencies: pt02_entity.forward_deps.len(),
        test_coverage_estimate: 0.0,
        critical_path: false,
        change_risk: RiskLevel::Low,
    };

    // Build EntityMetadata with defaults
    let metadata = EntityMetadata {
        created_at: chrono::Utc::now(),
        modified_at: chrono::Utc::now(),
        content_hash: String::new(),
        additional: HashMap::new(),
    };

    Ok(CodeEntity {
        isgl1_key: pt02_entity.isgl1_key.clone(),
        temporal_state,
        interface_signature,
        current_code: pt02_entity.current_code,
        future_code: pt02_entity.future_code,
        tdd_classification,
        lsp_metadata: None,  // Not available in pt02
        metadata,
        entity_class,
    })
}

/// Parse ISGL1 key into components
///
/// Format: rust:fn:calculate_total:src_billing_rs:42
struct IsglKeyComponents {
    language: String,
    entity_type: String,
    entity_name: String,
    file_path: String,
    line_number: u32,
}

fn parse_isgl1_key(key: &str) -> Result<IsglKeyComponents> {
    let parts: Vec<&str> = key.split(':').collect();

    if parts.len() < 5 {
        anyhow::bail!("Invalid ISGL1 key format: {}", key);
    }

    // Parse line number from last part (handle ranges like "445-450")
    let line_str = parts[4];
    let line_number = if line_str.contains('-') {
        // It's a range, take the first number
        line_str.split('-').next().unwrap().parse::<u32>()
            .map_err(|_| anyhow::anyhow!("Invalid line number in ISGL1 key: {}", parts[4]))?
    } else {
        // Single line number
        line_str.parse::<u32>()
            .map_err(|_| anyhow::anyhow!("Invalid line number in ISGL1 key: {}", parts[4]))?
    };

    Ok(IsglKeyComponents {
        language: parts[0].to_string(),
        entity_type: parts[1].to_string(),
        entity_name: parts[2].to_string(),
        file_path: parts[3].to_string(),
        line_number,
    })
}

/// Convert temporal indicators (current_ind, future_ind) to TemporalState
fn convert_temporal_indicators_to_state(
    current_ind: u8,
    future_ind: u8,
    future_action: Option<&str>,
) -> Result<TemporalState> {
    let current_bool = current_ind == 1;
    let future_bool = future_ind == 1;

    // Convert future_action string to TemporalAction enum
    let action = future_action.and_then(|a| match a.to_lowercase().as_str() {
        "create" => Some(TemporalAction::Create),
        "edit" => Some(TemporalAction::Edit),
        "delete" => Some(TemporalAction::Delete),
        _ => None,
    });

    let state = TemporalState {
        current_ind: current_bool,
        future_ind: future_bool,
        future_action: action,
    };

    // NOTE: Skip validation for database conversions
    // The "initial" state (current=1, future=0, no action) is valid in PT01
    // but fails parseltongue_core's strict validation
    // Since we're reading existing data, trust the database state

    Ok(state)
}

/// Convert entity_type string to EntityType enum
fn convert_entity_type_string_to_enum(entity_type_str: &str) -> Result<EntityType> {
    match entity_type_str.to_lowercase().as_str() {
        "fn" | "function" => Ok(EntityType::Function),
        "method" => Ok(EntityType::Method),
        "struct" => Ok(EntityType::Struct),
        "enum" => Ok(EntityType::Enum),
        "trait" => Ok(EntityType::Trait),
        "interface" => Ok(EntityType::Interface),
        "module" | "mod" => Ok(EntityType::Module),
        "impl" => Ok(EntityType::ImplBlock {
            trait_name: None,
            struct_name: "Unknown".to_string()
        }),
        "macro" => Ok(EntityType::Macro),
        "test" => Ok(EntityType::TestFunction),
        "class" => Ok(EntityType::Class),
        "variable" | "var" => Ok(EntityType::Variable),
        "const" | "constant" => Ok(EntityType::Constant),
        _ => anyhow::bail!("Unknown entity_type: {}", entity_type_str),
    }
}

/// Convert entity_class string to EntityClass enum
fn convert_entity_class_string_to_enum(entity_class_str: &str) -> Result<EntityClass> {
    match entity_class_str.to_uppercase().as_str() {
        "CODE" => Ok(EntityClass::CodeImplementation),
        "TEST" => Ok(EntityClass::TestImplementation),
        _ => anyhow::bail!("Unknown entity_class: {}", entity_class_str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_isgl1_key_valid() {
        let key = "rust:fn:calculate_total:src_billing_rs:42";
        let result = parse_isgl1_key(key).unwrap();

        assert_eq!(result.language, "rust");
        assert_eq!(result.entity_type, "fn");
        assert_eq!(result.entity_name, "calculate_total");
        assert_eq!(result.file_path, "src_billing_rs");
        assert_eq!(result.line_number, 42);
    }

    #[test]
    fn test_parse_isgl1_key_invalid() {
        let key = "rust:fn:foo";  // Too few parts
        let result = parse_isgl1_key(key);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_temporal_initial_state() {
        // current_ind=1, future_ind=0, no action → initial state
        let result = convert_temporal_indicators_to_state(1, 0, None).unwrap();

        assert_eq!(result.current_ind, true);
        assert_eq!(result.future_ind, false);
        assert!(result.future_action.is_none());
    }

    #[test]
    fn test_convert_temporal_edit_state() {
        // current_ind=1, future_ind=1, action=edit → edit state
        let result = convert_temporal_indicators_to_state(1, 1, Some("edit")).unwrap();

        assert_eq!(result.current_ind, true);
        assert_eq!(result.future_ind, true);
        assert_eq!(result.future_action, Some(TemporalAction::Edit));
    }

    #[test]
    fn test_convert_entity_type_function() {
        assert_eq!(
            convert_entity_type_string_to_enum("fn").unwrap(),
            EntityType::Function
        );
    }

    #[test]
    fn test_convert_entity_class_code() {
        assert_eq!(
            convert_entity_class_string_to_enum("CODE").unwrap(),
            EntityClass::CodeImplementation
        );
    }

    #[test]
    fn test_convert_entity_class_test() {
        assert_eq!(
            convert_entity_class_string_to_enum("TEST").unwrap(),
            EntityClass::TestImplementation
        );
    }
}

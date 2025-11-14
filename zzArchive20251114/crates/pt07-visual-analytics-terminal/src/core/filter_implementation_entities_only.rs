//! Filter entities to show only implementation code (production)
//!
//! By default, Parseltongue analytics focuses on the Pareto principle:
//! Show only `CodeImplementation` entities, filtering out all `TestImplementation`.
//!
//! ## TDD Contract
//! - **Precondition**: Valid `Vec<CodeEntity>` from CozoDB
//! - **Postcondition**: Returns only entities where `tdd_classification.entity_class == CodeImplementation`
//! - **Error Conditions**: None (filtering is infallible)

use parseltongue_core::entities::{CodeEntity, EntityClass};

/// Filter entities to only Implementation (excludes all test entities)
///
/// This implements the core Pareto principle: focus on production code quality.
/// Test code is important but analyzed separately.
///
/// # Arguments
/// * `entities` - All entities from CozoDB
///
/// # Returns
/// Only entities classified as `CodeImplementation`
///
/// # Example
/// ```no_run
/// use parseltongue_core::entities::CodeEntity;
/// use pt07_visual_analytics_terminal::core::filter_implementation_entities_only;
///
/// let all_entities = vec![/* from CozoDB */];
/// let impl_only = filter_implementation_entities_only(all_entities);
/// // impl_only contains NO TestImplementation entities
/// ```
pub fn filter_implementation_entities_only(
    entities: Vec<CodeEntity>,
) -> Vec<CodeEntity> {
    entities
        .into_iter()
        .filter(|e| e.tdd_classification.entity_class == EntityClass::CodeImplementation)
        .collect()
}

/// Include all entities (when --include-tests flag is passed)
///
/// No filtering applied - returns all entities as-is.
///
/// # Arguments
/// * `entities` - All entities from CozoDB
///
/// # Returns
/// Same vec, unchanged
pub fn filter_include_all_entity_types(
    entities: Vec<CodeEntity>,
) -> Vec<CodeEntity> {
    entities  // No filtering
}

#[cfg(test)]
mod tests {
    use super::*;
    use parseltongue_core::entities::{
        CodeEntity, EntityClass, EntityMetadata, EntityType, InterfaceSignature,
        LanguageSpecificSignature, LineRange, RustSignature, TddClassification,
        TemporalState, TestabilityLevel, ComplexityLevel, RiskLevel, Visibility,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn create_test_entity(entity_class: EntityClass, name: &str) -> CodeEntity {
        CodeEntity {
            isgl1_key: format!("rust:fn:{}:test_file_rs:1-10", name),
            temporal_state: TemporalState::initial(),
            interface_signature: InterfaceSignature {
                entity_type: EntityType::Function,
                name: name.to_string(),
                visibility: Visibility::Public,
                file_path: PathBuf::from("test.rs"),
                line_range: LineRange { start: 1, end: 10 },
                module_path: vec!["test".to_string()],
                documentation: None,
                language_specific: LanguageSpecificSignature::Rust(RustSignature {
                    generics: vec![],
                    lifetimes: vec![],
                    where_clauses: vec![],
                    attributes: vec![],
                    trait_impl: None,
                }),
            },
            current_code: Some("fn test() {}".to_string()),
            future_code: None,
            tdd_classification: TddClassification {
                entity_class,
                testability: TestabilityLevel::Medium,
                complexity: ComplexityLevel::Simple,
                dependencies: 0,
                test_coverage_estimate: 0.0,
                critical_path: false,
                change_risk: RiskLevel::Low,
            },
            lsp_metadata: None,
            metadata: EntityMetadata {
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
                content_hash: String::new(),
                additional: HashMap::new(),
            },
            entity_class,  // v0.9.0: mandatory field
        }
    }

    #[test]
    fn test_filter_removes_all_test_implementations() {
        // Arrange: Create mix of code and test entities
        let entities = vec![
            create_test_entity(EntityClass::CodeImplementation, "production_fn"),
            create_test_entity(EntityClass::TestImplementation, "test_fn"),
            create_test_entity(EntityClass::CodeImplementation, "another_production_fn"),
            create_test_entity(EntityClass::TestImplementation, "another_test_fn"),
        ];

        // Act: Filter to implementation only
        let result = filter_implementation_entities_only(entities);

        // Assert: Only CodeImplementation entities remain
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|e| e.tdd_classification.entity_class == EntityClass::CodeImplementation));

        // Verify names
        let names: Vec<_> = result.iter()
            .map(|e| e.interface_signature.name.as_str())
            .collect();
        assert!(names.contains(&"production_fn"));
        assert!(names.contains(&"another_production_fn"));
        assert!(!names.contains(&"test_fn"));
        assert!(!names.contains(&"another_test_fn"));
    }

    #[test]
    fn test_filter_handles_empty_input() {
        // Arrange: Empty vector
        let entities: Vec<CodeEntity> = vec![];

        // Act
        let result = filter_implementation_entities_only(entities);

        // Assert: Returns empty vec
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_handles_all_test_entities() {
        // Arrange: Only test entities
        let entities = vec![
            create_test_entity(EntityClass::TestImplementation, "test1"),
            create_test_entity(EntityClass::TestImplementation, "test2"),
        ];

        // Act
        let result = filter_implementation_entities_only(entities);

        // Assert: Returns empty vec (all filtered out)
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_handles_all_code_entities() {
        // Arrange: Only code entities
        let entities = vec![
            create_test_entity(EntityClass::CodeImplementation, "code1"),
            create_test_entity(EntityClass::CodeImplementation, "code2"),
        ];

        // Act
        let result = filter_implementation_entities_only(entities);

        // Assert: Returns all entities unchanged
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_include_all_returns_unchanged_vec() {
        // Arrange: Mix of entities
        let entities = vec![
            create_test_entity(EntityClass::CodeImplementation, "code"),
            create_test_entity(EntityClass::TestImplementation, "test"),
        ];
        let original_count = entities.len();

        // Act
        let result = filter_include_all_entity_types(entities);

        // Assert: Count unchanged
        assert_eq!(result.len(), original_count);

        // Contains both types
        assert!(result.iter().any(|e| e.tdd_classification.entity_class == EntityClass::CodeImplementation));
        assert!(result.iter().any(|e| e.tdd_classification.entity_class == EntityClass::TestImplementation));
    }
}

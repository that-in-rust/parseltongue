//! Query Builder: Pure functional Datalog query composition
//!
//! Following S01 principles:
//! - Pure functions (no side effects)
//! - Functional composition
//! - Clear layering (L1→L2→L3)
//!
//! All functions take inputs, return outputs, no mutation, no I/O.

/// L1 Pure Function: Build export query with optional WHERE filter
///
/// # Arguments
/// * `include_current_code` - If true, include current_code and future_code columns
/// * `where_clause` - Filter fragment (default: "ALL" means no filter)
///
/// # Returns
/// Complete Datalog query string ready for CozoDB execution
///
/// # Examples
/// ```ignore
/// // Minimal projection (no code)
/// let query = build_export_query(false, "ALL");
/// // → ?[isgl1_key, interface_signature, tdd_classification, temporal_state] := ...
///
/// // With filter
/// let query = build_export_query(false, "future_action != null");
/// // → ?[...] := *CodeGraph{...}, future_action != null
///
/// // Full projection (with code)
/// let query = build_export_query(true, "entity_type ~ 'Function'");
/// // → ?[isgl1_key, current_code, future_code, ...] := ...
/// ```
pub fn build_export_query(include_current_code: bool, where_clause: &str) -> String {
    let (columns, fields) = if include_current_code {
        // Full projection (expensive - includes code)
        (
            "isgl1_key, current_code, future_code, interface_signature, tdd_classification, temporal_state, file_path, entity_type, language",
            "isgl1_key, current_code, future_code, interface_signature, tdd_classification, temporal_state, file_path, entity_type, language"
        )
    } else {
        // Minimal projection (cheap - signatures only)
        (
            "isgl1_key, interface_signature, tdd_classification, temporal_state, file_path, entity_type, language",
            "isgl1_key, interface_signature, tdd_classification, temporal_state, file_path, entity_type, language"
        )
    };

    match where_clause {
        "ALL" => {
            // No filter - select all
            format!("?[{}] := *CodeGraph{{{}}}", columns, fields)
        }
        filter => {
            // Apply WHERE filter
            format!("?[{}] := *CodeGraph{{{}}}, {}", columns, fields, filter)
        }
    }
}

/// L2 Pure Function: Compose WHERE clause from individual conditions
///
/// # Arguments
/// * `conditions` - Vector of filter conditions to AND together
///
/// # Returns
/// Composed WHERE clause string
///
/// # Examples
/// ```ignore
/// let clause = compose_where_clause(vec![
///     "future_action != null",
///     "entity_type ~ 'Function'"
/// ]);
/// // → "future_action != null AND entity_type ~ 'Function'"
/// ```
pub fn compose_where_clause(conditions: Vec<&str>) -> String {
    if conditions.is_empty() {
        "ALL".to_string()
    } else {
        conditions.join(" AND ")
    }
}

/// L1 Pure Function: Extract column list from include_current_code flag
///
/// Returns tuple of (query_columns, storage_fields)
#[allow(dead_code)]  // Reserved for future L2 composition patterns
fn get_projection_columns(include_current_code: bool) -> (&'static str, &'static str) {
    if include_current_code {
        (
            "isgl1_key, current_code, future_code, interface_signature, tdd_classification, temporal_state, file_path, entity_type, language",
            "isgl1_key, current_code, future_code, interface_signature, tdd_classification, temporal_state, file_path, entity_type, language"
        )
    } else {
        (
            "isgl1_key, interface_signature, tdd_classification, temporal_state, file_path, entity_type, language",
            "isgl1_key, interface_signature, tdd_classification, temporal_state, file_path, entity_type, language"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_export_query_minimal_no_filter() {
        let query = build_export_query(false, "ALL");
        assert!(query.contains("?[isgl1_key"));
        assert!(!query.contains("current_code"));
        assert!(!query.contains(","));  // No WHERE clause
    }

    #[test]
    fn test_build_export_query_minimal_with_filter() {
        let query = build_export_query(false, "future_action != null");
        assert!(query.contains("?[isgl1_key"));
        assert!(!query.contains("current_code"));
        assert!(query.contains("future_action != null"));
    }

    #[test]
    fn test_build_export_query_full_with_code() {
        let query = build_export_query(true, "ALL");
        assert!(query.contains("current_code"));
        assert!(query.contains("future_code"));
    }

    #[test]
    fn test_compose_where_clause_empty() {
        let clause = compose_where_clause(vec![]);
        assert_eq!(clause, "ALL");
    }

    #[test]
    fn test_compose_where_clause_single() {
        let clause = compose_where_clause(vec!["future_action != null"]);
        assert_eq!(clause, "future_action != null");
    }

    #[test]
    fn test_compose_where_clause_multiple() {
        let clause = compose_where_clause(vec![
            "future_action != null",
            "entity_type ~ 'Function'"
        ]);
        assert_eq!(clause, "future_action != null AND entity_type ~ 'Function'");
    }
}

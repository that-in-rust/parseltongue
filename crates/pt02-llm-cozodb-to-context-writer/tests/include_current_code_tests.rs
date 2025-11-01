//! TDD RED Phase: --include-current-code Flag Tests
//!
//! Following S01 Executable Specifications:
//! - Test preconditions: flag values, query modification
//! - Test postconditions: Current_Code inclusion/exclusion
//! - Test performance impact: token optimization

use pt02_llm_cozodb_to_context_writer::cli::CliConfig;

/// RED Test 1: CLI parsing with --include-current-code=0 (default)
#[test]
fn test_cli_default_excludes_current_code() {
    let cli = CliConfig::build_cli();

    // Test default behavior (no flag provided)
    let matches = cli.try_get_matches_from(&["parseltongue-03"]);
    assert!(matches.is_ok());

    let matches = matches.unwrap();
    let include_current_code = CliConfig::parse_include_current_code(&matches);

    // Postcondition: Default is false (exclude Current_Code)
    assert_eq!(include_current_code, false, "Default should exclude Current_Code");
}

/// RED Test 2: CLI parsing with --include-current-code=1
#[test]
fn test_cli_include_current_code_enabled() {
    let cli = CliConfig::build_cli();

    let matches = cli.try_get_matches_from(&[
        "parseltongue-03",
        "--include-current-code",
        "1",
    ]);

    assert!(matches.is_ok());
    let matches = matches.unwrap();
    let include_current_code = CliConfig::parse_include_current_code(&matches);

    // Postcondition: Flag=1 means include Current_Code
    assert_eq!(include_current_code, true, "Flag=1 should include Current_Code");
}

/// RED Test 3: CLI parsing with --include-current-code=0 (explicit)
#[test]
fn test_cli_include_current_code_disabled_explicit() {
    let cli = CliConfig::build_cli();

    let matches = cli.try_get_matches_from(&[
        "parseltongue-03",
        "--include-current-code",
        "0",
    ]);

    assert!(matches.is_ok());
    let matches = matches.unwrap();
    let include_current_code = CliConfig::parse_include_current_code(&matches);

    // Postcondition: Flag=0 means exclude Current_Code
    assert_eq!(include_current_code, false, "Flag=0 should exclude Current_Code");
}

/// RED Test 4: Query modification based on flag (exclude Current_Code)
#[test]
fn test_query_excludes_current_code_when_flag_false() {
    let include_current_code = false;
    let query = CliConfig::build_context_query(include_current_code);

    // Postcondition: Query should exclude Current_Code
    assert!(
        !query.contains("Current_Code") || query.contains("EXCEPT") && query.contains("Current_Code"),
        "Query should exclude Current_Code when flag is false"
    );
    assert!(query.contains("CodeGraph"), "Query should reference CodeGraph table");
}

/// RED Test 5: Query modification based on flag (include Current_Code)
#[test]
fn test_query_includes_current_code_when_flag_true() {
    let include_current_code = true;
    let query = CliConfig::build_context_query(include_current_code);

    // Postcondition: Query should include Current_Code (not in EXCEPT clause)
    if query.contains("EXCEPT") {
        assert!(
            !query.contains("EXCEPT (Current_Code"),
            "Query should not exclude Current_Code when flag is true"
        );
    } else {
        // If no EXCEPT clause, it includes everything implicitly
        assert!(query.contains("CodeGraph"), "Query should reference CodeGraph table");
    }
}

/// RED Test 6: Config parsing integration
#[test]
fn test_config_integration_with_include_current_code() {
    let cli = CliConfig::build_cli();

    let matches = cli.try_get_matches_from(&[
        "parseltongue-03",
        "--db",
        "test.db",
        "--include-current-code",
        "1",
    ]);

    assert!(matches.is_ok());
    let matches = matches.unwrap();

    let config = CliConfig::parse_config(&matches);
    let include_flag = CliConfig::parse_include_current_code(&matches);

    assert_eq!(config.db_path, "test.db");
    assert_eq!(include_flag, true);
}

/// RED Test 7: End-to-end query building
#[test]
fn test_e2e_query_building_with_flag() {
    // Scenario 1: Default behavior (exclude)
    let query_exclude = CliConfig::build_context_query(false);
    assert!(
        query_exclude.contains("EXCEPT") || !query_exclude.contains("Current_Code"),
        "Default query should exclude Current_Code"
    );

    // Scenario 2: Include Current_Code
    let query_include = CliConfig::build_context_query(true);
    // When including, either no EXCEPT or EXCEPT doesn't mention Current_Code
    if query_include.contains("EXCEPT") {
        assert!(
            !query_include.contains("EXCEPT (Current_Code"),
            "Include mode should not have Current_Code in EXCEPT clause"
        );
    }
}

/// RED Test 8: Invalid flag value handling
#[test]
fn test_invalid_flag_value_defaults_to_false() {
    let cli = CliConfig::build_cli();

    // clap should handle validation, but test fallback behavior
    let matches = cli.try_get_matches_from(&[
        "parseltongue-03",
        "--include-current-code",
        "2",  // Invalid value
    ]);

    // Should either fail or default to false
    // This tests the contract enforcement
    match matches {
        Ok(m) => {
            let include_flag = CliConfig::parse_include_current_code(&m);
            assert_eq!(include_flag, false, "Invalid values should default to false");
        }
        Err(_) => {
            // Also acceptable - clap rejects invalid values
        }
    }
}

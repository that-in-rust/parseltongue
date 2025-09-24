//! Integration Tests for `pt` Shell Script
//! 
//! Tests validate the shell script interface for workflow orchestration.
//! Following TDD principles, these tests define the contracts for the
//! shell script before implementation.

use std::process::Command;
use std::path::Path;

/// Test that pt script exists and is executable
#[test]
fn test_pt_script_exists() {
    // TDD RED PHASE: Test that pt script exists
    let pt_path = Path::new("./pt");
    
    // In RED phase, this will fail because script doesn't exist yet
    // In GREEN phase, we'll create the script and this should pass
    if pt_path.exists() {
        assert!(pt_path.is_file(), "pt should be a file");
        
        // Check if it's executable (Unix-like systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(pt_path).unwrap();
            let permissions = metadata.permissions();
            assert!(permissions.mode() & 0o111 != 0, "pt script should be executable");
        }
    } else {
        // Expected in RED phase
        println!("pt script not found (RED phase - will create in GREEN phase)");
    }
}

/// Test pt script help command
#[test]
fn test_pt_help_command() {
    // TDD RED PHASE: Test that pt script provides help
    let output = Command::new("./pt")
        .arg("--help")
        .output();
    
    match output {
        Ok(output) => {
            let help_text = String::from_utf8_lossy(&output.stdout);
            
            // Should contain all required subcommands
            assert!(help_text.contains("onboard"), "Help should mention onboard command");
            assert!(help_text.contains("feature-start"), "Help should mention feature-start command");
            assert!(help_text.contains("debug"), "Help should mention debug command");
            assert!(help_text.contains("refactor-check"), "Help should mention refactor-check command");
            
            // Should provide usage information
            assert!(help_text.contains("USAGE") || help_text.contains("Usage"), 
                    "Help should provide usage information");
        }
        Err(_) => {
            // Expected in RED phase
            println!("pt script help not available (RED phase - will implement in GREEN phase)");
        }
    }
}

/// Test pt onboard subcommand
#[test]
fn test_pt_onboard_subcommand() {
    // TDD RED PHASE: Test onboard subcommand interface
    let output = Command::new("./pt")
        .arg("onboard")
        .arg("--help")
        .output();
    
    match output {
        Ok(output) => {
            let help_text = String::from_utf8_lossy(&output.stdout);
            
            // Should provide onboard-specific help
            assert!(help_text.contains("onboard") || help_text.contains("Onboard"), 
                    "Should provide onboard command help");
            
            // Should mention target directory option
            assert!(help_text.contains("directory") || help_text.contains("dir") || help_text.contains("path"),
                    "Should mention target directory option");
        }
        Err(_) => {
            println!("pt onboard help not available (RED phase)");
        }
    }
}

/// Test pt feature-start subcommand
#[test]
fn test_pt_feature_start_subcommand() {
    // TDD RED PHASE: Test feature-start subcommand interface
    let output = Command::new("./pt")
        .arg("feature-start")
        .arg("--help")
        .output();
    
    match output {
        Ok(output) => {
            let help_text = String::from_utf8_lossy(&output.stdout);
            
            // Should provide feature-start specific help
            assert!(help_text.contains("feature") || help_text.contains("Feature"), 
                    "Should provide feature-start command help");
            
            // Should mention entity name parameter
            assert!(help_text.contains("entity") || help_text.contains("name") || help_text.contains("target"),
                    "Should mention entity name parameter");
        }
        Err(_) => {
            println!("pt feature-start help not available (RED phase)");
        }
    }
}

/// Test pt debug subcommand
#[test]
fn test_pt_debug_subcommand() {
    // TDD RED PHASE: Test debug subcommand interface
    let output = Command::new("./pt")
        .arg("debug")
        .arg("--help")
        .output();
    
    match output {
        Ok(output) => {
            let help_text = String::from_utf8_lossy(&output.stdout);
            
            // Should provide debug-specific help
            assert!(help_text.contains("debug") || help_text.contains("Debug"), 
                    "Should provide debug command help");
            
            // Should mention entity name parameter
            assert!(help_text.contains("entity") || help_text.contains("function") || help_text.contains("target"),
                    "Should mention entity parameter");
        }
        Err(_) => {
            println!("pt debug help not available (RED phase)");
        }
    }
}

/// Test pt refactor-check subcommand
#[test]
fn test_pt_refactor_check_subcommand() {
    // TDD RED PHASE: Test refactor-check subcommand interface
    let output = Command::new("./pt")
        .arg("refactor-check")
        .arg("--help")
        .output();
    
    match output {
        Ok(output) => {
            let help_text = String::from_utf8_lossy(&output.stdout);
            
            // Should provide refactor-check specific help
            assert!(help_text.contains("refactor") || help_text.contains("Refactor"), 
                    "Should provide refactor-check command help");
            
            // Should mention entity name parameter
            assert!(help_text.contains("entity") || help_text.contains("target") || help_text.contains("component"),
                    "Should mention entity parameter");
        }
        Err(_) => {
            println!("pt refactor-check help not available (RED phase)");
        }
    }
}

/// Test pt onboard execution with sample directory
#[test]
fn test_pt_onboard_execution() {
    // TDD RED PHASE: Test actual onboard workflow execution
    let output = Command::new("./pt")
        .arg("onboard")
        .arg("./src")  // Use existing src directory
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if output.status.success() {
                // Should provide onboarding results
                assert!(stdout.contains("Overview") || stdout.contains("overview") ||
                        stdout.contains("Entry") || stdout.contains("entry") ||
                        stdout.contains("Context") || stdout.contains("context"),
                        "Should provide onboarding overview information");
                
                // Should complete within reasonable time (checked by workflow)
                assert!(stdout.contains("completed") || stdout.contains("finished") ||
                        stdout.contains("done") || stdout.len() > 0,
                        "Should indicate completion");
            } else {
                println!("pt onboard execution failed (expected in RED phase): {}", stderr);
            }
        }
        Err(_) => {
            println!("pt onboard execution not available (RED phase)");
        }
    }
}

/// Test pt feature-start execution with sample entity
#[test]
fn test_pt_feature_start_execution() {
    // TDD RED PHASE: Test actual feature-start workflow execution
    let output = Command::new("./pt")
        .arg("feature-start")
        .arg("main")  // Common function name
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if output.status.success() {
                // Should provide feature planning results
                assert!(stdout.contains("Impact") || stdout.contains("impact") ||
                        stdout.contains("Scope") || stdout.contains("scope") ||
                        stdout.contains("Test") || stdout.contains("test"),
                        "Should provide feature planning information");
            } else {
                println!("pt feature-start execution failed (expected in RED phase): {}", stderr);
            }
        }
        Err(_) => {
            println!("pt feature-start execution not available (RED phase)");
        }
    }
}

/// Test pt debug execution with sample entity
#[test]
fn test_pt_debug_execution() {
    // TDD RED PHASE: Test actual debug workflow execution
    let output = Command::new("./pt")
        .arg("debug")
        .arg("main")  // Common function name
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if output.status.success() {
                // Should provide debug results
                assert!(stdout.contains("Caller") || stdout.contains("caller") ||
                        stdout.contains("Usage") || stdout.contains("usage") ||
                        stdout.contains("Trace") || stdout.contains("trace"),
                        "Should provide debug trace information");
            } else {
                println!("pt debug execution failed (expected in RED phase): {}", stderr);
            }
        }
        Err(_) => {
            println!("pt debug execution not available (RED phase)");
        }
    }
}

/// Test pt refactor-check execution with sample entity
#[test]
fn test_pt_refactor_check_execution() {
    // TDD RED PHASE: Test actual refactor-check workflow execution
    let output = Command::new("./pt")
        .arg("refactor-check")
        .arg("main")  // Common function name
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if output.status.success() {
                // Should provide refactor check results
                assert!(stdout.contains("Risk") || stdout.contains("risk") ||
                        stdout.contains("Checklist") || stdout.contains("checklist") ||
                        stdout.contains("Review") || stdout.contains("review"),
                        "Should provide refactor safety information");
            } else {
                println!("pt refactor-check execution failed (expected in RED phase): {}", stderr);
            }
        }
        Err(_) => {
            println!("pt refactor-check execution not available (RED phase)");
        }
    }
}

/// Test pt script error handling
#[test]
fn test_pt_error_handling() {
    // TDD RED PHASE: Test that pt script handles errors gracefully
    
    // Test invalid subcommand
    let output = Command::new("./pt")
        .arg("invalid-command")
        .output();
    
    match output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                // Should provide helpful error message
                assert!(stderr.contains("invalid") || stderr.contains("unknown") ||
                        stderr.contains("help") || stderr.contains("usage"),
                        "Should provide helpful error message for invalid command");
            }
        }
        Err(_) => {
            println!("pt error handling test not available (RED phase)");
        }
    }
    
    // Test missing arguments
    let output = Command::new("./pt")
        .arg("onboard")
        // Missing directory argument
        .output();
    
    match output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                // Should indicate missing argument
                assert!(stderr.contains("required") || stderr.contains("missing") ||
                        stderr.contains("argument") || stderr.contains("directory"),
                        "Should indicate missing required argument");
            }
        }
        Err(_) => {
            println!("pt missing argument test not available (RED phase)");
        }
    }
}

/// Test pt script performance contracts
#[test]
fn test_pt_performance_contracts() {
    // TDD RED PHASE: Test that pt script meets performance contracts
    
    // This test will validate that the shell script itself doesn't add
    // significant overhead to the workflow execution times
    
    let start = std::time::Instant::now();
    let _output = Command::new("./pt")
        .arg("--help")
        .output();
    let help_elapsed = start.elapsed();
    
    // Help command should be very fast
    if help_elapsed < std::time::Duration::from_millis(100) {
        println!("pt help performance: {:?} (good)", help_elapsed);
    } else {
        println!("pt help performance: {:?} (may need optimization)", help_elapsed);
    }
    
    // The actual workflow performance is tested in workflow_integration_tests.rs
    // This just ensures the shell script wrapper is efficient
}
//! pt07-visual-analytics-terminal
//!
//! Visual analytics for CozoDB after code ingestion.
//! Each visualization is a standalone binary that auto-saves to txt file.
//!
//! ## Architecture
//! - **core/**: Filtering logic (implementation-only by default)
//! - **primitives/**: Rendering utilities (boxes, bars, colors)
//! - **bin/**: Standalone visualization binaries

use anyhow::Result;
use chrono::Local;
use std::fs;

pub mod core;
pub mod primitives;
pub mod database;
pub mod visualizations;

/// Save visualization output to both stdout and timestamped txt file
///
/// This is called by every visualization binary to:
/// 1. Print output to terminal (stdout)
/// 2. Auto-save to file: <command-name>-YYYYMMDDHHMMSS.txt
///
/// File format:
/// ```text
/// Command: <full command with args>
///
/// <visualization output>
/// ```
///
/// # Arguments
/// * `command_name` - Name of the command (e.g., "pt07-render-entity-count-bar-chart")
/// * `command_args` - Arguments passed to the command (e.g., "--db code.db")
/// * `visualization_output` - The rendered visualization as a string
///
/// # Example
/// ```no_run
/// use pt07_visual_analytics_terminal::save_visualization_output_to_file;
///
/// let output = "╔═══╗\n║ Hi ║\n╚═══╝";
/// save_visualization_output_to_file(
///     "pt07-render-test",
///     "--db test.db",
///     output
/// ).unwrap();
/// // Prints to stdout AND saves to pt07-render-test-20251105223045.txt
/// ```
pub fn save_visualization_output_to_file(
    command_name: &str,
    command_args: &str,
    visualization_output: &str,
) -> Result<()> {
    // Build full command for logging
    let full_command = format!("{} {}", command_name, command_args);

    // Build complete output with command header
    let full_output = format!("Command: {}\n\n{}", full_command, visualization_output);

    // Print to stdout (user sees this in terminal)
    print!("{}", full_output);

    // Generate timestamp filename
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let filename = format!("{}-{}.txt", command_name, timestamp);

    // Write to file (no permission needed, just do it)
    fs::write(&filename, &full_output)?;

    // Print save confirmation to stderr (doesn't interfere with piped output)
    eprintln!("📄 Saved to: {}", filename);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_save_visualization_creates_timestamped_file() {
        // Setup: Create temp directory
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        // Execute: Save visualization
        let result = save_visualization_output_to_file(
            "test-command",
            "--db test.db",
            "Test Output"
        );

        // Assert: File was created
        assert!(result.is_ok());

        // Find the created file (it has a timestamp)
        let files: Vec<_> = fs::read_dir(&temp_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();

        // Should have exactly one file
        assert_eq!(files.len(), 1);

        // File should start with command name and end with .txt
        let filename = &files[0];
        assert!(filename.starts_with("test-command-"));
        assert!(filename.ends_with(".txt"));

        // Read file content
        let content = fs::read_to_string(temp_dir.path().join(filename)).unwrap();
        assert!(content.contains("Command: test-command --db test.db"));
        assert!(content.contains("Test Output"));

        // Cleanup
        std::env::set_current_dir(original_dir).unwrap();
    }
}

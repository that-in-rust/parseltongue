include!(concat!(env!("OUT_DIR"), "/summary.rs"));

use tree_sitter::{Parser, Language};
use serde_json::Value;
use tokio::process::Command;
use std::sync::Arc;

// Declare external functions for tree-sitter languages
extern "C" {
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_javascript() -> Language;
    fn tree_sitter_python() -> Language;
    fn tree_sitter_java() -> Language;
    fn tree_sitter_c() -> Language;
    fn tree_sitter_cpp() -> Language;
    fn tree_sitter_go() -> Language;
}

// Update analyze_file function to include AST parsing and linters
fn analyze_file(entry: &ZipEntry) -> Result<FileSummary> {
    // ... existing code ...

    // Initialize Tree-sitter parser
    let mut parser = Parser::new();

    // Set language
    let ts_language = match language {
        LanguageType::Rust => unsafe { tree_sitter_rust() },
        LanguageType::JavaScript => unsafe { tree_sitter_javascript() },
        LanguageType::Python => unsafe { tree_sitter_python() },
        LanguageType::Java => unsafe { tree_sitter_java() },
        LanguageType::C => unsafe { tree_sitter_c() },
        LanguageType::Cpp => unsafe { tree_sitter_cpp() },
        LanguageType::Go => unsafe { tree_sitter_go() },
        LanguageType::Unknown => return Err(anyhow::anyhow!("Unsupported language")),
    };

    parser.set_language(ts_language).context("Failed to set language for parser")?;

    // Parse the content
    let tree = parser.parse(&content, None).context("Failed to parse content")?;

    // Analyze the AST
    let ast_depth = calculate_ast_depth(tree.root_node());
    let ast_node_count = tree.root_node().child_count();

    // Run linters (synchronous execution)
    let (lint_errors, lint_warnings) = match language {
        LanguageType::Python => run_pylint_sync(&entry.name, &content)?,
        LanguageType::JavaScript => run_eslint_sync(&entry.name, &content)?,
        // Add linters for other languages as needed
        _ => (0, 0),
    };

    Ok(FileSummary {
        // ... existing fields ...
        ast_depth: ast_depth as u32,
        ast_node_count: ast_node_count as u32,
        lint_errors,
        lint_warnings,
        // ... populate other fields as needed ...
        ..Default::default()
    })
}

// Helper function to calculate AST depth
fn calculate_ast_depth(node: tree_sitter::Node) -> usize {
    if node.child_count() == 0 {
        1
    } else {
        node.children().map(calculate_ast_depth).max().unwrap_or(0) + 1
    }
}

// Function to run pylint synchronously
fn run_pylint_sync(file_name: &str, content: &str) -> Result<(u32, u32)> {
    // Write content to temporary file
    let tmp_dir = std::env::temp_dir();
    let tmp_file_path = tmp_dir.join(file_name);
    std::fs::write(&tmp_file_path, content)?;

    // Run pylint
    let output = std::process::Command::new("pylint")
        .arg(&tmp_file_path)
        .arg("--output-format=json")
        .output()?;

    // Parse output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let messages: Vec<Value> = serde_json::from_str(&stdout).unwrap_or_default();
    let lint_errors = messages.iter().filter(|msg| msg["type"] == "error").count() as u32;
    let lint_warnings = messages.iter().filter(|msg| msg["type"] == "warning").count() as u32;

    // Clean up temporary file
    std::fs::remove_file(&tmp_file_path)?;

    Ok((lint_errors, lint_warnings))
}

// Function to run ESLint synchronously
fn run_eslint_sync(file_name: &str, content: &str) -> Result<(u32, u32)> {
    // Write content to temporary file
    let tmp_dir = std::env::temp_dir();
    let tmp_file_path = tmp_dir.join(file_name);
    std::fs::write(&tmp_file_path, content)?;

    // Run ESLint
    let output = std::process::Command::new("eslint")
        .arg(&tmp_file_path)
        .arg("--format=json")
        .output()?;

    // Parse output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let messages_json: Vec<Value> = serde_json::from_str(&stdout).unwrap_or_default();
    let messages = messages_json.get(0)
        .and_then(|v| v["messages"].as_array())
        .cloned()
        .unwrap_or_default();
    let lint_errors = messages.iter().filter(|msg| msg["severity"] == 2).count() as u32;
    let lint_warnings = messages.iter().filter(|msg| msg["severity"] == 1).count() as u32;

    // Clean up temporary file
    std::fs::remove_file(&tmp_file_path)?;

    Ok((lint_errors, lint_warnings))
}

// Modify the processing of entries to use spawn_blocking
async fn process_entries(
    rx: mpsc::Receiver<ZipEntry>,
    db_manager: Arc<DatabaseManager>,
    analyzed_files: Arc<std::sync::Mutex<Vec<FileSummary>>>,
    error_logger: Arc<ErrorLogger>,
    pb: Arc<ProgressBar>,
) {
    while let Some(entry) = rx.recv().await {
        let db_manager = db_manager.clone();
        let analyzed_files = analyzed_files.clone();
        let error_logger = error_logger.clone();
        let pb = pb.clone();

        tokio::spawn(async move {
            // Use spawn_blocking to handle non-Send types
            match tokio::task::spawn_blocking(move || analyze_file(&entry)).await {
                Ok(Ok(parsed_file)) => {
                    if let Err(e) = db_manager.store(entry.name.as_bytes(), &entry.content) {
                        let error_msg = format!("Failed to store file {}: {:?}", entry.name, e);
                        error!("{}", error_msg);
                        if let Err(log_err) = error_logger.log_error(&error_msg) {
                            error!("Failed to log error: {:?}", log_err);
                        }
                    }
                    analyzed_files.lock().unwrap().push(parsed_file);
                }
                Ok(Err(e)) => {
                    let error_msg = format!("Failed to analyze file {}: {:?}", entry.name, e);
                    error!("{}", error_msg);
                    if let Err(log_err) = error_logger.log_error(&error_msg) {
                        error!("Failed to log error: {:?}", log_err);
                    }
                }
                Err(e) => {
                    let error_msg = format!("Task join error for file {}: {:?}", entry.name, e);
                    error!("{}", error_msg);
                    if let Err(log_err) = error_logger.log_error(&error_msg) {
                        error!("Failed to log error: {:?}", log_err);
                    }
                }
            }
            pb.inc(1);
        });
    }
}

// ... existing code ...

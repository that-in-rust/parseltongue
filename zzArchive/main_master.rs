++ b/mvp01/src/main.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debugging: Print the path being included
    println!("Including file from: {}", concat!(env!("OUT_DIR"), "/summary.rs"));
    // Your main logic here
    println!("Hello, world!");
    Ok(())
include!(concat!(env!("OUT_DIR"), "/summary.rs"));
++ b/mvp01/src/main.rs
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
        // ... existing fields ...
        ast_depth: ast_depth as u32,
        ast_node_count: ast_node_count as u32,
        lint_errors,
        lint_warnings,
        // ... populate other fields as needed ...
// Helper function to calculate AST depth
fn calculate_ast_depth(node: tree_sitter::Node) -> usize {
    if node.child_count() == 0 {
        1
    } else {
        node.children().map(calculate_ast_depth).max().unwrap_or(0) + 1
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
// Modify the processing of entries to use spawn_blocking
async fn process_entries(
    rx: mpsc::Receiver<ZipEntry>,
    db_manager: Arc<DatabaseManager>,
    analyzed_files: Arc<std::sync::Mutex<Vec<FileSummary>>>,
    error_logger: Arc<ErrorLogger>,
    pb: Arc<ProgressBar>,
) {
        let db_manager = db_manager.clone();
        let analyzed_files = analyzed_files.clone();
        let error_logger = error_logger.clone();
        let pb = pb.clone();
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

// ... existing code ...
++ b/mvp01/src/main.rs
fn analyze_file(entry: &ZipEntry) -> Result<FileSummary> {
    Ok(FileSummary {
        language: language.to_string(),
        loc: loc as u32,
        code_lines: code_lines as u32,
        comment_lines: comment_lines as u32,
        blank_lines: blank_lines as u32,
        function_count: function_count as u32,
        class_count: class_count as u32,
        cyclomatic_complexity: cyclomatic_complexity as u32,
        cognitive_complexity: cognitive_complexity as u32,
        // Initialize other fields as needed
        ..Default::default()
    fn write_llm_ready_output(&self, files: &[FileSummary]) -> Result<()> {
            files: files.to_vec(),
            total_loc: files.iter().map(|f| f.loc).sum(),
                *acc.entry(f.language.clone()).or_insert(0) += 1;
            total_files: files.len() as u32,
            // Initialize other fields as needed
++ b/mvp01/src/main.rs
    let (cyclomatic_complexity, cognitive_complexity) = analyze_code_complexity(&content);
fn analyze_code_complexity(content: &str) -> (usize, usize) {
    // This is a placeholder implementation
    // TODO: Implement actual cyclomatic and cognitive complexity analysis
    let cyclomatic_complexity = content.lines().filter(|line| line.contains("if") || line.contains("for") || line.contains("while")).count();
    let cognitive_complexity = content.lines().filter(|line| line.contains("if") || line.contains("for") || line.contains("while") || line.contains("switch")).count();
    (cyclomatic_complexity, cognitive_complexity)
++ b/mvp01/src/main.rs
use prost::Message;
use prost_types::{ProjectSummary, FileSummary};
++ b/mvp01/src/main.rs
    zip_path: PathBuf,
        let file = File::open(&zip_path).context("Failed to open ZIP file")?;
        if let Err(e) = process_zip(config.input_zip, tx, pb_clone).await {
++ b/mvp01/src/main.rs
use std::io::{BufWriter, Write, Read};
use clap::Parser;
use chrono::Utc;
use colored::Colorize;
    tokio::task::spawn_blocking(move || -> Result<()> {
        let file = File::open(zip_path).context("Failed to open ZIP file")?;
        let mut archive = ZipArchive::new(file).context("Failed to create ZIP archive")?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).context("Failed to get ZIP entry")?;
            if file.is_dir() {
                continue;
            }
            let name = file.name().to_string();
            let mut content = Vec::new();
            file.read_to_end(&mut content).context("Failed to read ZIP entry content")?;
            tx.blocking_send(ZipEntry { name, content }).context("Failed to send ZIP entry")?;
            pb.inc(1);
        }
        Ok(())
    }).await?
++ b/mvp01/src/main.rs
use log::{error, info};
use clap::Parser; // Import Parser trait
    let error_logger_clone = Arc::clone(&error_logger); // Clone error_logger
        if let Err(e) = process_zip(&config.input_zip, tx, pb_clone).await {
++ b/mvp01/src/main.rs
use chrono::Utc;
use colored::*;
use std::io::{BufWriter, Write, Read}; // Added Read trait
use std::fs::{File, create_dir_all, remove_file};
use zip::ZipArchive;

// Use fully qualified paths for conflicting types
use clap;
use tree_sitter;
// Proto generated code
include!(concat!(env!("OUT_DIR"), "/summary.rs"));

// CLI module
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    #[clap(short, long, value_parser)]
    input_zip: PathBuf,

    #[clap(short, long, value_parser)]
    output_dir: PathBuf,

    /// Enable verbose output
    #[clap(short, long)]
    verbose: bool,
}

// ZIP Processing module
struct ZipEntry {
    name: String,
    content: Vec<u8>,
}

async fn process_zip(
    zip_path: &Path,
    tx: mpsc::Sender<ZipEntry>,
    pb: Arc<ProgressBar>,
    error_logger: Arc<ErrorLogger>,
) -> Result<()> {
    let file = File::open(zip_path).context("Failed to open ZIP file")?;
    let mut archive = ZipArchive::new(file).context("Failed to create ZIP archive")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).context("Failed to get ZIP entry")?;
        if file.is_dir() {
            continue;
        }

        let name = file.name().to_string();
        let mut content = Vec::new();
        file.read_to_end(&mut content).context("Failed to read ZIP entry content")?;

        tx.send(ZipEntry { name, content }).await.context("Failed to send ZIP entry")?;
        pb.inc(1);
    }

    Ok(())
}

fn count_zip_entries(zip_path: &Path) -> Result<usize> {
    let file = File::open(zip_path).context("Failed to open ZIP file")?;
    let archive = ZipArchive::new(file).context("Failed to create ZIP archive")?;
    Ok(archive.len())
// Database Management module
struct DatabaseManager {
    db: sled::Db,
    cache: moka::sync::Cache<Vec<u8>, Vec<u8>>,
    fn new(path: &Path) -> Result<Self> {
        let cache = moka::sync::Cache::new(10_000);
        Ok(Self { db, cache })
    fn store(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.cache.insert(key.to_vec(), value.to_vec());
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        if let Some(cached) = self.cache.get(key) {
            return Ok(Some(cached));
        }
        let result = self.db.get(key)
            .context("Failed to retrieve from database")?
            .map(|ivec| ivec.to_vec());
        if let Some(ref data) = result {
            self.cache.insert(key.to_vec(), data.clone());
        }
        Ok(result)
    }

    fn close(self) -> Result<()> {
        self.db.flush().context("Failed to flush database")?;
        Ok(())
// Code Analysis module
#[derive(Debug, Serialize, Deserialize, Clone)]
enum LanguageType {
impl LanguageType {
    fn from_extension(ext: &str) -> Self {
        match ext {
            "rs" => LanguageType::Rust,
            "js" => LanguageType::JavaScript,
            "py" => LanguageType::Python,
            "java" => LanguageType::Java,
            "c" => LanguageType::C,
            "cpp" | "cxx" | "cc" => LanguageType::Cpp,
            "go" => LanguageType::Go,
            _ => LanguageType::Unknown,
#[derive(Debug, Serialize, Deserialize)]
struct ParsedFile {
    name: String,
    language: LanguageType,
    loc: usize,
    code_lines: usize,
    comment_lines: usize,
    blank_lines: usize,
    function_count: usize,
    class_count: usize,
    cyclomatic_complexity: usize,
    cognitive_complexity: usize,
fn analyze_file(entry: &ZipEntry) -> Result<ParsedFile> {
    let extension = Path::new(&entry.name)
        .extension()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("");
    let language = LanguageType::from_extension(extension);
    
    let content = String::from_utf8_lossy(&entry.content);
    let (loc, code_lines, comment_lines, blank_lines) = count_lines(&content);
    
    let (function_count, class_count) = count_functions_and_classes(&content, &language);
    let cyclomatic_complexity = calculate_cyclomatic_complexity(&content, &language);
    let cognitive_complexity = calculate_cognitive_complexity(&content, &language);
        name: entry.name.clone(),
        code_lines,
        comment_lines,
        blank_lines,
        function_count,
        class_count,
        cyclomatic_complexity,
        cognitive_complexity,
fn count_lines(content: &str) -> (usize, usize, usize, usize) {
    let mut code_lines = 0;
    let mut comment_lines = 0;
    let mut blank_lines = 0;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            blank_lines += 1;
        } else if trimmed.starts_with("//") || trimmed.starts_with("#") {
            comment_lines += 1;
            code_lines += 1;
    (loc, code_lines, comment_lines, blank_lines)
fn count_functions_and_classes(content: &str, language: &LanguageType) -> (usize, usize) {
    // This is a simplified implementation. In a real-world scenario, you'd use tree-sitter for more accurate parsing.
    let function_keywords = match language {
        LanguageType::Rust => vec!["fn"],
        LanguageType::JavaScript => vec!["function", "=>"],
        LanguageType::Python => vec!["def"],
        LanguageType::Java | LanguageType::C | LanguageType::Cpp => vec!["("],
        LanguageType::Go => vec!["func"],
        LanguageType::Unknown => vec![],
    };

    let class_keywords = match language {
        LanguageType::Rust => vec!["struct", "enum"],
        LanguageType::JavaScript | LanguageType::Java => vec!["class"],
        LanguageType::Python => vec!["class"],
        LanguageType::C => vec!["struct"],
        LanguageType::Cpp => vec!["class", "struct"],
        LanguageType::Go => vec!["type"],
        LanguageType::Unknown => vec![],
    };

    let function_count = function_keywords.iter().map(|&kw| content.matches(kw).count()).sum();
    let class_count = class_keywords.iter().map(|&kw| content.matches(kw).count()).sum();

    (function_count, class_count)
fn calculate_cyclomatic_complexity(content: &str, language: &LanguageType) -> usize {
    // This is a simplified implementation. In a real-world scenario, you'd use tree-sitter for more accurate parsing.
    let complexity_keywords = match language {
        LanguageType::Rust | LanguageType::JavaScript | LanguageType::Java | LanguageType::C | LanguageType::Cpp | LanguageType::Go =>
            vec!["if", "else", "while", "for", "&&", "||", "?", "switch", "case"],
        LanguageType::Python =>
            vec!["if", "elif", "else", "while", "for", "and", "or"],
        LanguageType::Unknown => vec![],
    };

    1 + complexity_keywords.iter().map(|&kw| content.matches(kw).count()).sum::<usize>()
fn calculate_cognitive_complexity(content: &str, language: &LanguageType) -> usize {
    // This is a simplified implementation. In a real-world scenario, you'd use tree-sitter for more accurate parsing.
    let complexity_keywords = match language {
        LanguageType::Rust | LanguageType::JavaScript | LanguageType::Java | LanguageType::C | LanguageType::Cpp | LanguageType::Go =>
            vec!["if", "else", "while", "for", "switch", "case", "try", "catch", "&&", "||", "?:"],
        LanguageType::Python =>
            vec!["if", "elif", "else", "while", "for", "try", "except", "and", "or"],
        LanguageType::Unknown => vec![],
    };
    complexity_keywords.iter().map(|&kw| content.matches(kw).count()).sum()
// Output Management module
struct OutputManager {
    fn new(output_dir: PathBuf) -> Result<Self> {
        create_dir_all(&output_dir).context("Failed to create output directory")?;
        Ok(Self { output_dir })
    fn write_llm_ready_output(&self, files: &[ParsedFile]) -> Result<()> {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let output_path = self.output_dir.join(format!("LLM-ready-{}.pb", timestamp));

        let file = File::create(&output_path).context("Failed to create output file")?;
        let mut writer = BufWriter::new(file);

        let project_summary = ProjectSummary {
            files: files.iter().map(|f| FileSummary {
                name: f.name.clone(),
                language: format!("{:?}", f.language),
                loc: f.loc as u32,
                code_lines: f.code_lines as u32,
                comment_lines: f.comment_lines as u32,
                blank_lines: f.blank_lines as u32,
                function_count: f.function_count as u32,
                class_count: f.class_count as u32,
                cyclomatic_complexity: f.cyclomatic_complexity as u32,
                cognitive_complexity: f.cognitive_complexity as u32,
                ..Default::default()
            }).collect(),
            total_loc: files.iter().map(|f| f.loc).sum::<usize>() as u32,
            language_breakdown: files.iter().fold(std::collections::HashMap::new(), |mut acc, f| {
                *acc.entry(format!("{:?}", f.language)).or_insert(0) += 1;
                acc
            }),
            ..Default::default()
        };

        let mut buf = Vec::new();
        project_summary.encode(&mut buf).context("Failed to encode project summary")?;
        writer.write_all(&buf).context("Failed to write output")?;
        writer.flush().context("Failed to flush output")?;

    fn write_progress(&self, message: &str) -> Result<()> {
        writeln!(file, "[{}] {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), message)

    fn cleanup_old_files(&self, max_files: usize) -> Result<()> {
        let mut files: Vec<_> = std::fs::read_dir(&self.output_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("pb") {
                    Some((entry.metadata().ok()?.modified().ok()?, path))
                } else {
                    None
                }
            })
            .collect();

        if files.len() > max_files {
            files.sort_by(|a, b| b.0.cmp(&a.0));
            for (_, path) in files.iter().skip(max_files) {
                remove_file(path).context("Failed to remove old output file")?;
            }
        }

        Ok(())
    }
// Error Logging module
struct ErrorLogger {
    file: std::sync::Mutex<File>,
    fn new(path: &Path) -> Result<Self> {
        let file = File::create(path).context("Failed to create error log file")?;
        Ok(Self { file: std::sync::Mutex::new(file) })
    fn log_error(&self, message: &str) -> Result<()> {
        writeln!(file, "[{}] {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), message)
// Avengers-themed Logging
struct AvengersLogger {
    file: std::sync::Mutex<File>,
impl AvengersLogger {
    fn new(path: &Path) -> Result<Self> {
        let file = File::create(path).context("Failed to create log file")?;
        Ok(Self { file: std::sync::Mutex::new(file) })
}
impl log::Log for AvengersLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let (color, avenger) = match record.level() {
                log::Level::Error => ("red", "Iron Man"),
                log::Level::Warn => ("yellow", "Captain America"),
                log::Level::Info => ("green", "Black Widow"),
                log::Level::Debug => ("blue", "Hulk"),
                log::Level::Trace => ("magenta", "Thor"),
            };

            let message = format!("[{}] {} - {}: {}",
                Utc::now().format("%Y-%m-%d %H:%M:%S"),
                avenger,
                record.level(),
                record.args()
            );

            println!("{}", message.color(color));

            if let Ok(mut file) = self.file.lock() {
                writeln!(file, "{}", message).expect("Failed to write to log file");
            }
        }
    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            file.flush().expect("Failed to flush log file");
        }
}
fn init_logger(path: &Path) -> Result<()> {
    let logger = AvengersLogger::new(path)?;
    log::set_boxed_logger(Box::new(logger)).context("Failed to set logger")?;
    log::set_max_level(log::LevelFilter::Info);
    Ok(())
// Main function
    let db_manager = DatabaseManager::new(&config.output_dir.join("db"))?;
    let output_manager = OutputManager::new(config.output_dir.clone())?;
    let error_logger = Arc::new(ErrorLogger::new(&config.output_dir.join("error.log"))?);
    let total_files = count_zip_entries(&config.input_zip)?;
    output_manager.write_progress("Starting ZIP processing")?;
    tokio::spawn(async move {
        if let Err(e) = process_zip(&config.input_zip, tx, pb_clone, error_logger_clone).await {
        match analyze_file(&entry) {
            Ok(parsed_file) => {
                db_manager.store(entry.name.as_bytes(), &entry.content)?;
                analyzed_files.push(parsed_file);
            },
                let error_msg = format!("Failed to analyze file {}: {:?}", entry.name, e);
                error_logger.log_error(&error_msg)?;
    output_manager.write_progress("File analysis completed")?;
    output_manager.write_llm_ready_output(&analyzed_files)?;
    output_manager.write_progress("LLM-ready output generated")?;
    output_manager.cleanup_old_files(5)?;

    db_manager.close()?;
// Test module
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;
    #[tokio::test]
    async fn test_main_workflow() -> Result<()> {
        // Create a temporary directory for our test
        let temp_dir = tempdir()?;
        let zip_path = temp_dir.path().join("test.zip");
        let output_dir = temp_dir.path().join("output");
        // Create a test ZIP file
        {
            let file = File::create(&zip_path)?;
            let mut zip = zip::ZipWriter::new(file);
            zip.start_file("test1.rs", zip::write::FileOptions::default())?;
            zip.write_all(b"fn main() { println!(\"Hello, World!\"); }")?;
            zip.start_file("test2.py", zip::write::FileOptions::default())?;
            zip.write_all(b"print('Hello, World!')")?;
            zip.finish()?;
        }
        // Set up test configuration
        let config = Config {
            input_zip: zip_path,
            output_dir: output_dir.clone(),
            verbose: false,
        };

        // Run the main workflow
        tokio::spawn(async move {
            if let Err(e) = main().await {
                panic!("Main workflow failed: {:?}", e);
            }
        })
        .await?;

        // Check if output files were created
        assert!(output_dir.join("log.txt").exists());
        assert!(output_dir.join("error.log").exists());
        assert!(output_dir.join("db").exists());
        assert!(output_dir.join("processProgress.txt").exists());

        // Check if LLM-ready output files were created
        let llm_ready_files: Vec<_> = std::fs::read_dir(&output_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().into_string().ok()?;
                if name.starts_with("LLM-ready-") && name.ends_with(".pb") {
                    Some(name)
                } else {
                    None
                }
            })
            .collect();
        assert!(!llm_ready_files.is_empty(), "No LLM-ready output files found");
        Ok(())
++ b/mvp01/src/main.rs
use tree_sitter::{Parser, Language, Node};
use crate::logging::{init_logger, ErrorLogger};
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;
fn calculate_cyclomatic_complexity(node: &Node) -> usize {
fn calculate_cognitive_complexity(node: &Node) -> usize {
    use std::io::Write;
    fn test_detect_language() {
        assert_eq!(detect_language("test.rs"), LanguageType::Rust);
        assert_eq!(detect_language("script.js"), LanguageType::JavaScript);
        assert_eq!(detect_language("main.py"), LanguageType::Python);
        assert_eq!(detect_language("unknown.txt"), LanguageType::Unknown);
    fn test_count_lines() {
        let content = b"fn main() {\n    // This is a comment\n    println!(\"Hello, World!\");\n}\n";
        let (loc, code, comments, blanks) = count_lines(content);
        assert_eq!(loc, 4);
        assert_eq!(code, 2);
        assert_eq!(comments, 1);
        assert_eq!(blanks, 1);
    }

    #[test]
    fn test_database_manager() -> Result<()> {
        let temp_dir = tempdir()?;
        let db_manager = DatabaseManager::new(temp_dir.path())?;

        let key = b"test_key";
        let value = b"test_value";

        db_manager.store(key, value)?;
        let retrieved = db_manager.get(key)?;

        assert_eq!(retrieved, Some(value.to_vec()));
        Ok(())
    }

    #[test]
    fn test_output_manager() -> Result<()> {
        let temp_dir = tempdir()?;
        let output_manager = OutputManager::new(temp_dir.path().to_path_buf())?;

                cyclomatic_complexity: 2,
                cognitive_complexity: 3,

        output_manager.write_llm_ready_output(&files)?;

        // Check if the output file was created
        let output_files: Vec<_> = std::fs::read_dir(temp_dir.path())?.filter_map(|entry| {
            entry.ok().and_then(|e| {
                let name = e.file_name().into_string().ok()?;
                if name.starts_with("LLM-ready-") && name.ends_with(".pb.gz") {
                    Some(name)
                } else {
                    None
                }
            })
        }).collect();

        assert_eq!(output_files.len(), 1);

        Ok(())
    }

    #[test]
    fn test_process_zip() -> Result<()> {
        let temp_dir = tempdir()?;
        let zip_path = temp_dir.path().join("test.zip");
        let mut zip = zip::ZipWriter::new(File::create(&zip_path)?);

        zip.start_file("test.rs", Default::default())?;
        zip.write_all(b"fn main() {\n    println!(\"Hello, World!\");\n}\n")?;
        zip.finish()?;

        let (tx, mut rx) = mpsc::channel(100);
        let pb = Arc::new(ProgressBar::new(1));
        let error_logger = Arc::new(ErrorLogger::new(temp_dir.path().join("error.log"))?);

        let zip_file = File::open(&zip_path)?;
        let archive = zip::ZipArchive::new(zip_file)?;

        process_zip(archive, tx, pb, error_logger)?;

        let entry = rx.try_recv()?;
        assert_eq!(entry.name, "test.rs");
        assert_eq!(entry.content, b"fn main() {\n    println!(\"Hello, World!\");\n}\n");

        Ok(())
    init_logger(&config.output_dir.join("log.txt"))?;
    let output_manager = OutputManager::new(config.output_dir.clone())
    // Cleanup old files, keeping only the last 5
    output_manager.cleanup_old_files(5).context("Failed to cleanup old files")?;

mod cli;
mod zip_processing;
mod database;
mod code_analysis;
mod output;
mod logging;

use cli::Config;
use zip_processing::process_zip;
use database::DatabaseManager;
use code_analysis::{analyze_file, calculate_cyclomatic_complexity, calculate_cognitive_complexity};
use output::OutputManager;
use logging::ErrorLogger;

async fn process_files(
    mut rx: mpsc::Receiver<ZipEntry>,
    db_manager: Arc<DatabaseManager>,
    output_manager: Arc<OutputManager>,
    error_logger: Arc<ErrorLogger>,
) -> Result<Vec<ParsedFile>> {
    let batch_size = AtomicUsize::new(100); // Initial batch size
    let mut analyzed_files = Vec::new();

    while let Some(entries) = receive_batch(&mut rx, batch_size.load(Ordering::Relaxed)).await {
        let start_time = std::time::Instant::now();

        let batch_results: Vec<Result<ParsedFile>> = entries.par_iter()
            .map(|entry| {
                let result = analyze_file(entry);
                if let Ok(parsed_file) = &result {
                    if let Err(e) = db_manager.store(entry.name.as_bytes(), &entry.content) {
                        error_logger.log_error(&format!("Failed to store file content: {:?}", e))?;
                    }
                }
                result
            })
            .collect();

        for result in batch_results {
            match result {
                Ok(parsed_file) => analyzed_files.push(parsed_file),
                Err(e) => {
                    let error_msg = format!("Failed to analyze file: {:?}", e);
                    error!("{}", error_msg);
                    error_logger.log_error(&error_msg)?;
                }
            }
        }

        let elapsed = start_time.elapsed();
        adjust_batch_size(&batch_size, elapsed);

        output_manager.write_progress(&format!("Processed {} files", analyzed_files.len()))?;
    }

    Ok(analyzed_files)
}

async fn receive_batch(rx: &mut mpsc::Receiver<ZipEntry>, batch_size: usize) -> Option<Vec<ZipEntry>> {
    let mut batch = Vec::with_capacity(batch_size);
    while let Ok(entry) = rx.try_recv() {
        batch.push(entry);
        if batch.len() >= batch_size {
            break;
        }
    }
    if batch.is_empty() {
        None
    } else {
        Some(batch)
    }
}

fn adjust_batch_size(batch_size: &AtomicUsize, elapsed: std::time::Duration) {
    const TARGET_DURATION: std::time::Duration = std::time::Duration::from_millis(100);
    if elapsed > TARGET_DURATION {
        batch_size.fetch_max(batch_size.load(Ordering::Relaxed) / 2, Ordering::Relaxed);
    } else {
        batch_size.fetch_min(batch_size.load(Ordering::Relaxed) * 2, Ordering::Relaxed);
    }
}
++ b/mvp01/src/main.rs
use serde::{Serialize, Deserialize};
use tree_sitter::{Parser, Language};
use prost::Message;
use flate2::write::ZlibEncoder;
use flate2::Compression;
    pub cyclomatic_complexity: usize,
    pub cognitive_complexity: usize,
        cyclomatic_complexity: 0,
        cognitive_complexity: 0,
    pub cyclomatic_complexity: usize,
    pub cognitive_complexity: usize,
                cyclomatic_complexity: f.cyclomatic_complexity,
                cognitive_complexity: f.cognitive_complexity,
// New imports and structs for Protocol Buffers
mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto_gen.rs"));
}

use proto::{ProjectSummary as ProtoProjectSummary, FileSummary as ProtoFileSummary};

// Function to perform advanced code analysis
fn perform_advanced_code_analysis(entry: &ZipEntry) -> Result<ParsedFile> {
    let mut parser = Parser::new();
    let language = match detect_language(&entry.name) {
        LanguageType::Rust => tree_sitter_rust::language(),
        LanguageType::JavaScript => tree_sitter_javascript::language(),
        LanguageType::Python => tree_sitter_python::language(),
        LanguageType::Java => tree_sitter_java::language(),
        LanguageType::C => tree_sitter_c::language(),
        LanguageType::Cpp => tree_sitter_cpp::language(),
        LanguageType::Go => tree_sitter_go::language(),
        LanguageType::Unknown => return Err(anyhow::anyhow!("Unsupported language")),
    };
    parser.set_language(language).expect("Error loading language");

    let tree = parser.parse(&entry.content, None).expect("Failed to parse");
    let root_node = tree.root_node();

    // Implement metrics extraction here (e.g., cyclomatic complexity, cognitive complexity)
    let (loc, code, comments, blanks) = count_lines(&entry.content);
    let cyclomatic_complexity = calculate_cyclomatic_complexity(&root_node);
    let cognitive_complexity = calculate_cognitive_complexity(&root_node);

    Ok(ParsedFile {
        name: entry.name.clone(),
        language: detect_language(&entry.name),
        loc,
        code,
        comments,
        blanks,
        cyclomatic_complexity,
        cognitive_complexity,
    })
}

fn calculate_cyclomatic_complexity(node: &tree_sitter::Node) -> usize {
    // Implement cyclomatic complexity calculation
    // This is a placeholder implementation
    1 + node.child_count()
}

fn calculate_cognitive_complexity(node: &tree_sitter::Node) -> usize {
    // Implement cognitive complexity calculation
    // This is a placeholder implementation
    node.child_count()
}

// Function to generate LLM-ready output using Protocol Buffers
fn generate_llm_ready_output(
    files: &[ParsedFile],
    output_dir: &Path,
) -> Result<()> {
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let path = output_dir.join(format!("LLM-ready-{}.pb", timestamp));
    let file = std::fs::File::create(path).context("Failed to create LLM-ready output file")?;
    let mut writer = std::io::BufWriter::new(file);

    let proto_files: Vec<ProtoFileSummary> = files.iter().map(|f| ProtoFileSummary {
        name: f.name.clone(),
        language: f.language.to_string(),
        loc: f.loc as u32,
        code: f.code as u32,
        comments: f.comments as u32,
        blanks: f.blanks as u32,
        cyclomatic_complexity: f.cyclomatic_complexity as u32,
        cognitive_complexity: f.cognitive_complexity as u32,
    }).collect();

    let proto_summary = ProtoProjectSummary {
        files: proto_files,
        total_loc: files.iter().map(|f| f.loc).sum::<usize>() as u32,
    };

    let mut encoder = ZlibEncoder::new(writer, Compression::default());
    proto_summary.encode(&mut encoder).context("Failed to encode and compress summary")?;
    encoder.finish().context("Failed to finish compression")?;

    Ok(())
}

// Function to optimize memory usage
fn optimize_memory_usage<R: Read>(reader: R) -> impl Read {
    std::io::BufReader::with_capacity(8192, reader)
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_perform_advanced_code_analysis() {
        let content = b"fn main() { println!(\"Hello, World!\"); }";
        let entry = ZipEntry {
            name: "test.rs".to_string(),
            content: content.to_vec(),
        };
        let result = perform_advanced_code_analysis(&entry);
        assert!(result.is_ok());
        let parsed_file = result.unwrap();
        assert_eq!(parsed_file.language, LanguageType::Rust);
        assert!(parsed_file.loc > 0);
    }

    #[test]
    fn test_generate_llm_ready_output() {
        let temp_dir = tempdir().unwrap();
        let files = vec![
            ParsedFile {
                name: "test.rs".to_string(),
                language: LanguageType::Rust,
                loc: 10,
                code: 8,
                comments: 1,
                blanks: 1,
                cyclomatic_complexity: 1,
                cognitive_complexity: 1,
            },
        ];
        let result = generate_llm_ready_output(&files, temp_dir.path());
        assert!(result.is_ok());
        assert!(temp_dir.path().join("LLM-ready-*.pb").exists());
    }
}

// Main function (updated)
        match perform_advanced_code_analysis(&entry) {
    generate_llm_ready_output(&analyzed_files, &config.output_dir)
        .context("Failed to generate LLM-ready output")?;
    output_manager.write_progress("LLM-ready output generated").context("Failed to write progress")?;
++ b/mvp01/src/main.rs
++ b/mvp01/src/main.rs
        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, summary).context("Failed to write summary to file")?;
++ b/mvp01/src/main.rs
        let path = self.output_dir.join(format!("LLM-ready-{}.txt", timestamp));
        let file = std::fs::File::create(path).context("Failed to create summary file")?;
++ b/mvp01/src/main.rs
        let path = self.output_dir.join(format!("LLM-ready-{}.txt", timestamp)); // Changed extension to .txt
        let file = std::fs::File::create(&path).context("Failed to create summary file")?;
++ b/mvp01/src/main.rs
    _error_logger: Arc<ErrorLogger>,
        let file = std::fs::File::create(path).context("Failed to create summary file")?;
++ b/mvp01/src/main.rs
use std::sync::Arc;
use serde::Serialize;
use flate2::write::GzEncoder;
use flate2::Compression;
use tokio::task;
        std::fs::create_dir_all(path).context("Failed to create database directory")?;
        let db = sled::open(path).context("Failed to open sled database")?;
        self.db.insert(key, value).context("Failed to insert into database")?;
        Ok(self.db.get(key).context("Failed to retrieve from database")?.map(|ivec| ivec.to_vec()))
pub fn process_zip(
        let mut file = zip.by_index(i).context("Failed to get ZIP entry")?;
        
        if file.is_dir() {
            warn!("Skipping directory entry: {}", file.name());
            continue;
        let name = file.name().to_string();
        let mut content = Vec::new();
        file.read_to_end(&mut content).context("Failed to read ZIP entry content")?;

        tx.blocking_send(ZipEntry { name, content }).context("Failed to send ZIP entry")?;
        std::fs::create_dir_all(&config.output_dir).context("Failed to create output directory")?;
        let timestamp = Local::now().format("%Y%m%d%H%M%S");
        let path = self.output_dir.join(format!("LLM-ready-{}.json.gz", timestamp));
        let file = std::fs::File::create(&path).context("Failed to create summary file")?;
        let encoder = GzEncoder::new(file, Compression::default());
        let mut writer = std::io::BufWriter::new(encoder);
        serde_json::to_writer(&mut writer, summary).context("Failed to write summary to file")?;
        writer.flush().context("Failed to flush summary file")?;
        Ok(())
    }

    pub fn write_progress(&self, message: &str) -> Result<()> {
        let path = self.output_dir.join("processProgress.txt");
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .context("Failed to open progress file")?;
        writeln!(file, "[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message)
            .context("Failed to write to progress file")?;
            .open(path)
            .context("Failed to create or open error log file")?;
        let mut file = self.file.lock().map_err(|e| anyhow::anyhow!("Failed to lock error log file: {}", e))?;
        writeln!(file, "[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message)
            .context("Failed to write to error log file")?;
    let error_logger = Arc::new(ErrorLogger::new(&config.output_dir.join("error.log"))
        .context("Failed to create error logger")?);
    let progress_bar = Arc::new(ProgressBar::new(total_files as u64));
        .context("Failed to set progress bar style")?
    output_manager.write_progress("Starting ZIP processing").context("Failed to write progress")?;
    let error_logger_clone = Arc::clone(&error_logger);
    let pb_clone = Arc::clone(&progress_bar);
    task::spawn_blocking(move || {
        if let Err(e) = process_zip(archive, tx, pb_clone, error_logger_clone.clone()) {
            if let Err(log_err) = error_logger_clone.log_error(&format!("Error in ZIP processing task: {:?}", e)) {
                error!("Failed to log error: {:?}", log_err);
            }
                error_logger.log_error(&error_msg).context("Failed to log error")?;
    output_manager.write_progress("File analysis completed").context("Failed to write progress")?;
    output_manager.write_progress("Summary written").context("Failed to write progress")?;
        let temp_dir = tempdir().context("Failed to create temporary directory")?;
++ b/mvp01/src/main.rs
use std::fmt;
/// Configuration for the OSS Code Analyzer and LLM-Ready Summarizer
    /// Path to the input ZIP file
    /// Path to the output directory
/// Manages the embedded database for storing file contents
impl fmt::Display for LanguageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageType::Rust => write!(f, "Rust"),
            LanguageType::JavaScript => write!(f, "JavaScript"),
            LanguageType::Python => write!(f, "Python"),
            LanguageType::Java => write!(f, "Java"),
            LanguageType::C => write!(f, "C"),
            LanguageType::Cpp => write!(f, "C++"),
            LanguageType::Go => write!(f, "Go"),
            LanguageType::Unknown => write!(f, "Unknown"),
        }
    }
}

        let result = (async || -> Result<()> {
        })().await;
    let (loc, code, comments, blanks) = count_lines(&entry.content);
        code,
        comments,
        blanks,
        if line.is_empty() {
        } else if line.starts_with(b"//") || line.starts_with(b"#") {
    let mut language_breakdown: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            *language_breakdown.entry(f.language.to_string()).or_insert(0) += 1;
        let mut file = self.file.lock().map_err(|e| anyhow::anyhow!("Failed to lock file: {}", e))?;
            error_logger_clone.log_error(&format!("Error in ZIP processing task: {:?}", e)).unwrap();
++ b/mvp01/src/main.rs
use log::{error, info, warn};
use std::io::{Read, Write};
use zip::ZipArchive;
use std::fs::File;
use tokio::sync::Arc;
use walkdir::WalkDir;
use std::sync::Mutex;
use chrono::Local;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LanguageType {
    Rust,
    JavaScript,
    Python,
    Java,
    C,
    Cpp,
    Go,
    Unknown,
}

pub async fn process_zip(
    mut zip: ZipArchive<File>,
    tx: mpsc::Sender<ZipEntry>,
    pb: Arc<ProgressBar>,
    error_logger: Arc<ErrorLogger>,
) -> Result<()> {
    for i in 0..zip.len() {
        let result = (|| -> Result<()> {
            let mut file = zip.by_index(i)?;
            
            if file.is_dir() {
                warn!("Skipping directory entry: {}", file.name());
                return Ok(());
            }

            let name = file.name().to_string();
            file.read_to_end(&mut content)?;

            tx.send(ZipEntry { name, content }).await?;
            Ok(())
        })();

        if let Err(e) = result {
            let error_msg = format!("Error processing ZIP entry {}: {:?}", i, e);
            warn!("{}", error_msg);
            error_logger.log_error(&error_msg)?;

        pb.inc(1);
    }
    Ok(())
    pub language: LanguageType,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
fn detect_language(filename: &str) -> LanguageType {
        Some("rs") => LanguageType::Rust,
        Some("js") => LanguageType::JavaScript,
        Some("py") => LanguageType::Python,
        Some("java") => LanguageType::Java,
        Some("c") | Some("h") => LanguageType::C,
        Some("cpp") | Some("hpp") => LanguageType::Cpp,
        Some("go") => LanguageType::Go,
        _ => LanguageType::Unknown,
    }
}

fn count_lines(content: &[u8]) -> (usize, usize, usize, usize) {
    let mut loc = 0;
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    for line in content.split(|&b| b == b'\n') {
        loc += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            blanks += 1;
        } else if trimmed.starts_with(b"//") || trimmed.starts_with(b"#") {
            comments += 1;
        } else {
            code += 1;
        }
    }

    (loc, code, comments, blanks)
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
                language: f.language.to_string(),
                code: f.code,
                comments: f.comments,
                blanks: f.blanks,
pub struct ErrorLogger {
    file: Mutex<std::fs::File>,
}

impl ErrorLogger {
    pub fn new(path: &Path) -> Result<Self> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self {
            file: Mutex::new(file),
        })
    }

    pub fn log_error(&self, message: &str) -> Result<()> {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message)?;
        Ok(())
    }
}

    let error_logger = ErrorLogger::new(&config.output_dir.join("progress.log"))
        .context("Failed to create error logger")?;

    let error_logger_clone = Arc::new(error_logger);
    let pb_clone = Arc::clone(&progress_bar);
    tokio::spawn(async move {
        if let Err(e) = process_zip(archive, tx, pb_clone, Arc::clone(&error_logger_clone)).await {
            error_logger_clone.log_error(&format!("Error in ZIP processing task: {:?}", e))?;
        match analyze_file(entry, &db_manager) {
            Ok(parsed_file) => analyzed_files.push(parsed_file),
            Err(e) => {
                let error_msg = format!("Failed to analyze file: {:?}", e);
                error!("{}", error_msg);
                error_logger.log_error(&error_msg)?;
            }
        }
++ b/mvp01/src/main.rs
use std::path::{Path, PathBuf};
use sled::Db;
use std::io::Write;
        self.db.insert(key, value)?;
pub struct ZipEntry {
    pub name: String,
    pub content: Vec<u8>,
}

pub fn process_zip(path: &Path) -> Result<impl Iterator<Item = Result<ZipEntry>>> {
    let file = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    
    Ok((0..archive.len()).map(move |i| {
        let mut file = archive.by_index(i)
            .context(format!("Failed to read file at index {}", i))?;
        if file.is_file() {
            let mut content = Vec::new();
            std::io::Read::read_to_end(&mut file, &mut content)
                .context(format!("Failed to read content of file {}", file.name()))?;
            Ok(ZipEntry {
                name: file.name().to_string(),
                content,
            })
        } else {
            Err(anyhow::anyhow!("Not a file: {}", file.name()))
        }
    }))
}

pub struct ParsedFile {
    pub name: String,
    pub language: String,
    pub loc: usize,
}

pub fn analyze_file(entry: ZipEntry, db: &DatabaseManager) -> Result<ParsedFile> {
    let language = detect_language(&entry.name);
    let loc = entry.content.iter().filter(|&&c| c == b'\n').count();

    db.store(entry.name.as_bytes(), &entry.content)
        .context("Failed to store file content in database")?;

    Ok(ParsedFile {
        name: entry.name,
        language,
        loc,
    })
}

fn detect_language(filename: &str) -> String {
    match filename.split('.').last() {
        Some("rs") => "Rust",
        Some("js") => "JavaScript",
        Some("py") => "Python",
        Some("java") => "Java",
        Some("c") | Some("h") => "C",
        Some("cpp") | Some("hpp") => "C++",
        Some("go") => "Go",
        _ => "Unknown",
    }.to_string()
}

use serde::Serialize;

#[derive(Serialize)]
pub struct FileSummary {
    pub name: String,
    pub language: String,
    pub loc: usize,
}

#[derive(Serialize)]
pub struct ProjectSummary {
    pub files: Vec<FileSummary>,
    pub total_loc: usize,
    pub language_breakdown: std::collections::HashMap<String, usize>,
}

pub fn generate_summary(files: Vec<ParsedFile>) -> ProjectSummary {
    let mut language_breakdown = std::collections::HashMap::new();
    let total_loc = files.iter().map(|f| f.loc).sum();

    let file_summaries: Vec<FileSummary> = files
        .into_iter()
        .map(|f| {
            *language_breakdown.entry(f.language.clone()).or_insert(0) += 1;
            FileSummary {
                name: f.name,
                language: f.language,
                loc: f.loc,
            }
        })
        .collect();

    ProjectSummary {
        files: file_summaries,
        total_loc,
        language_breakdown,
    }
}

pub struct OutputManager {
    output_dir: PathBuf,
}

impl OutputManager {
    pub fn new(config: &Config) -> Result<Self> {
        std::fs::create_dir_all(&config.output_dir)?;
        Ok(Self {
            output_dir: config.output_dir.clone(),
        })
    }

    pub fn write_summary(&self, summary: &ProjectSummary) -> Result<()> {
        let path = self.output_dir.join("summary.json");
        let file = std::fs::File::create(path)?;
        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, summary)?;
        writer.flush()?;
        Ok(())
    }
}

    let archive = zip::ZipArchive::new(zip_file)
        .expect("Failed to set progress bar style")
                tx.send(entry?).await?;
    let summary = generate_summary(analyzed_files);

use anyhow::{Context, Result};
use log::{error, info, warn};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use prost::Message;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use chrono::Utc;
use colored::*;
use std::io::{BufWriter, Write, Read}; // Added Read trait
use std::fs::{File, create_dir_all, remove_file};
use rayon::prelude::*;
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
    /// Path to the input ZIP file
    #[clap(short, long, value_parser)]
    input_zip: PathBuf,

    /// Path to the output directory
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
}

// Database Management module
struct DatabaseManager {
    db: sled::Db,
    cache: moka::sync::Cache<Vec<u8>, Vec<u8>>,
}

impl DatabaseManager {
    fn new(path: &Path) -> Result<Self> {
        let db = sled::open(path).context("Failed to open sled database")?;
        let cache = moka::sync::Cache::new(10_000);
        Ok(Self { db, cache })
    }

    fn store(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.insert(key, value).context("Failed to insert into database")?;
        self.cache.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

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
    }
}

// Code Analysis module
#[derive(Debug, Serialize, Deserialize, Clone)]
enum LanguageType {
    Rust,
    JavaScript,
    Python,
    Java,
    C,
    Cpp,
    Go,
    Unknown,
}

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
        }
    }
}

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
}

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

    Ok(ParsedFile {
        name: entry.name.clone(),
        language,
        loc,
        code_lines,
        comment_lines,
        blank_lines,
        function_count,
        class_count,
        cyclomatic_complexity,
        cognitive_complexity,
    })
}

fn count_lines(content: &str) -> (usize, usize, usize, usize) {
    let mut loc = 0;
    let mut code_lines = 0;
    let mut comment_lines = 0;
    let mut blank_lines = 0;

    for line in content.lines() {
        loc += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            blank_lines += 1;
        } else if trimmed.starts_with("//") || trimmed.starts_with("#") {
            comment_lines += 1;
        } else {
            code_lines += 1;
        }
    }

    (loc, code_lines, comment_lines, blank_lines)
}

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
}

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
}

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
}

// Output Management module
struct OutputManager {
    output_dir: PathBuf,
}

impl OutputManager {
    fn new(output_dir: PathBuf) -> Result<Self> {
        create_dir_all(&output_dir).context("Failed to create output directory")?;
        Ok(Self { output_dir })
    }

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

        Ok(())
    }

    fn write_progress(&self, message: &str) -> Result<()> {
        let path = self.output_dir.join("processProgress.txt");
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .context("Failed to open progress file")?;
        writeln!(file, "[{}] {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), message)
            .context("Failed to write to progress file")?;
        Ok(())
    }

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
}

// Error Logging module
struct ErrorLogger {
    file: std::sync::Mutex<File>,
}

impl ErrorLogger {
    fn new(path: &Path) -> Result<Self> {
        let file = File::create(path).context("Failed to create error log file")?;
        Ok(Self { file: std::sync::Mutex::new(file) })
    }

    fn log_error(&self, message: &str) -> Result<()> {
        let mut file = self.file.lock().map_err(|e| anyhow::anyhow!("Failed to lock error log file: {}", e))?;
        writeln!(file, "[{}] {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), message)
            .context("Failed to write to error log file")?;
        Ok(())
    }
}

// Avengers-themed Logging
struct AvengersLogger {
    file: std::sync::Mutex<File>,
}

impl AvengersLogger {
    fn new(path: &Path) -> Result<Self> {
        let file = File::create(path).context("Failed to create log file")?;
        Ok(Self { file: std::sync::Mutex::new(file) })
    }
}

impl log::Log for AvengersLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

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
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            file.flush().expect("Failed to flush log file");
        }
    }
}

fn init_logger(path: &Path) -> Result<()> {
    let logger = AvengersLogger::new(path)?;
    log::set_boxed_logger(Box::new(logger)).context("Failed to set logger")?;
    log::set_max_level(log::LevelFilter::Info);
    Ok(())
}

// Main function
#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();
    
    init_logger(&config.output_dir.join("log.txt"))?;
    info!("Starting OSS Code Analyzer and LLM-Ready Summarizer");

    let db_manager = DatabaseManager::new(&config.output_dir.join("db"))?;
    let output_manager = OutputManager::new(config.output_dir.clone())?;
    let error_logger = Arc::new(ErrorLogger::new(&config.output_dir.join("error.log"))?);

    let (tx, mut rx) = mpsc::channel(100);

    let total_files = count_zip_entries(&config.input_zip)?;

    let progress_bar = Arc::new(ProgressBar::new(total_files as u64));
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .context("Failed to set progress bar style")?
        .progress_chars("##-"));

    output_manager.write_progress("Starting ZIP processing")?;

    let error_logger_clone = Arc::clone(&error_logger);
    let pb_clone = Arc::clone(&progress_bar);
    tokio::spawn(async move {
        if let Err(e) = process_zip(&config.input_zip, tx, pb_clone, error_logger_clone).await {
            error!("Error in ZIP processing task: {:?}", e);
            if let Err(log_err) = error_logger_clone.log_error(&format!("Error in ZIP processing task: {:?}", e)) {
                error!("Failed to log error: {:?}", log_err);
            }
        }
    });

    let mut analyzed_files = Vec::new();
    while let Some(entry) = rx.recv().await {
        match analyze_file(&entry) {
            Ok(parsed_file) => {
                db_manager.store(entry.name.as_bytes(), &entry.content)?;
                analyzed_files.push(parsed_file);
            },
            Err(e) => {
                let error_msg = format!("Failed to analyze file {}: {:?}", entry.name, e);
                error!("{}", error_msg);
                error_logger.log_error(&error_msg)?;
            }
        }
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("File analysis completed");
    output_manager.write_progress("File analysis completed")?;

    output_manager.write_llm_ready_output(&analyzed_files)?;
    output_manager.write_progress("LLM-ready output generated")?;

    output_manager.cleanup_old_files(5)?;

    db_manager.close()?;

    info!("Analysis completed successfully");
    Ok(())
}

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
    }
}

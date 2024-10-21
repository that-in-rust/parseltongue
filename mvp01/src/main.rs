use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info, warn};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use indicatif::{ProgressBar, ProgressStyle};
use sled::Db;
use std::io::{Read, Write};
use zip::ZipArchive;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;
use chrono::Local;
use std::fmt;
use serde::Serialize;
use flate2::write::GzEncoder;
use flate2::Compression;
use tokio::task;

/// Configuration for the OSS Code Analyzer and LLM-Ready Summarizer
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Config {
    /// Path to the input ZIP file
    pub input_zip: PathBuf,
    /// Path to the output directory
    pub output_dir: PathBuf,
}

/// Manages the embedded database for storing file contents
pub struct DatabaseManager {
    db: Db,
}

impl DatabaseManager {
    pub fn new(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path).context("Failed to create database directory")?;
        let db = sled::open(path).context("Failed to open sled database")?;
        Ok(Self { db })
    }

    pub fn store(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.insert(key, value).context("Failed to insert into database")?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key).context("Failed to retrieve from database")?.map(|ivec| ivec.to_vec()))
    }
}

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

pub struct ZipEntry {
    pub name: String,
    pub content: Vec<u8>,
}

pub fn process_zip(
    mut zip: ZipArchive<File>,
    tx: mpsc::Sender<ZipEntry>,
    pb: Arc<ProgressBar>,
    _error_logger: Arc<ErrorLogger>,
) -> Result<()> {
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).context("Failed to get ZIP entry")?;
        
        if file.is_dir() {
            warn!("Skipping directory entry: {}", file.name());
            continue;
        }

        let name = file.name().to_string();
        let mut content = Vec::new();
        file.read_to_end(&mut content).context("Failed to read ZIP entry content")?;

        tx.blocking_send(ZipEntry { name, content }).context("Failed to send ZIP entry")?;
        pb.inc(1);
    }
    Ok(())
}

pub struct ParsedFile {
    pub name: String,
    pub language: LanguageType,
    pub loc: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
}

pub fn analyze_file(entry: ZipEntry, db: &DatabaseManager) -> Result<ParsedFile> {
    let language = detect_language(&entry.name);
    let (loc, code, comments, blanks) = count_lines(&entry.content);

    db.store(entry.name.as_bytes(), &entry.content)
        .context("Failed to store file content in database")?;

    Ok(ParsedFile {
        name: entry.name,
        language,
        loc,
        code,
        comments,
        blanks,
    })
}

fn detect_language(filename: &str) -> LanguageType {
    match filename.split('.').last() {
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
        if line.is_empty() {
            blanks += 1;
        } else if line.starts_with(b"//") || line.starts_with(b"#") {
            comments += 1;
        } else {
            code += 1;
        }
    }

    (loc, code, comments, blanks)
}

#[derive(Serialize)]
pub struct FileSummary {
    pub name: String,
    pub language: String,
    pub loc: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
}

#[derive(Serialize)]
pub struct ProjectSummary {
    pub files: Vec<FileSummary>,
    pub total_loc: usize,
    pub language_breakdown: std::collections::HashMap<String, usize>,
}

pub fn generate_summary(files: Vec<ParsedFile>) -> ProjectSummary {
    let mut language_breakdown: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let total_loc = files.iter().map(|f| f.loc).sum();

    let file_summaries: Vec<FileSummary> = files
        .into_iter()
        .map(|f| {
            *language_breakdown.entry(f.language.to_string()).or_insert(0) += 1;
            FileSummary {
                name: f.name,
                language: f.language.to_string(),
                loc: f.loc,
                code: f.code,
                comments: f.comments,
                blanks: f.blanks,
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
        std::fs::create_dir_all(&config.output_dir).context("Failed to create output directory")?;
        Ok(Self {
            output_dir: config.output_dir.clone(),
        })
    }

    pub fn write_summary(&self, summary: &ProjectSummary) -> Result<()> {
        let timestamp = Local::now().format("%Y%m%d%H%M%S");
        let path = self.output_dir.join(format!("LLM-ready-{}.txt", timestamp));
        let file = std::fs::File::create(path).context("Failed to create summary file")?;
        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, summary).context("Failed to write summary to file")?;
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
        Ok(())
    }
}

pub struct ErrorLogger {
    file: Mutex<std::fs::File>,
}

impl ErrorLogger {
    pub fn new(path: &Path) -> Result<Self> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .context("Failed to create or open error log file")?;
        Ok(Self {
            file: Mutex::new(file),
        })
    }

    pub fn log_error(&self, message: &str) -> Result<()> {
        let mut file = self.file.lock().map_err(|e| anyhow::anyhow!("Failed to lock error log file: {}", e))?;
        writeln!(file, "[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message)
            .context("Failed to write to error log file")?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();
    
    env_logger::init();
    info!("Starting OSS Code Analyzer and LLM-Ready Summarizer");

    let db_manager = DatabaseManager::new(&config.output_dir.join("db"))
        .context("Failed to initialize database")?;

    let output_manager = OutputManager::new(&config)
        .context("Failed to initialize output manager")?;

    let error_logger = Arc::new(ErrorLogger::new(&config.output_dir.join("error.log"))
        .context("Failed to create error logger")?);

    let (tx, mut rx) = mpsc::channel(100);

    let zip_file = std::fs::File::open(&config.input_zip)
        .context("Failed to open ZIP file")?;
    let archive = zip::ZipArchive::new(zip_file)
        .context("Failed to create ZIP archive")?;
    let total_files = archive.len();

    let progress_bar = Arc::new(ProgressBar::new(total_files as u64));
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .context("Failed to set progress bar style")?
        .progress_chars("##-"));

    output_manager.write_progress("Starting ZIP processing").context("Failed to write progress")?;

    let error_logger_clone = Arc::clone(&error_logger);
    let pb_clone = Arc::clone(&progress_bar);
    task::spawn_blocking(move || {
        if let Err(e) = process_zip(archive, tx, pb_clone, error_logger_clone.clone()) {
            error!("Error in ZIP processing task: {:?}", e);
            if let Err(log_err) = error_logger_clone.log_error(&format!("Error in ZIP processing task: {:?}", e)) {
                error!("Failed to log error: {:?}", log_err);
            }
        }
    });

    let mut analyzed_files = Vec::new();
    while let Some(entry) = rx.recv().await {
        match analyze_file(entry, &db_manager) {
            Ok(parsed_file) => analyzed_files.push(parsed_file),
            Err(e) => {
                let error_msg = format!("Failed to analyze file: {:?}", e);
                error!("{}", error_msg);
                error_logger.log_error(&error_msg).context("Failed to log error")?;
            }
        }
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("File analysis completed");
    output_manager.write_progress("File analysis completed").context("Failed to write progress")?;

    let summary = generate_summary(analyzed_files);

    output_manager.write_summary(&summary)
        .context("Failed to write summary")?;
    output_manager.write_progress("Summary written").context("Failed to write progress")?;

    info!("Analysis completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_parsing() {
        let args = vec!["program", "input.zip", "output_dir"];
        let config = Config::parse_from(args);
        assert_eq!(config.input_zip, PathBuf::from("input.zip"));
        assert_eq!(config.output_dir, PathBuf::from("output_dir"));
    }

    #[test]
    fn test_database_operations() -> Result<()> {
        let temp_dir = tempdir().context("Failed to create temporary directory")?;
        let db_manager = DatabaseManager::new(temp_dir.path())?;

        let key = b"test_key";
        let value = b"test_value";

        db_manager.store(key, value)?;
        let retrieved_value = db_manager.get(key)?;

        assert_eq!(retrieved_value, Some(value.to_vec()));
        Ok(())
    }

    // Add more tests as needed...
}

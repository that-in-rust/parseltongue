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
use tokio::sync::Arc;
use walkdir::WalkDir;
use std::sync::Mutex;
use chrono::Local;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Config {
    pub input_zip: PathBuf,
    pub output_dir: PathBuf,
}

pub struct DatabaseManager {
    db: Db,
}

impl DatabaseManager {
    pub fn new(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path)?;
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn store(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.insert(key, value)?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key)?.map(|ivec| ivec.to_vec()))
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

pub struct ZipEntry {
    pub name: String,
    pub content: Vec<u8>,
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
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;

            tx.send(ZipEntry { name, content }).await?;
            Ok(())
        })();

        if let Err(e) = result {
            let error_msg = format!("Error processing ZIP entry {}: {:?}", i, e);
            warn!("{}", error_msg);
            error_logger.log_error(&error_msg)?;
        }

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
    let loc = entry.content.iter().filter(|&&c| c == b'\n').count();

    db.store(entry.name.as_bytes(), &entry.content)
        .context("Failed to store file content in database")?;

    Ok(ParsedFile {
        name: entry.name,
        language,
        loc,
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
}

use serde::Serialize;

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
    let mut language_breakdown = std::collections::HashMap::new();
    let total_loc = files.iter().map(|f| f.loc).sum();

    let file_summaries: Vec<FileSummary> = files
        .into_iter()
        .map(|f| {
            *language_breakdown.entry(f.language.clone()).or_insert(0) += 1;
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

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();
    
    env_logger::init();
    info!("Starting OSS Code Analyzer and LLM-Ready Summarizer");

    let db_manager = DatabaseManager::new(&config.output_dir.join("db"))
        .context("Failed to initialize database")?;

    let output_manager = OutputManager::new(&config)
        .context("Failed to initialize output manager")?;

    let error_logger = ErrorLogger::new(&config.output_dir.join("progress.log"))
        .context("Failed to create error logger")?;

    let (tx, mut rx) = mpsc::channel(100);

    let zip_file = std::fs::File::open(&config.input_zip)
        .context("Failed to open ZIP file")?;
    let archive = zip::ZipArchive::new(zip_file)
        .context("Failed to create ZIP archive")?;
    let total_files = archive.len();

    let progress_bar = ProgressBar::new(total_files as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .expect("Failed to set progress bar style")
        .progress_chars("##-"));

    let error_logger_clone = Arc::new(error_logger);
    let pb_clone = Arc::clone(&progress_bar);

    tokio::spawn(async move {
        if let Err(e) = process_zip(archive, tx, pb_clone, Arc::clone(&error_logger_clone)).await {
            error!("Error in ZIP processing task: {:?}", e);
            error_logger_clone.log_error(&format!("Error in ZIP processing task: {:?}", e))?;
        }
    });

    let mut analyzed_files = Vec::new();
    while let Some(entry) = rx.recv().await {
        match analyze_file(entry, &db_manager) {
            Ok(parsed_file) => analyzed_files.push(parsed_file),
            Err(e) => {
                let error_msg = format!("Failed to analyze file: {:?}", e);
                error!("{}", error_msg);
                error_logger.log_error(&error_msg)?;
            }
        }
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("File analysis completed");

    let summary = generate_summary(analyzed_files);

    output_manager.write_summary(&summary)
        .context("Failed to write summary")?;

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
        let temp_dir = tempdir()?;
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

use std::fs::OpenOptions;
use std::io::{Write, BufReader, BufWriter};
use sled::Db;
use log::{info, error, debug, warn};
use anyhow::{Context, Result, bail};
use clap::Parser;
use tokio::sync::mpsc;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::time::timeout;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

mod logger {
    use std::fs::OpenOptions;
    use std::io::{Write, BufWriter};
    use std::time::{SystemTime, UNIX_EPOCH};
    use log::{info, error, debug, warn};
    use anyhow::{Result, Context};
    use std::path::{Path, PathBuf};
    use chrono::Local;

    pub struct Logger {
        log_file: PathBuf,
    }

    impl Logger {
        pub fn new(zip_filename: &str) -> Result<Self> {
            let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
            let log_filename = format!("Log{}_{}.txt", zip_filename, timestamp);
            let log_file = PathBuf::from(&log_filename);
            Ok(Self { log_file })
        }

        pub fn log(&self, message: &str) -> Result<()> {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("Failed to get system time")?
                .as_secs();
            
            let log_message = format!("[{}] {}", timestamp, message);
            
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&self.log_file)
                .context("Failed to open log file")?;

            let mut writer = BufWriter::new(file);
            writeln!(writer, "{}", log_message).context("Failed to write to log file")?;
            writer.flush().context("Failed to flush log file")?;
            debug!("{}", log_message);
            Ok(())
        }
    }
}

mod database {
    use anyhow::{Result, Context};
    use moka::sync::Cache;
    use sled::Db;
    use std::path::Path;
    use std::sync::Arc;
    use log::{debug, error};

    pub struct DatabaseManager {
        db: Db,
        cache: Cache<Vec<u8>, Vec<u8>>,
    }

    impl DatabaseManager {
        pub fn new(path: &Path) -> Result<Self> {
            let db = sled::open(path).context("Failed to open database")?;
            let cache = Cache::new(10_000); // Cache size of 10,000 items
            Ok(Self { db, cache })
        }

        pub fn store(&self, key: &[u8], value: &[u8]) -> Result<()> {
            self.db.insert(key, value).context("Failed to insert into database")?;
            self.cache.insert(key.to_vec(), value.to_vec());
            debug!("Stored key: {:?}", String::from_utf8_lossy(key));
            Ok(())
        }

        pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
            if let Some(value) = self.cache.get(key) {
                debug!("Cache hit for key: {:?}", String::from_utf8_lossy(key));
                return Ok(Some(value));
            }
            match self.db.get(key).context("Failed to retrieve from database")? {
                Some(ivec) => {
                    let value = ivec.to_vec();
                    self.cache.insert(key.to_vec(), value.clone());
                    debug!("Database hit for key: {:?}", String::from_utf8_lossy(key));
                    Ok(Some(value))
                }
                None => {
                    debug!("Key not found: {:?}", String::from_utf8_lossy(key));
                    Ok(None)
                }
            }
        }

        pub fn delete(&self, key: &[u8]) -> Result<()> {
            self.db.remove(key).context("Failed to delete from database")?;
            self.cache.remove(key);
            debug!("Deleted key: {:?}", String::from_utf8_lossy(key));
            Ok(())
        }

        pub fn iter(&self) -> impl Iterator<Item = Result<(Vec<u8>, Vec<u8>)>> + '_ {
            self.db.iter().map(|result| {
                result
                    .context("Failed to iterate over database")
                    .map(|(key, value)| (key.to_vec(), value.to_vec()))
            })
        }

        pub fn flush(&self) -> Result<()> {
            self.db.flush().context("Failed to flush database")?;
            debug!("Database flushed");
            Ok(())
        }

        pub fn close(self) -> Result<()> {
            self.flush()?;
            self.db.close().context("Failed to close database")?;
            debug!("Database closed");
            Ok(())
        }

        pub fn get_with_timeout(&self, key: &[u8], timeout: Duration) -> Result<Option<Vec<u8>>> {
            tokio::time::timeout(timeout, async {
                self.get(key)
            }).await.context("Database operation timed out")?
        }

        pub fn atomic_operation<F, T>(&self, f: F) -> Result<T>
        where
            F: FnOnce(&sled::Tree) -> Result<T>,
        {
            let tree = self.db.open_tree("default")?;
            tree.transaction(|tx_db| f(tx_db).map_err(sled::transaction::ConflictableTransactionError::Abort))
                .map_err(|e| anyhow::anyhow!("Transaction failed: {:?}", e))
        }
    }
}

mod cli {
    use anyhow::Result;
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    pub struct Config {
        #[clap(short, long)]
        pub input: String,
        #[clap(short, long)]
        pub output: String,
        #[clap(short, long, default_value = "info")]
        pub verbosity: String,
    }

    pub fn parse_config() -> Result<Config> {
        let config = Config::parse();
        if config.input.is_empty() || config.output.is_empty() {
            bail!("Input and output paths must be specified");
        }
        Ok(config)
    }
}

mod zip_processing {
    use anyhow::Result;
    use std::path::PathBuf;
    use tokio::sync::mpsc;
    use zip::ZipArchive;
    use std::fs::File;
    use std::io::Read;

    pub struct ZipEntry {
        pub name: String,
        pub content: Vec<u8>,
    }

    pub async fn process_zip(zip_path: PathBuf) -> Result<mpsc::Receiver<Result<ZipEntry>>> {
        let (tx, rx) = mpsc::channel(100);

        tokio::task::spawn(async move {
            match tokio::task::spawn_blocking(move || -> Result<()> {
                let file = File::open(&zip_path).context("Failed to open ZIP file")?;
                let mut archive = ZipArchive::new(file).context("Failed to create ZIP archive")?;

                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).with_context(|| format!("Failed to get ZIP entry at index {}", i))?;
                    let mut content = Vec::new();
                    file.read_to_end(&mut content).with_context(|| format!("Failed to read content of ZIP entry '{}'", file.name()))?;

                    let entry = ZipEntry {
                        name: file.name().to_string(),
                        content,
                    };

                    tx.blocking_send(Ok(entry)).with_context(|| format!("Failed to send ZIP entry '{}'", file.name()))?;
                }

                Ok(())
            }).await {
                Ok(result) => result.context("ZIP processing task failed"),
                Err(e) => Err(anyhow::anyhow!("ZIP processing task panicked: {}", e)),
            }
        });

        Ok(rx)
    }
}

mod code_analysis {
    use anyhow::{Result, Context};
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ParsedFile {
        pub name: String,
        pub language: LanguageType,
        pub loc: usize,
        pub complexity: usize,
        pub linter_errors: usize,
        pub linter_warnings: usize,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum LanguageType {
        Rust,
        Python,
        JavaScript,
        Unknown,
    }

    pub fn analyze_file(name: &str, content: &[u8]) -> Result<ParsedFile> {
        let language = detect_language(name, content);
        let loc = count_lines(content).context("Failed to count lines")?;
        let complexity = calculate_complexity(content).context("Failed to calculate complexity")?;
        let (linter_errors, linter_warnings) = run_linter(&language, content).context("Failed to run linter")?;

        if complexity > 100 {
            warn!("High complexity detected in file: {}", name);
        }

        Ok(ParsedFile {
            name: name.to_string(),
            language,
            loc,
            complexity,
            linter_errors,
            linter_warnings,
        })
    }

    fn detect_language(filename: &str, content: &[u8]) -> LanguageType {
        if let Some(lang) = detect_from_extension(filename) {
            return lang;
        }
        detect_from_content(content)
    }

    fn detect_from_extension(filename: &str) -> Option<LanguageType> {
        match filename.split('.').last()? {
            "rs" => Some(LanguageType::Rust),
            "py" => Some(LanguageType::Python),
            "js" => Some(LanguageType::JavaScript),
            _ => None,
        }
    }

    fn detect_from_content(content: &[u8]) -> LanguageType {
        // Implement content-based detection here
        // This is a placeholder implementation
        if content.starts_with(b"fn ") || content.starts_with(b"use ") {
            LanguageType::Rust
        } else if content.starts_with(b"def ") || content.starts_with(b"import ") {
            LanguageType::Python
        } else if content.starts_with(b"function ") || content.starts_with(b"var ") {
            LanguageType::JavaScript
        } else {
            LanguageType::Unknown
        }
    }

    fn count_lines(content: &[u8]) -> Result<usize> {
        Ok(std::str::from_utf8(content)
            .context("Failed to convert content to UTF-8")?
            .lines()
            .count())
    }

    fn calculate_complexity(content: &[u8]) -> Result<usize> {
        // This is a placeholder. In a real implementation, you'd use a proper
        // complexity calculation algorithm.
        Ok(content.len() / 100)
    }

    fn run_linter(language: &LanguageType, content: &[u8]) -> Result<(usize, usize)> {
        // This is a placeholder. In a real implementation, you'd run an actual linter.
        Ok((0, content.len() / 1000))
    }
}

mod summary {
    use super::code_analysis::ParsedFile;
    use anyhow::{Result, Context};
    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProjectSummary {
        pub total_files: usize,
        pub total_loc: usize,
        pub language_breakdown: HashMap<String, usize>,
        pub average_complexity: f64,
        pub total_linter_errors: usize,
        pub total_linter_warnings: usize,
    }

    pub fn generate_summary(files: Vec<ParsedFile>) -> Result<ProjectSummary> {
        let total_files = files.len();
        let total_loc: usize = files.iter().map(|f| f.loc).sum();
        let mut language_breakdown = HashMap::new();
        let total_complexity: usize = files.iter().map(|f| f.complexity).sum();
        let total_linter_errors: usize = files.iter().map(|f| f.linter_errors).sum();
        let total_linter_warnings: usize = files.iter().map(|f| f.linter_warnings).sum();

        for file in &files {
            *language_breakdown.entry(format!("{:?}", file.language)).or_insert(0) += 1;
        }

        let average_complexity = if total_files > 0 {
            total_complexity as f64 / total_files as f64
        } else {
            0.0
        };

        Ok(ProjectSummary {
            total_files,
            total_loc,
            language_breakdown,
            average_complexity,
            total_linter_errors,
            total_linter_warnings,
        })
    }
}

mod output {
    use super::summary::ProjectSummary;
    use anyhow::{Result, Context};
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    pub fn write_summary(summary: &ProjectSummary, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(summary).context("Failed to serialize summary")?;
        let file = File::create(output_path).context("Failed to create output file")?;
        let mut writer = BufWriter::new(file);
        writer.write_all(json.as_bytes()).context("Failed to write summary to file")?;
        writer.flush().context("Failed to flush output file")?;
        Ok(())
    }

    pub async fn write_summary_async(summary: &ProjectSummary, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(summary).context("Failed to serialize summary")?;
        let mut file = File::create(output_path).await.context("Failed to create output file")?;
        file.write_all(json.as_bytes()).await.context("Failed to write summary to file")?;
        file.flush().await.context("Failed to flush output file")?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let config = cli::parse_config().context("Failed to parse config")?;
    info!("Config: {:?}", config);

    let zip_filename = Path::new(&config.input)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    let logger = logger::Logger::new(zip_filename).context("Failed to create logger")?;
    
    logger.log("Starting the application").context("Failed to log start message")?;

    let db_path = Path::new("huffelpuff_db");
    let db_manager = Arc::new(database::DatabaseManager::new(db_path).context("Failed to create DatabaseManager")?);

    let zip_path = PathBuf::from(&config.input);
    let mut receiver = zip_processing::process_zip(zip_path).await.context("Failed to process ZIP file")?;

    let mut parsed_files = Vec::new();

    let progress_bar = ProgressBar::new(0);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .expect("Failed to set progress bar style"));

    while let Ok(Some(entry_result)) = timeout(Duration::from_secs(10), receiver.recv()).await {
        let entry = entry_result.context("Failed to receive ZIP entry")?;
        info!("Processing file: {}", entry.name);
        
        let analysis_result = code_analysis::analyze_file(&entry.name, &entry.content)
            .context("Failed to analyze file")?;
        
        let analysis_key = format!("analysis:{}", entry.name).into_bytes();
        let analysis_value = serde_json::to_vec(&analysis_result).context("Failed to serialize analysis result")?;
        db_manager.store(&analysis_key, &analysis_value)
            .context("Failed to store analysis result")?;

        db_manager.store(entry.name.as_bytes(), &entry.content)
            .context("Failed to store file content")?;

        parsed_files.push(analysis_result);
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Processing complete");

    let project_summary = summary::generate_summary(parsed_files).context("Failed to generate project summary")?;
    
    let output_path = Path::new(&config.output);
    output::write_summary(&project_summary, output_path).context("Failed to write summary")?;

    info!("Summary written to: {:?}", output_path);

    match Arc::try_unwrap(db_manager) {
        Ok(manager) => manager.close().context("Failed to close database")?,
        Err(_) => error!("Failed to unwrap Arc, database may not be properly closed"),
    }

    logger.log("Application finished").context("Failed to log finish message")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ... (keep existing tests)

    #[test]
    fn test_database_operations() -> Result<()> {
        let temp_dir = TempDir::new().context("Failed to create temp dir")?;
        let db_path = temp_dir.path().join("test_db");
        let db_manager = database::DatabaseManager::new(&db_path).context("Failed to create DatabaseManager")?;

        let key = b"test_key";
        let value = b"test_value";

        db_manager.store(key, value).context("Failed to store value")?;
        let retrieved = db_manager.get(key).context("Failed to get value")?;
        assert_eq!(retrieved, Some(value.to_vec()));

        db_manager.delete(key).context("Failed to delete value")?;
        let deleted = db_manager.get(key).context("Failed to check deleted value")?;
        assert_eq!(deleted, None);

        Ok(())
    }
}

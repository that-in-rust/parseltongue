use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info};
use std::path::PathBuf;
use tokio::sync::mpsc;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

mod cli;
mod database;
mod zip_processing;
mod code_analysis;
mod summary;
mod output;

use cli::Config;
use database::DatabaseManager;
use zip_processing::{process_zip, ZipEntry};
use code_analysis::{analyze_file, ParsedFile};
use summary::{generate_summary, FileSummary, ProjectSummary};
use output::OutputManager;

/// Configuration for the OSS Code Analyzer
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Config {
    /// Path to the input ZIP file
    pub input_zip: PathBuf,
    /// Directory for output files
    pub output_dir: PathBuf,
}

/// Manages database operations
pub struct DatabaseManager {
    db: Db,
}

impl DatabaseManager {
    /// Creates a new DatabaseManager
    pub fn new(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path)?;
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn store(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.transaction(|tx| {
            tx.insert(key, value)?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key)?.map(|ivec| ivec.to_vec()))
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

    let (tx, mut rx) = mpsc::channel(100);

    let zip_file = std::fs::File::open(&config.input_zip)
        .context("Failed to open ZIP file")?;
    let mut archive = zip::ZipArchive::new(zip_file)
        .context("Failed to create ZIP archive")?;
    let total_files = archive.len();

    let progress_bar = ProgressBar::new(total_files as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-"));

    tokio::spawn(async move {
        if let Err(e) = async {
            let zip_stream = process_zip(&config.input_zip)
                .context("Failed to process ZIP file")?;

            for entry in zip_stream {
                tx.send(entry).await?;
            }
            Ok::<_, anyhow::Error>(())
        }.await {
            error!("Error in ZIP processing task: {:?}", e);
        }
    });

    let mut analyzed_files = Vec::new();
    while let Some(entry) = rx.recv().await {
        let parsed_file = analyze_file(entry, &db_manager)
            .context("Failed to analyze file")?;
        analyzed_files.push(parsed_file);
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("File analysis completed");

    let summary = generate_summary(analyzed_files)
        .context("Failed to generate summary")?;

    output_manager.write_summary(&summary)
        .context("Failed to write summary")?;

    info!("Analysis completed successfully");
    Ok(())
}

// Implement the modules

mod cli {
    use clap::Parser;
    use std::path::PathBuf;

    #[derive(Parser, Debug)]
    #[clap(author, version, about)]
    pub struct Config {
        pub input_zip: PathBuf,
        pub output_dir: PathBuf,
    }
}

mod database {
    use anyhow::Result;
    use sled::Db;
    use std::path::Path;

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
            self.db.transaction(|tx| {
                tx.insert(key, value)?;
                Ok(())
            })?;
            Ok(())
        }

        pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
            Ok(self.db.get(key)?.map(|ivec| ivec.to_vec()))
        }
    }
}

mod zip_processing {
    use anyhow::{Result, Context};
    use std::path::Path;
    use zip::ZipArchive;
    use std::fs::File;

    pub struct ZipEntry {
        pub name: String,
        pub content: Vec<u8>,
    }

    pub fn process_zip(path: &Path) -> Result<impl Iterator<Item = Result<ZipEntry>>> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        
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
}

mod code_analysis {
    use crate::database::DatabaseManager;
    use crate::zip_processing::ZipEntry;
    use anyhow::{Result, Context};

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
}

mod summary {
    use crate::code_analysis::ParsedFile;
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
}

mod output {
    use crate::cli::Config;
    use crate::summary::ProjectSummary;
    use anyhow::Result;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::io::BufWriter;

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
            let file = File::create(path)?;
            let mut writer = BufWriter::new(file);
            let json = serde_json::to_string_pretty(summary)?;
            writer.write_all(json.as_bytes())?;
            writer.flush()?;
            Ok(())
        }
    }
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

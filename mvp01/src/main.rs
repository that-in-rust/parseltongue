use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use indicatif::{ProgressBar, ProgressStyle};
use sled::Db;
use std::io::Write;

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
    let archive = zip::ZipArchive::new(zip_file)
        .context("Failed to create ZIP archive")?;
    let total_files = archive.len();

    let progress_bar = ProgressBar::new(total_files as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .expect("Failed to set progress bar style")
        .progress_chars("##-"));

    tokio::spawn(async move {
        if let Err(e) = async {
            let zip_stream = process_zip(&config.input_zip)
                .context("Failed to process ZIP file")?;

            for entry in zip_stream {
                tx.send(entry?).await?;
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

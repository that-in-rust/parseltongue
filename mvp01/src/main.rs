use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser as ClapParser;
use flate2::write::GzEncoder;
use flate2::Compression;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, warn};
use moka::sync::Cache;
use prost::Message;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;
use tree_sitter::{Language, Parser};
use zip::ZipArchive;

// Import tree-sitter language functions
use tree_sitter_rust::language as rust_language;
use tree_sitter_javascript::language as javascript_language;
use tree_sitter_python::language as python_language;
use tree_sitter_java::language as java_language;
use tree_sitter_c::language as c_language;
use tree_sitter_cpp::language as cpp_language;
use tree_sitter_go::language as go_language;

// Include generated Protocol Buffers code
pub mod proto {
    tonic::include_proto!("summary");
}
use proto::{FileSummary, ProjectSummary};

#[derive(ClapParser, Debug)]
#[clap(version = "1.0", author = "Your Name")]
struct Config {
    #[clap(short, long)]
    input_zip: PathBuf,
    #[clap(short, long)]
    output_dir: PathBuf,
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
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

impl std::fmt::Display for LanguageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

struct DatabaseManager {
    db: Db,
    cache: Cache<Vec<u8>, Vec<u8>>,
}

impl DatabaseManager {
    fn new(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path).context("Failed to create database directory")?;
        let db = sled::open(path).context("Failed to open sled database")?;
        let cache = Cache::new(10_000);
        Ok(Self { db, cache })
    }

    fn store(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.cache.insert(key.to_vec(), value.to_vec());
        self.db.insert(key, value).context("Failed to insert into database")?;
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

struct ZipEntry {
    name: String,
    content: Vec<u8>,
}

struct OutputManager {
    output_dir: PathBuf,
}

impl OutputManager {
    fn new(output_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;
        Ok(Self { output_dir })
    }

    fn write_llm_ready_output(&self, files: &[ParsedFile]) -> Result<()> {
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let output_path = self.output_dir.join(format!("LLM-ready-{}.bin", timestamp));

        let file = File::create(&output_path).context("Failed to create output file")?;
        let mut writer = BufWriter::new(GzEncoder::new(file, Compression::default()));

        let project_summary = ProjectSummary {
            files: files.iter().map(|f| FileSummary {
                name: f.name.clone(),
                language: f.language.to_string(),
                loc: f.loc as u32,
                code_lines: f.code_lines as u32,
                comment_lines: f.comment_lines as u32,
                blank_lines: f.blank_lines as u32,
                function_count: f.function_count as u32,
                class_count: f.class_count as u32,
                cyclomatic_complexity: f.cyclomatic_complexity as u32,
                cognitive_complexity: f.cognitive_complexity as u32,
                ..Default::default() // This will set default values for any fields we haven't explicitly set
            }).collect(),
            total_loc: files.iter().map(|f| f.loc).sum::<usize>() as u32,
            language_breakdown: files.iter().fold(std::collections::HashMap::new(), |mut acc, f| {
                *acc.entry(f.language.to_string()).or_insert(0) += 1;
                acc
            }),
            ..Default::default() // This will set default values for any fields we haven't explicitly set
        };

        let encoded = project_summary.encode_to_vec();
        writer.write_all(&encoded).context("Failed to write encoded project summary")?;
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
        writeln!(file, "[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message)
            .context("Failed to write to progress file")?;
        Ok(())
    }
}

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
        writeln!(file, "[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message)
            .context("Failed to write to error log file")?;
        Ok(())
    }
}

fn detect_language(filename: &str) -> LanguageType {
    match Path::new(filename).extension().and_then(std::ffi::OsStr::to_str) {
        Some("rs") => LanguageType::Rust,
        Some("js") => LanguageType::JavaScript,
        Some("py") => LanguageType::Python,
        Some("java") => LanguageType::Java,
        Some("c") | Some("h") => LanguageType::C,
        Some("cpp") | Some("hpp") | Some("cxx") | Some("cc") => LanguageType::Cpp,
        Some("go") => LanguageType::Go,
        _ => LanguageType::Unknown,
    }
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

fn analyze_file(entry: &ZipEntry) -> Result<ParsedFile> {
    let language = detect_language(&entry.name);
    let content = String::from_utf8_lossy(&entry.content);
    let (loc, code_lines, comment_lines, blank_lines) = count_lines(&content);

    let mut parser = Parser::new();
    let ts_language = match language {
        LanguageType::Rust => rust_language(),
        LanguageType::JavaScript => javascript_language(),
        LanguageType::Python => python_language(),
        LanguageType::Java => java_language(),
        LanguageType::C => c_language(),
        LanguageType::Cpp => cpp_language(),
        LanguageType::Go => go_language(),
        LanguageType::Unknown => return Err(anyhow::anyhow!("Unsupported language")),
    };
    parser.set_language(ts_language).context("Failed to set language for parser")?;

    let tree = parser.parse(&content, None).context("Failed to parse content")?;
    let ast_depth = calculate_ast_depth(tree.root_node());
    let ast_node_count = tree.root_node().child_count();

    Ok(ParsedFile {
        name: entry.name.clone(),
        language,
        loc,
        code_lines,
        comment_lines,
        blank_lines,
        function_count: count_functions(&tree),
        class_count: count_classes(&tree),
        cyclomatic_complexity: calculate_cyclomatic_complexity(&tree),
        cognitive_complexity: calculate_cognitive_complexity(&tree),
    })
}

fn calculate_ast_depth(node: tree_sitter::Node) -> usize {
    if node.child_count() == 0 {
        1
    } else {
        node.children(&mut node.walk()).map(calculate_ast_depth).max().unwrap_or(0) + 1
    }
}

fn count_functions(tree: &tree_sitter::Tree) -> usize {
    // This is a simplified implementation. In a real-world scenario, you'd use more sophisticated queries.
    tree.root_node().descendant_count()
}

fn count_classes(tree: &tree_sitter::Tree) -> usize {
    // This is a simplified implementation. In a real-world scenario, you'd use more sophisticated queries.
    tree.root_node().descendant_count()
}

fn calculate_cyclomatic_complexity(tree: &tree_sitter::Tree) -> usize {
    // This is a simplified implementation. In a real-world scenario, you'd use more sophisticated analysis.
    tree.root_node().descendant_count()
}

fn calculate_cognitive_complexity(tree: &tree_sitter::Tree) -> usize {
    // This is a simplified implementation. In a real-world scenario, you'd use more sophisticated analysis.
    tree.root_node().descendant_count()
}

async fn process_zip(
    path: PathBuf,
    tx: mpsc::Sender<ZipEntry>,
    pb: Arc<ProgressBar>,
    error_logger: Arc<ErrorLogger>,
) -> Result<()> {
    let file = File::open(&path).context("Failed to open ZIP file")?;
    let mut archive = ZipArchive::new(file).context("Failed to create ZIP archive")?;

    for i in 0..archive.len() {
        let result = task::spawn_blocking(move || -> Result<ZipEntry> {
            let mut file = archive.by_index(i).context("Failed to get ZIP entry")?;
            
            if file.is_dir() {
                return Err(anyhow::anyhow!("Skipping directory entry: {}", file.name()));
            }

            let name = file.name().to_string();
            let mut content = Vec::new();
            file.read_to_end(&mut content).context("Failed to read ZIP entry content")?;

            Ok(ZipEntry { name, content })
        }).await.context("Failed to spawn blocking task")?;

        match result {
            Ok(entry) => tx.send(entry).await.context("Failed to send ZIP entry")?,
            Err(e) => {
                let error_msg = format!("Error processing ZIP entry {}: {:?}", i, e);
                warn!("{}", error_msg);
                error_logger.log_error(&error_msg).context("Failed to log error")?;
            }
        }

        pb.inc(1);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();

    // Initialize logger
    env_logger::init();

    let db_manager = DatabaseManager::new(&config.output_dir.join("db"))
        .context("Failed to create database manager")?;
    let output_manager = OutputManager::new(config.output_dir.clone())
        .context("Failed to create output manager")?;
    let error_logger = Arc::new(ErrorLogger::new(&config.output_dir.join("error.log"))
        .context("Failed to create error logger")?);

    let zip_file = File::open(&config.input_zip).context("Failed to open ZIP file")?;
    let mut archive = ZipArchive::new(zip_file).context("Failed to create ZIP archive")?;
    let total_files = archive.len();

    let progress_bar = Arc::new(ProgressBar::new(total_files as u64));
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-")
    );

    output_manager.write_progress("Starting ZIP processing").context("Failed to write progress")?;

    let (tx, mut rx) = mpsc::channel(100);
    let error_logger_clone = Arc::clone(&error_logger);
    let pb_clone = Arc::clone(&progress_bar);

    tokio::spawn(async move {
        if let Err(e) = process_zip(config.input_zip, tx, pb_clone, error_logger_clone).await {
            error!("Error in ZIP processing task: {:?}", e);
        }
    });

    let mut analyzed_files = Vec::new();

    while let Some(entry) = rx.recv().await {
        match analyze_file(&entry) {
            Ok(parsed_file) => {
                db_manager.store(entry.name.as_bytes(), &entry.content)
                    .context("Failed to store file in database")?;
                analyzed_files.push(parsed_file);
            },
            Err(e) => {
                let error_msg = format!("Failed to analyze file {}: {:?}", entry.name, e);
                error!("{}", error_msg);
                error_logger.log_error(&error_msg).context("Failed to log error")?;
            }
        }
    }

    progress_bar.finish_with_message("ZIP processing completed");
    output_manager.write_progress("File analysis completed").context("Failed to write progress")?;

    output_manager.write_llm_ready_output(&analyzed_files).context("Failed to write LLM-ready output")?;
    output_manager.write_progress("LLM-ready output generated").context("Failed to write progress")?;

    db_manager.close().context("Failed to close database")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_main_workflow() -> Result<()> {
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
                if name.starts_with("LLM-ready-") && name.ends_with(".bin") {
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


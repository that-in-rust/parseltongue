use log::info;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json;

mod logger {
    use std::fs::OpenOptions;
    use std::io::Write;
    use log::{debug, error, warn, info, trace};
    use anyhow::{Result, Context};
    use std::path::{Path, PathBuf};
    use chrono::Local;

    pub struct Logger {
        log_file: PathBuf,
    }

    impl Logger {
        pub fn new(output_dir: &Path) -> Result<Self> {
            let log_file = output_dir.join("log.txt");
            Ok(Self { log_file })
        }

        pub fn log(&self, level: log::Level, message: &str) -> Result<()> {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_message = format!("[{}] [{:?}] {}", timestamp, level, message);
            
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&self.log_file)?;
            writeln!(file, "{}", log_message)?;
            
            match level {
                log::Level::Error => error!("{}", message),
                log::Level::Warn => warn!("{}", message),
                log::Level::Info => info!("{}", message),
                log::Level::Debug => debug!("{}", message),
                log::Level::Trace => trace!("{}", message),
            }
            
            Ok(())
        }
    }
}

mod database {
    use anyhow::{Result, Context};
    use sled::Db;
    use std::path::Path;
    use log::debug;

    pub struct DatabaseManager {
        db: Db,
    }

    impl DatabaseManager {
        pub fn new(path: &Path) -> Result<Self> {
            let db = sled::open(path).context("Failed to open database")?;
            Ok(Self { db })
        }

        pub fn store(&self, key: &[u8], value: &[u8]) -> Result<()> {
            self.db.insert(key, value).context("Failed to insert into database")?;
            debug!("Stored key: {:?}", String::from_utf8_lossy(key));
            Ok(())
        }
    }
}

mod zip_processing {
    use anyhow::{Result, Context};
    use std::path::{PathBuf, Path};
    use zip::ZipArchive;
    use std::fs::{File, create_dir_all};
    use std::io::{Read, Write};

    pub struct ZipEntry {
        pub name: String,
        pub content: Vec<u8>,
    }

    pub fn process_zip(zip_path: &Path, extract: bool, output_dir: &Path) -> Result<Vec<ZipEntry>> {
        let file = File::open(zip_path).context("Failed to open ZIP file")?;
        let mut archive = ZipArchive::new(file).context("Failed to create ZIP archive")?;
        let mut entries = Vec::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).with_context(|| format!("Failed to get ZIP entry at index {}", i))?;
            let mut content = Vec::new();
            file.read_to_end(&mut content).with_context(|| format!("Failed to read content of ZIP entry '{}'", file.name()))?;

            let entry = ZipEntry {
                name: file.name().to_string(),
                content: content.clone(),
            };

            if extract {
                let output_path = output_dir.join(file.name());
                if let Some(p) = output_path.parent() {
                    create_dir_all(p).context("Failed to create directory")?;
                }
                let mut output_file = File::create(&output_path).context("Failed to create output file")?;
                output_file.write_all(&content).context("Failed to write content to file")?;
            }

            entries.push(entry);
        }

        Ok(entries)
    }
}

mod code_analysis {
    use anyhow::{Result, Context};
    use serde::{Serialize, Deserialize};
    use regex::Regex;
    use std::collections::HashSet;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ParsedFile {
        pub name: String,
        pub language: LanguageType,
        pub loc: usize,
        pub cyclomatic_complexity: usize,
        pub cognitive_complexity: usize,
        pub halstead_metrics: HalsteadMetrics,
        pub functions: Vec<String>,
        pub ast: Option<Expr>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct HalsteadMetrics {
        pub program_vocabulary: usize,
        pub program_length: usize,
        pub calculated_length: f64,
        pub volume: f64,
        pub difficulty: f64,
        pub effort: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum LanguageType {
        Rust,
        Python,
        JavaScript,
        Unknown,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Expr {
        Number(f64),
        Add(Box<Expr>, Box<Expr>),
        Subtract(Box<Expr>, Box<Expr>),
    }

    pub fn analyze_file(name: &str, content: &[u8]) -> Result<ParsedFile> {
        let language = detect_language(name, content);
        let loc = count_lines(content).context("Failed to count lines")?;
        let cyclomatic_complexity = calculate_cyclomatic_complexity(content).context("Failed to calculate cyclomatic complexity")?;
        let cognitive_complexity = calculate_cognitive_complexity(content).context("Failed to calculate cognitive complexity")?;
        let halstead_metrics = calculate_halstead_metrics(content).context("Failed to calculate Halstead metrics")?;
        let functions = find_functions(content).context("Failed to find functions")?;
        let ast = parse_expression(content).ok();

        Ok(ParsedFile {
            name: name.to_string(),
            language,
            loc,
            cyclomatic_complexity,
            cognitive_complexity,
            halstead_metrics,
            functions,
            ast,
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

    fn calculate_cyclomatic_complexity(content: &[u8]) -> Result<usize> {
        let text = std::str::from_utf8(content).context("Failed to convert content to UTF-8")?;
        let complexity = 1 + text.matches("if ").count()
            + text.matches("for ").count()
            + text.matches("while ").count()
            + text.matches("case ").count()
            + text.matches("&&").count()
            + text.matches("||").count();
        Ok(complexity)
    }

    fn calculate_cognitive_complexity(content: &[u8]) -> Result<usize> {
        let text = std::str::from_utf8(content).context("Failed to convert content to UTF-8")?;
        let mut complexity = 0;
        let mut nesting_level: usize = 0;

        for line in text.lines() {
            if line.contains("if ") || line.contains("for ") || line.contains("while ") {
                complexity += 1 + nesting_level;
                nesting_level += 1;
            }
            if line.contains("}") {
                nesting_level = nesting_level.saturating_sub(1);
            }
            if line.contains("&&") || line.contains("||") {
                complexity += 1;
            }
        }

        Ok(complexity)
    }

    fn calculate_halstead_metrics(content: &[u8]) -> Result<HalsteadMetrics> {
        let text = std::str::from_utf8(content).context("Failed to convert content to UTF-8")?;
        let operators = Regex::new(r"[+\-*/=<>!&|^~%]|\b(if|else|for|while|return)\b").unwrap();
        let operands = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b|\d+").unwrap();

        let unique_operators: HashSet<_> = operators.find_iter(text).map(|m| m.as_str()).collect();
        let unique_operands: HashSet<_> = operands.find_iter(text).map(|m| m.as_str()).collect();

        let n1 = unique_operators.len();
        let n2 = unique_operands.len();
        let n1_count = operators.find_iter(text).count();
        let n2_count = operands.find_iter(text).count();

        let program_vocabulary = n1 + n2;
        let program_length = n1_count + n2_count;
        let calculated_length = (n1 as f64 * (n2 as f64).log2() + n2 as f64 * (n1 as f64).log2()) as f64;
        let volume = (program_length as f64) * (program_vocabulary as f64).log2();
        let difficulty = (n1 as f64 / 2.0) * (n2_count as f64 / n2 as f64);
        let effort = difficulty * volume;

        Ok(HalsteadMetrics {
            program_vocabulary,
            program_length,
            calculated_length,
            volume,
            difficulty,
            effort,
        })
    }

    fn find_functions(content: &[u8]) -> Result<Vec<String>> {
        let content = std::str::from_utf8(content).context("Failed to convert content to UTF-8")?;
        let re = Regex::new(r"(?m)^(?:fn|def|function)\s+([a-zA-Z_][a-zA-Z0-9_]*)")
            .context("Failed to create regex")?;
        Ok(re.captures_iter(content)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect())
    }

    fn parse_expression(content: &[u8]) -> Result<Expr> {
        let content = std::str::from_utf8(content).context("Failed to convert content to UTF-8")?;
        parse_expr(content).context("Failed to parse expression")
    }

    fn parse_expr(input: &str) -> Result<Expr> {
        let (expr, remaining) = parse_term(input)?;
        parse_expr_rest(expr, remaining.trim())
    }

    fn parse_expr_rest(left: Expr, input: &str) -> Result<Expr> {
        if input.starts_with('+') {
            let (right, remaining) = parse_term(&input[1..])?;
            parse_expr_rest(Expr::Add(Box::new(left), Box::new(right)), remaining.trim())
        } else if input.starts_with('-') {
            let (right, remaining) = parse_term(&input[1..])?;
            parse_expr_rest(Expr::Subtract(Box::new(left), Box::new(right)), remaining.trim())
        } else {
            Ok(left)
        }
    }

    fn parse_term(input: &str) -> Result<(Expr, &str)> {
        if let Some(num_end) = input.find(|c: char| !c.is_digit(10) && c != '.') {
            let (num_str, rest) = input.split_at(num_end);
            let num = num_str.parse::<f64>().context("Failed to parse number")?;
            Ok((Expr::Number(num), rest))
        } else {
            Err(anyhow::anyhow!("Invalid input"))
        }
    }
}

mod summary {
    use super::code_analysis::ParsedFile;
    use anyhow::Result;
    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProjectSummary {
        pub total_files: usize,
        pub total_loc: usize,
        pub language_breakdown: HashMap<String, usize>,
        pub average_complexity: f64,
    }

    pub fn generate_summary(files: Vec<ParsedFile>) -> Result<ProjectSummary> {
        let total_files = files.len();
        let total_loc: usize = files.iter().map(|f| f.loc).sum();
        let mut language_breakdown = HashMap::new();
        let total_complexity: usize = files.iter().map(|f| f.cyclomatic_complexity).sum();

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
        })
    }
}

mod output {
    use super::summary::ProjectSummary;
    use anyhow::{Result, Context};
    use std::fs::File;
    use std::io::{Write, BufWriter};
    use std::path::Path;
    use serde_json;

    pub fn write_summary(summary: &ProjectSummary, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(summary).context("Failed to serialize summary")?;
        let file = File::create(output_path).context("Failed to create output file")?;
        let mut writer = BufWriter::new(file);
        writer.write_all(json.as_bytes()).context("Failed to write summary to file")?;
        writer.flush().context("Failed to flush output file")?;
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    
    // Hardcoded values instead of command-line arguments
    let input_path = "/home/amuldotexe/Downloads/tokei-master.zip";
    let output_dir = "/home/amuldotexe/Desktop/TempResults2024/Parseltongue2024/Play2024";
    let extract = false;

    info!("Input: {}, Output: {}, Extract: {}", input_path, output_dir, extract);

    let logger: logger::Logger = logger::Logger::new(Path::new(output_dir)).context("Failed to create logger")?;
    
    logger.log(log::Level::Info, "Starting the application").context("Failed to log start message")?;

    let db_path: &Path = Path::new("huffelpuff_db");
    let db_manager: Arc<database::DatabaseManager> = Arc::new(database::DatabaseManager::new(db_path).context("Failed to create DatabaseManager")?);

    let zip_path: PathBuf = PathBuf::from(input_path);
    let output_dir: PathBuf = PathBuf::from(output_dir);
    let zip_entries: Vec<zip_processing::ZipEntry> = zip_processing::process_zip(&zip_path, extract, &output_dir).context("Failed to process ZIP file")?;

    let mut parsed_files: Vec<code_analysis::ParsedFile> = Vec::new();

    let total_files: usize = zip_entries.len();
    let progress_bar: ProgressBar = ProgressBar::new(total_files as u64);
    let progress_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .expect("Failed to set progress bar style")
        .progress_chars("=>-");
    progress_bar.set_style(progress_style);

    for entry in zip_entries {
        progress_bar.set_message(format!("Processing: {}", entry.name));
        
        let analysis_result: code_analysis::ParsedFile = code_analysis::analyze_file(&entry.name, &entry.content)
            .context("Failed to analyze file")?;
        
        let analysis_key: Vec<u8> = format!("analysis:{}", entry.name).into_bytes();
        let analysis_value: Vec<u8> = serde_json::to_vec(&analysis_result).context("Failed to serialize analysis result")?;
        db_manager.store(&analysis_key, &analysis_value)
            .context("Failed to store analysis result")?;

        db_manager.store(entry.name.as_bytes(), &entry.content)
            .context("Failed to store file content")?;

        parsed_files.push(analysis_result);
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Processing complete");

    let project_summary: summary::ProjectSummary = summary::generate_summary(parsed_files).context("Failed to generate project summary")?;
    
    let output_path: PathBuf = output_dir.join("summary.json");
    output::write_summary(&project_summary, &output_path).context("Failed to write summary")?;

    info!("Summary written to: {:?}", output_path);

    logger.log(log::Level::Info, "Application finished").context("Failed to log finish message")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_database_operations() -> Result<()> {
        let temp_dir = TempDir::new().context("Failed to create temp dir")?;
        let db_path = temp_dir.path().join("test_db");
        let db_manager = database::DatabaseManager::new(&db_path).context("Failed to create DatabaseManager")?;

        let key = b"test_key";
        let value = b"test_value";

        db_manager.store(key, value).context("Failed to store value")?;

        Ok(())
    }
}

use log::{info, warn};
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json;
use chrono::Local;
use clap::Parser;
use encoding_rs::WINDOWS_1252;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;
use std::fs::{File, OpenOptions}; // Ensure File is imported
// use std::collections::HashMap; // Commented out as it's not used

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
    use encoding_rs::WINDOWS_1252;
    use log::warn; // Ensure `warn` is imported in this module
    use std::path::PathBuf; // ✅ Imported here

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct ParsedFile {
        pub name: String,
        pub language: LanguageType,
        pub loc: usize,
        pub cyclomatic_complexity: usize,
        pub cognitive_complexity: usize,
        pub halstead_metrics: HalsteadMetrics,
        pub functions: Vec<String>,
        pub ast: Option<Expr>,
        pub content: Vec<u8>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct HalsteadMetrics {
        pub program_vocabulary: usize,
        pub program_length: usize,
        pub calculated_length: f64,
        pub volume: f64,
        pub difficulty: f64,
        pub effort: f64,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
    pub enum LanguageType {
        Rust,
        Python,
        JavaScript,
        Unknown,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum Expr {
        Number(f64),
        Add(Box<Expr>, Box<Expr>),
        Subtract(Box<Expr>, Box<Expr>),
    }

    pub fn analyze_file(name: &str, content: &[u8]) -> Result<ParsedFile> {
        let (cow, _, had_errors) = WINDOWS_1252.decode(content);
        let text = cow.into_owned();
        
        if had_errors {
            warn!("File {} had encoding errors, some characters may be incorrect", name);
        }

        let language = detect_language(name, &text);
        let loc = count_lines(&text).context("Failed to count lines")?;
        let cyclomatic_complexity = calculate_cyclomatic_complexity(&text).context("Failed to calculate cyclomatic complexity")?;
        let cognitive_complexity = calculate_cognitive_complexity(&text).context("Failed to calculate cognitive complexity")?;
        let halstead_metrics = calculate_halstead_metrics(&text).context("Failed to calculate Halstead metrics")?;
        let functions = find_functions(&text).context("Failed to find functions")?;
        let ast = parse_expression(&text).ok();

        Ok(ParsedFile {
            name: name.to_string(),
            language,
            loc,
            cyclomatic_complexity,
            cognitive_complexity,
            halstead_metrics,
            functions,
            ast,
            content: content.to_vec(),
        })
    }

    fn detect_language(filename: &str, content: &str) -> LanguageType {
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

    fn detect_from_content(content: &str) -> LanguageType {
        if content.starts_with("fn ") || content.starts_with("use ") {
            LanguageType::Rust
        } else if content.starts_with("def ") || content.starts_with("import ") {
            LanguageType::Python
        } else if content.starts_with("function ") || content.starts_with("var ") {
            LanguageType::JavaScript
        } else {
            LanguageType::Unknown
        }
    }

    fn count_lines(content: &str) -> Result<usize> {
        Ok(content.lines().count())
    }

    fn calculate_cyclomatic_complexity(content: &str) -> Result<usize> {
        let complexity = 1 + content.matches("if ").count()
            + content.matches("for ").count()
            + content.matches("while ").count()
            + content.matches("case ").count()
            + content.matches("&&").count()
            + content.matches("||").count();
        Ok(complexity)
    }

    fn calculate_cognitive_complexity(content: &str) -> Result<usize> {
        let mut complexity = 0;
        let mut nesting_level: usize = 0;

        for line in content.lines() {
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

    fn calculate_halstead_metrics(content: &str) -> Result<HalsteadMetrics> {
        let operators = Regex::new(r"[+\-*/=<>!&|^~%]|\b(if|else|for|while|return)\b").unwrap();
        let operands = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b|\d+").unwrap();

        let unique_operators: HashSet<_> = operators.find_iter(content).map(|m| m.as_str()).collect();
        let unique_operands: HashSet<_> = operands.find_iter(content).map(|m| m.as_str()).collect();

        let n1 = unique_operators.len();
        let n2 = unique_operands.len();
        let n1_count = operators.find_iter(content).count();
        let n2_count = operands.find_iter(content).count();

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

    fn find_functions(content: &str) -> Result<Vec<String>> {
        let re = Regex::new(r"(?m)^(?:fn|def|function)\s+([a-zA-Z_][a-zA-Z0-9_]*)")
            .context("Failed to create regex")?;
        Ok(re.captures_iter(content)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect())
    }

    pub fn parse_expression(content: &str) -> Result<Expr> {
        parse_expr(content)
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
    use super::code_analysis::{ParsedFile, Expr, LanguageType};
    use anyhow::Result;
    use serde::{Serialize, Deserialize};
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProjectSummary {
        pub total_files: usize,
        pub total_loc: usize,
        pub language_breakdown: HashMap<LanguageType, usize>,
        pub average_complexity: f64,
        pub function_analysis: FunctionAnalysis,
        pub ast_analysis: ASTAnalysis,
        pub top_complex_files: Vec<ComplexFile>,
        pub language_metrics: HashMap<LanguageType, LanguageMetrics>,
        pub dependency_graph: DependencyGraph,
        pub code_duplication: CodeDuplication,
        pub security_metrics: SecurityMetrics,
        pub code_quality_score: f64,
        pub language_feature_usage: HashMap<LanguageType, LanguageFeatureUsage>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FunctionAnalysis {
        pub total_functions: usize,
        pub average_function_length: f64,
        pub most_complex_functions: Vec<ComplexFunction>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ASTAnalysis {
        pub total_expressions: usize,
        pub expression_types: HashMap<String, usize>,
        pub max_nesting_depth: usize,
        pub average_nesting_depth: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ComplexFile {
        pub name: String,
        pub complexity: usize,
        pub loc: usize,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ComplexFunction {
        pub name: String,
        pub file: String,
        pub complexity: usize,
        pub loc: usize,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LanguageMetrics {
        pub file_count: usize,
        pub total_loc: usize,
        pub average_complexity: f64,
        pub average_halstead_difficulty: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DependencyGraph {
        pub nodes: Vec<String>,
        pub edges: Vec<(String, String)>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CodeDuplication {
        pub total_duplicated_lines: usize,
        pub duplication_percentage: f64,
        pub largest_clone: Clone,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Clone {
        pub files: Vec<String>,
        pub line_count: usize,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SecurityMetrics {
        pub deprecated_function_usage: usize,
        pub potential_vulnerabilities: Vec<Vulnerability>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Vulnerability {
        pub file: String,
        pub line: usize,
        pub description: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LanguageFeatureUsage {
        pub control_structures: HashMap<String, usize>,
        pub advanced_features: HashMap<String, usize>,
    }

    pub fn generate_summary(files: Vec<ParsedFile>) -> Result<ProjectSummary> {
        let total_files = files.len();
        let total_loc: usize = files.iter().map(|f| f.loc).sum();
        let mut language_breakdown = HashMap::new();
        let total_complexity: usize = files.iter().map(|f| f.cyclomatic_complexity).sum();

        let mut function_analysis = FunctionAnalysis {
            total_functions: 0,
            average_function_length: 0.0,
            most_complex_functions: Vec::new(),
        };

        let mut ast_analysis = ASTAnalysis {
            total_expressions: 0,
            expression_types: HashMap::new(),
            max_nesting_depth: 0,
            average_nesting_depth: 0.0,
        };

        let mut top_complex_files = Vec::new();
        let mut language_metrics = HashMap::new();
        let mut language_feature_usage = HashMap::new();

        for file in &files {
            *language_breakdown.entry(file.language.clone()).or_insert(0) += 1;
            
            function_analysis.total_functions += file.functions.len();
            for func in &file.functions {
                function_analysis.most_complex_functions.push(ComplexFunction {
                    name: func.clone(),
                    file: file.name.clone(),
                    complexity: file.cyclomatic_complexity, // This is a simplification, ideally we'd calculate complexity per function
                    loc: 0, // This is a simplification, ideally we'd calculate LOC per function
                });
            }
            
            if let Some(ast) = &file.ast {
                analyze_ast(ast, &mut ast_analysis);
            }
            
            top_complex_files.push(ComplexFile {
                name: file.name.clone(),
                complexity: file.cyclomatic_complexity,
                loc: file.loc,
            });
            
            let lang_metrics = language_metrics.entry(file.language.clone())
                .or_insert(LanguageMetrics {
                    file_count: 0,
                    total_loc: 0,
                    average_complexity: 0.0,
                    average_halstead_difficulty: 0.0,
                });
            lang_metrics.file_count += 1;
            lang_metrics.total_loc += file.loc;
            lang_metrics.average_complexity += file.cyclomatic_complexity as f64;
            lang_metrics.average_halstead_difficulty += file.halstead_metrics.difficulty;

            let usage = language_feature_usage.entry(file.language.clone()).or_insert(LanguageFeatureUsage {
                control_structures: HashMap::new(),
                advanced_features: HashMap::new(),
            });

            // This is a simplification. In reality, you'd need to parse the file content more thoroughly.
            for line in std::str::from_utf8(&file.content).unwrap_or("").lines() {
                if line.contains("if ") { *usage.control_structures.entry("if".to_string()).or_insert(0) += 1; }
                if line.contains("for ") { *usage.control_structures.entry("for".to_string()).or_insert(0) += 1; }
                if line.contains("while ") { *usage.control_structures.entry("while".to_string()).or_insert(0) += 1; }
                // Add more checks for advanced features based on the language
                match file.language {
                    LanguageType::Rust => {
                        if line.contains("async ") { *usage.advanced_features.entry("async".to_string()).or_insert(0) += 1; }
                        if line.contains("impl ") { *usage.advanced_features.entry("impl".to_string()).or_insert(0) += 1; }
                    },
                    LanguageType::Python => {
                        if line.contains("def ") { *usage.advanced_features.entry("function".to_string()).or_insert(0) += 1; }
                        if line.contains("class ") { *usage.advanced_features.entry("class".to_string()).or_insert(0) += 1; }
                    },
                    LanguageType::JavaScript => {
                        if line.contains("function ") { *usage.advanced_features.entry("function".to_string()).or_insert(0) += 1; }
                        if line.contains("class ") { *usage.advanced_features.entry("class".to_string()).or_insert(0) += 1; }
                    },
                    LanguageType::Unknown => {},
                }
            }
        }

        let average_complexity = if total_files > 0 {
            total_complexity as f64 / total_files as f64
        } else {
            0.0
        };

        function_analysis.average_function_length = if function_analysis.total_functions > 0 {
            total_loc as f64 / function_analysis.total_functions as f64
        } else {
            0.0
        };

        function_analysis.most_complex_functions.sort_by(|a, b| b.complexity.cmp(&a.complexity));
        function_analysis.most_complex_functions.truncate(10);

        top_complex_files.sort_by(|a, b| b.complexity.cmp(&a.complexity));
        top_complex_files.truncate(10);

        for (language, metrics) in &mut language_metrics {
            let file_count = *language_breakdown.get(language).unwrap_or(&1) as f64;
            metrics.average_complexity /= file_count;
            metrics.average_halstead_difficulty /= file_count;
        }

        let dependency_graph = analyze_dependencies(&files);
        let code_duplication = analyze_code_duplication(&files);
        let security_metrics = analyze_security(&files);
        let code_quality_score = calculate_code_quality_score(&files);

        Ok(ProjectSummary {
            total_files,
            total_loc,
            language_breakdown,
            average_complexity,
            function_analysis,
            ast_analysis,
            top_complex_files,
            language_metrics,
            dependency_graph,
            code_duplication,
            security_metrics,
            code_quality_score,
            language_feature_usage,
        })
    }

    fn analyze_ast(expr: &Expr, analysis: &mut ASTAnalysis) {
        analysis.total_expressions += 1;
        *analysis.expression_types.entry(format!("{:?}", expr)).or_insert(0) += 1;

        match expr {
            Expr::Add(left, right) | Expr::Subtract(left, right) => {
                analyze_ast(left, analysis);
                analyze_ast(right, analysis);
            }
            Expr::Number(_) => {}
        }
    }

    fn analyze_dependencies(_files: &[ParsedFile]) -> DependencyGraph {
        // Implement the function logic here
        DependencyGraph {
            nodes: vec![],
            edges: vec![],
        }
    }

    fn analyze_code_duplication(_files: &[ParsedFile]) -> CodeDuplication {
        // Implement the function logic here
        CodeDuplication {
            total_duplicated_lines: 0,
            duplication_percentage: 0.0,
            largest_clone: Clone {
                files: vec![],
                line_count: 0,
            },
        }
    }

    fn analyze_security(_files: &[ParsedFile]) -> SecurityMetrics {
        // Implement the function logic here
        SecurityMetrics {
            deprecated_function_usage: 0,
            potential_vulnerabilities: vec![],
        }
    }

    fn calculate_code_quality_score(_files: &[ParsedFile]) -> f64 {
        // Implement the function logic here
        0.0
    }
}

mod output {
    use super::summary::ProjectSummary;
    use anyhow::{Result, Context};
    use std::fs::{File, OpenOptions};
    use std::io::{Write, BufWriter};
    use std::path::{Path, PathBuf};
    use serde_json;
    use chrono::Local;
    use colored::*;
    use flate2::write::GzEncoder;
    use flate2::Compression;

    pub struct OutputManager {
        output_dir: PathBuf,
    }

    impl OutputManager {
        pub fn new(output_dir: &Path) -> Result<Self> {
            std::fs::create_dir_all(output_dir).context("Failed to create output directory")?;
            Ok(Self { output_dir: output_dir.to_path_buf() })
        }

        pub fn write_summary(&self, summary: &ProjectSummary, filename: &str) -> Result<()> {
            let json = serde_json::to_string_pretty(summary).context("Failed to serialize summary")?;
            let path = self.output_dir.join(filename);
            let file = File::create(&path).context("Failed to create output file")?;
            let mut encoder = GzEncoder::new(file, Compression::default());
            encoder.write_all(json.as_bytes()).context("Failed to write compressed summary")?;
            encoder.finish().context("Failed to finish compression")?;
            Ok(())
        }

        pub fn write_progress(&self, message: &str) -> Result<()> {
            let path = self.output_dir.join("progress.txt");
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .context("Failed to open progress file")?;
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            writeln!(file, "[{}] {}", timestamp, message).context("Failed to write progress")?;
            Ok(())
        }

        pub fn log_message(&self, level: log::Level, message: &str) -> Result<()> {
            let path = self.output_dir.join("log.txt");
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .context("Failed to open log file")?;
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_message = format!("[{}] [{:?}] {}", timestamp, level, message);
            writeln!(file, "{}", log_message).context("Failed to write log")?;
            
            let colored_message = match level {
                log::Level::Error => log_message.red(),
                log::Level::Warn => log_message.yellow(),
                log::Level::Info => log_message.green(),
                log::Level::Debug => log_message.blue(),
                log::Level::Trace => log_message.magenta(),
            };
            println!("{}", colored_message);
            Ok(())
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    /// Input ZIP file path
    #[clap(short, long, value_parser)]
    input_zip: PathBuf,

    /// Output directory path
    #[clap(short, long, value_parser)]
    output_dir: PathBuf,

    /// Whether to extract the ZIP contents
    #[clap(short, long)]
    extract: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let config = parse_config()?;

    let start_timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();

    let folder_name = config.input_zip
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");

    let output_manager = output::OutputManager::new(&config.output_dir)
        .context("Failed to create OutputManager")?;

    output_manager.log_message(log::Level::Info, &format!(
        "Starting analysis of {} with output to {}",
        config.input_zip.display(),
        config.output_dir.display()
    ))?;

    // Use the user-provided output directory for the database
    let db_path = config.output_dir.join("huffelpuff_db");
    let db_manager = Arc::new(database::DatabaseManager::new(&db_path).context("Failed to create DatabaseManager")?);

    let zip_entries = zip_processing::process_zip(&config.input_zip, config.extract, &config.output_dir)
        .context("Failed to process ZIP file")?;

    let mut parsed_files = Vec::new();

    let progress_bar = ProgressBar::new(zip_entries.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .expect("Failed to set template")
        .progress_chars("=>-"));

    for entry in zip_entries {
        progress_bar.set_message(format!("Processing: {}", entry.name));
        
        match code_analysis::analyze_file(&entry.name, &entry.content) {
            Ok(analysis_result) => {
                let analysis_key = format!("analysis:{}", entry.name).into_bytes();
                let analysis_value = serde_json::to_vec(&analysis_result).context("Failed to serialize analysis result")?;
                db_manager.store(&analysis_key, &analysis_value)
                    .context("Failed to store analysis result")?;

                db_manager.store(entry.name.as_bytes(), &entry.content)
                    .context("Failed to store file content")?;

                parsed_files.push(analysis_result);
            },
            Err(e) => {
                output_manager.log_message(log::Level::Warn, &format!("Failed to analyze file {}: {}", entry.name, e))?;
            }
        }
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Processing complete");

    let project_summary = summary::generate_summary(parsed_files).context("Failed to generate project summary")?;
    
    // Change the output filename to use .txt instead of .json.gz
    let output_filename = format!("{}-{}.txt", folder_name, start_timestamp);
    
    // Writing the summary directly to a .txt file without compression
    let json = serde_json::to_string_pretty(&project_summary).context("Failed to serialize summary")?;
    let path = config.output_dir.join(&output_filename);
    let mut file = File::create(&path).context("Failed to create output file")?; // Now File is recognized
    file.write_all(json.as_bytes()).context("Failed to write summary")?;

    output_manager.log_message(log::Level::Info, &format!("Summary written to: {}", output_filename))?;

    // Add some logging of interesting metrics
    output_manager.log_message(log::Level::Info, &format!("Total files analyzed: {}", project_summary.total_files))?;
    output_manager.log_message(log::Level::Info, &format!("Total lines of code: {}", project_summary.total_loc))?;
    output_manager.log_message(log::Level::Info, &format!("Average complexity: {:.2}", project_summary.average_complexity))?;
    output_manager.log_message(log::Level::Info, &format!("Code quality score: {:.2}", project_summary.code_quality_score))?;
    output_manager.log_message(log::Level::Info, "Analysis completed successfully")?;
    Ok(())
}

fn parse_config() -> Result<Config> {
    let config = Config::parse();

    // Validate input ZIP path
    if !config.input_zip.exists() {
        return Err(anyhow::anyhow!("Input ZIP file does not exist: {}", config.input_zip.display()));
    }
    if !config.input_zip.is_file() {
        return Err(anyhow::anyhow!("Input path is not a file: {}", config.input_zip.display()));
    }

    // Ensure output directory exists or can be created
    if !config.output_dir.exists() {
        std::fs::create_dir_all(&config.output_dir)
            .context("Failed to create output directory")?;
    }
    if !config.output_dir.is_dir() {
        return Err(anyhow::anyhow!("Output path is not a directory: {}", config.output_dir.display()));
    }

    Ok(config)
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

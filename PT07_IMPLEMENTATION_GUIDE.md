# pt07-cozodb-code-as-visuals: Implementation Guide

**Quick-Start Guide for Building the Analytics Tool**

This document provides step-by-step implementation guidance for developers building `pt07-cozodb-code-as-visuals`.

---

## Phase 1: Foundation (Week 1)

### 1.1 Project Setup

```bash
# Create new crate in workspace
cd crates/
cargo new pt07-cozodb-code-as-visuals
cd pt07-cozodb-code-as-visuals
```

**Cargo.toml Dependencies**:
```toml
[dependencies]
# Core dependencies (reuse from parseltongue-core)
parseltongue-core = { path = "../parseltongue-core" }
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }

# CLI
clap = { version = "4.0", features = ["derive"] }

# Table rendering
comfy-table = "7.0"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
tokio-test = "0.4"
```

### 1.2 CLI Skeleton

**src/cli.rs**:
```rust
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "pt07-cozodb-code-as-visuals")]
#[command(about = "Generate analytics and visualizations from ISG data")]
#[command(version)]
pub struct Cli {
    /// Database path
    #[arg(long, default_value = "parseltongue.db")]
    pub db: String,

    /// Report type to generate
    #[arg(long, value_enum, default_value = "dashboard")]
    pub report: ReportType,

    /// Output format
    #[arg(long, value_enum, default_value = "table")]
    pub format: OutputFormat,

    /// Filter expression (e.g., "risk=High,complexity=Complex")
    #[arg(long)]
    pub filter: Option<String>,

    /// Entity key for blast-radius report
    #[arg(long)]
    pub entity: Option<String>,

    /// Limit results (default: 20)
    #[arg(long, default_value = "20")]
    pub limit: usize,

    /// Sort by column
    #[arg(long)]
    pub sort: Option<String>,

    /// Enable color output
    #[arg(long, default_value = "true")]
    pub color: bool,

    /// Verbose output
    #[arg(long, short)]
    pub verbose: bool,
}

#[derive(Clone, ValueEnum)]
pub enum ReportType {
    Dashboard,
    Health,
    Complexity,
    Coverage,
    Dependencies,
    Changes,
    BlastRadius,
    Entities,
    Modules,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}
```

### 1.3 Main Entry Point

**src/main.rs**:
```rust
mod cli;
mod reports;
mod analytics;
mod renderer;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use parseltongue_core::storage::CozoDbStorage;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Connect to database
    let db = CozoDbStorage::new(&cli.db).await?;

    // Check connection
    if !db.is_connected().await {
        anyhow::bail!("Failed to connect to database: {}", cli.db);
    }

    if cli.verbose {
        println!("Connected to database: {}", cli.db);
    }

    // Generate report
    let output = match cli.report {
        cli::ReportType::Dashboard => reports::dashboard::generate(&db, &cli).await?,
        cli::ReportType::Entities => reports::entities::generate(&db, &cli).await?,
        // ... more reports
        _ => {
            anyhow::bail!("Report type {:?} not yet implemented", cli.report);
        }
    };

    // Render output
    match cli.format {
        cli::OutputFormat::Table => println!("{}", output),
        cli::OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&output)?),
        cli::OutputFormat::Csv => println!("{}", output), // CSV renderer
    }

    Ok(())
}
```

### 1.4 Analytics Module Foundation

**src/analytics/mod.rs**:
```rust
pub mod stats;
pub mod filters;
pub mod queries;

use parseltongue_core::entities::CodeEntity;

/// Core statistics for codebase analysis
#[derive(Debug, Clone)]
pub struct CodebaseStats {
    pub total_entities: usize,
    pub total_files: usize,
    pub test_coverage: f64,
    pub avg_complexity: String,
    pub high_risk_count: usize,
    pub complex_count: usize,
    pub avg_dependencies: f64,
    pub public_api_count: usize,
    pub public_api_doc_coverage: f64,
}

impl CodebaseStats {
    pub fn calculate(entities: &[CodeEntity]) -> Self {
        // Implementation
        todo!()
    }
}
```

---

## Phase 2: Core Analytics (Week 1-2)

### 2.1 Query Builder

**src/analytics/queries.rs**:
```rust
use parseltongue_core::storage::CozoDbStorage;
use parseltongue_core::entities::CodeEntity;
use anyhow::Result;

/// Query all entities from database
pub async fn get_all_entities(db: &CozoDbStorage) -> Result<Vec<CodeEntity>> {
    db.get_all_entities().await
}

/// Query entities by complexity level
pub async fn get_entities_by_complexity(
    db: &CozoDbStorage,
    complexity: &str,
) -> Result<Vec<CodeEntity>> {
    let all_entities = db.get_all_entities().await?;

    Ok(all_entities
        .into_iter()
        .filter(|e| {
            let level = match &e.tdd_classification.complexity {
                parseltongue_core::entities::ComplexityLevel::Simple => "Simple",
                parseltongue_core::entities::ComplexityLevel::Moderate => "Moderate",
                parseltongue_core::entities::ComplexityLevel::Complex => "Complex",
            };
            level == complexity
        })
        .collect())
}

/// Query high-risk entities
pub async fn get_high_risk_entities(db: &CozoDbStorage) -> Result<Vec<CodeEntity>> {
    let all_entities = db.get_all_entities().await?;

    Ok(all_entities
        .into_iter()
        .filter(|e| {
            matches!(
                e.tdd_classification.change_risk,
                parseltongue_core::entities::RiskLevel::High
            )
        })
        .collect())
}

/// Query entities with low coverage
pub async fn get_low_coverage_entities(
    db: &CozoDbStorage,
    threshold: f64,
) -> Result<Vec<CodeEntity>> {
    let all_entities = db.get_all_entities().await?;

    Ok(all_entities
        .into_iter()
        .filter(|e| e.tdd_classification.test_coverage_estimate < threshold)
        .collect())
}
```

### 2.2 Statistics Calculator

**src/analytics/stats.rs**:
```rust
use parseltongue_core::entities::{CodeEntity, ComplexityLevel, RiskLevel, EntityClass};
use super::CodebaseStats;

pub fn calculate_stats(entities: &[CodeEntity]) -> CodebaseStats {
    let total_entities = entities.len();

    // Calculate unique files
    let files: std::collections::HashSet<_> = entities
        .iter()
        .map(|e| &e.interface_signature.file_path)
        .collect();
    let total_files = files.len();

    // Average test coverage
    let total_coverage: f64 = entities
        .iter()
        .map(|e| e.tdd_classification.test_coverage_estimate)
        .sum();
    let test_coverage = if total_entities > 0 {
        total_coverage / total_entities as f64
    } else {
        0.0
    };

    // High-risk count
    let high_risk_count = entities
        .iter()
        .filter(|e| matches!(e.tdd_classification.change_risk, RiskLevel::High))
        .count();

    // Complex count
    let complex_count = entities
        .iter()
        .filter(|e| matches!(e.tdd_classification.complexity, ComplexityLevel::Complex))
        .count();

    // Average complexity (weighted)
    let complexity_score: f64 = entities
        .iter()
        .map(|e| match e.tdd_classification.complexity {
            ComplexityLevel::Simple => 1.0,
            ComplexityLevel::Moderate => 2.0,
            ComplexityLevel::Complex => 3.0,
        })
        .sum();
    let avg_complexity_val = complexity_score / total_entities as f64;
    let avg_complexity = if avg_complexity_val < 1.5 {
        "Simple".to_string()
    } else if avg_complexity_val < 2.5 {
        "Moderate".to_string()
    } else {
        "Complex".to_string()
    };

    // Average dependencies
    let total_deps: usize = entities
        .iter()
        .map(|e| e.tdd_classification.dependencies)
        .sum();
    let avg_dependencies = if total_entities > 0 {
        total_deps as f64 / total_entities as f64
    } else {
        0.0
    };

    // Public API stats
    let public_apis: Vec<_> = entities
        .iter()
        .filter(|e| {
            matches!(
                e.interface_signature.visibility,
                parseltongue_core::entities::Visibility::Public
            )
        })
        .collect();
    let public_api_count = public_apis.len();

    // Public API documentation coverage (check if documentation field is Some)
    let documented_public_apis = public_apis
        .iter()
        .filter(|e| e.interface_signature.documentation.is_some())
        .count();
    let public_api_doc_coverage = if public_api_count > 0 {
        documented_public_apis as f64 / public_api_count as f64
    } else {
        0.0
    };

    CodebaseStats {
        total_entities,
        total_files,
        test_coverage,
        avg_complexity,
        high_risk_count,
        complex_count,
        avg_dependencies,
        public_api_count,
        public_api_doc_coverage,
    }
}

pub fn calculate_health_score(stats: &CodebaseStats) -> u8 {
    let mut score = 100u8;

    // Test coverage (max -30 points)
    if stats.test_coverage < 0.8 {
        let penalty = ((0.8 - stats.test_coverage) * 30.0) as u8;
        score = score.saturating_sub(penalty);
    }

    // Complexity ratio (max -20 points)
    let complex_ratio = stats.complex_count as f64 / stats.total_entities as f64;
    if complex_ratio > 0.1 {
        let penalty = ((complex_ratio - 0.1) * 200.0) as u8;
        score = score.saturating_sub(penalty);
    }

    // High-risk entities (max -20 points)
    let risk_ratio = stats.high_risk_count as f64 / stats.total_entities as f64;
    if risk_ratio > 0.05 {
        let penalty = ((risk_ratio - 0.05) * 400.0) as u8;
        score = score.saturating_sub(penalty);
    }

    // Public API documentation (max -15 points)
    if stats.public_api_doc_coverage < 0.9 {
        let penalty = ((0.9 - stats.public_api_doc_coverage) * 15.0) as u8;
        score = score.saturating_sub(penalty);
    }

    // Coupling (max -15 points)
    if stats.avg_dependencies > 5.0 {
        let penalty = ((stats.avg_dependencies - 5.0) * 3.0) as u8;
        score = score.saturating_sub(penalty);
    }

    score
}

pub fn score_to_grade(score: u8) -> &'static str {
    match score {
        90..=100 => "A+ (Excellent)",
        80..=89 => "A (Very Good)",
        70..=79 => "B+ (Good)",
        60..=69 => "B (Acceptable)",
        50..=59 => "C (Needs Work)",
        _ => "D (Critical Issues)",
    }
}
```

### 2.3 Filter Parser

**src/analytics/filters.rs**:
```rust
use parseltongue_core::entities::{CodeEntity, EntityType, RiskLevel, ComplexityLevel, Visibility};
use anyhow::{Result, bail};

pub struct Filter {
    pub entity_type: Option<String>,
    pub risk: Option<String>,
    pub complexity: Option<String>,
    pub visibility: Option<String>,
    pub coverage_lt: Option<f64>,
    pub coverage_gt: Option<f64>,
}

impl Filter {
    pub fn parse(filter_str: &str) -> Result<Self> {
        let mut filter = Filter {
            entity_type: None,
            risk: None,
            complexity: None,
            visibility: None,
            coverage_lt: None,
            coverage_gt: None,
        };

        for part in filter_str.split(',') {
            let parts: Vec<&str> = part.trim().splitn(2, '=').collect();
            if parts.len() != 2 {
                bail!("Invalid filter format: {}", part);
            }

            let key = parts[0].trim();
            let value = parts[1].trim();

            match key {
                "entity_type" => filter.entity_type = Some(value.to_string()),
                "risk" => filter.risk = Some(value.to_string()),
                "complexity" => filter.complexity = Some(value.to_string()),
                "visibility" => filter.visibility = Some(value.to_string()),
                "coverage" if value.starts_with('<') => {
                    filter.coverage_lt = Some(value[1..].parse()?);
                }
                "coverage" if value.starts_with('>') => {
                    filter.coverage_gt = Some(value[1..].parse()?);
                }
                "coverage" => {
                    filter.coverage_lt = Some(value.parse::<f64>()? + 0.01);
                    filter.coverage_gt = Some(value.parse::<f64>()? - 0.01);
                }
                _ => bail!("Unknown filter key: {}", key),
            }
        }

        Ok(filter)
    }

    pub fn apply(&self, entities: Vec<CodeEntity>) -> Vec<CodeEntity> {
        entities
            .into_iter()
            .filter(|e| self.matches(e))
            .collect()
    }

    fn matches(&self, entity: &CodeEntity) -> bool {
        // Entity type filter
        if let Some(ref et) = self.entity_type {
            let entity_type_str = format!("{:?}", entity.interface_signature.entity_type);
            if !entity_type_str.to_lowercase().contains(&et.to_lowercase()) {
                return false;
            }
        }

        // Risk filter
        if let Some(ref risk) = self.risk {
            let risk_str = format!("{:?}", entity.tdd_classification.change_risk);
            if risk_str != *risk {
                return false;
            }
        }

        // Complexity filter
        if let Some(ref complexity) = self.complexity {
            let complexity_str = format!("{:?}", entity.tdd_classification.complexity);
            if complexity_str != *complexity {
                return false;
            }
        }

        // Visibility filter
        if let Some(ref visibility) = self.visibility {
            let visibility_str = format!("{:?}", entity.interface_signature.visibility);
            if visibility_str != *visibility {
                return false;
            }
        }

        // Coverage filters
        let coverage = entity.tdd_classification.test_coverage_estimate;
        if let Some(lt) = self.coverage_lt {
            if coverage >= lt {
                return false;
            }
        }
        if let Some(gt) = self.coverage_gt {
            if coverage <= gt {
                return false;
            }
        }

        true
    }
}
```

---

## Phase 3: Report Generators (Week 2)

### 3.1 Entities Report (Simplest)

**src/reports/entities.rs**:
```rust
use crate::cli::Cli;
use crate::analytics::{queries, filters};
use crate::renderer::table::TableRenderer;
use parseltongue_core::storage::CozoDbStorage;
use anyhow::Result;

pub async fn generate(db: &CozoDbStorage, cli: &Cli) -> Result<String> {
    // Fetch all entities
    let mut entities = queries::get_all_entities(db).await?;

    // Apply filters
    if let Some(ref filter_str) = cli.filter {
        let filter = filters::Filter::parse(filter_str)?;
        entities = filter.apply(entities);
    }

    // Sort (default by name)
    entities.sort_by(|a, b| {
        a.interface_signature.name.cmp(&b.interface_signature.name)
    });

    // Limit results
    let total_matches = entities.len();
    entities.truncate(cli.limit);

    // Render table
    let mut renderer = TableRenderer::new();
    renderer.add_header(&["#", "Name", "Type", "Visibility", "Complexity", "Risk", "Coverage", "Location"]);

    for (i, entity) in entities.iter().enumerate() {
        renderer.add_row(&[
            &(i + 1).to_string(),
            &entity.interface_signature.name,
            &format!("{:?}", entity.interface_signature.entity_type),
            &format!("{:?}", entity.interface_signature.visibility),
            &format!("{:?}", entity.tdd_classification.complexity),
            &format!("{:?}", entity.tdd_classification.change_risk),
            &format!("{:.0}%", entity.tdd_classification.test_coverage_estimate * 100.0),
            &entity.interface_signature.file_path.display().to_string(),
        ]);
    }

    let table_str = renderer.render();

    // Add summary
    let mut output = String::new();
    output.push_str("ENTITY LISTING\n");
    output.push_str(&"━".repeat(80));
    output.push_str("\n\n");

    if cli.filter.is_some() {
        output.push_str(&format!("Filter: {}\n", cli.filter.as_ref().unwrap()));
    }
    output.push_str(&format!("Results: {} of {} total matches\n\n", entities.len(), total_matches));

    output.push_str(&table_str);

    if total_matches > cli.limit {
        output.push_str(&format!("\n{} more entities match (use --limit {} to see all)\n",
            total_matches - cli.limit,
            total_matches
        ));
    }

    Ok(output)
}
```

### 3.2 Dashboard Report

**src/reports/dashboard.rs**:
```rust
use crate::cli::Cli;
use crate::analytics::{queries, stats};
use parseltongue_core::storage::CozoDbStorage;
use anyhow::Result;

pub async fn generate(db: &CozoDbStorage, cli: &Cli) -> Result<String> {
    let entities = queries::get_all_entities(db).await?;
    let codebase_stats = stats::calculate_stats(&entities);
    let health_score = stats::calculate_health_score(&codebase_stats);
    let grade = stats::score_to_grade(health_score);

    let mut output = String::new();

    // Header
    output.push_str("╔═══════════════════════════════════════════════════════════════════════╗\n");
    output.push_str("║                     PARSELTONGUE CODE ANALYTICS                       ║\n");
    output.push_str("║                                                                       ║\n");
    output.push_str(&format!("║  Database: {:<58} ║\n", cli.db));
    output.push_str("╠═══════════════════════════════════════════════════════════════════════╣\n\n");

    // Codebase snapshot
    output.push_str("📊 CODEBASE SNAPSHOT\n\n");
    output.push_str(&format!("  Total Entities:  {}\n", codebase_stats.total_entities));
    output.push_str(&format!("  Files Analyzed:  {}\n", codebase_stats.total_files));
    output.push_str("\n");
    output.push_str(&"─".repeat(80));
    output.push_str("\n\n");

    // Health score
    output.push_str(&format!("🎯 HEALTH SCORE: {} ({}/100)\n\n", grade, health_score));
    output.push_str(&format!("  Test Coverage:       {:.1}%\n", codebase_stats.test_coverage * 100.0));
    output.push_str(&format!("  Avg Complexity:      {}\n", codebase_stats.avg_complexity));
    output.push_str(&format!("  High-Risk Entities:  {}\n", codebase_stats.high_risk_count));
    output.push_str(&format!("  Public APIs:         {}\n", codebase_stats.public_api_count));
    output.push_str("\n");
    output.push_str(&"─".repeat(80));
    output.push_str("\n\n");

    // Recommendations
    output.push_str("⚠️  TOP PRIORITIES\n\n");
    if codebase_stats.test_coverage < 0.7 {
        output.push_str(&format!("  1. Boost test coverage from {:.1}% to 70%\n",
            codebase_stats.test_coverage * 100.0));
    }
    if codebase_stats.high_risk_count > 10 {
        output.push_str(&format!("  2. Review {} high-risk entities\n", codebase_stats.high_risk_count));
    }
    if codebase_stats.complex_count > 20 {
        output.push_str(&format!("  3. Refactor {} complex entities\n", codebase_stats.complex_count));
    }

    output.push_str("\n");
    output.push_str(&"─".repeat(80));
    output.push_str("\n\n");

    // Quick commands
    output.push_str("💡 QUICK COMMANDS\n\n");
    output.push_str("  Complexity Hotspots:\n");
    output.push_str("    parseltongue pt07-cozodb-code-as-visuals --report complexity\n\n");
    output.push_str("  Coverage Gaps:\n");
    output.push_str("    parseltongue pt07-cozodb-code-as-visuals --report coverage\n\n");

    output.push_str("╚═══════════════════════════════════════════════════════════════════════╝\n");

    Ok(output)
}
```

### 3.3 Table Renderer

**src/renderer/table.rs**:
```rust
use comfy_table::{Table, Cell, ContentArrangement};

pub struct TableRenderer {
    table: Table,
}

impl TableRenderer {
    pub fn new() -> Self {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);
        Self { table }
    }

    pub fn add_header(&mut self, headers: &[&str]) {
        self.table.set_header(headers);
    }

    pub fn add_row(&mut self, cells: &[&str]) {
        self.table.add_row(cells);
    }

    pub fn render(&self) -> String {
        self.table.to_string()
    }
}
```

---

## Phase 4: Advanced Reports (Week 3)

### 4.1 Complexity Report

**src/reports/complexity.rs**:
```rust
use crate::cli::Cli;
use crate::analytics::queries;
use crate::renderer::table::TableRenderer;
use parseltongue_core::storage::CozoDbStorage;
use parseltongue_core::entities::{CodeEntity, ComplexityLevel, RiskLevel};
use anyhow::Result;

pub async fn generate(db: &CozoDbStorage, cli: &Cli) -> Result<String> {
    let mut entities = queries::get_all_entities(db).await?;

    // Sort by complexity score (complex + high risk = highest priority)
    entities.sort_by(|a, b| {
        let score_a = complexity_risk_score(a);
        let score_b = complexity_risk_score(b);
        score_b.partial_cmp(&score_a).unwrap()
    });

    // Limit
    entities.truncate(cli.limit);

    let mut output = String::new();
    output.push_str("COMPLEXITY HOTSPOTS\n");
    output.push_str(&"━".repeat(80));
    output.push_str("\n\n");

    // Table
    let mut renderer = TableRenderer::new();
    renderer.add_header(&[
        "#", "Entity", "Complexity", "Risk", "Coverage", "Action", "Location"
    ]);

    for (i, entity) in entities.iter().enumerate() {
        let action = determine_action(entity);

        renderer.add_row(&[
            &(i + 1).to_string(),
            &entity.interface_signature.name,
            &format!("{:?}", entity.tdd_classification.complexity),
            &format!("{:?}", entity.tdd_classification.change_risk),
            &format!("{:.0}%", entity.tdd_classification.test_coverage_estimate * 100.0),
            action,
            &entity.interface_signature.file_path.file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        ]);
    }

    output.push_str(&renderer.render());
    output.push_str("\n");

    // Recommendations
    output.push_str("\nRECOMMENDATIONS\n\n");
    output.push_str("  Immediate (This Sprint):\n");

    let critical: Vec<_> = entities.iter()
        .filter(|e| is_critical(e))
        .take(3)
        .collect();

    for (i, entity) in critical.iter().enumerate() {
        output.push_str(&format!("    {}. Add tests for {} ({:?}, {:?}, {:.0}%)\n",
            i + 1,
            entity.interface_signature.name,
            entity.tdd_classification.complexity,
            entity.tdd_classification.change_risk,
            entity.tdd_classification.test_coverage_estimate * 100.0
        ));
    }

    Ok(output)
}

fn complexity_risk_score(entity: &CodeEntity) -> f64 {
    let complexity_score = match entity.tdd_classification.complexity {
        ComplexityLevel::Simple => 1.0,
        ComplexityLevel::Moderate => 2.0,
        ComplexityLevel::Complex => 3.0,
    };

    let risk_score = match entity.tdd_classification.change_risk {
        RiskLevel::Low => 1.0,
        RiskLevel::Medium => 2.0,
        RiskLevel::High => 3.0,
    };

    let coverage_penalty = (1.0 - entity.tdd_classification.test_coverage_estimate) * 2.0;

    complexity_score * risk_score + coverage_penalty
}

fn is_critical(entity: &CodeEntity) -> bool {
    let is_complex_or_moderate = matches!(
        entity.tdd_classification.complexity,
        ComplexityLevel::Complex | ComplexityLevel::Moderate
    );
    let is_high_risk = matches!(
        entity.tdd_classification.change_risk,
        RiskLevel::High
    );
    let low_coverage = entity.tdd_classification.test_coverage_estimate < 0.5;

    is_complex_or_moderate && is_high_risk && low_coverage
}

fn determine_action(entity: &CodeEntity) -> &'static str {
    if is_critical(entity) {
        "✗✗ CRIT"
    } else if entity.tdd_classification.test_coverage_estimate < 0.5 {
        "✗ TEST"
    } else if entity.tdd_classification.test_coverage_estimate < 0.7 {
        "⚠ TEST"
    } else if entity.interface_signature.documentation.is_none() {
        "⚠ DOC"
    } else {
        "✓ OK"
    }
}
```

### 4.2 Blast Radius Report

**src/reports/blast_radius.rs**:
```rust
use crate::cli::Cli;
use parseltongue_core::storage::CozoDbStorage;
use anyhow::{Result, bail};

pub async fn generate(db: &CozoDbStorage, cli: &Cli) -> Result<String> {
    let entity_key = cli.entity.as_ref()
        .ok_or_else(|| anyhow::anyhow!("--entity required for blast-radius report"))?;

    // Get entity details
    let entity = db.get_entity(entity_key).await?
        .ok_or_else(|| anyhow::anyhow!("Entity not found: {}", entity_key))?;

    // Calculate blast radius (5 hops)
    let affected = db.calculate_blast_radius(entity_key, 5).await?;

    let mut output = String::new();
    output.push_str("BLAST RADIUS ANALYSIS\n");
    output.push_str(&"━".repeat(80));
    output.push_str("\n\n");

    output.push_str(&format!("Target Entity: {}\n", entity.interface_signature.name));
    output.push_str(&format!("Location:      {}:{}-{}\n",
        entity.interface_signature.file_path.display(),
        entity.interface_signature.line_range.start,
        entity.interface_signature.line_range.end
    ));
    output.push_str(&format!("Type:          {:?}\n", entity.interface_signature.entity_type));
    output.push_str(&format!("Complexity:    {:?}\n", entity.tdd_classification.complexity));
    output.push_str(&format!("Risk:          {:?}\n", entity.tdd_classification.change_risk));
    output.push_str("\n");
    output.push_str(&"━".repeat(80));
    output.push_str("\n\n");

    // Impact summary
    output.push_str("IMPACT SUMMARY\n\n");
    output.push_str(&format!("  Transitive impact:  {} entities (within 5 hops)\n", affected.len()));

    // Count by distance
    let mut distance_counts = std::collections::HashMap::new();
    for (_, distance) in &affected {
        *distance_counts.entry(*distance).or_insert(0) += 1;
    }

    output.push_str("\n  By Distance:\n");
    for dist in 1..=5 {
        if let Some(count) = distance_counts.get(&dist) {
            output.push_str(&format!("    {} hop:  {} entities\n", dist, count));
        }
    }

    output.push_str("\n");
    output.push_str(&"━".repeat(80));
    output.push_str("\n\n");

    // Risk assessment
    let risk_level = if affected.len() > 50 {
        "⚠ HIGH"
    } else if affected.len() > 20 {
        "⚠ MEDIUM"
    } else {
        "✓ LOW"
    };

    output.push_str("RISK ASSESSMENT\n\n");
    output.push_str(&format!("  Overall Risk Level: {}\n\n", risk_level));
    output.push_str(&format!("  Factors:\n"));
    output.push_str(&format!("    • {} entities affected\n", affected.len()));
    output.push_str(&format!("    • {:?} complexity\n", entity.tdd_classification.complexity));
    output.push_str(&format!("    • {:?} risk\n", entity.tdd_classification.change_risk));

    Ok(output)
}
```

---

## Phase 5: Testing & Polish (Week 4)

### 5.1 Integration Tests

**tests/integration_test.rs**:
```rust
use pt07_cozodb_code_as_visuals::cli::Cli;
use pt07_cozodb_code_as_visuals::reports;
use parseltongue_core::storage::CozoDbStorage;
use parseltongue_core::entities::*;

#[tokio::test]
async fn test_dashboard_report() {
    // Create in-memory database
    let db = CozoDbStorage::new("mem").await.unwrap();
    db.create_schema().await.unwrap();

    // Insert test entities
    let entity = create_test_entity();
    db.insert_entity(&entity).await.unwrap();

    // Generate dashboard
    let cli = Cli {
        db: "mem".to_string(),
        report: pt07_cozodb_code_as_visuals::cli::ReportType::Dashboard,
        format: pt07_cozodb_code_as_visuals::cli::OutputFormat::Table,
        filter: None,
        entity: None,
        limit: 20,
        sort: None,
        color: false,
        verbose: false,
    };

    let output = reports::dashboard::generate(&db, &cli).await.unwrap();

    // Assertions
    assert!(output.contains("CODEBASE SNAPSHOT"));
    assert!(output.contains("HEALTH SCORE"));
}

fn create_test_entity() -> CodeEntity {
    // Helper to create test entity
    todo!()
}
```

### 5.2 Unit Tests

**src/analytics/stats.rs** (add tests):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_score_perfect() {
        let stats = CodebaseStats {
            total_entities: 100,
            total_files: 10,
            test_coverage: 0.9,
            avg_complexity: "Simple".to_string(),
            high_risk_count: 2,
            complex_count: 5,
            avg_dependencies: 3.0,
            public_api_count: 30,
            public_api_doc_coverage: 0.95,
        };

        let score = calculate_health_score(&stats);
        assert_eq!(score, 100);
    }

    #[test]
    fn test_health_score_low_coverage() {
        let stats = CodebaseStats {
            total_entities: 100,
            total_files: 10,
            test_coverage: 0.5, // Low coverage
            avg_complexity: "Simple".to_string(),
            high_risk_count: 2,
            complex_count: 5,
            avg_dependencies: 3.0,
            public_api_count: 30,
            public_api_doc_coverage: 0.95,
        };

        let score = calculate_health_score(&stats);
        assert!(score < 80); // Should lose points for low coverage
    }
}
```

---

## Quick Start Checklist

### Week 1: Foundation
- [ ] Create crate structure
- [ ] Add dependencies to Cargo.toml
- [ ] Implement CLI argument parsing
- [ ] Create main.rs entry point
- [ ] Implement analytics/queries.rs (basic queries)
- [ ] Implement analytics/stats.rs (statistics calculation)
- [ ] Implement analytics/filters.rs (filter parsing)
- [ ] Test connection to existing database

### Week 2: Core Reports
- [ ] Implement renderer/table.rs (comfy-table wrapper)
- [ ] Implement reports/entities.rs
- [ ] Implement reports/dashboard.rs
- [ ] Test with real database (parseltongue self-analysis)
- [ ] Add integration tests

### Week 3: Advanced Reports
- [ ] Implement reports/complexity.rs
- [ ] Implement reports/coverage.rs
- [ ] Implement reports/blast_radius.rs
- [ ] Implement reports/dependencies.rs
- [ ] Add JSON output format

### Week 4: Polish
- [ ] Add CSV output format
- [ ] Improve error messages
- [ ] Add --verbose flag support
- [ ] Performance optimization
- [ ] Documentation
- [ ] README with examples

---

## Common Patterns

### Error Handling

```rust
use anyhow::{Result, Context};

pub async fn some_query(db: &CozoDbStorage) -> Result<Vec<CodeEntity>> {
    db.get_all_entities()
        .await
        .context("Failed to fetch entities from database")
}
```

### Optional Features

```rust
// Add to Cargo.toml
[features]
default = []
color = ["owo-colors"]

// Use in code
#[cfg(feature = "color")]
use owo_colors::OwoColorize;

#[cfg(feature = "color")]
fn format_risk(risk: &RiskLevel) -> String {
    match risk {
        RiskLevel::High => format!("{}", "High".red()),
        RiskLevel::Medium => format!("{}", "Medium".yellow()),
        RiskLevel::Low => format!("{}", "Low".green()),
    }
}

#[cfg(not(feature = "color"))]
fn format_risk(risk: &RiskLevel) -> String {
    format!("{:?}", risk)
}
```

---

## Performance Tips

1. **Query Once**: Fetch all entities once, filter in memory
2. **Limit Early**: Apply --limit before rendering
3. **Lazy Rendering**: Don't compute strings until needed
4. **Cache Stats**: Calculate stats once, reuse across reports
5. **Async Where Possible**: Use async/await for DB queries

---

## Debugging Tips

```bash
# Test with verbose output
cargo run -- --report dashboard --db rocksdb:test.db --verbose

# Test with small database
cargo run -- --report entities --db mem --limit 5

# Test filter parsing
cargo run -- --report entities --filter "risk=High,complexity=Complex" --limit 3

# Check JSON output
cargo run -- --report dashboard --format json --db rocksdb:test.db | jq .
```

---

## Next Steps After MVP

1. **Trend Analysis**: Compare snapshots over time
2. **Interactive Mode**: TUI with keyboard navigation
3. **Export Reports**: PDF/HTML generation
4. **Custom Queries**: User-defined Datalog queries
5. **CI Integration**: Exit codes based on thresholds
6. **Web Dashboard**: Optional web UI for team sharing

---

**End of Implementation Guide**

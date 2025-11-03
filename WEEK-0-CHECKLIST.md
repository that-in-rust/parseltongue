# Week 0 Action Checklist (Nov 4-10, 2025)

**Goal**: Validate strategic plan and start Phase 1 development  
**Team**: parseltongue developers + stakeholders  
**Deliverables**: Approved plan + working prototype

---

## Monday, Nov 4, 2025

### Morning (2 hours)
- [ ] **Team Review Meeting** (all stakeholders)
  - [ ] Present Executive Summary (15 min)
  - [ ] Walk through Multi-Tier Architecture diagram (15 min)
  - [ ] Discuss Integration Roadmap phases (30 min)
  - [ ] Q&A and feedback (30 min)
  - [ ] Decision: Approve/modify plan (30 min)

### Afternoon (3 hours)
- [ ] **Set Up Project Tracking**
  - [ ] Create GitHub Project board "CPU Integration"
  - [ ] Create 5 Milestones (Phase 1-5)
  - [ ] Create issues for Phase 1 tasks
    - [ ] Issue #1: Design pt00-metrics-analyzer CLI
    - [ ] Issue #2: Implement scc wrapper
    - [ ] Issue #3: Add --filter-complexity to pt01
    - [ ] Issue #4: Extend pt02-level00 with metrics
    - [ ] Issue #5: Write tests
    - [ ] Issue #6: Documentation
  - [ ] Set up labels: `phase-1`, `metrics`, `testing`, `docs`

---

## Tuesday, Nov 5, 2025

### Morning (4 hours)
- [ ] **Technical Spec: pt00-metrics-analyzer**
  - [ ] CLI interface design (clap args)
  - [ ] Input: directory path
  - [ ] Output: JSON with metrics per file
  - [ ] Error handling (missing scc binary, permission errors)
  - [ ] Integration points with pt01/pt02
  - [ ] Test strategy (unit + integration)
  
  **Example CLI**:
  ```bash
  parseltongue pt00-metrics-analyzer ./src \
    --output metrics.json \
    --filter-complexity 10 \
    --summary
  ```

### Afternoon (3 hours)
- [ ] **Review Tool Research**
  - [ ] Read `.ref/SUMMARY_REPORT.md` (11KB, 10 min)
  - [ ] Read `.ref/TOOLS_CATALOG.md` section on scc (5 min)
  - [ ] Explore scc repository `.ref/tool-scc/` (30 min)
  - [ ] Test scc CLI locally:
    ```bash
    cd .ref/tool-scc
    go build
    ./scc ../../crates/ --format json
    ```
  - [ ] Understand JSON output format
  - [ ] Document findings in `docs/scc-integration-notes.md`

---

## Wednesday, Nov 6, 2025

### Full Day (6-8 hours)
- [ ] **Prototype pt00-metrics-analyzer (Part 1)**
  - [ ] Create crate structure:
    ```bash
    mkdir -p crates/pt00-metrics-analyzer/src
    mkdir -p crates/pt00-metrics-analyzer/tests
    ```
  - [ ] Add to workspace `Cargo.toml`:
    ```toml
    [workspace]
    members = [
        "crates/*",
        "crates/pt00-metrics-analyzer",  # NEW
    ]
    ```
  - [ ] Create `crates/pt00-metrics-analyzer/Cargo.toml`:
    ```toml
    [package]
    name = "pt00-metrics-analyzer"
    version = "0.1.0"
    edition = "2021"
    
    [dependencies]
    anyhow = { workspace = true }
    clap = { workspace = true, features = ["derive"] }
    serde = { workspace = true }
    serde_json = { workspace = true }
    ```
  - [ ] Implement basic CLI (`src/cli.rs`):
    ```rust
    use clap::Parser;
    
    #[derive(Parser, Debug)]
    pub struct Cli {
        /// Directory to analyze
        #[arg(default_value = ".")]
        pub directory: String,
        
        /// Output JSON file path
        #[arg(short, long)]
        pub output: Option<String>,
        
        /// Filter files by minimum complexity
        #[arg(long)]
        pub filter_complexity: Option<u32>,
        
        /// Show summary statistics
        #[arg(long)]
        pub summary: bool,
    }
    ```
  - [ ] Implement scc wrapper (`src/scc_wrapper.rs`):
    ```rust
    use std::process::Command;
    use serde_json::Value;
    
    pub fn run_scc(directory: &str) -> anyhow::Result<Value> {
        let output = Command::new("scc")
            .args(&[directory, "--format", "json"])
            .output()?;
        
        if !output.status.success() {
            anyhow::bail!("scc command failed");
        }
        
        let json = serde_json::from_slice(&output.stdout)?;
        Ok(json)
    }
    ```

---

## Thursday, Nov 7, 2025

### Full Day (6-8 hours)
- [ ] **Prototype pt00-metrics-analyzer (Part 2)**
  - [ ] Implement metrics parsing (`src/metrics.rs`):
    ```rust
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FileMetrics {
        pub path: String,
        pub language: String,
        pub lines: u32,
        pub code: u32,
        pub comments: u32,
        pub blanks: u32,
        pub complexity: u32,
    }
    
    pub fn parse_scc_output(json: Value) -> Vec<FileMetrics> {
        // Parse scc JSON into our FileMetrics struct
        // Handle edge cases (missing fields, null values)
    }
    ```
  - [ ] Implement filtering logic (`src/filter.rs`):
    ```rust
    pub fn filter_by_complexity(
        files: Vec<FileMetrics>,
        min_complexity: u32
    ) -> Vec<FileMetrics> {
        files.into_iter()
            .filter(|f| f.complexity >= min_complexity)
            .collect()
    }
    ```
  - [ ] Implement summary stats (`src/summary.rs`):
    ```rust
    pub struct Summary {
        pub total_files: u32,
        pub total_lines: u32,
        pub avg_complexity: f64,
        pub languages: HashMap<String, u32>,
    }
    
    pub fn calculate_summary(files: &[FileMetrics]) -> Summary {
        // Aggregate statistics
    }
    ```
  - [ ] Wire up main (`src/main.rs`):
    ```rust
    fn main() -> anyhow::Result<()> {
        let cli = Cli::parse();
        
        // Run scc
        let scc_output = run_scc(&cli.directory)?;
        
        // Parse metrics
        let mut metrics = parse_scc_output(scc_output);
        
        // Filter if requested
        if let Some(min) = cli.filter_complexity {
            metrics = filter_by_complexity(metrics, min);
        }
        
        // Show summary if requested
        if cli.summary {
            let summary = calculate_summary(&metrics);
            println!("{:#?}", summary);
        }
        
        // Write output
        if let Some(path) = cli.output {
            write_metrics_json(&metrics, &path)?;
        }
        
        Ok(())
    }
    ```

---

## Friday, Nov 8, 2025

### Morning (3 hours)
- [ ] **Test Prototype**
  - [ ] Test on parseltongue codebase:
    ```bash
    cargo run --bin pt00-metrics-analyzer -- ./crates \
      --output metrics.json \
      --summary
    ```
  - [ ] Expected output:
    - Total files: ~50-100 Rust files
    - Average complexity: ~15-25
    - Filtered (>20): ~30-50% reduction
  - [ ] Verify JSON format is parsable
  - [ ] Test edge cases:
    - Empty directory
    - Directory with no code files
    - Very large directory (10K+ files)
    - Missing scc binary (graceful error)

### Afternoon (3 hours)
- [ ] **Team Demo**
  - [ ] Prepare demo script:
    ```bash
    # Show before/after with filtering
    scc ./crates --format json | jq '.[] | length'  # ALL files
    
    cargo run --bin pt00-metrics-analyzer -- ./crates \
      --filter-complexity 20 \
      --output filtered.json
    
    jq 'length' filtered.json  # Filtered files
    
    # Show summary stats
    cargo run --bin pt00-metrics-analyzer -- ./crates --summary
    ```
  - [ ] Present to team (30 min):
    - [ ] Live demo
    - [ ] Show metrics.json output
    - [ ] Discuss findings (complexity distribution)
    - [ ] Q&A
  - [ ] Collect feedback:
    - [ ] CLI usability
    - [ ] Output format
    - [ ] Performance
    - [ ] Missing features
  - [ ] Update Phase 1 plan based on feedback

### End of Day
- [ ] **Finalize Phase 1 Plan**
  - [ ] Incorporate feedback from demo
  - [ ] Lock requirements for Phase 1
  - [ ] Create detailed task breakdown for Week 1-2
  - [ ] Assign tasks (if multiple devs)
  - [ ] Schedule daily standups for Phase 1

---

## Success Criteria for Week 0

### Must-Have ✅
- [ ] Strategic plan reviewed and approved by team
- [ ] Prototype demonstrates scc integration working
- [ ] Phase 1 plan finalized with locked requirements
- [ ] Project tracking set up (GitHub Projects)
- [ ] Team aligned on goals and timeline

### Nice-to-Have ⭐
- [ ] Prototype includes filtering + summary features
- [ ] Documentation started (`docs/scc-integration-notes.md`)
- [ ] Performance benchmarks collected
- [ ] User feedback positive ("looks promising")

---

## Blockers & Escalations

If any of these occur, escalate immediately:

### Technical Blockers
- [ ] **scc binary not available**: Install via brew/apt/build from source
- [ ] **scc JSON format incompatible**: Parse alternative format or switch to tokei
- [ ] **Performance too slow**: Profile and optimize or defer filtering to pt01

### Process Blockers
- [ ] **Team not aligned**: Schedule additional meeting to resolve concerns
- [ ] **Resources unavailable**: Adjust timeline or reduce scope
- [ ] **Requirements unclear**: Clarify with stakeholders before proceeding

---

## Quick Links

- **Strategic Plan**: `CPU-BASED-ANALYSIS-INTEGRATION-PLAN.md` (70KB, comprehensive)
- **Executive Summary**: `EXECUTIVE-SUMMARY.md` (15KB, high-level)
- **Tool Research**: `.ref/SUMMARY_REPORT.md`, `.ref/TOOLS_CATALOG.md`
- **scc Repository**: `.ref/tool-scc/`
- **Project Board**: (Create on Monday)

---

## Daily Standup Questions (Starting Nov 11)

Each morning, answer:
1. **What did I accomplish yesterday?**
2. **What will I work on today?**
3. **Any blockers?**

Example for Monday Nov 11:
1. ✅ Finished prototype, team demo went well
2. 🎯 Implement pt00 tests, extend pt01 CLI
3. ❌ No blockers

---

## Week 1 Preview (Nov 11-15)

Once Week 0 complete, Week 1 tasks:
- [ ] Add comprehensive tests to pt00
- [ ] Extend pt01 with `--filter-complexity` flag
- [ ] Integration testing: pt00 → pt01 pipeline
- [ ] Write "Metrics-First Analysis Guide" docs

**Target**: pt00 ready for PR review by Friday Nov 15

---

**Status**: 📋 READY TO START  
**Next Action**: Monday morning team review meeting  
**Owner**: parseltongue team lead

🚀 Let's ship Phase 1!

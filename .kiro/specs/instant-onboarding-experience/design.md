# Design Document: Instant Onboarding Experience

## Overview

The Instant Onboarding Experience creates a "curl moment" for parseltongue adoption by providing immediate visual architectural insights through GitHub-hosted scripts. This design addresses two distinct user segments with different time constraints and jobs-to-be-done:

1. **Tool Users**: Developers evaluating parseltongue for their codebases (2-minute visual wow)
2. **Tool Contributors**: Developers wanting to contribute to parseltongue itself (60-second architecture understanding)

The solution transforms the traditional 20+ minute onboarding process into a 2-minute visual experience that opens an interactive HTML architecture map in the user's browser.

## Architecture

### High-Level System Architecture

```mermaid
graph TB
    subgraph "GitHub Distribution"
        GH[GitHub Releases API]
        IS[instant-analyze.sh]
        SA[self-analyze.sh]
    end
    
    subgraph "User Environment"
        UC[User's Codebase]
        PT[Parseltongue Binary]
        HTML[Interactive HTML]
        BR[Browser]
    end
    
    subgraph "Analysis Pipeline"
        GD[Git Detection]
        PD[Platform Detection]
        BD[Binary Download]
        CA[Codebase Analysis]
        HG[HTML Generation]
        BL[Browser Launch]
    end
    
    UC -->|curl command| IS
    IS --> GD
    GD --> PD
    PD --> GH
    GH --> BD
    BD --> PT
    PT --> CA
    CA --> HG
    HG --> HTML
    HTML --> BL
    BL --> BR
```

### Component Architecture

```mermaid
graph LR
    subgraph "Curl Scripts Layer"
        IA[instant-analyze.sh]
        SA[self-analyze.sh]
    end
    
    subgraph "Detection Layer"
        GIT[Git Validator]
        PLAT[Platform Detector]
        RUST[Rust Project Validator]
    end
    
    subgraph "Distribution Layer"
        API[GitHub API Client]
        DL[Binary Downloader]
        VER[Checksum Verifier]
    end
    
    subgraph "Analysis Layer"
        PE[Parseltongue Engine]
        GI[GitIngest Parser]
        EH[Entity Highlighter]
    end
    
    subgraph "Visualization Layer"
        HG[HTML Generator]
        D3[D3.js Integration]
        IG[Interactive Graph]
        SB[Sidebar Generator]
    end
    
    IA --> GIT
    SA --> GIT
    GIT --> PLAT
    PLAT --> API
    API --> DL
    DL --> PE
    PE --> EH
    EH --> HG
    HG --> IG
```

## Components and Interfaces

### 1. GitHub-Hosted Curl Scripts

#### instant-analyze.sh
**Purpose**: Entry point for tool users analyzing their own codebases
**Interface**:
```bash
# Standard usage
curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/instant-analyze.sh | bash

# GitIngest format
curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/instant-analyze.sh | bash -s -- --gitingest mycode.txt
```

**Responsibilities**:
- Git repository detection and validation
- Rust project validation
- Platform detection (macOS Intel/ARM, Linux x86_64)
- Binary download orchestration
- Analysis execution
- HTML generation and browser launch
- Cleanup management

#### self-analyze.sh
**Purpose**: Entry point for contributors analyzing parseltongue itself
**Interface**:
```bash
curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/self-analyze.sh | bash
```

**Responsibilities**:
- Parseltongue repository detection
- Contributor-focused analysis configuration
- Architecture visualization with contribution opportunities
- Development setup guidance

### 2. Platform Detection Module

**Interface**:
```bash
detect_platform() -> (os_type, arch_type, binary_name)
```

**Supported Platforms**:
- Linux x86_64: `parseltongue-linux-x86_64`
- macOS Intel: `parseltongue-macos-intel`
- macOS ARM: `parseltongue-macos-arm64`

**Design Rationale**: Focus on primary developer platforms (macOS/Linux) to maximize adoption while keeping complexity manageable.

### 3. GitHub API Integration

**Interface**:
```bash
get_latest_release() -> release_info
download_binary(platform, target_dir) -> binary_path
verify_checksum(binary_path, expected_checksum) -> bool
```

**Design Decisions**:
- Use GitHub's public API (no authentication required)
- Implement retry logic for network failures
- Verify binary integrity with checksums
- Handle rate limiting gracefully

### 4. Analysis Engine Integration

**Interface**:
```bash
parseltongue analyze --output-format html --instant-mode [--self-analysis] <target>
```

**New Command Line Options**:
- `--output-format html`: Generate self-contained HTML visualization
- `--instant-mode`: Optimize for speed over completeness
- `--self-analysis`: Enable contributor-focused analysis mode

**Design Rationale**: Extend existing parseltongue CLI rather than creating separate tools to maintain consistency and leverage existing analysis capabilities.

### 5. Interactive HTML Generator

**Architecture**:
```mermaid
graph TB
    subgraph "HTML Structure"
        HEAD[HTML Head + Embedded CSS/JS]
        HEADER[Analysis Header + Stats]
        MAIN[Main Container]
    end
    
    subgraph "Main Container"
        GRAPH[D3.js Force Graph]
        SIDEBAR[Interactive Sidebar]
    end
    
    subgraph "Sidebar Components"
        SEARCH[Entity Search]
        INSIGHTS[Key Insights]
        STATS[Architecture Stats]
        ACTIONS[Next Steps]
    end
    
    HEAD --> HEADER
    HEADER --> MAIN
    MAIN --> GRAPH
    MAIN --> SIDEBAR
    SIDEBAR --> SEARCH
    SIDEBAR --> INSIGHTS
    SIDEBAR --> STATS
    SIDEBAR --> ACTIONS
```

**Interface**:
```rust
pub struct HtmlGenerator {
    pub fn generate_interactive_html(
        analysis_result: &AnalysisResult,
        mode: VisualizationMode,
    ) -> Result<String, HtmlGenerationError>
}

pub enum VisualizationMode {
    ToolUser,
    Contributor,
}
```

## Data Models

### Analysis Result Structure

```rust
#[derive(Debug, Serialize)]
pub struct InstantAnalysisResult {
    pub metadata: AnalysisMetadata,
    pub entities: Vec<EntityInfo>,
    pub relationships: Vec<RelationshipInfo>,
    pub key_insights: KeyInsights,
    pub architecture_patterns: Vec<ArchitecturePattern>,
}

#[derive(Debug, Serialize)]
pub struct AnalysisMetadata {
    pub codebase_name: String,
    pub analysis_duration: Duration,
    pub file_count: usize,
    pub entity_count: usize,
    pub relationship_count: usize,
    pub analysis_mode: AnalysisMode,
}

#[derive(Debug, Serialize)]
pub struct EntityInfo {
    pub id: String,
    pub name: String,
    pub entity_type: EntityType,
    pub file_path: String,
    pub line_number: Option<usize>,
    pub relationship_count: usize,
    pub centrality_score: f64,
    pub is_key_entity: bool,
}

#[derive(Debug, Serialize)]
pub struct KeyInsights {
    pub entry_points: Vec<EntityInfo>,
    pub most_connected: Vec<EntityInfo>,
    pub architecture_patterns: Vec<String>,
    pub contribution_opportunities: Option<Vec<ContributionOpportunity>>,
}

#[derive(Debug, Serialize)]
pub struct ContributionOpportunity {
    pub area: String,
    pub description: String,
    pub file_path: String,
    pub difficulty: ContributionDifficulty,
}
```

### HTML Visualization Data

```typescript
interface VisualizationData {
    nodes: Node[];
    links: Link[];
    metadata: AnalysisMetadata;
    insights: KeyInsights;
}

interface Node {
    id: string;
    name: string;
    type: EntityType;
    size: number;
    color: string;
    filePath: string;
    relationshipCount: number;
    isKeyEntity: boolean;
}

interface Link {
    source: string;
    target: string;
    type: RelationshipType;
    strength: number;
}
```

## Error Handling

### Comprehensive Error Hierarchy

```rust
#[derive(Error, Debug)]
pub enum InstantOnboardingError {
    #[error("Git repository not found in current directory")]
    NoGitRepository,
    
    #[error("Not a Rust project: {reason}")]
    NotRustProject { reason: String },
    
    #[error("Unsupported platform: {platform}")]
    UnsupportedPlatform { platform: String },
    
    #[error("GitHub API error: {status} - {message}")]
    GitHubApi { status: u16, message: String },
    
    #[error("Binary download failed: {url} - {cause}")]
    DownloadFailed { url: String, cause: String },
    
    #[error("Checksum verification failed: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    
    #[error("Analysis failed: {cause}")]
    AnalysisFailed { cause: String },
    
    #[error("HTML generation failed: {cause}")]
    HtmlGenerationFailed { cause: String },
    
    #[error("Browser launch failed: {cause}")]
    BrowserLaunchFailed { cause: String },
    
    #[error("GitIngest parsing failed: {line} - {cause}")]
    GitIngestParsing { line: usize, cause: String },
}
```

### Error Recovery Strategies

```mermaid
graph TD
    START[Start Analysis] --> GIT{Git Repo?}
    GIT -->|No| GITFAIL[Show Git Setup Instructions]
    GIT -->|Yes| RUST{Rust Project?}
    RUST -->|No| RUSTFAIL[Show Rust Project Requirements]
    RUST -->|Yes| PLATFORM{Supported Platform?}
    PLATFORM -->|No| PLATFAIL[Show Manual Installation]
    PLATFORM -->|Yes| DOWNLOAD{Download Success?}
    DOWNLOAD -->|No| FALLBACK[Try Alternative Download]
    DOWNLOAD -->|Yes| ANALYZE{Analysis Success?}
    ANALYZE -->|No| TERMINAL[Fall back to Terminal Output]
    ANALYZE -->|Yes| HTML{HTML Generation?}
    HTML -->|No| TERMINAL
    HTML -->|Yes| BROWSER{Browser Launch?}
    BROWSER -->|No| SHOWPATH[Show HTML File Path]
    BROWSER -->|Yes| SUCCESS[Complete Success]
```

## Testing Strategy

### Test Categories

#### 1. Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_detection() {
        // Test platform detection logic
    }
    
    #[test]
    fn test_github_api_parsing() {
        // Test GitHub API response parsing
    }
    
    #[test]
    fn test_html_generation() {
        // Test HTML generation with mock data
    }
    
    #[tokio::test]
    async fn test_gitingest_parsing() {
        // Test GitIngest format parsing
    }
}
```

#### 2. Integration Tests
```rust
#[tokio::test]
async fn test_end_to_end_analysis() {
    // Test complete analysis pipeline with test repository
}

#[tokio::test]
async fn test_github_api_integration() {
    // Test actual GitHub API calls (with rate limiting)
}

#[test]
fn test_cross_platform_compatibility() {
    // Test platform detection and binary selection
}
```

#### 3. Performance Tests
```rust
#[tokio::test]
async fn test_analysis_speed_contract() {
    let start = Instant::now();
    let result = run_instant_analysis(test_codebase).await.unwrap();
    let elapsed = start.elapsed();
    
    // Tool users: <2 minutes
    assert!(elapsed < Duration::from_secs(120));
    
    // Contributors: <60 seconds for self-analysis
    if result.mode == AnalysisMode::SelfAnalysis {
        assert!(elapsed < Duration::from_secs(60));
    }
}
```

#### 4. Browser Compatibility Tests
```javascript
// Test HTML compatibility across browsers
describe('Interactive Visualization', () => {
    test('D3.js graph renders correctly', () => {
        // Test graph rendering
    });
    
    test('Sidebar interactions work', () => {
        // Test search, filtering, navigation
    });
    
    test('Responsive design works', () => {
        // Test different screen sizes
    });
});
```

### Test Data Strategy

```rust
pub struct TestDataGenerator;

impl TestDataGenerator {
    pub fn create_test_rust_project() -> TempDir {
        // Generate realistic Rust project structure
    }
    
    pub fn create_gitingest_file() -> String {
        // Generate GitIngest format test data
    }
    
    pub fn create_parseltongue_self_analysis() -> AnalysisResult {
        // Generate self-analysis test data
    }
}
```

## Implementation Phases

### Phase 1: Core Infrastructure (Week 1-2)
**Goal**: Basic curl script functionality with binary download

**Components**:
- Platform detection logic
- GitHub API integration
- Binary download and verification
- Basic error handling and fallbacks

**Success Criteria**:
- Scripts successfully download correct binary for platform
- Graceful handling of network failures
- Clear error messages for unsupported scenarios

### Phase 2: Analysis Integration (Week 3-4)
**Goal**: Integration with parseltongue analysis engine

**Components**:
- CLI flag additions (`--output-format html`, `--instant-mode`)
- Git repository validation
- Rust project detection
- GitIngest format parsing
- Basic HTML output generation

**Success Criteria**:
- Analysis completes within time constraints
- HTML output contains structured data
- GitIngest format properly parsed

### Phase 3: Interactive Visualization (Week 5-6)
**Goal**: Rich HTML visualization with D3.js

**Components**:
- Self-contained HTML with embedded D3.js
- Force-directed graph layout
- Interactive sidebar with search and insights
- Entity highlighting and categorization
- Browser auto-launch functionality

**Success Criteria**:
- Interactive graph renders correctly across browsers
- Visual "wow factor" achieved
- Performance meets user experience goals

### Phase 4: Polish and Optimization (Week 7-8)
**Goal**: Production-ready experience with comprehensive testing

**Components**:
- Cross-platform testing and refinement
- Performance optimization for large codebases
- Comprehensive error handling
- Documentation and examples
- Contributor-focused self-analysis mode

**Success Criteria**:
- All test scenarios pass
- Performance contracts met
- User feedback validates "wow factor"
- Ready for public release

## Design Decisions and Rationales

### 1. GitHub-Hosted Distribution
**Decision**: Use GitHub raw URLs instead of custom domain
**Rationale**: 
- Zero infrastructure cost and maintenance
- Leverages existing GitHub reliability
- Familiar pattern for developers
- Automatic HTTPS and global CDN

### 2. Self-Contained HTML Output
**Decision**: Embed all dependencies (D3.js, CSS) in single HTML file
**Rationale**:
- Works offline after generation
- Easy to share with team members
- No external dependencies or CDN failures
- Consistent experience across environments

### 3. Platform-Specific Binaries
**Decision**: Pre-built binaries for macOS/Linux only
**Rationale**:
- Target primary developer platforms
- Avoid Windows complexity for MVP
- Faster download and execution
- Consistent user experience

### 4. Force-Directed Graph Visualization
**Decision**: Use D3.js force-directed layout for architecture map
**Rationale**:
- Intuitive representation of relationships
- Interactive exploration capabilities
- Proven pattern for code visualization
- Scales well with entity count

### 5. Dual-Mode Analysis
**Decision**: Separate modes for tool users vs contributors
**Rationale**:
- Different time constraints (2min vs 60sec)
- Different information needs
- Tailored insights and next steps
- Optimized user experience per segment

### 6. GitIngest Format Support
**Decision**: Support GitIngest as alternative input method
**Rationale**:
- Enables analysis without Git repository
- Supports shared code samples
- Flexible input for various workflows
- Maintains same visual output quality

### 7. Instant Mode Optimization
**Decision**: Trade analysis completeness for speed
**Rationale**:
- Prioritize "wow factor" over comprehensive analysis
- Users can run full analysis after being convinced
- Focus on architectural insights, not exhaustive details
- Maintain confidence-building accuracy

## Security Considerations

### 1. Binary Verification
- Checksum validation for all downloaded binaries
- HTTPS-only downloads from GitHub
- Clear error messages for verification failures

### 2. Input Validation
- Git repository validation before analysis
- File size limits for GitIngest format
- Sanitization of user-provided paths

### 3. Execution Safety
- No arbitrary code execution from downloaded content
- Temporary file cleanup after analysis
- Clear permission requirements for binary execution

### 4. Privacy Protection
- No data transmission to external services
- Local-only analysis and HTML generation
- User control over generated file sharing

## Performance Contracts

### Time Constraints
- **Tool Users**: Complete analysis and browser launch <2 minutes
- **Contributors**: Self-analysis and visualization <60 seconds
- **HTML Generation**: <10 seconds for typical codebase
- **Binary Download**: <30 seconds on typical connection

### Resource Limits
- **Memory Usage**: <500MB during analysis
- **Disk Usage**: <100MB for binary and temporary files
- **HTML File Size**: <5MB for self-contained visualization
- **GitIngest Support**: Files up to 50MB

### Scalability Targets
- **Codebase Size**: Up to 1000 Rust files
- **Entity Count**: Up to 10,000 entities in visualization
- **Relationship Count**: Up to 50,000 relationships
- **Browser Performance**: Smooth interaction with 500+ nodes

## LLM Integration Workflows

### Terminal Command Integration

The instant onboarding experience should provide LLM-friendly terminal commands that can be easily referenced and executed programmatically.

#### Core Command Structure
```bash
# Primary instant analysis command
parseltongue instant [OPTIONS] [TARGET]

# Subcommands for different workflows
parseltongue instant analyze [--format=html|json|terminal] [--mode=user|contributor] [TARGET]
parseltongue instant map [--interactive] [--export=PATH] [TARGET]
parseltongue instant context [--entity=NAME] [--depth=N] [TARGET]
parseltongue instant workflow [--list|--run=NAME] [TARGET]
```

#### LLM Workflow Commands

```bash
# 1. Quick Architecture Overview (LLM Context Building)
parseltongue instant overview --format=json --compact
# Output: Structured JSON with key entities, patterns, entry points

# 2. Entity Context Extraction (For LLM Code Understanding)
parseltongue instant context --entity="DiscoveryEngine" --depth=2 --format=json
# Output: Entity relationships, dependencies, usage patterns

# 3. Pattern Detection (For LLM Architecture Analysis)
parseltongue instant patterns --list --confidence=high --format=json
# Output: Detected architectural patterns with confidence scores

# 4. Contribution Guidance (For LLM Development Assistance)
parseltongue instant contribute --area=performance --difficulty=beginner --format=json
# Output: Specific contribution opportunities with file locations

# 5. Code Navigation (For LLM File Discovery)
parseltongue instant navigate --from="main" --to="ISGNode" --format=json
# Output: Code paths and relationship chains between entities

# 6. Blast Radius Analysis (For LLM Impact Assessment)
parseltongue instant impact --entity="SigHash" --depth=3 --format=json
# Output: What would be affected by changes to specific entities
```

### LLM Integration Architecture

```mermaid
graph TB
    subgraph "LLM Interaction Layer"
        LLM[LLM Agent]
        CMD[Command Interface]
        JSON[JSON Output Parser]
    end
    
    subgraph "Instant Analysis Engine"
        CORE[Core Analysis]
        CTX[Context Extractor]
        PAT[Pattern Detector]
        NAV[Navigation Engine]
        IMP[Impact Analyzer]
    end
    
    subgraph "Output Formatters"
        TERM[Terminal Formatter]
        JFMT[JSON Formatter]
        HTML[HTML Formatter]
        MD[Markdown Formatter]
    end
    
    LLM --> CMD
    CMD --> CORE
    CORE --> CTX
    CORE --> PAT
    CORE --> NAV
    CORE --> IMP
    
    CTX --> JSON
    PAT --> JSON
    NAV --> JSON
    IMP --> JSON
    
    JSON --> TERM
    JSON --> JFMT
    JSON --> HTML
    JSON --> MD
```

### Workflow Templates for LLMs

#### 1. Codebase Onboarding Workflow
```bash
#!/bin/bash
# LLM Codebase Onboarding Script

echo "🔍 Analyzing codebase architecture..."
parseltongue instant overview --format=json > architecture.json

echo "🎯 Identifying key entry points..."
parseltongue instant context --entity="main" --depth=1 --format=json > entry_points.json

echo "🏗️ Detecting architectural patterns..."
parseltongue instant patterns --list --format=json > patterns.json

echo "📊 Generating visual map..."
parseltongue instant map --interactive --export=architecture.html

echo "✅ Onboarding complete! Key files:"
echo "  - architecture.json: Overall structure"
echo "  - entry_points.json: Where to start reading"
echo "  - patterns.json: Architectural insights"
echo "  - architecture.html: Interactive visualization"
```

#### 2. Contribution Discovery Workflow
```bash
#!/bin/bash
# LLM Contribution Discovery Script

echo "🔍 Finding contribution opportunities..."
parseltongue instant contribute --difficulty=beginner --format=json > beginner_tasks.json
parseltongue instant contribute --difficulty=intermediate --format=json > intermediate_tasks.json

echo "🎯 Analyzing extension points..."
parseltongue instant context --entity="DiscoveryEngine" --depth=2 --format=json > extension_points.json

echo "📈 Identifying performance opportunities..."
parseltongue instant patterns --type=performance --format=json > performance_opportunities.json

echo "✅ Contribution analysis complete!"
```

#### 3. Impact Analysis Workflow
```bash
#!/bin/bash
# LLM Impact Analysis Script for Changes

ENTITY=${1:-"ISGNode"}
echo "🔍 Analyzing impact of changes to: $ENTITY"

parseltongue instant impact --entity="$ENTITY" --depth=2 --format=json > impact_direct.json
parseltongue instant impact --entity="$ENTITY" --depth=3 --format=json > impact_extended.json

echo "🎯 Finding dependent entities..."
parseltongue instant navigate --to="$ENTITY" --format=json > dependents.json

echo "📊 Generating impact visualization..."
parseltongue instant map --focus="$ENTITY" --export=impact_map.html

echo "✅ Impact analysis complete for: $ENTITY"
```

### JSON Output Schemas for LLM Consumption

#### Architecture Overview Schema
```json
{
  "metadata": {
    "codebase_name": "string",
    "analysis_timestamp": "ISO8601",
    "entity_count": "number",
    "file_count": "number",
    "analysis_duration_ms": "number"
  },
  "key_entities": [
    {
      "name": "string",
      "type": "function|struct|trait|module",
      "file_path": "string",
      "line_number": "number",
      "centrality_score": "number",
      "relationship_count": "number",
      "description": "string"
    }
  ],
  "entry_points": [
    {
      "name": "string",
      "file_path": "string",
      "type": "main|public_api|trait_impl",
      "description": "string"
    }
  ],
  "architectural_patterns": [
    {
      "pattern": "string",
      "confidence": "number",
      "examples": ["string"],
      "description": "string"
    }
  ],
  "next_steps": {
    "for_understanding": ["string"],
    "for_contribution": ["string"],
    "for_extension": ["string"]
  }
}
```

#### Entity Context Schema
```json
{
  "entity": {
    "name": "string",
    "type": "string",
    "file_path": "string",
    "line_number": "number",
    "signature": "string",
    "documentation": "string"
  },
  "relationships": {
    "depends_on": [
      {
        "name": "string",
        "type": "string",
        "relationship_type": "uses|implements|extends|calls",
        "file_path": "string"
      }
    ],
    "used_by": [
      {
        "name": "string",
        "type": "string",
        "relationship_type": "string",
        "file_path": "string"
      }
    ]
  },
  "context": {
    "module_path": "string",
    "visibility": "public|private|crate",
    "traits_implemented": ["string"],
    "generic_parameters": ["string"]
  },
  "usage_examples": [
    {
      "file_path": "string",
      "line_number": "number",
      "code_snippet": "string",
      "context": "string"
    }
  ]
}
```

### LLM Command Reference

#### Quick Reference Card
```bash
# Essential LLM Commands for Parseltongue Integration

# 1. Get architectural overview (start here)
parseltongue instant overview --format=json

# 2. Understand specific entity
parseltongue instant context --entity="EntityName" --format=json

# 3. Find entry points
parseltongue instant context --entity="main" --depth=1 --format=json

# 4. Discover patterns
parseltongue instant patterns --list --format=json

# 5. Find contribution opportunities
parseltongue instant contribute --difficulty=beginner --format=json

# 6. Analyze change impact
parseltongue instant impact --entity="EntityName" --format=json

# 7. Navigate code relationships
parseltongue instant navigate --from="A" --to="B" --format=json

# 8. Generate visual map
parseltongue instant map --interactive --export=map.html

# 9. Export for sharing
parseltongue instant overview --format=markdown > README_ARCHITECTURE.md

# 10. Validate understanding
parseltongue instant validate --entity="EntityName" --expected-type="struct"
```

### Integration with Development Workflows

#### Git Hook Integration
```bash
# .git/hooks/post-checkout
#!/bin/bash
# Auto-generate architecture context for LLMs after branch switches

if [ $3 == 1 ]; then  # Branch checkout
    echo "🔄 Branch switched, updating architecture context..."
    parseltongue instant overview --format=json > .parseltongue/current_architecture.json
    parseltongue instant patterns --format=json > .parseltongue/current_patterns.json
    echo "✅ Architecture context updated for LLMs"
fi
```

#### IDE Integration Patterns
```bash
# VS Code task.json integration
{
  "label": "Update Parseltongue Context",
  "type": "shell",
  "command": "parseltongue",
  "args": ["instant", "overview", "--format=json"],
  "group": "build",
  "presentation": {
    "echo": true,
    "reveal": "silent",
    "focus": false,
    "panel": "shared"
  },
  "problemMatcher": []
}
```

#### CI/CD Integration
```yaml
# GitHub Actions workflow
name: Update Architecture Documentation
on:
  push:
    branches: [main]
    
jobs:
  update-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Parseltongue
        run: |
          curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/instant-analyze.sh | bash
          sudo mv parseltongue /usr/local/bin/
      - name: Generate Architecture Documentation
        run: |
          parseltongue instant overview --format=markdown > docs/ARCHITECTURE.md
          parseltongue instant contribute --format=markdown > docs/CONTRIBUTING_OPPORTUNITIES.md
      - name: Commit Documentation
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "GitHub Action"
          git add docs/
          git commit -m "Auto-update architecture documentation" || exit 0
          git push
```

## Success Metrics

### Primary Metrics
- **Time-to-Visual-Wow**: <2 minutes from curl to browser opening
- **Exploration Engagement**: >3 minutes spent in HTML visualization
- **Conversion Rate**: >30% proceed to full onboarding after instant analysis
- **Cross-Platform Success**: >95% success rate on macOS/Linux
- **LLM Integration Success**: >80% of LLM workflows complete successfully

### Quality Metrics
- **Analysis Accuracy**: Correctly identifies key architectural patterns
- **Visual Impact**: Users report "impressive" or "wow" reactions
- **Sharing Behavior**: >20% share generated HTML with team members
- **Error Recovery**: <5% of users encounter unrecoverable errors
- **LLM Usability**: JSON outputs parse correctly in >99% of cases

### Technical Metrics
- **Performance Compliance**: 100% of analyses meet time constraints
- **Browser Compatibility**: Works in Chrome, Firefox, Safari
- **Platform Coverage**: Supports all target platforms
- **Reliability**: <1% failure rate for supported scenarios
- **Command Consistency**: All LLM commands follow consistent patterns
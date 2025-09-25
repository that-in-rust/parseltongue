# Requirements Document

## Introduction

The **Instant Onboarding Experience** addresses the critical first-impression gap in parseltongue adoption. While parseltongue has exceptional technical capabilities and comprehensive workflows, we have **two distinct user segments** with fundamentally different jobs-to-be-done and time constraints.

## User Segmentation (Shreyas Doshi Framework)

### Segment 1: **Parseltongue Users** (Tool Adopters)
**Who**: Rust developers who want to use parseltongue to understand codebases
**JTBD**: "Help me understand and navigate this complex Rust codebase I need to work with"
**Time Constraint**: 2-3 minutes to see value, then willing to invest 15+ minutes for full setup
**Success Metric**: Adoption and daily usage of parseltongue for their work
**Pain Point**: "I don't know if this tool will actually help with MY specific codebase"

### Segment 2: **Parseltongue Contributors** (Tool Builders)
**Who**: Developers who want to contribute to, extend, or rebuild parseltongue itself
**JTBD**: "Help me understand parseltongue's architecture so I can contribute or build similar tools"
**Time Constraint**: 30-60 seconds to assess if the codebase is approachable for contribution
**Success Metric**: Successful contributions to parseltongue, or successful rebuilds/forks
**Pain Point**: "I want to contribute but the codebase is complex and I don't know where to start"

## The Dual Value Proposition

**For Tool Users**: "See if parseltongue understands YOUR codebase in 2 minutes"
**For Tool Contributors**: "Understand parseltongue's architecture and find contribution opportunities in 60 seconds"

**The Shreyas Doshi Insight**: These segments have different time tolerances and different "wow moments":
- **Users** need proof parseltongue works on their specific code → `parseltongue instant-map .`
- **Contributors** need rapid understanding of parseltongue itself → `parseltongue instant-map . --self-analysis`

**Current Problem**: Even with excellent onboarding scripts, both segments face barriers:

**Parseltongue Users**:
1. Clone the repository
2. Build the binary (2+ minutes)
3. Run onboarding scripts (15+ minutes)
4. Navigate through multiple output files

**Parseltongue Contributors**:
1. Clone parseltongue repository
2. Spend 30+ minutes reading docs and exploring code structure
3. Try to understand the architecture before contributing
4. Often abandon due to complexity of the discovery engine and ISG system

**The Gap**: There's no "curl https://api.github.com/repos/user/repo | parseltongue instant-map" equivalent that gives immediate architectural insight for either segment.

## The Vision: The Ultimate "Curl Moment"

### Track 1: Tool Users (Zero-Setup Instant Analysis)
**The Dream Experience**:
```bash
# User sits in their Rust project root
curl -sSL https://parseltongue.dev/instant | bash
# ✅ Downloads binary, analyzes THEIR codebase, shows results in <2 minutes
# ✅ Zero setup, zero configuration, instant architectural insights
# ✅ Demonstrates parseltongue "gets" their specific code immediately
```

### Track 2: Tool Contributors (Self-Analysis via Curl)
**The Dream Experience**:
```bash
# From parseltongue repository root
curl -sSL https://parseltongue.dev/self-analysis | bash
# ✅ Downloads and runs self-analysis in <60 seconds
# ✅ Shows parseltongue's architecture and contribution opportunities
# ✅ Immediate understanding of how to contribute or rebuild
```

### Unified Success Criteria
- **Tool Users**: See meaningful insights about their specific codebase in <2 minutes
- **Tool Contributors**: Understand parseltongue's architecture and contribution opportunities in <60 seconds
- **Both Segments**: Output that immediately demonstrates deep Rust code understanding

## Requirements

### REQ-INSTANT-001.0: Curl-to-Analysis Pipeline (Tool User Segment)

**User Story:** As a Rust developer evaluating parseltongue, I want to run a single curl command from my project directory that downloads and runs architectural analysis, so that I can see immediate value without any setup or installation steps.

#### Acceptance Criteria

1. WHEN I run `curl -sSL https://parseltongue.dev/instant | bash` from my Rust project root THEN the system SHALL download the binary and analyze my codebase in under 2 minutes
2. WHEN the curl script runs THEN it SHALL detect the current directory as a Rust project and automatically run instant analysis
3. WHEN analysis completes THEN the system SHALL display a terminal-based architectural overview of MY specific codebase
4. WHEN displaying results THEN the system SHALL show entity counts, key patterns, and entry points specific to my project
5. WHEN the script finishes THEN it SHALL offer to keep the binary for future use or clean up automatically
6. WHEN errors occur THEN the system SHALL provide clear diagnostics and fallback options

### REQ-INSTANT-002.0: Self-Analysis Curl Pipeline (Tool Contributor Segment)

**User Story:** As a developer interested in contributing to parseltongue, I want to run a curl command that downloads and runs self-analysis on the parseltongue codebase, so that I can quickly understand the architecture and identify contribution opportunities.

#### Acceptance Criteria

1. WHEN I run `curl -sSL https://parseltongue.dev/self-analysis | bash` from the parseltongue repository root THEN the system SHALL download and run self-analysis in under 60 seconds
2. WHEN the self-analysis script runs THEN it SHALL automatically detect this is the parseltongue codebase and run specialized contributor analysis
3. WHEN analysis completes THEN the system SHALL display parseltongue's architecture with contribution-focused insights
4. WHEN showing results THEN the system SHALL highlight extension points, key components, and areas suitable for contribution
5. WHEN displaying architecture THEN the system SHALL explain core concepts (ISG, SigHash, DiscoveryEngine) in contributor-friendly terms
6. WHEN successful THEN the system SHALL provide specific next steps for development setup and first contributions

### REQ-INSTANT-003.0: Visual Architecture Map

**User Story:** As a developer who learns visually, I want an immediate graphical representation of my codebase architecture, so that I can quickly grasp the overall structure and key relationships.

#### Acceptance Criteria

1. WHEN I run `parseltongue instant-map --visual <directory>` THEN the system SHALL generate a simple architectural diagram
2. WHEN creating visuals THEN the system SHALL use ASCII art or simple text-based diagrams that display in any terminal
3. WHEN showing architecture THEN the system SHALL highlight module relationships, key traits, and main entry points
4. WHEN displaying diagrams THEN the system SHALL keep complexity manageable (max 20-30 nodes for instant view)
5. WHEN visual output is generated THEN the system SHALL include entity counts and relationship statistics
6. WHEN I add `--save-html` THEN the system SHALL generate a shareable HTML visualization for team communication

### REQ-INSTANT-004.0: Smart Entity Highlighting

**User Story:** As a developer trying to understand a new codebase, I want the instant analysis to automatically identify and highlight the most important entities, so that I can focus on the architectural elements that matter most.

#### Acceptance Criteria

1. WHEN performing instant analysis THEN the system SHALL identify "key entities" using heuristics (main functions, public APIs, core traits)
2. WHEN highlighting entities THEN the system SHALL prioritize based on centrality (high in-degree/out-degree in the relationship graph)
3. WHEN displaying results THEN the system SHALL show top 10-15 most important entities with brief descriptions
4. WHEN entities are identified THEN the system SHALL categorize them (Entry Points, Core APIs, Data Models, Utilities)
5. WHEN showing key entities THEN the system SHALL include file locations for immediate navigation
6. WHEN analysis finds patterns THEN the system SHALL identify common Rust patterns (Builder, State Machine, Service Layer)

### REQ-INSTANT-005.0: Confidence Building Output

**User Story:** As a developer skeptical about code analysis tools, I want the instant analysis to demonstrate deep understanding of my specific codebase, so that I gain confidence this tool actually "gets" Rust code structure.

#### Acceptance Criteria

1. WHEN displaying results THEN the system SHALL show specific, accurate insights about the analyzed codebase (not generic output)
2. WHEN analysis completes THEN the system SHALL demonstrate understanding by identifying actual function names, trait implementations, and module structure
3. WHEN showing insights THEN the system SHALL include "proof points" like exact entity counts, relationship statistics, and pattern detection
4. WHEN output is generated THEN the system SHALL avoid generic language and show specific architectural facts
5. WHEN displaying results THEN the system SHALL include performance metrics (analysis time, entities processed) to demonstrate capability
6. WHEN successful THEN the system SHALL provide clear next steps for deeper analysis with specific command examples

### REQ-INSTANT-006.0: GitHub Repository Analysis (General Use)

**User Story:** As a developer exploring any Rust project on GitHub, I want to analyze repositories directly without cloning, so that I can quickly understand project architecture before deciding to use, contribute to, or learn from the code.

#### Acceptance Criteria

1. WHEN I run `parseltongue instant-github <github-url>` THEN the system SHALL fetch repository contents and perform architectural analysis
2. WHEN analyzing GitHub repos THEN the system SHALL work with public repositories without authentication requirements
3. WHEN fetching code THEN the system SHALL use GitHub's API to get file contents efficiently without full git clone
4. WHEN analysis completes THEN the system SHALL display architectural overview with key entities and patterns
5. WHEN repositories are large THEN the system SHALL sample key files (src/main.rs, lib.rs, key modules) for rapid analysis
6. WHEN GitHub API limits are hit THEN the system SHALL provide clear instructions for local analysis

### REQ-INSTANT-007.0: GitHub-Direct Distribution

**User Story:** As a developer who wants to try parseltongue instantly, I want the curl script to fetch the latest binary and analysis scripts directly from GitHub releases, so that I always get the most current version without any manual download steps.

#### Acceptance Criteria

1. WHEN the curl script runs THEN it SHALL fetch the latest parseltongue binary from GitHub releases automatically
2. WHEN downloading THEN the system SHALL detect the user's platform (Linux, macOS, Windows) and download the appropriate binary
3. WHEN fetching from GitHub THEN the system SHALL use GitHub's API to get the latest release without requiring authentication
4. WHEN download completes THEN the system SHALL verify the binary integrity and make it executable
5. WHEN GitHub is unavailable THEN the system SHALL provide fallback instructions for manual download
6. WHEN analysis completes THEN the system SHALL offer to install the binary permanently or clean up temporary files

## Success Metrics (Segment-Specific)

### Tool Users (Adoption & Usage)
**North Star**: Time-to-"This works on MY code" <2 minutes
**Supporting Metrics**:
1. **Proof Speed**: <2 minutes for local instant-map
2. **Conversion Rate**: % who proceed to full onboarding after instant-map
3. **Confidence Building**: Specific insights about their actual codebase
4. **Adoption Funnel**: instant-map → onboarding → daily usage

### Tool Contributors (Community & Development)
**North Star**: Time-to-contribution-understanding <60 seconds
**Supporting Metrics**:
1. **Self-Analysis Speed**: <60 seconds for parseltongue architecture understanding
2. **Contribution Quality**: % who successfully contribute after self-analysis
3. **Onboarding Success**: Faster contributor onboarding to parseltongue development
4. **Community Growth**: More developers able to extend/rebuild parseltongue

### Unified Anti-Metrics (Don't Optimize For)
- Comprehensive analysis depth (that's for full workflows)
- Advanced feature exposure (keep it simple for first impression)
- Perfect accuracy (good enough to build confidence/make decisions)

## Implementation Strategy

### Phase 1: Local Instant Analysis
- Single command that works on any Rust directory
- Fast entity discovery and relationship extraction
- Terminal-based architectural overview
- Smart highlighting of key entities

### Phase 2: GitHub Integration
- Direct GitHub repository analysis
- API-based file fetching
- Same output format as local analysis
- Graceful handling of API limits

### Phase 3: Visual Enhancement
- ASCII art architectural diagrams
- HTML export for sharing
- Pattern detection and highlighting
- Performance metrics display

## The "Curl Moment" Design (Dual-Track)

**Inspiration**: `curl -s "https://api.github.com/repos/user/repo" | jq '.stargazers_count'`

### Track 1: Parseltongue Users
**Command**: `parseltongue instant-map .`
**Expected Output**:
```
🐍 Parseltongue Analysis: YOUR Codebase
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Architecture Overview (analyzed in 1m 23s)
   127 files → 847 entities → 1,456 relationships

🎯 Your Key Entry Points
   • main()                  - src/main.rs:15
   • ConcurrentEngine::new() - Core analysis engine
   • WorkflowOrchestrator    - Main coordination

🏗️ Your Architecture Patterns  
   • Discovery Engine (trait-based)
   • Concurrent Processing (Arc<RwLock>)
   • Builder Pattern (ISG construction)

⚡ Your Most Connected Entities
   • ISGNode (89 relationships) - Central data model
   • DiscoveryEngine (67 relationships) - Core logic
   • SigHash (45 relationships) - Identity system

✅ Parseltongue understands YOUR codebase!
   Ready for full workflow? Run: parseltongue onboard .
```

### Track 2: Tool Contributors
**Command**: `parseltongue instant-map . --self-analysis`
**Expected Output**:
```
🐍 Parseltongue Self-Analysis: Understanding the Tool
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Architecture Overview (analyzed in 43s)
   127 files → 847 entities → 1,456 relationships

🎯 Core Components
   • DiscoveryEngine         - Main analysis engine (trait-based)
   • ISGNode/ISGEdge        - Graph data structures
   • WorkflowOrchestrator   - Coordination layer

🏗️ Key Patterns
   • Trait-based Discovery (extensible engines)
   • Arc<RwLock> Concurrency (thread-safe ISG)
   • Builder Pattern (ISG construction)

💡 Contribution Opportunities
   • New DiscoveryEngine implementations (language support)
   • Output formatters (visualization, export formats)
   • Performance optimizations (string interning, indexing)

� RExtension Points
   • src/discovery/engine.rs - Add new discovery strategies
   • src/discovery/output_formatter.rs - Add output formats
   • src/isg.rs - Enhance graph capabilities

🚀 Ready to contribute?
   See: docs/ONBOARDING_GUIDE.md
   Start: cargo build --release && cargo test
```

### Dual Value Demonstration
**Tool Users**: "This actually understands MY specific code"
**Tool Contributors**: "I understand how parseltongue works and where I can help"

Both outputs demonstrate:
1. **Scale Understanding**: Accurate file/entity counts
2. **Architectural Insight**: Identifies actual patterns and key entities  
3. **Specific Knowledge**: Shows real function names and traits
4. **Segment-Specific Value**: Tailored next steps for each user type

## Scope Control

### In Scope (Instant Onboarding)
- ✅ Single-command instant analysis
- ✅ GitHub repository support
- ✅ Terminal-based architectural overview
- ✅ Smart entity highlighting
- ✅ Zero-setup experience
- ✅ Confidence-building output

### Deliberately Cut (Full Workflows)
- ❌ Comprehensive relationship analysis (use full onboarding)
- ❌ Interactive exploration (use existing workflows)
- ❌ Persistent workspace management (instant = ephemeral)
- ❌ Advanced pattern detection (keep it simple)
- ❌ LLM context generation (use existing features)
- ❌ Real-time monitoring (instant = one-shot)

**Strategic Focus**: Create the "curl moment" that gets users excited about parseltongue's capabilities, then funnel them to existing comprehensive workflows.

## The Ultimate Curl Experience

### The Docker-Style Installation
**Command**: `curl -sSL https://parseltongue.dev/instant | bash`

**What the Script Does**:
1. **Platform Detection**: Automatically detects Linux/macOS/Windows
2. **Binary Download**: Fetches latest release from GitHub
3. **Project Detection**: Identifies if current directory is a Rust project
4. **Instant Analysis**: Runs architectural analysis automatically
5. **Interactive Results**: Shows analysis with next-step options
6. **Cleanup Options**: Offers to keep binary or clean up

**The Magic**: Zero setup, zero configuration, immediate architectural insights about THEIR specific codebase.

## The Business Impact

**Problem Solved**: Eliminates the adoption friction that prevents developers from experiencing parseltongue's value

**User Journey Transformation**:
- **Before**: Clone → Build → Setup → Run → Navigate outputs (20+ minutes)
- **After**: Single curl command → See value (2 minutes)

**Conversion Funnel**:
1. **Curl Command** (30 seconds) → Zero friction trial
2. **Instant Analysis** (2 minutes) → Builds confidence  
3. **Full Onboarding** (15 minutes) → Demonstrates depth
4. **Daily Workflows** (ongoing) → Delivers sustained value

This creates the smoothest possible progression from "curious" to "convinced" to "committed" user - just like Docker's installation experience.
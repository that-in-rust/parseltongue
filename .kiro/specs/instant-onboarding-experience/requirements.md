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

## The Vision: The Ultimate "GitHub-Hosted Curl Moment"

### Track 1: Tool Users (Zero-Setup Visual Analysis)
**The Dream Experience**:
```bash
# User sits in their Rust project root (must have .git)
curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/instant-analyze.sh | bash
# ✅ Downloads binary, analyzes THEIR codebase, generates HTML visualization
# ✅ Opens browser with interactive architectural map of their code
# ✅ Zero setup, immediate visual "wow moment"
```

**Alternative for GitIngest Format**:
```bash
# User has a gitingest-format file
curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/instant-analyze.sh | bash -s -- --gitingest mycode.txt
# ✅ Analyzes from gitingest format, same visual output
```

### Track 2: Tool Contributors (Self-Analysis via GitHub)
**The Dream Experience**:
```bash
# From parseltongue repository root
curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/self-analyze.sh | bash
# ✅ Downloads and runs self-analysis, generates contributor-focused HTML
# ✅ Visual architecture map with contribution opportunities highlighted
# ✅ Opens in browser with interactive exploration of parseltongue internals
```

### Unified Success Criteria
- **Tool Users**: See meaningful insights about their specific codebase in <2 minutes
- **Tool Contributors**: Understand parseltongue's architecture and contribution opportunities in <60 seconds
- **Both Segments**: Output that immediately demonstrates deep Rust code understanding

## Requirements

### REQ-INSTANT-001.0: GitHub-Hosted Visual Analysis Pipeline (Tool User Segment)

**User Story:** As a Rust developer evaluating parseltongue, I want to run a single curl command from my Git-tracked project directory that downloads, analyzes, and opens a beautiful HTML visualization of my codebase architecture, so that I get an immediate visual "wow moment" that demonstrates parseltongue's deep understanding.

#### Acceptance Criteria

1. WHEN I run `curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/instant-analyze.sh | bash` from my Rust project root with .git THEN the system SHALL download the binary and generate HTML visualization in under 2 minutes
2. WHEN the curl script runs THEN it SHALL detect Git repository, validate Rust project, and run architectural analysis
3. WHEN analysis completes THEN the system SHALL generate an interactive HTML file with visual architecture map and automatically open it in the default browser
4. WHEN displaying results THEN the HTML SHALL show interactive node graph, entity counts, key patterns, and entry points with hover details
5. WHEN using gitingest format THEN `bash -s -- --gitingest mycode.txt` SHALL analyze the provided file instead of local directory
6. WHEN the script finishes THEN it SHALL offer to keep the binary and HTML report or clean up automatically
7. WHEN errors occur THEN the system SHALL provide clear diagnostics and fallback to terminal-only output

### REQ-INSTANT-002.0: GitHub-Hosted Self-Analysis Pipeline (Tool Contributor Segment)

**User Story:** As a developer interested in contributing to parseltongue, I want to run a curl command that downloads and runs self-analysis with an interactive HTML visualization of parseltongue's architecture, so that I can quickly understand the codebase structure and identify contribution opportunities through visual exploration.

#### Acceptance Criteria

1. WHEN I run `curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/self-analyze.sh | bash` from the parseltongue repository root THEN the system SHALL download and generate contributor-focused HTML visualization in under 60 seconds
2. WHEN the self-analysis script runs THEN it SHALL automatically detect this is the parseltongue codebase and run specialized contributor analysis
3. WHEN analysis completes THEN the system SHALL generate interactive HTML with parseltongue's architecture highlighting contribution opportunities and open it in browser
4. WHEN showing results THEN the HTML SHALL highlight extension points, key components, and areas suitable for contribution with clickable nodes and detailed tooltips
5. WHEN displaying architecture THEN the visualization SHALL explain core concepts (ISG, SigHash, DiscoveryEngine) with interactive exploration and code links
6. WHEN successful THEN the HTML SHALL provide specific next steps for development setup and first contributions with embedded links and commands

### REQ-INSTANT-003.0: Interactive HTML Architecture Map

**User Story:** As a developer who learns visually, I want an immediate interactive HTML visualization of my codebase architecture that opens in my browser, so that I can explore the structure through clicking, hovering, and zooming rather than reading text output.

#### Acceptance Criteria

1. WHEN the instant analysis runs THEN the system SHALL generate an interactive HTML file with D3.js or similar visualization library
2. WHEN creating visuals THEN the system SHALL use force-directed graph layout with nodes representing entities and edges representing relationships
3. WHEN showing architecture THEN the system SHALL highlight module relationships, key traits, and main entry points with different colors and sizes
4. WHEN displaying diagrams THEN the system SHALL keep initial view manageable (top 20-30 most important nodes) with ability to expand
5. WHEN visual output is generated THEN the system SHALL include interactive sidebar with entity counts, relationship statistics, and search functionality
6. WHEN nodes are clicked THEN the system SHALL show detailed information panel with entity details, relationships, and source code location
7. WHEN the HTML is generated THEN it SHALL be self-contained (no external dependencies) and work offline for sharing

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

### REQ-INSTANT-006.0: GitIngest Format Support

**User Story:** As a developer who has a codebase in GitIngest format (single text file with all code), I want to analyze it using the same curl command with a flag, so that I can get the same visual analysis without needing a Git repository.

#### Acceptance Criteria

1. WHEN I run `curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/instant-analyze.sh | bash -s -- --gitingest mycode.txt` THEN the system SHALL analyze the GitIngest format file instead of local directory
2. WHEN processing GitIngest format THEN the system SHALL parse the file markers (FILE: path/to/file.rs) to reconstruct file structure
3. WHEN analyzing GitIngest content THEN the system SHALL generate the same interactive HTML visualization as Git repository analysis
4. WHEN GitIngest file is large THEN the system SHALL handle files up to 50MB efficiently
5. WHEN GitIngest parsing fails THEN the system SHALL provide clear error messages about format issues
6. WHEN analysis completes THEN the HTML SHALL show the same architecture map with file paths from the GitIngest markers

### REQ-INSTANT-007.0: Cross-Platform GitHub Distribution

**User Story:** As a developer on macOS or Linux who wants to try parseltongue instantly, I want the curl script to fetch the latest binary directly from GitHub releases and work seamlessly on my platform, so that I get consistent experience regardless of my operating system.

#### Acceptance Criteria

1. WHEN the curl script runs THEN it SHALL fetch the latest parseltongue binary from GitHub releases automatically using GitHub API
2. WHEN downloading THEN the system SHALL detect the user's platform (Linux x86_64, macOS Intel, macOS ARM) and download the appropriate binary
3. WHEN fetching from GitHub THEN the system SHALL use GitHub's public API to get the latest release without requiring authentication
4. WHEN download completes THEN the system SHALL verify the binary integrity using checksums and make it executable with proper permissions
5. WHEN running on macOS THEN the system SHALL handle Gatekeeper restrictions and provide clear instructions if binary is blocked
6. WHEN running on Linux THEN the system SHALL ensure compatibility with common distributions (Ubuntu, CentOS, Alpine)
7. WHEN GitHub is unavailable THEN the system SHALL provide fallback instructions for manual download with direct links
8. WHEN analysis completes THEN the system SHALL offer to install the binary permanently to PATH or clean up temporary files

## Success Metrics (Visual-First Approach)

### Tool Users (Visual Wow & Adoption)
**North Star**: Time-to-"Visual Wow Moment" <2 minutes (browser opens with their architecture)
**Supporting Metrics**:
1. **Visual Impact Speed**: <2 minutes from curl to browser opening with interactive map
2. **Exploration Engagement**: Time spent exploring the HTML visualization (target: >3 minutes)
3. **Conversion Rate**: % who proceed to full onboarding after seeing visual analysis
4. **Sharing Behavior**: % who share the generated HTML with team members
5. **Confidence Building**: Specific visual insights about their actual codebase architecture

### Tool Contributors (Architecture Understanding)
**North Star**: Time-to-contribution-readiness <60 seconds (visual architecture + contribution points)
**Supporting Metrics**:
1. **Self-Analysis Speed**: <60 seconds from curl to contributor HTML opening
2. **Architecture Comprehension**: Visual exploration of parseltongue's structure
3. **Contribution Quality**: % who successfully contribute after visual self-analysis
4. **Extension Point Discovery**: Time to identify where to add features (target: <5 minutes)
5. **Community Growth**: More developers able to extend/rebuild parseltongue

### Unified Success Indicators
- **Visual Engagement**: Users spend time exploring the interactive graph
- **Browser Compatibility**: HTML works across Chrome, Firefox, Safari
- **Self-Contained Sharing**: HTML files work offline and can be shared
- **Cross-Platform Success**: Works on both macOS and Linux consistently

### Anti-Metrics (Don't Optimize For)
- Terminal-only output (prioritize visual experience)
- Comprehensive analysis depth (that's for full workflows)
- Perfect accuracy (good enough for visual confidence building)
- Windows support (focus on macOS/Linux developer adoption)

## Implementation Strategy

### Phase 1: GitHub-Hosted Curl Scripts
- Create `instant-analyze.sh` and `self-analyze.sh` in parseltongue repository
- GitHub releases integration for binary distribution
- Platform detection (macOS Intel/ARM, Linux x86_64)
- Git repository validation and Rust project detection

### Phase 2: Interactive HTML Generation
- Self-contained HTML with embedded D3.js for force-directed graphs
- Entity visualization with color coding and size based on relationships
- Interactive sidebar with search, statistics, and next steps
- Automatic browser launching for immediate visual impact

### Phase 3: GitIngest Support & Polish
- GitIngest format parsing for alternative input method
- Cross-platform testing and refinement
- Error handling and fallback scenarios
- Performance optimization for large codebases

### Technical Architecture

**Curl Script Flow**:
1. Platform detection → Download appropriate binary from GitHub releases
2. Git/GitIngest validation → Ensure valid input source
3. Analysis execution → Run parseltongue with HTML output flag
4. HTML generation → Create self-contained visualization file
5. Browser launch → Open HTML automatically for immediate wow factor
6. Cleanup options → Keep or remove temporary files

**HTML Visualization**:
- Force-directed graph using D3.js (embedded, no CDN)
- Node types: Functions (orange), Structs (green), Traits (blue), Modules (purple)
- Node size: Based on relationship count (centrality)
- Interactive features: Hover details, click for code location, zoom/pan
- Sidebar: Search, statistics, key insights, next steps

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
- ✅ Single-command GitHub-hosted curl script
- ✅ Interactive HTML visualization (opens in browser)
- ✅ Git repository detection and validation
- ✅ GitIngest format support as alternative input
- ✅ Cross-platform support (macOS, Linux)
- ✅ Visual "wow moment" with immediate browser opening
- ✅ Self-contained HTML (no external dependencies)

### Deliberately Cut (Full Workflows)
- ❌ Custom domain hosting (use GitHub raw URLs)
- ❌ Windows support (focus on macOS/Linux developers)
- ❌ Real-time collaboration features
- ❌ Persistent workspace management (instant = ephemeral)
- ❌ Advanced pattern detection (keep it simple)
- ❌ LLM context generation (use existing features)
- ❌ Authentication or user accounts

**Strategic Focus**: Create the "curl moment" that gets users excited about parseltongue's capabilities, then funnel them to existing comprehensive workflows.

## The Ultimate GitHub-Hosted Curl Experience

### The Visual-First Installation
**Primary Command**: `curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/instant-analyze.sh | bash`
**GitIngest Alternative**: `curl -sSL https://raw.githubusercontent.com/user/parseltongue/main/scripts/instant-analyze.sh | bash -s -- --gitingest mycode.txt`

**What the Script Does**:
1. **Git Detection**: Validates current directory has .git and is Rust project
2. **Platform Detection**: Automatically detects macOS (Intel/ARM) or Linux
3. **Binary Download**: Fetches latest parseltongue binary from GitHub releases
4. **Instant Analysis**: Runs architectural analysis automatically
5. **HTML Generation**: Creates interactive visualization with D3.js force graph
6. **Browser Launch**: Automatically opens HTML in default browser for immediate "wow"
7. **Cleanup Options**: Offers to keep binary and HTML or clean up

**The Magic**: Zero setup, immediate visual impact, browser opens with interactive architecture map of THEIR specific codebase.

### The Shreyas Doshi "Jobs to be Done" Alignment

**Tool Users Job**: "Show me this tool understands MY code"
**Solution**: Interactive HTML visualization of their actual codebase architecture

**Tool Contributors Job**: "Help me understand how to contribute to parseltongue"  
**Solution**: Self-analysis HTML with contribution opportunities highlighted

**The Visual Wow Factor**: Instead of terminal text, users get an interactive graph they can explore, zoom, click, and share.

## The HTML Visualization Experience

### Track 1: Tool Users HTML Output
**What Opens in Browser**:
```html
<!DOCTYPE html>
<html>
<head>
    <title>Parseltongue Analysis: my-rust-project</title>
    <!-- Self-contained D3.js and styling -->
</head>
<body>
    <div class="header">
        <h1>🐍 Parseltongue Architecture Map</h1>
        <div class="stats">
            <span>127 files</span> → <span>847 entities</span> → <span>1,456 relationships</span>
            <span class="analysis-time">Analyzed in 1m 23s</span>
        </div>
    </div>
    
    <div class="main-container">
        <div class="graph-container">
            <!-- Interactive D3.js force-directed graph -->
            <!-- Nodes colored by type: traits (blue), structs (green), functions (orange) -->
            <!-- Node size based on relationship count -->
            <!-- Hover shows entity details -->
            <!-- Click shows code location and relationships -->
        </div>
        
        <div class="sidebar">
            <div class="search-box">
                <input placeholder="Search entities..." />
            </div>
            
            <div class="key-insights">
                <h3>🎯 Key Entry Points</h3>
                <ul>
                    <li><code>main()</code> - src/main.rs:15</li>
                    <li><code>ConcurrentEngine::new()</code> - Core analysis engine</li>
                </ul>
                
                <h3>🏗️ Architecture Patterns</h3>
                <ul>
                    <li>Discovery Engine (trait-based)</li>
                    <li>Concurrent Processing (Arc&lt;RwLock&gt;)</li>
                </ul>
                
                <h3>⚡ Most Connected</h3>
                <ul>
                    <li><code>ISGNode</code> (89 relationships)</li>
                    <li><code>DiscoveryEngine</code> (67 relationships)</li>
                </ul>
            </div>
            
            <div class="next-steps">
                <h3>🚀 Next Steps</h3>
                <button onclick="runFullOnboarding()">Run Full Onboarding</button>
                <button onclick="shareVisualization()">Share This Map</button>
            </div>
        </div>
    </div>
</body>
</html>
```

### Track 2: Contributors HTML Output
**What Opens in Browser**:
- Same interactive graph structure
- Green highlighting on extension points
- Tooltips showing "Add Feature Here" on extensible components
- Embedded getting started guide
- Links to specific files for contribution
- Code complexity heatmap overlay
- TODO comments highlighted as contribution opportunities

### The Immediate Impact
**Visual Wow**: Browser opens automatically with beautiful, interactive architecture map
**Exploration**: Users can click, zoom, hover to explore their codebase visually
**Sharing**: Self-contained HTML can be shared with team members
**Confidence**: Seeing their actual code structure proves parseltongue "gets it"

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
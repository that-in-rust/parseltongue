# AIM Daemon: File Discovery and Monitoring Strategy

## The File Discovery Challenge

**Core Problem**: How does AIM Daemon know which files to parse and monitor in a real codebase?

---

## File Discovery Strategies

### 1. **Project Structure Detection** (Primary Approach)

**Language-Specific Project Files:**
```rust
// Rust projects
if Path::new("Cargo.toml").exists() {
    // Parse Cargo.toml to find source directories
    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let config: CargoConfig = toml::from_str(&cargo_toml)?;
    
    // Default Rust source paths
    let source_dirs = vec![
        "src/",           // Main source
        "examples/",      // Example code
        "tests/",         // Integration tests
        "benches/",       // Benchmarks
    ];
    
    // Workspace detection
    if let Some(workspace) = config.workspace {
        for member in workspace.members {
            discover_rust_files(&member)?;
        }
    }
}

// TypeScript/JavaScript projects  
if Path::new("package.json").exists() {
    let package_json = fs::read_to_string("package.json")?;
    let config: PackageConfig = serde_json::from_str(&package_json)?;
    
    // Common TS/JS source paths
    let source_dirs = vec![
        "src/", "lib/", "app/", "components/", 
        "pages/", "utils/", "hooks/", "services/"
    ];
}

// Python projects
if Path::new("pyproject.toml").exists() || Path::new("setup.py").exists() {
    // Python package discovery
}
```

### 2. **Recursive File System Walking**

**Implementation with `walkdir`:**
```rust
use walkdir::WalkDir;
use std::path::Path;

pub struct FileDiscovery {
    root_path: PathBuf,
    language_filters: Vec<LanguageFilter>,
    ignore_patterns: Vec<String>,
}

impl FileDiscovery {
    pub fn discover_files(&self) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(&self.root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Skip directories
            if !path.is_file() { continue; }
            
            // Apply ignore patterns
            if self.should_ignore(path) { continue; }
            
            // Check if file matches language filters
            if self.matches_language_filter(path) {
                files.push(path.to_path_buf());
            }
        }
        
        Ok(files)
    }
    
    fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Standard ignore patterns
        let ignore_patterns = [
            "target/",      // Rust build artifacts
            "node_modules/", // Node.js dependencies
            ".git/",        // Git metadata
            "dist/",        // Build outputs
            "build/",       // Build outputs
            "__pycache__/", // Python cache
            ".pytest_cache/", // Pytest cache
            "coverage/",    // Coverage reports
        ];
        
        for pattern in &ignore_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        
        // Custom ignore patterns from .gitignore
        self.ignore_patterns.iter().any(|pattern| {
            path_str.contains(pattern)
        })
    }
    
    fn matches_language_filter(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            match ext.as_str() {
                "rs" => true,           // Rust
                "ts" | "tsx" => true,   // TypeScript
                "js" | "jsx" => true,   // JavaScript
                "py" => true,           // Python
                "go" => true,           // Go
                "java" => true,         // Java
                "cpp" | "cc" | "cxx" | "hpp" | "h" => true, // C++
                _ => false,
            }
        } else {
            false
        }
    }
}
```

### 3. **Git Integration** (Smart Discovery)

**Use Git to find tracked files:**
```rust
use git2::Repository;

pub fn discover_git_tracked_files(repo_path: &Path) -> Result<Vec<PathBuf>, Error> {
    let repo = Repository::open(repo_path)?;
    let index = repo.index()?;
    let mut files = Vec::new();
    
    for entry in index.iter() {
        let path = Path::new(std::str::from_utf8(&entry.path)?);
        
        // Only include source files
        if is_source_file(path) {
            files.push(repo_path.join(path));
        }
    }
    
    Ok(files)
}

// Alternative: Use git command
pub fn discover_via_git_command(repo_path: &Path) -> Result<Vec<PathBuf>, Error> {
    let output = std::process::Command::new("git")
        .args(&["ls-files", "--cached", "--others", "--exclude-standard"])
        .current_dir(repo_path)
        .output()?;
    
    let files_str = String::from_utf8(output.stdout)?;
    let files: Vec<PathBuf> = files_str
        .lines()
        .filter(|line| is_source_file(Path::new(line)))
        .map(|line| repo_path.join(line))
        .collect();
    
    Ok(files)
}
```

---

## Real-Time File Monitoring

### 1. **File System Watcher Implementation**

```rust
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use std::sync::mpsc;
use std::time::Duration;

pub struct FileMonitor {
    watcher: RecommendedWatcher,
    receiver: mpsc::Receiver<notify::Result<Event>>,
    watched_paths: HashSet<PathBuf>,
}

impl FileMonitor {
    pub fn new() -> Result<Self, Error> {
        let (tx, rx) = mpsc::channel();
        
        let watcher = RecommendedWatcher::new(
            move |res| {
                if let Err(e) = tx.send(res) {
                    eprintln!("Watch error: {}", e);
                }
            },
            notify::Config::default(),
        )?;
        
        Ok(FileMonitor {
            watcher,
            receiver: rx,
            watched_paths: HashSet::new(),
        })
    }
    
    pub fn watch_directory(&mut self, path: &Path) -> Result<(), Error> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        self.watched_paths.insert(path.to_path_buf());
        Ok(())
    }
    
    pub fn process_events(&self) -> Vec<FileEvent> {
        let mut events = Vec::new();
        
        // Process all pending events (non-blocking)
        while let Ok(Ok(event)) = self.receiver.try_recv() {
            match event.kind {
                EventKind::Create(_) => {
                    for path in event.paths {
                        if is_source_file(&path) {
                            events.push(FileEvent::Created(path));
                        }
                    }
                }
                EventKind::Modify(_) => {
                    for path in event.paths {
                        if is_source_file(&path) {
                            events.push(FileEvent::Modified(path));
                        }
                    }
                }
                EventKind::Remove(_) => {
                    for path in event.paths {
                        events.push(FileEvent::Deleted(path));
                    }
                }
                _ => {} // Ignore other events
            }
        }
        
        events
    }
}

#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
}
```

### 2. **Intelligent Filtering**

```rust
pub struct SmartFileFilter {
    // Cache of known source files
    known_files: HashSet<PathBuf>,
    // Language-specific patterns
    language_patterns: HashMap<Language, Vec<String>>,
    // Project-specific ignore patterns
    ignore_patterns: Vec<glob::Pattern>,
}

impl SmartFileFilter {
    pub fn should_process_file(&self, path: &Path) -> bool {
        // Quick checks first (performance)
        if !path.is_file() { return false; }
        
        // Check extension
        if !self.has_source_extension(path) { return false; }
        
        // Check ignore patterns
        if self.matches_ignore_pattern(path) { return false; }
        
        // Check if it's in a source directory
        if !self.is_in_source_directory(path) { return false; }
        
        true
    }
    
    fn has_source_extension(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            matches!(ext.to_string_lossy().as_ref(), 
                "rs" | "ts" | "tsx" | "js" | "jsx" | "py" | "go" | "java" | "cpp" | "hpp")
        } else {
            false
        }
    }
    
    fn is_in_source_directory(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Common source directory patterns
        let source_patterns = [
            "/src/", "/lib/", "/app/", "/components/",
            "/pages/", "/utils/", "/hooks/", "/services/",
            "/examples/", "/tests/", "/benches/"
        ];
        
        source_patterns.iter().any(|pattern| path_str.contains(pattern))
    }
}
```

---

## Configuration-Based Discovery

### 1. **AIM Configuration File**

```toml
# aim.toml - Project configuration
[project]
name = "my-rust-project"
root = "."
languages = ["rust", "typescript"]

[discovery]
# Explicit source directories
source_dirs = [
    "src/",
    "examples/", 
    "frontend/src/",
    "shared/lib/"
]

# File patterns to include
include_patterns = [
    "**/*.rs",
    "**/*.ts", 
    "**/*.tsx"
]

# Patterns to ignore (beyond defaults)
ignore_patterns = [
    "generated/",
    "vendor/",
    "third_party/"
]

[monitoring]
# Enable real-time file watching
watch_enabled = true

# Debounce delay for file changes (ms)
debounce_delay = 100

# Maximum files to watch
max_watched_files = 10000

[languages.rust]
# Rust-specific configuration
cargo_workspace = true
include_tests = true
include_examples = true

[languages.typescript]
# TypeScript-specific configuration  
include_node_modules = false
tsconfig_path = "tsconfig.json"
```

### 2. **Configuration Parser**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AimConfig {
    pub project: ProjectConfig,
    pub discovery: DiscoveryConfig,
    pub monitoring: MonitoringConfig,
    pub languages: HashMap<String, LanguageConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscoveryConfig {
    pub source_dirs: Vec<String>,
    pub include_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
}

impl AimConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        let config: AimConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn discover_files(&self, root: &Path) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();
        
        // Use configured source directories
        for source_dir in &self.discovery.source_dirs {
            let dir_path = root.join(source_dir);
            if dir_path.exists() {
                files.extend(self.scan_directory(&dir_path)?);
            }
        }
        
        Ok(files)
    }
}
```

---

## Complete File Discovery Workflow

### 1. **Initialization Phase**

```rust
pub struct AimDaemon {
    config: AimConfig,
    file_monitor: FileMonitor,
    discovered_files: HashSet<PathBuf>,
    graph: InterfaceGraph,
}

impl AimDaemon {
    pub async fn initialize(project_root: &Path) -> Result<Self, Error> {
        // 1. Load configuration
        let config = AimConfig::load_or_default(project_root)?;
        
        // 2. Discover initial files
        let mut discovered_files = HashSet::new();
        
        // Try multiple discovery methods
        if let Ok(git_files) = discover_git_tracked_files(project_root) {
            discovered_files.extend(git_files);
        } else {
            // Fallback to filesystem walking
            let walker_files = FileDiscovery::new(project_root)
                .with_config(&config)
                .discover_files()?;
            discovered_files.extend(walker_files);
        }
        
        // 3. Set up file monitoring
        let mut file_monitor = FileMonitor::new()?;
        for source_dir in &config.discovery.source_dirs {
            let dir_path = project_root.join(source_dir);
            if dir_path.exists() {
                file_monitor.watch_directory(&dir_path)?;
            }
        }
        
        // 4. Initial extraction
        let mut graph = InterfaceGraph::new();
        for file_path in &discovered_files {
            if let Ok(nodes) = extract_file_interfaces(file_path) {
                graph.add_nodes(nodes);
            }
        }
        
        Ok(AimDaemon {
            config,
            file_monitor,
            discovered_files,
            graph,
        })
    }
    
    pub async fn run(&mut self) -> Result<(), Error> {
        loop {
            // Process file system events
            let events = self.file_monitor.process_events();
            
            for event in events {
                match event {
                    FileEvent::Created(path) | FileEvent::Modified(path) => {
                        if self.should_process_file(&path) {
                            self.update_file_interfaces(&path).await?;
                        }
                    }
                    FileEvent::Deleted(path) => {
                        self.remove_file_interfaces(&path).await?;
                    }
                }
            }
            
            // Small delay to prevent busy waiting
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}
```

### 2. **CLI Interface**

```bash
# Initialize AIM daemon in current directory
aim init

# Discover files in specific directory
aim discover /path/to/project

# Watch for changes (daemon mode)
aim watch --daemon

# Manual file addition
aim add-file src/new_module.rs

# Show discovered files
aim list-files

# Configuration
aim config --source-dirs src/,lib/ --languages rust,typescript
```

---

## Performance Considerations

### File Discovery Performance

| **Method** | **Speed** | **Accuracy** | **Use Case** |
|------------|-----------|--------------|--------------|
| **Git ls-files** | Very Fast | High | Git repositories |
| **Filesystem walk** | Medium | Medium | Non-git projects |
| **Configuration-based** | Fast | Very High | Configured projects |
| **Hybrid approach** | Fast | Very High | Production use |

### Monitoring Performance

```rust
// Optimized file monitoring
impl FileMonitor {
    // Batch events to reduce processing overhead
    pub fn process_events_batched(&self, batch_size: usize) -> Vec<Vec<FileEvent>> {
        let mut batches = Vec::new();
        let mut current_batch = Vec::new();
        
        while let Ok(Ok(event)) = self.receiver.try_recv() {
            current_batch.push(self.convert_event(event));
            
            if current_batch.len() >= batch_size {
                batches.push(current_batch);
                current_batch = Vec::new();
            }
        }
        
        if !current_batch.is_empty() {
            batches.push(current_batch);
        }
        
        batches
    }
}
```

---

## Conclusion

**File Discovery Strategy**: **Multi-layered Approach**

1. **Primary**: Git integration for tracked files
2. **Fallback**: Filesystem walking with smart filtering  
3. **Configuration**: Project-specific overrides
4. **Real-time**: File system watcher for live updates

**Key Benefits**:
- ✅ **Automatic discovery** - No manual file specification needed
- ✅ **Real-time updates** - Catches changes as they happen
- ✅ **Performance optimized** - Smart filtering reduces overhead
- ✅ **Configurable** - Project-specific customization
- ✅ **Robust** - Multiple fallback strategies

This approach ensures AIM Daemon can automatically discover and monitor files in any real-world project structure while maintaining the 3-12ms update latency targets.
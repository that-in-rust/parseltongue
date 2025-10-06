# Tokio Hierarchical Analysis

Interactive visualization of the Tokio async runtime codebase (717 files, 2,576 nodes).

## Files

- `index.md` - Architecture overview (30,000ft view)
- `explore.md` - Detailed exploration (1,000ft view)
- `tokio-interactive.html` - Interactive HTML with zoom/pan/search
- `data/full_isg.json` - Complete ISG data (1.8MB)
- `async-read-results.txt` - AsyncRead trait implementers (18 found)
- `async-write-results.txt` - AsyncWrite trait implementers (17 found)

## Viewing the Interactive HTML

The HTML file requires a local server or special browser settings due to Chrome's CORS security policy.

### Chrome/Chromium Users

**Option 1: Local Server (Recommended)**
```bash
# Start server in this directory
python3 -m http.server 8000
# Then open: http://localhost:8000/tokio-interactive.html
```

**Option 2: Chrome with File Access**
```bash
chrome --allow-file-access-from-files tokio-interactive.html
```

### Firefox/Safari Users

These browsers work normally:
```bash
open tokio-interactive.html  # macOS/Safari
firefox tokio-interactive.html  # Linux/Firefox
```

## Features

- **Hierarchical Levels**: Switch between Overview, Detailed, and Complete views
- **Interactive Search**: Find nodes by name, type, or file path
- **Zoom & Pan**: Mouse wheel zoom, click and drag to pan
- **Node Selection**: Click nodes for detailed information
- **Performance**: Handles 2,500+ nodes smoothly with ELK layout

## Architecture Insights

- **Modular Structure**: runtime, util, stream, test, macro components
- **Async Ecosystem**: Complete I/O with symmetric read/write capabilities
- **Testing Infrastructure**: Comprehensive test coverage across modules
- **Performance**: 0.22s ingestion, 2ms hierarchical export, 6μs queries
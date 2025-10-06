# Interactive HTML Viewer

GitHub doesn't render HTML files directly. To view the interactive HTML diagrams:

## Method 1: Download and View Locally

1. Download the HTML file:
   ```bash
   curl -o tokio-interactive.html https://raw.githubusercontent.com/that-in-rust/parseltongue/main/analysis/tokio-hierarchical/tokio-interactive.html
   ```

2. Open in your browser:
   ```bash
   open tokio-interactive.html  # macOS
   # or
   xdg-open tokio-interactive.html  # Linux
   # or
   start tokio-interactive.html  # Windows
   ```

## Method 2: Use GitHub Pages (Recommended)

The interactive HTML is available at:
**[https://that-in-rust.github.io/parseltongue/tokio-interactive.html](https://that-in-rust.github.io/parseltongue/tokio-interactive.html)**

## Method 3: Generate Your Own

```bash
# Clone the repository
git clone https://github.com/that-in-rust/parseltongue.git
cd parseltongue

# Build the project
cargo build --release

# Download the Tokio test data
curl -o tokio-data.txt https://raw.githubusercontent.com/that-in-rust/parseltongue/main/tests/tokio-rs-tokio-8a5edab282632443.txt

# Generate your own interactive HTML
./target/release/parseltongue ingest tokio-data.txt
./target/release/parseltongue export html --hierarchy --output my-tokio-interactive.html
```

## Features

- **Zoom & Pan**: Mouse wheel to zoom, click and drag to pan
- **Search**: Use search box to find nodes
- **Progressive Disclosure**: Switch between 3 hierarchy levels
- **Interactive**: Hover nodes for details, click to select
- **Performance**: Handles 2,500+ nodes smoothly
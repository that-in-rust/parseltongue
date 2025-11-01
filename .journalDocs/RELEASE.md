# Creating a GitHub Release

To make the curl install command work, create a GitHub release with pre-built binaries.

## Build the Binary (macOS ARM64)

```bash
# Build for macOS Apple Silicon (M1/M2/M3)
cargo build --release

# Verify it's ARM64
file target/release/parseltongue
# Should output: Mach-O 64-bit executable arm64

# Copy with proper naming
cp target/release/parseltongue parseltongue-macos-arm64
```

## Create GitHub Release

1. **Tag a version:**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

2. **Create release on GitHub:**
   - Go to https://github.com/that-in-rust/parseltongue/releases/new
   - Choose the tag `v0.1.0`
   - Title: `Parseltongue v0.1.0 - Unified Binary with 6-Tool Pipeline`
   - Description:
     ```markdown
     ## What's New

     - Unified binary with all 6 tools
     - Self-documenting command names
     - Complete demo with tangible artifacts
     - Temporal versioning system
     - Tree-sitter syntax validation
     - CodeDiff.json generation for LLM consumption

     ## Installation

     ### macOS Apple Silicon (M1/M2/M3)
     ```bash
     curl -L https://github.com/that-in-rust/parseltongue/releases/download/v0.1.0/parseltongue-macos-arm64 -o parseltongue
     chmod +x parseltongue
     sudo mv parseltongue /usr/local/bin/
     ```

     ### Build from Source
     ```bash
     cargo build --release
     ./target/release/parseltongue --help
     ```

     ## Quick Start

     See the [demo-walkthrough](https://github.com/that-in-rust/parseltongue/tree/main/demo-walkthrough) for a complete example.
     ```

3. **Upload binary:**
   - Drag and drop `parseltongue-macos-arm64` to the release assets
   - Publish release

## The curl Command Will Work

Once published, users can install with:
```bash
curl -L https://github.com/that-in-rust/parseltongue/releases/latest/download/parseltongue-macos-arm64 -o parseltongue
chmod +x parseltongue
```

## Future: Multi-Platform Builds

For cross-compilation to other platforms:

```bash
# macOS Intel (x86_64)
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Linux x86_64
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu

# Windows x86_64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

Then upload all binaries to the release:
- `parseltongue-macos-arm64`
- `parseltongue-macos-x86_64`
- `parseltongue-linux-x86_64`
- `parseltongue-windows-x86_64.exe`

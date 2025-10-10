## code-stats-rs

A simple CLI that counts functions and classes/structs from source code in multiple languages using Rust + tree-sitter.

- **Supported Languages**: Rust / Go / Python / JavaScript / TypeScript / Java

### Usage

```bash
# Build (optional)
cargo build --release

# Analyze a directory (summary)
cargo run -- .

# Analyze a single file
cargo run -- tests/fixtures/test.py

# Output in JSON format
cargo run -- . --format json

# Detailed output (per-file breakdown)
cargo run -- . --detail

# Help
cargo run -- --help
```

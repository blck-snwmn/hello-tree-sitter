# Suggested Commands for Development

## Prerequisites
**IMPORTANT**: All commands must be run from the `code-stats-rs/` directory:
```bash
cd code-stats-rs/
```

## Essential Development Commands

### Building and Running
- `cargo build` - Build debug version
- `cargo build --release` - Build optimized binary
- `cargo run -- <file_path>` - Run the analyzer on a file
- `cargo run -- --help` - Show help and usage

### Code Quality
- `cargo check` - Fast syntax and type checking
- `cargo fmt` - Format code according to Rust standards
- `cargo clippy` - Run Rust linter (if available)

### Testing
- `cargo test` - Run all integration tests
- `cargo test --test <test_name>` - Run specific test file
- `cargo test -- --nocapture` - Show print statements during tests

### Usage Examples
```bash
# Analyze a single Rust file
cargo run -- src/main.rs

# Analyze with JSON output
cargo run -- --format json src/main.rs

# Analyze entire directory
cargo run -- --directory src/

# Run with release build
./target/release/code-stats-rs tests/fixtures/test.py
```

## Darwin (macOS) System Commands
- `ls -la` - List files with details
- `find . -name "*.rs"` - Find Rust files
- `grep -r "pattern" .` - Search for pattern (prefer ripgrep: `rg`)
- `open .` - Open current directory in Finder
- `pbcopy < file` - Copy file contents to clipboard
- `pbpaste > file` - Paste clipboard to file

## Git Commands
- `git status` - Check repository status
- `git diff` - View unstaged changes
- `git add .` - Stage all changes
- `git commit -m "message"` - Commit changes
- `git log --oneline -10` - View recent commits
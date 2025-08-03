# Project Overview: hello-tree-sitter

## Purpose
Multi-language code statistics analyzer built with Rust and tree-sitter. The project analyzes source code files to count functions and classes/structures across 6 programming languages: Rust, Go, Python, JavaScript, TypeScript, and Java.

## Working Directory
**IMPORTANT**: Always work from the `code-stats-rs/` subdirectory for all development tasks.

## Project Structure
```
hello-tree-sitter/
├── code-stats-rs/          # Main project directory
│   ├── src/               # Source code
│   │   ├── cli.rs         # CLI argument parsing
│   │   ├── directory.rs   # Directory traversal logic
│   │   ├── error.rs       # Error types
│   │   ├── formatter.rs   # Output formatting
│   │   ├── language.rs    # Language detection and configuration
│   │   ├── lib.rs         # Library root
│   │   ├── main.rs        # Application entry point
│   │   ├── parser.rs      # Tree-sitter parsing logic
│   │   └── stats.rs       # Statistics data structures
│   ├── tests/             # Integration tests
│   │   ├── fixtures/      # Test files for each language
│   │   └── helpers/       # Test utilities
│   ├── Cargo.toml         # Rust dependencies
│   └── Cargo.lock         # Dependency lock file
├── .serena/               # Serena tool configuration
├── .claude/               # Claude configuration
├── CLAUDE.md              # Project instructions for Claude
└── .gitignore

## Architecture
- **SupportedLanguage enum**: Maps file extensions to tree-sitter languages
- **CodeStats struct**: Holds function and class/struct counts
- **FileStats/DirectoryStats/LanguageStats**: Statistics aggregation structures
- **Recursive AST traversal**: Uses tree-sitter cursor for efficient node counting
- **Language-specific patterns**: Each language has specific node types for functions and classes
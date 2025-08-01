# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a multi-language code statistics analyzer built with Rust and tree-sitter. The project analyzes source code files to count functions and classes/structures across 6 programming languages: Rust, Go, Python, JavaScript, TypeScript, and Java.

**Working Directory**: Always work from the `code-stats-rs/` subdirectory for all development tasks.

## Key Dependencies and Architecture

### Project Configuration
- **Rust Edition**: 2024 - Uses the latest Rust edition with modern language features

### Dependencies (from Cargo.toml)
- `tree-sitter = "0.24"` - Core incremental parsing library
- `tree-sitter-rust = "0.23"` - Rust language grammar
- `tree-sitter-go = "0.23"` - Go language grammar  
- `tree-sitter-python = "0.23"` - Python language grammar
- `tree-sitter-javascript = "0.23"` - JavaScript language grammar
- `tree-sitter-typescript = "0.23"` - TypeScript language grammar
- `tree-sitter-java = "0.23"` - Java language grammar

### Architecture
The application is structured around:
- **SupportedLanguage enum**: Maps file extensions to tree-sitter languages
- **CodeStats struct**: Holds function and class/struct counts
- **Recursive AST traversal**: Uses tree-sitter cursor for efficient node counting
- **Language-specific patterns**: Each language has specific node types for functions and classes

## Development Commands

**Prerequisites**: All commands must be run from the `code-stats-rs/` directory:

```bash
cd code-stats-rs/
```

### Essential Commands
- `cargo test` - Run all integration tests
- `cargo build --release` - Build optimized binary
- `cargo run -- <file_path>` - Run the analyzer on a file
- `cargo check` - Fast syntax and type checking
- `cargo fmt` - Format code

### Usage Examples
```bash
# Analyze a Rust file
cargo run -- src/main.rs

# Analyze with release build
./target/release/code-stats-rs tests/fixtures/test.py
```

## Supported Languages and Node Types

The analyzer counts these specific AST node types per language:

- **Rust**: `function_item`, `struct_item`, `enum_item`
- **Go**: `function_declaration`, `method_declaration`, `struct_type`
- **Python**: `function_definition`, `class_definition`
- **JavaScript**: `function_declaration`, `function_expression`, `arrow_function`, `method_definition`, `class_declaration`
- **TypeScript**: Same as JavaScript
- **Java**: `method_declaration`, `constructor_declaration`, `class_declaration`, `interface_declaration`

## Testing Strategy

Integration tests in `tests/integration_test.rs` verify:
- Correct parsing and counting for each supported language
- Error handling for unsupported file types
- Error handling for missing files
- CLI argument validation

Test fixtures are located in `tests/fixtures/` with sample files for each language.
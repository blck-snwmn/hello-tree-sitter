# Technology Stack

## Core Language
- **Rust** (Edition 2024) - Using latest Rust features

## Dependencies

### Core Dependencies
- `tree-sitter = "0.24"` - Core incremental parsing library
- `tree-sitter-rust = "0.23"` - Rust language grammar
- `tree-sitter-go = "0.23"` - Go language grammar  
- `tree-sitter-python = "0.23"` - Python language grammar
- `tree-sitter-javascript = "0.23"` - JavaScript language grammar
- `tree-sitter-typescript = "0.23"` - TypeScript language grammar
- `tree-sitter-java = "0.23"` - Java language grammar

### CLI and Utilities
- `clap = { version = "4.5", features = ["derive"] }` - Command line argument parsing
- `walkdir = "2.5"` - Recursive directory traversal
- `thiserror = "1.0"` - Error handling derive macros

### Serialization
- `serde = { version = "1.0", features = ["derive"] }` - Serialization framework
- `serde_json = "1.0"` - JSON support

### Development Dependencies
- `tempfile = "3"` - Temporary file creation for tests
- `assert_cmd = "2"` - CLI testing utilities
- `predicates = "3"` - Assertion predicates for tests

## Supported Languages and Node Types
- **Rust**: `function_item`, `struct_item`, `enum_item`
- **Go**: `function_declaration`, `method_declaration`, `struct_type`
- **Python**: `function_definition`, `class_definition`
- **JavaScript**: `function_declaration`, `function_expression`, `arrow_function`, `method_definition`, `class_declaration`
- **TypeScript**: Same as JavaScript
- **Java**: `method_declaration`, `constructor_declaration`, `class_declaration`, `interface_declaration`
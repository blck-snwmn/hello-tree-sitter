# Coding Style and Conventions

## Rust Code Style

### Module Organization
- Each major functionality in its own module file
- Public API exposed through `lib.rs`
- Clear separation of concerns:
  - `cli.rs` - CLI argument parsing
  - `parser.rs` - Core parsing logic
  - `stats.rs` - Data structures
  - `error.rs` - Error handling
  - `formatter.rs` - Output formatting

### Naming Conventions
- Structs: PascalCase (e.g., `FileStats`, `CodeStats`, `DirectoryStats`)
- Functions: snake_case (e.g., `analyze_code`, `count_nodes`, `create_parser`)
- Modules: snake_case (e.g., `cli`, `parser`, `stats`)
- Constants: SCREAMING_SNAKE_CASE (if any)

### Struct Definitions
- Use derive macros liberally: `#[derive(Debug, Clone, Serialize, Deserialize)]`
- Public structs with public fields
- Group related fields together

### Error Handling
- Use `thiserror` for error type definitions
- Return `Result<T, Error>` for fallible operations
- Custom error types in `error.rs`

### Testing
- Integration tests in separate files under `tests/`
- Test fixtures in `tests/fixtures/`
- Helper modules in `tests/helpers/`
- Use `assert_cmd` and `predicates` for CLI testing

### Code Organization Patterns
```rust
// Typical struct pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructName {
    pub field1: Type1,
    pub field2: Type2,
}

// Implementation blocks separate from struct definitions
impl StructName {
    pub fn new() -> Self { ... }
    pub fn method(&self) -> Result<T> { ... }
}
```

### Import Style
- Group imports by:
  1. Standard library
  2. External crates
  3. Internal modules
- Use explicit imports rather than glob imports

### Documentation
- Code is self-documenting through clear naming
- Complex algorithms should have brief comments
- Public API items should have doc comments (///)

### Formatting
- Use `cargo fmt` for consistent formatting
- 4-space indentation (Rust default)
- Maximum line length around 100 characters
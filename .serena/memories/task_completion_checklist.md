# Task Completion Checklist

When completing any coding task in this project, follow these steps:

## Before Starting
1. Change to the correct directory: `cd code-stats-rs/`
2. Review existing code for patterns and conventions
3. Check related modules and tests

## During Development
1. Write idiomatic Rust code following project conventions
2. Use appropriate error handling with `Result` types
3. Ensure new code integrates well with existing modules
4. Add appropriate derive macros to new structs

## After Coding
**MANDATORY CHECKS:**

1. **Format Code**
   ```bash
   cargo fmt
   ```

2. **Type Check**
   ```bash
   cargo check
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

4. **Build Check**
   ```bash
   cargo build
   ```

5. **Lint (if available)**
   ```bash
   cargo clippy --all-targets --all-features
   ```

## Before Committing
1. Ensure all tests pass
2. Verify no compilation warnings
3. Check that formatting is correct
4. Review changes with `git diff`

## Additional Checks for New Features
- Add integration tests for new functionality
- Update test fixtures if adding language support
- Ensure error messages are helpful
- Verify CLI help text is updated if adding options

## Important Notes
- Never commit code that doesn't compile
- Always run tests before marking task complete
- If tests fail, fix them before proceeding
- Keep commits focused and atomic
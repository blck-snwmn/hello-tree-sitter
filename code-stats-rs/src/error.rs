//! Error handling for the code statistics analyzer.

use thiserror::Error;

/// Represents all possible errors that can occur during code analysis.
///
/// This enum encapsulates various error conditions that may arise when analyzing
/// source code files, from file system operations to tree-sitter parsing failures.
/// Each variant provides specific context about the error that occurred.
#[derive(Debug, Error)]
pub(crate) enum CodeStatsError {
    /// Indicates that tree-sitter failed to parse a source code file.
    ///
    /// This error occurs when the tree-sitter parser encounters syntax errors
    /// or other parsing issues that prevent successful AST generation. The error
    /// includes the file path or name that caused the parsing failure.
    ///
    /// # Common causes
    /// - Malformed or corrupted source code files
    /// - Files with syntax errors that prevent parsing
    /// - Binary files mistakenly treated as text files
    #[error("Failed to parse file: {0}")]
    ParseError(String),

    /// Indicates that the tree-sitter language grammar could not be initialized.
    ///
    /// This error occurs when there's a failure in setting up the language-specific
    /// tree-sitter grammar. This is typically an internal error that suggests
    /// issues with the tree-sitter language bindings or memory allocation.
    ///
    /// # Common causes
    /// - Memory allocation failures during grammar setup
    /// - Corrupted or incompatible language grammar libraries
    /// - Internal tree-sitter initialization errors
    #[error("Failed to set language grammar")]
    LanguageSetupError,

    /// Indicates that the analyzer encountered a file type that is not supported.
    ///
    /// This error occurs when attempting to analyze files with extensions that
    /// are not recognized by the analyzer. See `SupportedLanguage` for the list
    /// of supported file types.
    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    /// Indicates that an I/O operation failed during file processing.
    ///
    /// This error wraps various file system related errors that can occur
    /// when reading files, traversing directories, or accessing file metadata.
    /// The error message provides details about the specific I/O failure.
    ///
    /// # Common causes
    /// - File or directory does not exist
    /// - Insufficient permissions to read files
    /// - Network issues when accessing remote files
    /// - Disk I/O errors or corrupted file systems
    #[error("IO error: {0}")]
    IoError(String),
}

/// A type alias for `Result<T, CodeStatsError>`.
///
/// This provides a convenient shorthand for functions that return results
/// with `CodeStatsError` as the error type. This is the standard pattern
/// used throughout the codebase for error handling.
pub(crate) type Result<T> = std::result::Result<T, CodeStatsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CodeStatsError::ParseError("test.rs".to_string());
        assert_eq!(err.to_string(), "Failed to parse file: test.rs");

        let err = CodeStatsError::LanguageSetupError;
        assert_eq!(err.to_string(), "Failed to set language grammar");

        let err = CodeStatsError::UnsupportedFileType("test.md".to_string());
        assert_eq!(err.to_string(), "Unsupported file type: test.md");

        let err = CodeStatsError::IoError("File not found".to_string());
        assert_eq!(err.to_string(), "IO error: File not found");
    }

    #[test]
    fn test_error_debug() {
        let err = CodeStatsError::ParseError("debug_test.rs".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("ParseError"));
        assert!(debug_str.contains("debug_test.rs"));
    }

    #[test]
    fn test_result_type() {
        // Test that Result type alias works correctly
        fn returns_ok() -> Result<i32> {
            Ok(42)
        }

        fn returns_err() -> Result<i32> {
            Err(CodeStatsError::LanguageSetupError)
        }

        assert!(returns_ok().is_ok());
        assert_eq!(returns_ok().unwrap(), 42);

        assert!(returns_err().is_err());
        match returns_err() {
            Err(CodeStatsError::LanguageSetupError) => {}
            _ => panic!("Expected LanguageSetupError"),
        }
    }

    #[test]
    fn test_error_variants() {
        // Test that all error variants can be created and pattern matched
        let errors = vec![
            CodeStatsError::ParseError("file.rs".to_string()),
            CodeStatsError::LanguageSetupError,
            CodeStatsError::UnsupportedFileType("file.doc".to_string()),
            CodeStatsError::IoError("Permission denied".to_string()),
        ];

        for error in errors {
            match error {
                CodeStatsError::ParseError(file) => {
                    assert!(!file.is_empty());
                }
                CodeStatsError::LanguageSetupError => {}
                CodeStatsError::UnsupportedFileType(file) => {
                    assert!(!file.is_empty());
                }
                CodeStatsError::IoError(msg) => {
                    assert!(!msg.is_empty());
                }
            }
        }
    }

    #[test]
    fn test_error_is_send_sync() {
        // Verify that errors can be sent between threads
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<CodeStatsError>();
        assert_sync::<CodeStatsError>();
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum CodeStatsError {
    #[error("Failed to parse file: {0}")]
    ParseError(String),

    #[error("Failed to set language grammar")]
    LanguageSetupError,

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("IO error: {0}")]
    IoError(String),
}

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

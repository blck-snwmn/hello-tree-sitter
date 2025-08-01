use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodeStatsError {
    #[error("Failed to parse file: {0}")]
    ParseError(String),
    
    #[error("Unsupported language for file: {0}")]
    UnsupportedLanguage(String),
    
    #[error("Failed to set language grammar")]
    LanguageSetupError,
}

pub type Result<T> = std::result::Result<T, CodeStatsError>;
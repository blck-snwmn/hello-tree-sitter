pub mod error;
pub mod language;
pub mod parser;

pub use error::{CodeStatsError, Result};
pub use language::SupportedLanguage;
pub use parser::{analyze_code, create_parser, CodeStats};
pub mod cli;
pub mod directory;
pub mod error;
pub mod formatter;
pub mod language;
pub mod parser;
pub mod stats;

pub use error::{CodeStatsError, Result};
pub use language::SupportedLanguage;
pub use parser::{analyze_code, create_parser, CodeStats};
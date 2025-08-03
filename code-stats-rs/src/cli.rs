use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "code-stats-rs")]
#[command(about = "Analyze code statistics for functions and classes", long_about = None)]
pub struct Cli {
    /// Path to analyze (file or directory)
    pub path: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Summary)]
    pub format: OutputFormat,

    /// Show detailed statistics for each file
    #[arg(short, long)]
    pub detail: bool,

    /// File patterns to ignore (can be used multiple times)
    #[arg(long, value_name = "PATTERN")]
    pub ignore: Vec<String>,

    /// Follow symbolic links
    #[arg(long)]
    pub follow_links: bool,

    /// Maximum depth for directory traversal
    #[arg(long, default_value_t = 100)]
    pub max_depth: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Summary statistics only
    Summary,
    /// Detailed file-by-file breakdown
    Detail,
    /// JSON output
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parse_basic() {
        let cli = Cli::try_parse_from(&["code-stats-rs", "src/main.rs"]).unwrap();

        assert_eq!(cli.path, PathBuf::from("src/main.rs"));
        assert_eq!(cli.format, OutputFormat::Summary);
        assert!(!cli.detail);
        assert!(cli.ignore.is_empty());
        assert!(!cli.follow_links);
        assert_eq!(cli.max_depth, 100);
    }

    #[test]
    fn test_cli_parse_with_format() {
        let cli = Cli::try_parse_from(&["code-stats-rs", "src", "--format", "json"]).unwrap();

        assert_eq!(cli.path, PathBuf::from("src"));
        assert_eq!(cli.format, OutputFormat::Json);
    }

    #[test]
    fn test_cli_parse_with_detail() {
        let cli = Cli::try_parse_from(&["code-stats-rs", "src", "--detail"]).unwrap();

        assert!(cli.detail);
    }

    #[test]
    fn test_cli_parse_with_short_options() {
        let cli = Cli::try_parse_from(&["code-stats-rs", "src", "-f", "detail", "-d"]).unwrap();

        assert_eq!(cli.format, OutputFormat::Detail);
        assert!(cli.detail);
    }

    #[test]
    fn test_cli_parse_with_ignore_patterns() {
        let cli = Cli::try_parse_from(&[
            "code-stats-rs",
            "src",
            "--ignore",
            "target",
            "--ignore",
            ".git",
        ])
        .unwrap();

        assert_eq!(cli.ignore, vec!["target", ".git"]);
    }

    #[test]
    fn test_cli_parse_with_follow_links() {
        let cli = Cli::try_parse_from(&["code-stats-rs", "src", "--follow-links"]).unwrap();

        assert!(cli.follow_links);
    }

    #[test]
    fn test_cli_parse_with_max_depth() {
        let cli = Cli::try_parse_from(&["code-stats-rs", "src", "--max-depth", "5"]).unwrap();

        assert_eq!(cli.max_depth, 5);
    }

    #[test]
    fn test_cli_parse_all_options() {
        let cli = Cli::try_parse_from(&[
            "code-stats-rs",
            "/path/to/analyze",
            "--format",
            "json",
            "--detail",
            "--ignore",
            "node_modules",
            "--ignore",
            "vendor",
            "--follow-links",
            "--max-depth",
            "3",
        ])
        .unwrap();

        assert_eq!(cli.path, PathBuf::from("/path/to/analyze"));
        assert_eq!(cli.format, OutputFormat::Json);
        assert!(cli.detail);
        assert_eq!(cli.ignore, vec!["node_modules", "vendor"]);
        assert!(cli.follow_links);
        assert_eq!(cli.max_depth, 3);
    }

    #[test]
    fn test_cli_parse_missing_path() {
        let result = Cli::try_parse_from(&["code-stats-rs"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parse_invalid_format() {
        let result = Cli::try_parse_from(&["code-stats-rs", "src", "--format", "invalid"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parse_invalid_max_depth() {
        let result = Cli::try_parse_from(&["code-stats-rs", "src", "--max-depth", "not-a-number"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_output_format_enum() {
        // Test ValueEnum derive
        assert_eq!(
            OutputFormat::from_str("summary", true).unwrap(),
            OutputFormat::Summary
        );
        assert_eq!(
            OutputFormat::from_str("detail", true).unwrap(),
            OutputFormat::Detail
        );
        assert_eq!(
            OutputFormat::from_str("json", true).unwrap(),
            OutputFormat::Json
        );

        // Test case insensitive
        assert_eq!(
            OutputFormat::from_str("SUMMARY", true).unwrap(),
            OutputFormat::Summary
        );
        assert_eq!(
            OutputFormat::from_str("Detail", true).unwrap(),
            OutputFormat::Detail
        );
        assert_eq!(
            OutputFormat::from_str("JSON", true).unwrap(),
            OutputFormat::Json
        );
    }

    #[test]
    fn test_output_format_ordering() {
        // Test PartialOrd and Ord derives
        assert!(OutputFormat::Summary < OutputFormat::Detail);
        assert!(OutputFormat::Detail < OutputFormat::Json);
    }

    #[test]
    fn test_cli_command_metadata() {
        let cmd = Cli::command();

        assert_eq!(cmd.get_name(), "code-stats-rs");
        assert!(
            cmd.get_about()
                .unwrap()
                .to_string()
                .contains("Analyze code statistics")
        );

        // Check that all expected arguments exist
        assert!(cmd.get_arguments().any(|arg| arg.get_id() == "path"));
        assert!(cmd.get_arguments().any(|arg| arg.get_id() == "format"));
        assert!(cmd.get_arguments().any(|arg| arg.get_id() == "detail"));
        assert!(cmd.get_arguments().any(|arg| arg.get_id() == "ignore"));
        assert!(
            cmd.get_arguments()
                .any(|arg| arg.get_id() == "follow_links")
        );
        assert!(cmd.get_arguments().any(|arg| arg.get_id() == "max_depth"));
    }

    #[test]
    fn test_cli_help_flag() {
        // Test that help flag is properly handled
        let result = Cli::try_parse_from(&["code-stats-rs", "--help"]);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn test_cli_version_flag() {
        // Test that version flag is properly handled
        let result = Cli::try_parse_from(&["code-stats-rs", "--version"]);
        // In clap v4, version is disabled by default unless explicitly added
        // So this will result in UnknownArgument error
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }
}

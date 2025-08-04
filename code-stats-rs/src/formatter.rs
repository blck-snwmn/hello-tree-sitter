//! Output formatting for code statistics in Summary, Detail, and JSON formats.

use crate::cli::OutputFormat;
use crate::stats::{DirectoryStats, FileStats};

/// Formats directory statistics according to the specified output format.
///
/// This is the main entry point for formatting directory-wide analysis results.
/// It dispatches to the appropriate formatting function based on the requested format.
///
/// # Arguments
///
/// * `stats` - Directory statistics containing aggregated results from all analyzed files
/// * `format` - The desired output format (Summary, Detail, or JSON)
/// * `_show_detail` - Currently unused parameter (reserved for future functionality)
///
/// # Returns
///
/// A formatted string ready for display or further processing
pub(crate) fn format_output(
    stats: &DirectoryStats,
    format: OutputFormat,
    _show_detail: bool,
) -> String {
    match format {
        OutputFormat::Summary => format_summary(stats),
        OutputFormat::Detail => format_detail(stats),
        OutputFormat::Json => format_json(stats),
    }
}

/// Formats statistics for a single file analysis.
///
/// This function is used when analyzing individual files rather than entire directories.
/// It provides a clear, human-readable summary of the code statistics for the specific file.
///
/// # Arguments
///
/// * `file_stats` - Statistics for a single file including path, language, and counts
///
/// # Returns
///
/// A formatted string containing the file path, detected language, and code statistics
pub(crate) fn format_single_file(file_stats: &FileStats) -> String {
    format!(
        "Analyzing file: {} (Language: {:?})\n\
         Code Statistics:\n\
         Functions: {}\n\
         Classes/Structs: {}",
        file_stats.path.display(),
        file_stats.language,
        file_stats.stats.function_count,
        file_stats.stats.class_struct_count
    )
}

/// Formats directory statistics as a summary view.
///
/// Creates a concise overview showing aggregated statistics by programming language,
/// followed by overall totals. Languages are sorted alphabetically for consistent output.
///
/// # Arguments
///
/// * `stats` - Directory statistics containing per-language aggregations
///
/// # Returns
///
/// A formatted string with language-wise summaries and grand totals
///
/// # Output Format
///
/// ```text
/// Language Summary:
///   Go:           15 functions,    3 structs/classes in 5 files
///   Python:       8 functions,    2 structs/classes in 3 files
///   Rust:         20 functions,   12 structs/classes in 8 files
///
/// Total: 43 functions, 17 structs/classes in 16 files
/// ```
fn format_summary(stats: &DirectoryStats) -> String {
    let mut output = String::new();

    output.push_str("Language Summary:\n");

    // Sort languages alphabetically for consistent output ordering
    let mut languages: Vec<_> = stats.total_by_language.iter().collect();
    languages.sort_by_key(|(lang, _)| format!("{lang:?}"));

    // Format each language's statistics with aligned columns
    for (language, lang_stats) in languages {
        output.push_str(&format!(
            "  {:12} {:4} functions, {:4} structs/classes in {} files\n",
            format!("{:?}:", language),
            lang_stats.function_count,
            lang_stats.class_struct_count,
            lang_stats.file_count
        ));
    }

    // Add grand totals at the end
    output.push_str(&format!(
        "\nTotal: {} functions, {} structs/classes in {} files",
        stats.total_stats.function_count,
        stats.total_stats.class_struct_count,
        stats.total_files()
    ));

    output
}

/// Formats directory statistics as a detailed view.
///
/// Provides comprehensive output showing individual file statistics followed by
/// the summary view. Files are sorted by path for deterministic output ordering.
///
/// # Arguments
///
/// * `stats` - Directory statistics containing individual file results
///
/// # Returns
///
/// A formatted string with per-file details and summary statistics
///
/// # Output Format
///
/// ```text
/// src/main.rs (Rust):
///   Functions: 3
///   Structs/Classes: 2
///
/// src/lib.rs (Rust):
///   Functions: 5
///   Structs/Classes: 1
///
/// Language Summary:
/// [... summary content ...]
/// ```
fn format_detail(stats: &DirectoryStats) -> String {
    let mut output = String::new();

    // Sort files by path for consistent, deterministic output
    let mut files = stats.files.clone();
    files.sort_by(|a, b| a.path.cmp(&b.path));

    // Display individual file statistics
    for file in &files {
        output.push_str(&format!(
            "{} ({:?}):\n  Functions: {}\n  Structs/Classes: {}\n\n",
            file.path.display(),
            file.language,
            file.stats.function_count,
            file.stats.class_struct_count
        ));
    }

    // Append summary statistics at the end
    output.push_str(&format_summary(stats));

    output
}

/// Formats directory statistics as JSON for machine consumption.
///
/// Serializes the complete directory statistics structure to pretty-printed JSON.
/// This format is ideal for programmatic processing, integration with other tools,
/// or storage for later analysis.
///
/// # Arguments
///
/// * `stats` - Directory statistics to serialize
///
/// # Returns
///
/// A JSON string with pretty formatting, or an error message if serialization fails
///
/// # JSON Structure
///
/// The output includes:
/// - `files`: Array of individual file statistics
/// - `total_by_language`: Language-aggregated statistics
/// - `total_stats`: Overall totals across all languages
///
/// # Error Handling
///
/// If JSON serialization fails (highly unlikely with our data structures),
/// returns a formatted error message instead of panicking.
fn format_json(stats: &DirectoryStats) -> String {
    serde_json::to_string_pretty(stats)
        .unwrap_or_else(|e| format!("Error serializing to JSON: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::SupportedLanguage;
    use crate::parser::CodeStats;
    use std::path::PathBuf;

    /// Creates a sample DirectoryStats for testing purposes.
    ///
    /// This helper function sets up realistic test data with multiple files
    /// across different programming languages to verify formatting behavior.
    fn create_test_directory_stats() -> DirectoryStats {
        let mut stats = DirectoryStats::new();

        stats.add_file(FileStats {
            path: PathBuf::from("src/main.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 3,
                class_struct_count: 2,
            },
        });

        stats.add_file(FileStats {
            path: PathBuf::from("src/lib.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 5,
                class_struct_count: 1,
            },
        });

        stats.add_file(FileStats {
            path: PathBuf::from("test.py"),
            language: SupportedLanguage::Python,
            stats: CodeStats {
                function_count: 2,
                class_struct_count: 1,
            },
        });

        stats
    }

    /// Tests single file formatting output.
    ///
    /// Verifies that format_single_file produces the expected content including
    /// file path, language detection, and statistical counts.
    #[test]
    fn test_format_single_file() {
        let file_stats = FileStats {
            path: PathBuf::from("test.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 10,
                class_struct_count: 5,
            },
        };

        let output = format_single_file(&file_stats);

        assert!(output.contains("Analyzing file: test.rs"));
        assert!(output.contains("Language: Rust"));
        assert!(output.contains("Functions: 10"));
        assert!(output.contains("Classes/Structs: 5"));
    }

    /// Tests summary format output structure and content.
    ///
    /// Validates that format_summary correctly aggregates statistics by language,
    /// sorts languages alphabetically, and includes accurate totals.
    #[test]
    fn test_format_summary() {
        let stats = create_test_directory_stats();
        let output = format_summary(&stats);

        // Check structure
        assert!(output.contains("Language Summary:"));
        assert!(output.contains("Total:"));

        // Check language stats
        assert!(output.contains("Rust:"));
        assert!(output.contains("8 functions")); // 3 + 5
        assert!(output.contains("3 structs/classes")); // 2 + 1
        assert!(output.contains("in 2 files"));

        assert!(output.contains("Python:"));
        assert!(output.contains("2 functions"));
        assert!(output.contains("1 structs/classes"));
        assert!(output.contains("in 1 files"));

        // Check totals
        assert!(output.contains("Total: 10 functions, 4 structs/classes in 3 files"));
    }

    /// Tests detailed format output including individual files and summary.
    ///
    /// Ensures that format_detail displays each file's statistics separately
    /// and includes the summary section at the end.
    #[test]
    fn test_format_detail() {
        let stats = create_test_directory_stats();
        let output = format_detail(&stats);

        // Check individual file details
        assert!(output.contains("src/lib.rs (Rust):"));
        assert!(output.contains("src/main.rs (Rust):"));
        assert!(output.contains("test.py (Python):"));

        // Should also include summary
        assert!(output.contains("Language Summary:"));
        assert!(output.contains("Total:"));
    }

    /// Tests JSON format serialization and structure.
    ///
    /// Verifies that format_json produces valid JSON with the expected
    /// structure and correct data values.
    #[test]
    fn test_format_json() {
        let stats = create_test_directory_stats();
        let output = format_json(&stats);

        // Parse JSON to verify it's valid
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();

        // Check structure
        assert!(parsed.get("files").is_some());
        assert!(parsed.get("total_by_language").is_some());
        assert!(parsed.get("total_stats").is_some());

        // Check file count
        let files = parsed["files"].as_array().unwrap();
        assert_eq!(files.len(), 3);

        // Check total stats
        assert_eq!(parsed["total_stats"]["function_count"], 10);
        assert_eq!(parsed["total_stats"]["class_struct_count"], 4);
    }

    /// Tests the main format_output function with all supported formats.
    ///
    /// Validates that the format dispatcher correctly routes to the appropriate
    /// formatting function based on the OutputFormat parameter.
    #[test]
    fn test_format_output_with_different_formats() {
        let stats = create_test_directory_stats();

        let summary = format_output(&stats, OutputFormat::Summary, false);
        assert!(summary.contains("Language Summary:"));
        assert!(!summary.contains("src/main.rs"));

        let detail = format_output(&stats, OutputFormat::Detail, false);
        assert!(detail.contains("src/main.rs"));
        assert!(detail.contains("Language Summary:"));

        let json = format_output(&stats, OutputFormat::Json, false);
        assert!(json.starts_with('{'));
        assert!(json.contains("\"files\""));
    }

    /// Tests formatting behavior with empty statistics.
    ///
    /// Ensures that all formatters handle the edge case of no analyzed files
    /// gracefully, displaying appropriate zero values.
    #[test]
    fn test_format_empty_stats() {
        let stats = DirectoryStats::new();

        let summary = format_summary(&stats);
        assert!(summary.contains("Total: 0 functions, 0 structs/classes in 0 files"));

        let detail = format_detail(&stats);
        assert!(detail.contains("Total: 0 functions, 0 structs/classes in 0 files"));

        let json = format_json(&stats);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["files"].as_array().unwrap().len(), 0);
    }

    /// Tests that languages are sorted alphabetically in summary output.
    ///
    /// Verifies the alphabetical ordering requirement by adding languages
    /// in non-alphabetical order and checking their position in the output.
    #[test]
    fn test_format_language_sorting() {
        let mut stats = DirectoryStats::new();

        // Add files in non-alphabetical order
        stats.add_file(FileStats {
            path: PathBuf::from("test.py"),
            language: SupportedLanguage::Python,
            stats: CodeStats {
                function_count: 1,
                class_struct_count: 0,
            },
        });

        stats.add_file(FileStats {
            path: PathBuf::from("test.go"),
            language: SupportedLanguage::Go,
            stats: CodeStats {
                function_count: 1,
                class_struct_count: 0,
            },
        });

        stats.add_file(FileStats {
            path: PathBuf::from("test.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 1,
                class_struct_count: 0,
            },
        });

        let output = format_summary(&stats);

        // Languages should be sorted alphabetically
        let go_pos = output.find("Go:").unwrap();
        let python_pos = output.find("Python:").unwrap();
        let rust_pos = output.find("Rust:").unwrap();

        assert!(go_pos < python_pos);
        assert!(python_pos < rust_pos);
    }
}

use crate::cli::OutputFormat;
use crate::stats::{DirectoryStats, FileStats};

pub fn format_output(stats: &DirectoryStats, format: OutputFormat, _show_detail: bool) -> String {
    match format {
        OutputFormat::Summary => format_summary(stats),
        OutputFormat::Detail => format_detail(stats),
        OutputFormat::Json => format_json(stats),
    }
}

pub fn format_single_file(file_stats: &FileStats) -> String {
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

fn format_summary(stats: &DirectoryStats) -> String {
    let mut output = String::new();

    output.push_str("Language Summary:\n");

    let mut languages: Vec<_> = stats.total_by_language.iter().collect();
    languages.sort_by_key(|(lang, _)| format!("{:?}", lang));

    for (language, lang_stats) in languages {
        output.push_str(&format!(
            "  {:12} {:4} functions, {:4} structs/classes in {} files\n",
            format!("{:?}:", language),
            lang_stats.function_count,
            lang_stats.class_struct_count,
            lang_stats.file_count
        ));
    }

    output.push_str(&format!(
        "\nTotal: {} functions, {} structs/classes in {} files",
        stats.total_stats.function_count,
        stats.total_stats.class_struct_count,
        stats.total_files()
    ));

    output
}

fn format_detail(stats: &DirectoryStats) -> String {
    let mut output = String::new();

    // Sort files by path for consistent output
    let mut files = stats.files.clone();
    files.sort_by(|a, b| a.path.cmp(&b.path));

    for file in &files {
        output.push_str(&format!(
            "{} ({:?}):\n  Functions: {}\n  Structs/Classes: {}\n\n",
            file.path.display(),
            file.language,
            file.stats.function_count,
            file.stats.class_struct_count
        ));
    }

    output.push_str(&format_summary(stats));

    output
}

fn format_json(stats: &DirectoryStats) -> String {
    serde_json::to_string_pretty(stats)
        .unwrap_or_else(|e| format!("Error serializing to JSON: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::SupportedLanguage;
    use crate::parser::CodeStats;
    use std::path::PathBuf;

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

//! Data structures for collecting and aggregating code statistics.

use crate::language::SupportedLanguage;
use crate::parser::CodeStats;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Statistics for a single source code file.
///
/// This structure holds the analysis results for an individual file, including
/// its path, detected programming language, and the computed code statistics
/// (function and class/struct counts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FileStats {
    /// The path to the analyzed source file
    pub path: PathBuf,
    /// The detected programming language of the file
    pub language: SupportedLanguage,
    /// The computed code statistics for this file
    pub stats: CodeStats,
}

/// Aggregated statistics for a directory containing multiple source files.
///
/// This structure accumulates code statistics across multiple files within a directory,
/// providing both individual file statistics and aggregated totals organized by
/// programming language.
///
/// # Fields
///
/// - `files`: Individual statistics for each analyzed file
/// - `total_by_language`: Aggregated statistics grouped by programming language
/// - `total_stats`: Overall totals across all files and languages
///
#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct DirectoryStats {
    /// Individual statistics for each analyzed file
    pub files: Vec<FileStats>,
    /// Statistics aggregated by programming language
    pub total_by_language: HashMap<SupportedLanguage, LanguageStats>,
    /// Overall totals across all files and languages
    pub total_stats: CodeStats,
}

/// Statistics aggregated for a specific programming language.
///
/// This structure holds the accumulated statistics for all files of a particular
/// programming language within a directory analysis.
///
/// # Fields
///
/// - `file_count`: Number of files analyzed for this language
/// - `function_count`: Total number of functions found across all files
/// - `class_struct_count`: Total number of classes/structs found across all files
///
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct LanguageStats {
    /// Number of files analyzed for this programming language
    pub file_count: usize,
    /// Total number of functions found across all files of this language
    pub function_count: usize,
    /// Total number of classes/structs found across all files of this language
    pub class_struct_count: usize,
}

impl DirectoryStats {
    /// Creates a new empty `DirectoryStats` instance.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds a file's statistics to the directory aggregation.
    ///
    /// This method updates both the overall totals and the language-specific
    /// statistics. It increments file counts, function counts, and class/struct
    /// counts appropriately.
    ///
    /// # Parameters
    ///
    /// * `file_stats` - The statistics for the file to be added to the aggregation
    pub(crate) fn add_file(&mut self, file_stats: FileStats) {
        // Update total stats
        self.total_stats.function_count += file_stats.stats.function_count;
        self.total_stats.class_struct_count += file_stats.stats.class_struct_count;

        // Update language-specific stats
        let lang_stats = self
            .total_by_language
            .entry(file_stats.language)
            .or_default();

        lang_stats.file_count += 1;
        lang_stats.function_count += file_stats.stats.function_count;
        lang_stats.class_struct_count += file_stats.stats.class_struct_count;

        // Add file to list
        self.files.push(file_stats);
    }

    /// Returns the total number of files that have been analyzed.
    pub(crate) fn total_files(&self) -> usize {
        self.files.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_file_stats_creation() {
        let file_stats = FileStats {
            path: PathBuf::from("test.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 5,
                class_struct_count: 2,
            },
        };

        assert_eq!(file_stats.path, PathBuf::from("test.rs"));
        assert_eq!(file_stats.language, SupportedLanguage::Rust);
        assert_eq!(file_stats.stats.function_count, 5);
        assert_eq!(file_stats.stats.class_struct_count, 2);
    }

    #[test]
    fn test_directory_stats_new() {
        let dir_stats = DirectoryStats::new();

        assert_eq!(dir_stats.files.len(), 0);
        assert_eq!(dir_stats.total_stats.function_count, 0);
        assert_eq!(dir_stats.total_stats.class_struct_count, 0);
        assert_eq!(dir_stats.total_by_language.len(), 0);
        assert_eq!(dir_stats.total_files(), 0);
    }

    #[test]
    fn test_directory_stats_add_single_file() {
        let mut dir_stats = DirectoryStats::new();

        let file_stats = FileStats {
            path: PathBuf::from("test.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 3,
                class_struct_count: 1,
            },
        };

        dir_stats.add_file(file_stats);

        assert_eq!(dir_stats.total_files(), 1);
        assert_eq!(dir_stats.total_stats.function_count, 3);
        assert_eq!(dir_stats.total_stats.class_struct_count, 1);

        let rust_stats = &dir_stats.total_by_language[&SupportedLanguage::Rust];
        assert_eq!(rust_stats.file_count, 1);
        assert_eq!(rust_stats.function_count, 3);
        assert_eq!(rust_stats.class_struct_count, 1);
    }

    #[test]
    fn test_directory_stats_add_multiple_files_same_language() {
        let mut dir_stats = DirectoryStats::new();

        // Add first Rust file
        dir_stats.add_file(FileStats {
            path: PathBuf::from("file1.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 2,
                class_struct_count: 1,
            },
        });

        // Add second Rust file
        dir_stats.add_file(FileStats {
            path: PathBuf::from("file2.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 3,
                class_struct_count: 2,
            },
        });

        assert_eq!(dir_stats.total_files(), 2);
        assert_eq!(dir_stats.total_stats.function_count, 5);
        assert_eq!(dir_stats.total_stats.class_struct_count, 3);

        let rust_stats = &dir_stats.total_by_language[&SupportedLanguage::Rust];
        assert_eq!(rust_stats.file_count, 2);
        assert_eq!(rust_stats.function_count, 5);
        assert_eq!(rust_stats.class_struct_count, 3);
    }

    #[test]
    fn test_directory_stats_add_multiple_languages() {
        let mut dir_stats = DirectoryStats::new();

        // Add Rust file
        dir_stats.add_file(FileStats {
            path: PathBuf::from("main.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 4,
                class_struct_count: 2,
            },
        });

        // Add Python file
        dir_stats.add_file(FileStats {
            path: PathBuf::from("script.py"),
            language: SupportedLanguage::Python,
            stats: CodeStats {
                function_count: 3,
                class_struct_count: 1,
            },
        });

        // Add Go file
        dir_stats.add_file(FileStats {
            path: PathBuf::from("main.go"),
            language: SupportedLanguage::Go,
            stats: CodeStats {
                function_count: 2,
                class_struct_count: 1,
            },
        });

        assert_eq!(dir_stats.total_files(), 3);
        assert_eq!(dir_stats.total_stats.function_count, 9);
        assert_eq!(dir_stats.total_stats.class_struct_count, 4);
        assert_eq!(dir_stats.total_by_language.len(), 3);

        // Check individual language stats
        let rust_stats = &dir_stats.total_by_language[&SupportedLanguage::Rust];
        assert_eq!(rust_stats.file_count, 1);
        assert_eq!(rust_stats.function_count, 4);

        let python_stats = &dir_stats.total_by_language[&SupportedLanguage::Python];
        assert_eq!(python_stats.file_count, 1);
        assert_eq!(python_stats.function_count, 3);

        let go_stats = &dir_stats.total_by_language[&SupportedLanguage::Go];
        assert_eq!(go_stats.file_count, 1);
        assert_eq!(go_stats.function_count, 2);
    }

    #[test]
    fn test_language_stats_default() {
        let lang_stats = LanguageStats::default();

        assert_eq!(lang_stats.file_count, 0);
        assert_eq!(lang_stats.function_count, 0);
        assert_eq!(lang_stats.class_struct_count, 0);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let file_stats = FileStats {
            path: PathBuf::from("test.rs"),
            language: SupportedLanguage::Rust,
            stats: CodeStats {
                function_count: 10,
                class_struct_count: 5,
            },
        };

        // Serialize to JSON
        let json = serde_json::to_string(&file_stats).unwrap();

        // Deserialize back
        let deserialized: FileStats = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.path, file_stats.path);
        assert_eq!(deserialized.language, file_stats.language);
        assert_eq!(
            deserialized.stats.function_count,
            file_stats.stats.function_count
        );
        assert_eq!(
            deserialized.stats.class_struct_count,
            file_stats.stats.class_struct_count
        );
    }
}

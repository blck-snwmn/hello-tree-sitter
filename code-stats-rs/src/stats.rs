use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::language::SupportedLanguage;
use crate::parser::CodeStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub path: PathBuf,
    pub language: SupportedLanguage,
    pub stats: CodeStats,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DirectoryStats {
    pub files: Vec<FileStats>,
    pub total_by_language: HashMap<SupportedLanguage, LanguageStats>,
    pub total_stats: CodeStats,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LanguageStats {
    pub file_count: usize,
    pub function_count: usize,
    pub class_struct_count: usize,
}

impl DirectoryStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, file_stats: FileStats) {
        // Update total stats
        self.total_stats.function_count += file_stats.stats.function_count;
        self.total_stats.class_struct_count += file_stats.stats.class_struct_count;

        // Update language-specific stats
        let lang_stats = self.total_by_language
            .entry(file_stats.language.clone())
            .or_insert_with(LanguageStats::default);
        
        lang_stats.file_count += 1;
        lang_stats.function_count += file_stats.stats.function_count;
        lang_stats.class_struct_count += file_stats.stats.class_struct_count;

        // Add file to list
        self.files.push(file_stats);
    }

    pub fn total_files(&self) -> usize {
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
        assert_eq!(deserialized.stats.function_count, file_stats.stats.function_count);
        assert_eq!(deserialized.stats.class_struct_count, file_stats.stats.class_struct_count);
    }
}
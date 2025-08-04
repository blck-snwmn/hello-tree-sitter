use crate::error::{CodeStatsError, Result};
use crate::language::SupportedLanguage;
use crate::parser::{analyze_code, create_parser};
use crate::stats::{DirectoryStats, FileStats};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tree_sitter::Parser;
use walkdir::{DirEntry, WalkDir};

pub(crate) struct CodeAnalyzer {
    parsers: HashMap<SupportedLanguage, Parser>,
}

impl CodeAnalyzer {
    pub(crate) fn new() -> Self {
        Self {
            parsers: HashMap::new(),
        }
    }

    pub(crate) fn analyze_file(&mut self, path: &Path) -> Result<FileStats> {
        if !path.is_file() {
            return Err(CodeStatsError::IoError(format!(
                "{} is not a file",
                path.display()
            )));
        }

        let path_str = path.to_string_lossy();
        let language = SupportedLanguage::from_file_extension(&path_str)
            .ok_or_else(|| CodeStatsError::UnsupportedFileType(path_str.to_string()))?;

        let source_code = fs::read_to_string(path)
            .map_err(|e| CodeStatsError::IoError(format!("Failed to read {path_str}: {e}")))?;

        let parser = self.get_or_create_parser(&language)?;
        let code_stats = analyze_code(parser, &source_code, &path_str, &language)?;

        Ok(FileStats {
            path: path.to_path_buf(),
            language,
            stats: code_stats,
        })
    }

    pub(crate) fn analyze_directory(
        &mut self,
        path: &Path,
        max_depth: usize,
        follow_links: bool,
        ignore_patterns: &[String],
    ) -> Result<DirectoryStats> {
        let mut stats = DirectoryStats::new();
        let mut errors = Vec::new();

        let walker = WalkDir::new(path)
            .max_depth(max_depth)
            .follow_links(follow_links);

        for entry in walker {
            match entry {
                Ok(dir_entry) => {
                    if let Err(e) = self.process_entry(&dir_entry, &mut stats, ignore_patterns) {
                        errors.push(e);
                    }
                }
                Err(e) => {
                    errors.push(CodeStatsError::IoError(e.to_string()));
                }
            }
        }

        if !errors.is_empty() && stats.total_files() == 0 {
            // If no files were successfully processed, return the first error
            return Err(errors.into_iter().next().unwrap());
        }

        Ok(stats)
    }

    fn process_entry(
        &mut self,
        entry: &DirEntry,
        stats: &mut DirectoryStats,
        ignore_patterns: &[String],
    ) -> Result<()> {
        let path = entry.path();

        // Skip if not a file
        if !path.is_file() {
            return Ok(());
        }

        // Check if path matches any ignore pattern
        let path_str = path.to_string_lossy();
        for pattern in ignore_patterns {
            if path_str.contains(pattern) {
                return Ok(());
            }
        }

        // Check if it's a supported language
        let language = match SupportedLanguage::from_file_extension(&path_str) {
            Some(lang) => lang,
            None => return Ok(()), // Skip unsupported files silently
        };

        // Read and analyze the file
        let source_code = fs::read_to_string(path)
            .map_err(|e| CodeStatsError::IoError(format!("Failed to read {path_str}: {e}")))?;

        let parser = self.get_or_create_parser(&language)?;
        let code_stats = analyze_code(parser, &source_code, &path_str, &language)?;

        let file_stats = FileStats {
            path: path.to_path_buf(),
            language,
            stats: code_stats,
        };

        stats.add_file(file_stats);
        Ok(())
    }

    fn get_or_create_parser(&mut self, language: &SupportedLanguage) -> Result<&mut Parser> {
        if !self.parsers.contains_key(language) {
            let parser = create_parser(language)?;
            self.parsers.insert(*language, parser);
        }
        Ok(self.parsers.get_mut(language).unwrap())
    }
}

impl Default for CodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_analyze_file_rejects_directory_path_with_error() {
        let mut analyzer = CodeAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();
        
        let result = analyzer.analyze_file(temp_dir.path());
        assert!(matches!(
            result,
            Err(CodeStatsError::IoError(msg)) if msg.contains("is not a file")
        ));
    }

    #[test]
    fn test_analyze_file_returns_error_for_unsupported_file_types() {
        let mut analyzer = CodeAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();
        let txt_file = temp_dir.path().join("test.txt");
        std::fs::write(&txt_file, "content").unwrap();
        
        let result = analyzer.analyze_file(&txt_file);
        assert!(matches!(result, Err(CodeStatsError::UnsupportedFileType(_))));
    }

    #[test]
    fn test_default_trait_creates_empty_analyzer() {
        let analyzer = CodeAnalyzer::default();
        assert_eq!(analyzer.parsers.len(), 0);
    }

    #[test]
    fn test_parser_cache_reuses_parser_for_same_language() {
        let mut analyzer = CodeAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();
        
        // Rustファイルを2回解析してキャッシュを確認
        let rs_file = temp_dir.path().join("test.rs");
        std::fs::write(&rs_file, "fn main() {}").unwrap();
        
        analyzer.analyze_file(&rs_file).unwrap();
        assert_eq!(analyzer.parsers.len(), 1);
        assert!(analyzer.parsers.contains_key(&SupportedLanguage::Rust));
        
        // 2回目も成功し、パーサー数は増えない
        analyzer.analyze_file(&rs_file).unwrap();
        assert_eq!(analyzer.parsers.len(), 1);
    }

    #[test]
    fn test_analyze_file_returns_io_error_for_nonexistent_file() {
        let mut analyzer = CodeAnalyzer::new();
        let non_existent = Path::new("/non/existent/file.rs");
        
        let result = analyzer.analyze_file(non_existent);
        assert!(matches!(
            result,
            Err(CodeStatsError::IoError(msg)) if !msg.is_empty()
        ));
    }

    #[test]
    fn test_analyze_directory_succeeds_with_zero_files_when_all_unsupported() {
        let mut analyzer = CodeAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();
        
        // サポートされていないファイルのみ作成
        std::fs::write(temp_dir.path().join("file1.txt"), "text").unwrap();
        std::fs::write(temp_dir.path().join("file2.md"), "markdown").unwrap();
        
        let result = analyzer.analyze_directory(temp_dir.path(), 100, false, &[]);
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_files(), 0);
        assert_eq!(stats.total_stats.function_count, 0);
    }

    #[test]
    fn test_analyze_directory_excludes_files_matching_ignore_patterns() {
        let mut analyzer = CodeAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();
        
        // ファイルを作成
        std::fs::write(temp_dir.path().join("main.rs"), "fn main() {}").unwrap();
        std::fs::write(temp_dir.path().join("test.rs"), "fn test() {}").unwrap();
        
        // "test"を含むファイルを無視
        let result = analyzer.analyze_directory(temp_dir.path(), 100, false, &["test".to_string()]);
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_files(), 1);
        assert_eq!(stats.total_stats.function_count, 1);
    }
}

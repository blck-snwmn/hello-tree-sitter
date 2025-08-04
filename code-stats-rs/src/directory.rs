use crate::analyzer::CodeAnalyzer;
use crate::error::Result;
use crate::stats::{DirectoryStats, FileStats};
use std::path::Path;

pub fn analyze_directory(
    path: &Path,
    max_depth: usize,
    follow_links: bool,
    ignore_patterns: &[String],
) -> Result<DirectoryStats> {
    let mut analyzer = CodeAnalyzer::new();
    analyzer.analyze_directory(path, max_depth, follow_links, ignore_patterns)
}

pub fn analyze_single_file(path: &Path) -> Result<FileStats> {
    let mut analyzer = CodeAnalyzer::new();
    analyzer.analyze_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CodeStatsError;
    use crate::language::SupportedLanguage;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let file_path = dir.join(name);
        fs::write(&file_path, content).unwrap();
        file_path
    }

    #[test]
    fn test_analyze_single_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = create_test_file(
            temp_dir.path(),
            "test.rs",
            r#"
fn main() {
    println!("Hello");
}

struct TestStruct {
    field: i32,
}
"#,
        );

        let result = analyze_single_file(&file_path).unwrap();

        assert_eq!(result.language, SupportedLanguage::Rust);
        assert_eq!(result.stats.function_count, 1);
        assert_eq!(result.stats.class_struct_count, 1);
    }

    #[test]
    fn test_analyze_single_file_not_a_file() {
        let temp_dir = TempDir::new().unwrap();

        let result = analyze_single_file(temp_dir.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            CodeStatsError::IoError(msg) => assert!(msg.contains("is not a file")),
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_analyze_single_file_unsupported_type() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = create_test_file(temp_dir.path(), "test.txt", "Not code");

        let result = analyze_single_file(&file_path);

        assert!(result.is_err());
        match result.unwrap_err() {
            CodeStatsError::UnsupportedFileType(_) => {}
            _ => panic!("Expected UnsupportedFileType error"),
        }
    }

    #[test]
    fn test_analyze_directory_basic() {
        let temp_dir = TempDir::new().unwrap();

        create_test_file(temp_dir.path(), "file1.rs", "fn test1() {}");
        create_test_file(temp_dir.path(), "file2.rs", "fn test2() {} struct S {}");
        create_test_file(temp_dir.path(), "script.py", "def test(): pass");

        let result = analyze_directory(temp_dir.path(), 10, false, &[]).unwrap();

        assert_eq!(result.total_files(), 3);
        assert_eq!(result.total_stats.function_count, 3);
        assert_eq!(result.total_stats.class_struct_count, 1);
        assert_eq!(result.total_by_language.len(), 2); // Rust and Python
    }

    #[test]
    fn test_analyze_directory_with_subdirs() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("src");
        fs::create_dir(&sub_dir).unwrap();

        create_test_file(temp_dir.path(), "main.rs", "fn main() {}");
        create_test_file(&sub_dir, "lib.rs", "fn lib_fn() {}");

        let result = analyze_directory(temp_dir.path(), 10, false, &[]).unwrap();

        assert_eq!(result.total_files(), 2);
        assert_eq!(result.total_stats.function_count, 2);
    }

    #[test]
    fn test_analyze_directory_with_ignore_patterns() {
        let temp_dir = TempDir::new().unwrap();
        let ignored_dir = temp_dir.path().join("target");
        fs::create_dir(&ignored_dir).unwrap();

        create_test_file(temp_dir.path(), "main.rs", "fn main() {}");
        create_test_file(&ignored_dir, "ignored.rs", "fn ignored() {}");

        let ignore_patterns = vec!["target".to_string()];
        let result = analyze_directory(temp_dir.path(), 10, false, &ignore_patterns).unwrap();

        assert_eq!(result.total_files(), 1);
        assert_eq!(result.total_stats.function_count, 1);
    }

    #[test]
    fn test_analyze_directory_max_depth() {
        let temp_dir = TempDir::new().unwrap();
        let level1 = temp_dir.path().join("level1");
        let level2 = level1.join("level2");
        fs::create_dir_all(&level2).unwrap();

        create_test_file(temp_dir.path(), "root.rs", "fn root() {}");
        create_test_file(&level1, "level1.rs", "fn level1() {}");
        create_test_file(&level2, "level2.rs", "fn level2() {}");

        // Max depth of 2 should exclude level2
        let result = analyze_directory(temp_dir.path(), 2, false, &[]).unwrap();

        assert_eq!(result.total_files(), 2);
        assert_eq!(result.total_stats.function_count, 2);
    }

    #[test]
    fn test_empty_directory() {
        let temp_dir = TempDir::new().unwrap();

        let result = analyze_directory(temp_dir.path(), 10, false, &[]).unwrap();

        assert_eq!(result.total_files(), 0);
        assert_eq!(result.total_stats.function_count, 0);
        assert_eq!(result.total_stats.class_struct_count, 0);
    }

    #[test]
    fn test_mixed_file_types() {
        let temp_dir = TempDir::new().unwrap();

        create_test_file(temp_dir.path(), "code.rs", "fn test() {}");
        create_test_file(temp_dir.path(), "readme.txt", "Documentation");
        create_test_file(temp_dir.path(), "data.json", "{}");

        let result = analyze_directory(temp_dir.path(), 10, false, &[]).unwrap();

        // Should only count the Rust file
        assert_eq!(result.total_files(), 1);
        assert_eq!(result.files[0].language, SupportedLanguage::Rust);
    }
}

//! Language support definitions and file type detection using Magika.

use std::path::Path;
use tree_sitter::Language;

/// Enumeration of supported programming languages.
///
/// Each variant corresponds to a programming language that can be analyzed
/// by the code statistics tool. The enum is used throughout the codebase
/// to maintain type safety when dealing with language-specific operations.
///
/// # Supported File Extensions
///
/// - `Rust` - `.rs` files
/// - `Go` - `.go` files
/// - `Python` - `.py` files
/// - `JavaScript` - `.js` files
/// - `TypeScript` - `.ts` files
/// - `Java` - `.java` files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub(crate) enum SupportedLanguage {
    Rust,
    Go,
    Python,
    JavaScript,
    TypeScript,
    Java,
}

impl SupportedLanguage {
    /// Maps Magika's content type label to a supported language.
    ///
    /// # Arguments
    ///
    /// * `label` - The content type label returned by Magika
    ///
    /// # Returns
    ///
    /// * `Some(SupportedLanguage)` if the label matches a supported language
    /// * `None` if the label is not a supported programming language
    fn from_magika_label(label: &str) -> Option<Self> {
        match label {
            "rust" => Some(Self::Rust),
            "go" => Some(Self::Go),
            "python" => Some(Self::Python),
            "javascript" => Some(Self::JavaScript),
            "typescript" => Some(Self::TypeScript),
            "java" => Some(Self::Java),
            _ => None,
        }
    }

    /// Determines the programming language from a file path using AI-powered content detection.
    ///
    /// This function uses Magika to analyze the actual file content rather than relying
    /// solely on file extensions. This provides more accurate detection and supports:
    /// - Files without extensions (e.g., shell scripts with shebangs)
    /// - Files with incorrect or misleading extensions
    /// - Various extension variations (e.g., .jsx, .tsx, .mjs)
    ///
    /// If Magika cannot confidently detect the file type, this function falls back to
    /// extension-based detection for maximum compatibility.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A path to a file (can be absolute or relative)
    ///
    /// # Returns
    ///
    /// * `Some(SupportedLanguage)` if the content matches a supported language
    /// * `None` if the file cannot be detected or is not a supported language
    ///
    /// # Fallback Behavior
    ///
    /// If Magika fails to analyze the file or returns an unsupported language label,
    /// this function automatically falls back to extension-based detection.
    pub fn from_file_path(file_path: &str) -> Option<Self> {
        // Try AI-powered detection first
        let mut magika = match magika::Session::new() {
            Ok(session) => session,
            Err(_) => {
                // Magika initialization failed, fall back to extension-based detection
                return Self::from_file_extension(file_path);
            }
        };

        // Identify the file type using Magika
        let result = match magika.identify_file_sync(file_path) {
            Ok(inferred) => inferred,
            Err(_) => {
                // Magika detection failed, fall back to extension-based detection
                return Self::from_file_extension(file_path);
            }
        };

        // Get the label and try to map it to our supported languages
        let label = result.info().label;

        // If Magika successfully identified a supported language, use it
        if let Some(lang) = Self::from_magika_label(label) {
            return Some(lang);
        }

        // Magika detected something else (e.g., 'txt', 'unknown'),
        // fall back to extension-based detection
        Self::from_file_extension(file_path)
    }

    /// Determines the programming language from a file path based on its extension.
    ///
    /// This function performs case-insensitive matching of file extensions.
    /// It extracts the extension from the provided path and maps it to the
    /// corresponding `SupportedLanguage` variant.
    ///
    /// Used internally as a fallback when Magika cannot detect the file type.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A path to a file (can be absolute or relative)
    ///
    /// # Returns
    ///
    /// * `Some(SupportedLanguage)` if the extension matches a supported language
    /// * `None` if the file has no extension or the extension is not supported
    pub(crate) fn from_file_extension(file_path: &str) -> Option<Self> {
        // Extract extension, convert to string, then to lowercase for case-insensitive matching
        let extension = Path::new(file_path).extension()?.to_str()?.to_lowercase();

        match extension.as_str() {
            "rs" => Some(Self::Rust),
            "go" => Some(Self::Go),
            "py" => Some(Self::Python),
            "js" => Some(Self::JavaScript),
            "ts" => Some(Self::TypeScript),
            "java" => Some(Self::Java),
            _ => None,
        }
    }

    /// Returns the tree-sitter `Language` instance for this language.
    ///
    /// This method provides the bridge between our language enum and the
    /// tree-sitter parser infrastructure. Each language variant maps to
    /// its corresponding tree-sitter language definition.
    ///
    /// # Note
    ///
    /// TypeScript uses `LANGUAGE_TYPESCRIPT` instead of `LANGUAGE` because
    /// the tree-sitter-typescript crate provides separate language definitions
    /// for TypeScript and TSX, with TypeScript being the primary one.
    pub fn get_language(&self) -> Language {
        match self {
            Self::Rust => tree_sitter_rust::LANGUAGE.into(),
            Self::Go => tree_sitter_go::LANGUAGE.into(),
            Self::Python => tree_sitter_python::LANGUAGE.into(),
            Self::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
            Self::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Self::Java => tree_sitter_java::LANGUAGE.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file_extension_supported_languages() {
        assert!(matches!(
            SupportedLanguage::from_file_extension("main.rs"),
            Some(SupportedLanguage::Rust)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("main.go"),
            Some(SupportedLanguage::Go)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("main.py"),
            Some(SupportedLanguage::Python)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("main.js"),
            Some(SupportedLanguage::JavaScript)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("main.ts"),
            Some(SupportedLanguage::TypeScript)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("Main.java"),
            Some(SupportedLanguage::Java)
        ));
    }

    #[test]
    fn test_from_file_extension_case_insensitive() {
        assert!(matches!(
            SupportedLanguage::from_file_extension("MAIN.RS"),
            Some(SupportedLanguage::Rust)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("Main.Go"),
            Some(SupportedLanguage::Go)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("script.PY"),
            Some(SupportedLanguage::Python)
        ));
    }

    #[test]
    fn test_from_file_extension_with_path() {
        assert!(matches!(
            SupportedLanguage::from_file_extension("src/main.rs"),
            Some(SupportedLanguage::Rust)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("/usr/local/bin/script.py"),
            Some(SupportedLanguage::Python)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("./test/index.js"),
            Some(SupportedLanguage::JavaScript)
        ));
    }

    #[test]
    fn test_from_file_extension_unsupported() {
        assert_eq!(SupportedLanguage::from_file_extension("readme.txt"), None);
        assert_eq!(SupportedLanguage::from_file_extension("document.md"), None);
        assert_eq!(SupportedLanguage::from_file_extension("style.css"), None);
    }

    #[test]
    fn test_from_file_extension_no_extension() {
        assert_eq!(SupportedLanguage::from_file_extension("Makefile"), None);
        assert_eq!(SupportedLanguage::from_file_extension("README"), None);
        assert_eq!(SupportedLanguage::from_file_extension(""), None);
    }

    #[test]
    fn test_from_file_extension_multiple_dots() {
        assert!(matches!(
            SupportedLanguage::from_file_extension("test.spec.js"),
            Some(SupportedLanguage::JavaScript)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("app.module.ts"),
            Some(SupportedLanguage::TypeScript)
        ));
        assert!(matches!(
            SupportedLanguage::from_file_extension("Main.test.java"),
            Some(SupportedLanguage::Java)
        ));
    }

    #[test]
    fn test_get_language() {
        // Test that each language variant returns a valid Language instance
        let languages = vec![
            SupportedLanguage::Rust,
            SupportedLanguage::Go,
            SupportedLanguage::Python,
            SupportedLanguage::JavaScript,
            SupportedLanguage::TypeScript,
            SupportedLanguage::Java,
        ];

        for lang in languages {
            let _language = lang.get_language();
            // If this doesn't panic, the language is valid
        }
    }

    // Tests for AI-powered detection with fallback
    #[test]
    fn test_from_file_path_uses_extension_fallback_for_short_files() {
        use tempfile::NamedTempFile;

        // Create a very short Rust file that Magika might not detect
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().with_extension("rs");
        std::fs::write(&path, "fn main() {}").unwrap();

        // Should still detect as Rust via extension fallback
        let result = SupportedLanguage::from_file_path(path.to_str().unwrap());
        assert!(matches!(result, Some(SupportedLanguage::Rust)));
    }

    #[test]
    fn test_from_file_path_works_with_supported_extensions() {
        // Test that from_file_path works for all supported languages
        let test_cases = vec![
            ("test.rs", "fn main() {}", SupportedLanguage::Rust),
            ("test.py", "def main():\n    pass", SupportedLanguage::Python),
            ("test.js", "function main() {}", SupportedLanguage::JavaScript),
            ("test.go", "package main\nfunc main() {}", SupportedLanguage::Go),
            ("test.java", "public class Test {}", SupportedLanguage::Java),
            ("test.ts", "function main(): void {}", SupportedLanguage::TypeScript),
        ];

        for (filename, content, expected_lang) in test_cases {
            let temp_dir = tempfile::TempDir::new().unwrap();
            let file_path = temp_dir.path().join(filename);
            std::fs::write(&file_path, content).unwrap();

            let result = SupportedLanguage::from_file_path(file_path.to_str().unwrap());
            assert_eq!(result, Some(expected_lang), "Failed for {}", filename);
        }
    }

    #[test]
    fn test_from_file_path_returns_none_for_unsupported_types() {
        use tempfile::NamedTempFile;

        // Create a text file
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().with_extension("txt");
        std::fs::write(&path, "This is plain text").unwrap();

        // Should return None for unsupported file type
        let result = SupportedLanguage::from_file_path(path.to_str().unwrap());
        assert_eq!(result, None);
    }

    #[test]
    fn test_from_magika_label() {
        // Test the internal label mapping
        assert_eq!(
            SupportedLanguage::from_magika_label("rust"),
            Some(SupportedLanguage::Rust)
        );
        assert_eq!(
            SupportedLanguage::from_magika_label("python"),
            Some(SupportedLanguage::Python)
        );
        assert_eq!(
            SupportedLanguage::from_magika_label("javascript"),
            Some(SupportedLanguage::JavaScript)
        );
        assert_eq!(
            SupportedLanguage::from_magika_label("typescript"),
            Some(SupportedLanguage::TypeScript)
        );
        assert_eq!(
            SupportedLanguage::from_magika_label("go"),
            Some(SupportedLanguage::Go)
        );
        assert_eq!(
            SupportedLanguage::from_magika_label("java"),
            Some(SupportedLanguage::Java)
        );
        assert_eq!(SupportedLanguage::from_magika_label("txt"), None);
        assert_eq!(SupportedLanguage::from_magika_label("unknown"), None);
    }
}

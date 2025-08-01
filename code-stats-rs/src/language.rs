use std::path::Path;
use tree_sitter::Language;

#[derive(Debug, PartialEq)]
pub enum SupportedLanguage {
    Rust,
    Go,
    Python,
    JavaScript,
    TypeScript,
    Java,
}

impl SupportedLanguage {
    pub fn from_file_extension(file_path: &str) -> Option<Self> {
        let extension = Path::new(file_path)
            .extension()?
            .to_str()?
            .to_lowercase();
        
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
        assert!(matches!(SupportedLanguage::from_file_extension("main.rs"), Some(SupportedLanguage::Rust)));
        assert!(matches!(SupportedLanguage::from_file_extension("main.go"), Some(SupportedLanguage::Go)));
        assert!(matches!(SupportedLanguage::from_file_extension("main.py"), Some(SupportedLanguage::Python)));
        assert!(matches!(SupportedLanguage::from_file_extension("main.js"), Some(SupportedLanguage::JavaScript)));
        assert!(matches!(SupportedLanguage::from_file_extension("main.ts"), Some(SupportedLanguage::TypeScript)));
        assert!(matches!(SupportedLanguage::from_file_extension("Main.java"), Some(SupportedLanguage::Java)));
    }

    #[test]
    fn test_from_file_extension_case_insensitive() {
        assert!(matches!(SupportedLanguage::from_file_extension("MAIN.RS"), Some(SupportedLanguage::Rust)));
        assert!(matches!(SupportedLanguage::from_file_extension("Main.Go"), Some(SupportedLanguage::Go)));
        assert!(matches!(SupportedLanguage::from_file_extension("script.PY"), Some(SupportedLanguage::Python)));
    }

    #[test]
    fn test_from_file_extension_with_path() {
        assert!(matches!(SupportedLanguage::from_file_extension("src/main.rs"), Some(SupportedLanguage::Rust)));
        assert!(matches!(SupportedLanguage::from_file_extension("/usr/local/bin/script.py"), Some(SupportedLanguage::Python)));
        assert!(matches!(SupportedLanguage::from_file_extension("./test/index.js"), Some(SupportedLanguage::JavaScript)));
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
        assert!(matches!(SupportedLanguage::from_file_extension("test.spec.js"), Some(SupportedLanguage::JavaScript)));
        assert!(matches!(SupportedLanguage::from_file_extension("app.module.ts"), Some(SupportedLanguage::TypeScript)));
        assert!(matches!(SupportedLanguage::from_file_extension("Main.test.java"), Some(SupportedLanguage::Java)));
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
}
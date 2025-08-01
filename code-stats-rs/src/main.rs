use std::env;
use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Node, Language};

#[derive(Debug)]
enum SupportedLanguage {
    Rust,
    Go,
    Python,
    JavaScript,
    TypeScript,
    Java,
}

impl SupportedLanguage {
    fn from_file_extension(file_path: &str) -> Option<Self> {
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
    
    fn get_language(&self) -> Language {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    
    let language = match SupportedLanguage::from_file_extension(file_path) {
        Some(lang) => lang,
        None => {
            eprintln!("Unsupported file type. Supported extensions: .rs, .go, .py, .js, .ts, .java");
            std::process::exit(1);
        }
    };
    
    let source_code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    };
    
    let mut parser = Parser::new();
    parser
        .set_language(&language.get_language())
        .expect("Error loading language grammar");
        
    analyze_code(&mut parser, &source_code, file_path, &language);
}

fn analyze_code(parser: &mut Parser, source_code: &str, file_path: &str, language: &SupportedLanguage) {
    println!("Analyzing file: {} (Language: {:?})", file_path, language);
    
    let tree = match parser.parse(source_code, None) {
        Some(tree) => tree,
        None => {
            eprintln!("Error parsing file");
            return;
        }
    };
    
    let root_node = tree.root_node();
    let mut stats = CodeStats::new();
    
    count_nodes(&root_node, &mut stats, language);
    
    println!("Code Statistics:");
    println!("Functions: {}", stats.function_count);
    println!("Classes/Structs: {}", stats.class_struct_count);
}

#[derive(Default)]
struct CodeStats {
    function_count: usize,
    class_struct_count: usize,
}

impl CodeStats {
    fn new() -> Self {
        Self::default()
    }
}

fn count_nodes(node: &Node, stats: &mut CodeStats, language: &SupportedLanguage) {
    let node_kind = node.kind();
    
    match language {
        SupportedLanguage::Rust => {
            match node_kind {
                "function_item" => stats.function_count += 1,
                "struct_item" | "enum_item" => stats.class_struct_count += 1,
                _ => {}
            }
        }
        SupportedLanguage::Go => {
            match node_kind {
                "function_declaration" | "method_declaration" => stats.function_count += 1,
                "type_spec" => {
                    // Check if it's a struct type
                    if let Some(type_node) = node.child_by_field_name("type") {
                        if type_node.kind() == "struct_type" {
                            stats.class_struct_count += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        SupportedLanguage::Python => {
            match node_kind {
                "function_definition" => stats.function_count += 1,
                "class_definition" => stats.class_struct_count += 1,
                _ => {}
            }
        }
        SupportedLanguage::JavaScript | SupportedLanguage::TypeScript => {
            match node_kind {
                "function_declaration" | "function_expression" | "arrow_function" | "method_definition" => {
                    stats.function_count += 1;
                }
                "class_declaration" => stats.class_struct_count += 1,
                _ => {}
            }
        }
        SupportedLanguage::Java => {
            match node_kind {
                "method_declaration" | "constructor_declaration" => stats.function_count += 1,
                "class_declaration" | "interface_declaration" => stats.class_struct_count += 1,
                _ => {}
            }
        }
    }
    
    // Recursively traverse child nodes
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        count_nodes(&child, stats, language);
    }
}

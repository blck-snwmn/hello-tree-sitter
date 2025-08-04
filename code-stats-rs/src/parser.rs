use crate::error::{CodeStatsError, Result};
use crate::language::SupportedLanguage;
use tree_sitter::{Node, Parser};

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct CodeStats {
    pub function_count: usize,
    pub class_struct_count: usize,
}

impl CodeStats {
    pub fn new() -> Self {
        Self::default()
    }
}

pub(crate) fn create_parser(language: &SupportedLanguage) -> Result<Parser> {
    let mut parser = Parser::new();
    parser
        .set_language(&language.get_language())
        .map_err(|_| CodeStatsError::LanguageSetupError)?;
    Ok(parser)
}

pub(crate) fn analyze_code(
    parser: &mut Parser,
    source_code: &str,
    file_path: &str,
    language: &SupportedLanguage,
) -> Result<CodeStats> {
    let tree = parser
        .parse(source_code, None)
        .ok_or_else(|| CodeStatsError::ParseError(file_path.to_string()))?;

    let root_node = tree.root_node();
    let mut stats = CodeStats::new();

    count_nodes(&root_node, &mut stats, language);

    Ok(stats)
}

fn count_nodes(node: &Node, stats: &mut CodeStats, language: &SupportedLanguage) {
    let node_kind = node.kind();

    match language {
        SupportedLanguage::Rust => match node_kind {
            "function_item" => stats.function_count += 1,
            "struct_item" | "enum_item" => stats.class_struct_count += 1,
            _ => {}
        },
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
        SupportedLanguage::Python => match node_kind {
            "function_definition" => stats.function_count += 1,
            "class_definition" => stats.class_struct_count += 1,
            _ => {}
        },
        SupportedLanguage::JavaScript | SupportedLanguage::TypeScript => match node_kind {
            "function_declaration"
            | "function_expression"
            | "arrow_function"
            | "method_definition" => {
                stats.function_count += 1;
            }
            "class_declaration" => stats.class_struct_count += 1,
            _ => {}
        },
        SupportedLanguage::Java => match node_kind {
            "method_declaration" | "constructor_declaration" => stats.function_count += 1,
            "class_declaration" | "interface_declaration" => stats.class_struct_count += 1,
            _ => {}
        },
    }

    // Recursively traverse child nodes
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        count_nodes(&child, stats, language);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_stats_new() {
        let stats = CodeStats::new();
        assert_eq!(stats.function_count, 0);
        assert_eq!(stats.class_struct_count, 0);
    }

    #[test]
    fn test_code_stats_default() {
        let stats = CodeStats::default();
        assert_eq!(stats.function_count, 0);
        assert_eq!(stats.class_struct_count, 0);
    }

    #[test]
    fn test_create_parser_all_languages() {
        let languages = vec![
            SupportedLanguage::Rust,
            SupportedLanguage::Go,
            SupportedLanguage::Python,
            SupportedLanguage::JavaScript,
            SupportedLanguage::TypeScript,
            SupportedLanguage::Java,
        ];

        for lang in languages {
            let parser = create_parser(&lang);
            assert!(parser.is_ok(), "Failed to create parser for {:?}", lang);
        }
    }

    #[test]
    fn test_analyze_code_rust() {
        let rust_code = r#"
fn main() {
    println!("Hello, world!");
}

fn helper() {
    // Helper function
}

struct Person {
    name: String,
}

enum Status {
    Active,
    Inactive,
}
"#;

        let language = SupportedLanguage::Rust;
        let mut parser = create_parser(&language).unwrap();
        let stats = analyze_code(&mut parser, rust_code, "test.rs", &language).unwrap();

        assert_eq!(stats.function_count, 2);
        assert_eq!(stats.class_struct_count, 2);
    }

    #[test]
    fn test_analyze_code_python() {
        let python_code = r#"
def main():
    print("Hello, world!")

def helper():
    pass

class Person:
    def __init__(self, name):
        self.name = name
    
    def greet(self):
        print(f"Hello, {self.name}")

class Animal:
    pass
"#;

        let language = SupportedLanguage::Python;
        let mut parser = create_parser(&language).unwrap();
        let stats = analyze_code(&mut parser, python_code, "test.py", &language).unwrap();

        assert_eq!(stats.function_count, 4); // main, helper, __init__, greet
        assert_eq!(stats.class_struct_count, 2); // Person, Animal
    }

    #[test]
    fn test_analyze_code_javascript() {
        let js_code = r#"
function main() {
    console.log("Hello, world!");
}

const helper = function() {
    // Helper function
};

const arrow = () => {
    return 42;
};

class Person {
    constructor(name) {
        this.name = name;
    }
    
    greet() {
        console.log(`Hello, ${this.name}`);
    }
}
"#;

        let language = SupportedLanguage::JavaScript;
        let mut parser = create_parser(&language).unwrap();
        let stats = analyze_code(&mut parser, js_code, "test.js", &language).unwrap();

        assert_eq!(stats.function_count, 5); // main, helper, arrow, constructor, greet
        assert_eq!(stats.class_struct_count, 1); // Person
    }

    #[test]
    fn test_analyze_code_go() {
        let go_code = r#"
package main

func main() {
    fmt.Println("Hello, world!")
}

func helper() {
    // Helper function
}

type Person struct {
    Name string
}

func (p Person) Greet() {
    fmt.Printf("Hello, %s\n", p.Name)
}
"#;

        let language = SupportedLanguage::Go;
        let mut parser = create_parser(&language).unwrap();
        let stats = analyze_code(&mut parser, go_code, "test.go", &language).unwrap();

        assert_eq!(stats.function_count, 3); // main, helper, Greet
        assert_eq!(stats.class_struct_count, 1); // Person
    }

    #[test]
    fn test_analyze_code_java() {
        let java_code = r#"
public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
    
    private void helper() {
        // Helper method
    }
    
    public Main() {
        // Constructor
    }
}

interface Runnable {
    void run();
}
"#;

        let language = SupportedLanguage::Java;
        let mut parser = create_parser(&language).unwrap();
        let stats = analyze_code(&mut parser, java_code, "Main.java", &language).unwrap();

        assert_eq!(stats.function_count, 4); // main, helper, constructor, run (interface method)
        assert_eq!(stats.class_struct_count, 2); // Main, Runnable
    }

    #[test]
    fn test_analyze_code_empty() {
        let languages = vec![
            SupportedLanguage::Rust,
            SupportedLanguage::Go,
            SupportedLanguage::Python,
            SupportedLanguage::JavaScript,
            SupportedLanguage::TypeScript,
            SupportedLanguage::Java,
        ];

        for lang in languages {
            let mut parser = create_parser(&lang).unwrap();
            let stats = analyze_code(&mut parser, "", "empty.file", &lang).unwrap();
            assert_eq!(stats.function_count, 0);
            assert_eq!(stats.class_struct_count, 0);
        }
    }

    #[test]
    fn test_analyze_code_nested_functions() {
        let js_code = r#"
function outer() {
    function inner() {
        const innerArrow = () => {
            return 42;
        };
        return innerArrow;
    }
    return inner;
}
"#;

        let language = SupportedLanguage::JavaScript;
        let mut parser = create_parser(&language).unwrap();
        let stats = analyze_code(&mut parser, js_code, "nested.js", &language).unwrap();

        assert_eq!(stats.function_count, 3); // outer, inner, innerArrow
    }

    #[test]
    fn test_analyze_code_comments_ignored() {
        let rust_code = r#"
// fn commented_function() {}
/* fn another_commented() {} */

fn actual_function() {
    // This is a real function
}

// struct CommentedStruct {}
"#;

        let language = SupportedLanguage::Rust;
        let mut parser = create_parser(&language).unwrap();
        let stats = analyze_code(&mut parser, rust_code, "comments.rs", &language).unwrap();

        assert_eq!(stats.function_count, 1);
        assert_eq!(stats.class_struct_count, 0);
    }
}

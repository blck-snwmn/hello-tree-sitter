use std::env;
use std::fs;
use tree_sitter::{Parser, Node};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    
    let source_code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            std::process::exit(1);
        }
    };
    
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar");
        
    analyze_code(&mut parser, &source_code, file_path);
}

fn analyze_code(parser: &mut Parser, source_code: &str, file_path: &str) {
    println!("Analyzing file: {}", file_path);
    
    let tree = match parser.parse(source_code, None) {
        Some(tree) => tree,
        None => {
            eprintln!("Error parsing file");
            return;
        }
    };
    
    let root_node = tree.root_node();
    let mut stats = CodeStats::new();
    
    count_nodes(&root_node, &mut stats, source_code.as_bytes());
    
    println!("Code Statistics:");
    println!("Functions: {}", stats.function_count);
    println!("Structs: {}", stats.struct_count);
}

#[derive(Default)]
struct CodeStats {
    function_count: usize,
    struct_count: usize,
}

impl CodeStats {
    fn new() -> Self {
        Self::default()
    }
}

fn count_nodes(node: &Node, stats: &mut CodeStats, source: &[u8]) {
    match node.kind() {
        "function_item" => {
            stats.function_count += 1;
        }
        "struct_item" => {
            stats.struct_count += 1;
        }
        _ => {}
    }
    
    // Recursively traverse child nodes
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        count_nodes(&child, stats, source);
    }
}

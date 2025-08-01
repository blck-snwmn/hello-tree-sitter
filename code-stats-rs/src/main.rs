use std::env;
use std::fs;
use code_stats_rs::{SupportedLanguage, analyze_code, create_parser};

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
    
    let mut parser = match create_parser(&language) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
        
    println!("Analyzing file: {} (Language: {:?})", file_path, language);
    
    match analyze_code(&mut parser, &source_code, file_path, &language) {
        Ok(stats) => {
            println!("Code Statistics:");
            println!("Functions: {}", stats.function_count);
            println!("Classes/Structs: {}", stats.class_struct_count);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

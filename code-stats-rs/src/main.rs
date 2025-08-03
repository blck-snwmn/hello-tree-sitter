use clap::Parser;
use code_stats_rs::cli::{Cli, OutputFormat};
use code_stats_rs::directory::{analyze_directory, analyze_single_file};
use code_stats_rs::formatter::{format_output, format_single_file};

fn main() {
    let cli = Cli::parse();

    let result = if cli.path.is_file() {
        // Single file analysis
        match analyze_single_file(&cli.path) {
            Ok(file_stats) => {
                println!("{}", format_single_file(&file_stats));
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else if cli.path.is_dir() {
        // Directory analysis
        match analyze_directory(&cli.path, cli.max_depth, cli.follow_links, &cli.ignore) {
            Ok(stats) => {
                // Determine output format
                let format = if cli.detail && cli.format == OutputFormat::Summary {
                    OutputFormat::Detail
                } else {
                    cli.format
                };

                println!("{}", format_output(&stats, format, cli.detail));
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else {
        eprintln!(
            "Error: {} is neither a file nor a directory",
            cli.path.display()
        );
        std::process::exit(1);
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

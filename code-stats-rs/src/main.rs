//! Entry point for the code-stats-rs command-line tool.

use clap::Parser;
use code_stats_rs::cli::Cli;

/// Main entry point for the code statistics analyzer.
///
/// Parses command-line arguments and executes the analysis.
/// Exits with status code 1 if an error occurs.
fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

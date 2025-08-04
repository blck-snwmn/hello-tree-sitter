//! Code statistics analyzer using tree-sitter for multi-language support.
//!
//! This crate provides a command-line tool for analyzing source code files
//! and extracting statistics about functions and class/struct definitions
//! across multiple programming languages.
//!
//! # Architecture
//!
//! The crate is organized into several modules:
//!
//! - `analyzer` - Core analysis engine that orchestrates parsing and statistics collection
//! - `cli` - Command-line interface and argument parsing
//! - `error` - Error types and handling
//! - `formatter` - Output formatting for different display modes
//! - `language` - Language detection and configuration
//! - `parser` - Tree-sitter integration and AST traversal
//! - `stats` - Data structures for storing analysis results
//!
//! See the `language` module for supported programming languages.

/// Core analysis engine for processing files and directories.
mod analyzer;

/// Command-line interface definitions and execution logic.
pub mod cli;

/// Error types and result definitions.
mod error;

/// Output formatting utilities for different display modes.
mod formatter;

/// Language detection and tree-sitter language configuration.
mod language;

/// Tree-sitter parsing and AST analysis.
mod parser;

/// Statistics data structures for storing analysis results.
mod stats;

use std::process::Command;
use std::path::PathBuf;

fn get_binary_path() -> PathBuf {
    // Get the path to the compiled binary
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.push("code-stats-rs");
    path
}

#[test]
fn test_rust_file_analysis() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("tests/fixtures/test.rs")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(output.status.success(), "Command failed with stderr: {}", stderr);
    assert!(stdout.contains("Language: Rust"));
    assert!(stdout.contains("Functions: 5"));
    assert!(stdout.contains("Classes/Structs: 2"));
}

#[test]
fn test_python_file_analysis() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("tests/fixtures/test.py")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(output.status.success(), "Command failed with stderr: {}", stderr);
    assert!(stdout.contains("Language: Python"));
    assert!(stdout.contains("Functions: 4"));
    assert!(stdout.contains("Classes/Structs: 1"));
}

#[test]
fn test_go_file_analysis() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("tests/fixtures/test.go")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(output.status.success(), "Command failed with stderr: {}", stderr);
    assert!(stdout.contains("Language: Go"));
    assert!(stdout.contains("Functions: 3"));
    assert!(stdout.contains("Classes/Structs: 1"));
}

#[test]
fn test_javascript_file_analysis() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("tests/fixtures/test.js")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(output.status.success(), "Command failed with stderr: {}", stderr);
    assert!(stdout.contains("Language: JavaScript"));
    assert!(stdout.contains("Functions: 5"));
    assert!(stdout.contains("Classes/Structs: 1"));
}

#[test]
fn test_unsupported_file_type() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("tests/fixtures/unsupported.txt")
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(!output.status.success());
    assert!(stderr.contains("Unsupported file type"));
}

#[test]
fn test_missing_file() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("tests/fixtures/nonexistent.rs")
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(!output.status.success());
    assert!(stderr.contains("Error reading file"));
}

#[test]
fn test_missing_argument() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(!output.status.success());
    assert!(stderr.contains("Usage:"));
}
use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn get_fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

#[test]
fn test_rust_file_analysis() {
    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    let fixture = get_fixtures_path().join("test.rs");

    cmd.arg(fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("Language: Rust"))
        .stdout(predicate::str::contains("Functions: 5"))
        .stdout(predicate::str::contains("Classes/Structs: 2"));
}

#[test]
fn test_python_file_analysis() {
    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    let fixture = get_fixtures_path().join("test.py");

    cmd.arg(fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("Language: Python"))
        .stdout(predicate::str::contains("Functions: 4"))
        .stdout(predicate::str::contains("Classes/Structs: 1"));
}

#[test]
fn test_go_file_analysis() {
    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    let fixture = get_fixtures_path().join("test.go");

    cmd.arg(fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("Language: Go"))
        .stdout(predicate::str::contains("Functions: 3"))
        .stdout(predicate::str::contains("Classes/Structs: 1"));
}

#[test]
fn test_javascript_file_analysis() {
    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    let fixture = get_fixtures_path().join("test.js");

    cmd.arg(fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("Language: JavaScript"))
        .stdout(predicate::str::contains("Functions: 5"))
        .stdout(predicate::str::contains("Classes/Structs: 1"));
}

#[test]
fn test_typescript_file_analysis() {
    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    let fixture = get_fixtures_path().join("test.ts");

    cmd.arg(fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("Language: TypeScript"))
        .stdout(predicate::str::contains("Functions: 7"))
        .stdout(predicate::str::contains("Classes/Structs: 1"));
}

#[test]
fn test_java_file_analysis() {
    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    let fixture = get_fixtures_path().join("test.java");

    cmd.arg(fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("Language: Java"))
        .stdout(predicate::str::contains("Functions: 8"))
        .stdout(predicate::str::contains("Classes/Structs: 4"));
}

#[test]
fn test_unsupported_file_type() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let unsupported_file = temp_dir.path().join("unsupported.txt");
    std::fs::write(&unsupported_file, "This is not a code file").unwrap();

    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    cmd.arg(unsupported_file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported file type"));
}

#[test]
fn test_missing_file() {
    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    cmd.arg("tests/fixtures/nonexistent.rs")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn test_empty_file() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let empty_file = temp_dir.path().join("empty.rs");
    std::fs::write(&empty_file, "").unwrap();

    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    cmd.arg(empty_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Functions: 0"))
        .stdout(predicate::str::contains("Classes/Structs: 0"));
}

#[test]
fn test_file_with_syntax_errors() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let invalid_file = temp_dir.path().join("invalid.rs");
    std::fs::write(&invalid_file, "fn test( { // syntax error").unwrap();

    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    cmd.arg(invalid_file)
        .assert()
        .success() // Should still succeed even with syntax errors
        .stdout(predicate::str::contains("Functions: 0")); // Tree-sitter won't count invalid syntax
}

#[test]
fn test_file_with_unicode() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let unicode_file = temp_dir.path().join("unicode.rs");
    std::fs::write(
        &unicode_file,
        r#"
// 日本語のコメント
fn こんにちは() {
    println!("Hello, 世界!");
}

struct 構造体 {
    フィールド: String,
}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    cmd.arg(unicode_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Functions: 1"))
        .stdout(predicate::str::contains("Classes/Structs: 1"));
}

#[test]
fn test_large_file() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let large_file = temp_dir.path().join("large.rs");

    // Generate a large file with many functions
    let mut content = String::new();
    for i in 0..1000 {
        content.push_str(&format!("fn function_{}() {{}}\n", i));
    }
    std::fs::write(&large_file, content).unwrap();

    let mut cmd = Command::cargo_bin("code-stats-rs").unwrap();
    cmd.arg(large_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Functions: 1000"))
        .stdout(predicate::str::contains("Classes/Structs: 0"));
}

mod common;

use assert_cmd::Command;
use common::{create_test_file, create_test_project};
use predicates::prelude::*;

#[test]
fn test_help_message() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Analyze code statistics"))
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Options:"))
        .stdout(predicate::str::contains("--format"))
        .stdout(predicate::str::contains("--detail"))
        .stdout(predicate::str::contains("--ignore"))
        .stdout(predicate::str::contains("--follow-links"))
        .stdout(predicate::str::contains("--max-depth"));
}

#[test]
fn test_version() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    // Version flag is not supported by default in clap v4
    cmd.arg("--version")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}

#[test]
fn test_missing_path_argument() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required arguments"));
}

#[test]
fn test_invalid_format_option() {
    let temp_dir = tempfile::TempDir::new().unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(temp_dir.path())
        .arg("--format")
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("possible values"));
}

#[test]
fn test_multiple_ignore_patterns() {
    let (_temp_dir, project_root) = create_test_project();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(project_root)
        .arg("--ignore")
        .arg(".git")
        .arg("--ignore")
        .arg("tests")
        .arg("--ignore")
        .arg("target")
        .assert()
        .success();
}

#[test]
fn test_max_depth_validation() {
    let temp_dir = tempfile::TempDir::new().unwrap();

    // Valid max-depth
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(temp_dir.path())
        .arg("--max-depth")
        .arg("5")
        .assert()
        .success();

    // Invalid max-depth (not a number)
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(temp_dir.path())
        .arg("--max-depth")
        .arg("abc")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn test_conflicting_format_options() {
    let temp_dir = tempfile::TempDir::new().unwrap();

    // --detail should override --format summary
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(temp_dir.path())
        .arg("--format")
        .arg("summary")
        .arg("--detail")
        .assert()
        .success();
}

#[test]
fn test_path_with_spaces() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let dir_with_spaces = temp_dir.path().join("dir with spaces");
    std::fs::create_dir(&dir_with_spaces).unwrap();

    create_test_file(&dir_with_spaces.join("test.rs"), "fn test() {}");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(&dir_with_spaces)
        .assert()
        .success()
        .stdout(predicate::str::contains("1 functions"));
}

#[test]
fn test_relative_path() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    create_test_file(&test_file, "fn test() {}");

    // Change to temp directory and use relative path
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.current_dir(temp_dir.path())
        .arg("test.rs")
        .assert()
        .success()
        .stdout(predicate::str::contains("Functions: 1"));
}

#[test]
fn test_absolute_path() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    create_test_file(&test_file, "fn test() {}");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Functions: 1"));
}

#[test]
fn test_nonexistent_path() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg("/this/path/does/not/exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn test_file_vs_directory_detection() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let test_file = temp_dir.path().join("single.rs");
    create_test_file(&test_file, "fn test() {}");

    // Test file - should show single file format
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(&test_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Analyzing file:"))
        .stdout(predicate::str::contains("Code Statistics:"));

    // Test directory - should show summary format
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Language Summary:"))
        .stdout(predicate::str::contains("Total:"));
}

#[test]
fn test_stdin_not_supported() {
    // Test that we properly handle when no path is provided
    // (stdin input is not supported)
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_all_options_combined() {
    let (_temp_dir, project_root) = create_test_project();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.arg(project_root)
        .arg("--format")
        .arg("json")
        .arg("--ignore")
        .arg(".git")
        .arg("--ignore")
        .arg("target")
        .arg("--max-depth")
        .arg("3")
        .arg("--follow-links")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r#"\{[\s\S]*"files"[\s\S]*\}"#).unwrap());
}

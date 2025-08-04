mod common;

use common::{
    assert_contains_all, create_controlled_test_project, create_symlink, create_test_file,
    create_test_project, run_code_stats,
};
use std::fs;

#[test]
fn test_directory_analysis_basic() {
    let (_temp_dir, project_root) = create_test_project();

    let output = run_code_stats(&[project_root.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        stderr
    );

    // Check summary format output
    assert!(stdout.contains("Language Summary:"));
    assert!(stdout.contains("Total:"));

    // Verify we found files in multiple languages
    assert!(stdout.contains("Rust:"));
    assert!(stdout.contains("Python:"));
    assert!(stdout.contains("JavaScript:"));
    assert!(stdout.contains("TypeScript:"));
    assert!(stdout.contains("Go:"));
    assert!(stdout.contains("Java:"));
}

#[test]
fn test_directory_analysis_with_controlled_counts() {
    let (_temp_dir, project_root) = create_controlled_test_project();

    let output = run_code_stats(&[project_root.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());

    // Rust: 3 functions (2 + 1), 4 structs/classes (1 + 1 struct + 1 enum)
    assert!(stdout.contains("Rust:"));
    assert!(
        stdout.contains("3 functions") && stdout.contains("4 structs/classes"),
        "Unexpected Rust counts in output:\n{}",
        stdout
    );

    // Python: 2 functions, 1 class
    assert!(stdout.contains("Python:"));
    assert!(
        stdout.contains("2 functions") && stdout.contains("1 structs/classes"),
        "Unexpected Python counts in output:\n{}",
        stdout
    );
}

#[test]
fn test_ignore_patterns() {
    let (_temp_dir, project_root) = create_test_project();

    // Test with .git ignored
    let output = run_code_stats(&[project_root.to_str().unwrap(), "--ignore", ".git"]);
    let _stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());

    // The .git/config.rs file should be ignored, so we shouldn't see it in detailed output
    let output_detail = run_code_stats(&[
        project_root.to_str().unwrap(),
        "--ignore",
        ".git",
        "--detail",
    ]);
    let stdout_detail = String::from_utf8_lossy(&output_detail.stdout);

    assert!(!stdout_detail.contains(".git/config.rs"));

    // Test with multiple ignore patterns
    let output_multi = run_code_stats(&[
        project_root.to_str().unwrap(),
        "--ignore",
        ".git",
        "--ignore",
        "tests",
    ]);
    let stdout_multi = String::from_utf8_lossy(&output_multi.stdout);

    assert!(output_multi.status.success());
    // With tests ignored, we should have fewer files
    assert!(stdout_multi.contains("Total:"));
}

#[test]
fn test_max_depth_option() {
    let (_temp_dir, project_root) = create_test_project();

    // Test with max-depth 1 (should only get root level files)
    let output = run_code_stats(&[project_root.to_str().unwrap(), "--max-depth", "1"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());

    // Should find Go and Java files at root level
    assert!(stdout.contains("Go:"));
    assert!(stdout.contains("Java:"));

    // Run with details to verify no nested files
    let output_detail = run_code_stats(&[
        project_root.to_str().unwrap(),
        "--max-depth",
        "1",
        "--detail",
    ]);
    let stdout_detail = String::from_utf8_lossy(&output_detail.stdout);

    // Should not contain files from subdirectories
    assert!(!stdout_detail.contains("src/main.rs"));
    assert!(!stdout_detail.contains("python/main.py"));
    assert!(!stdout_detail.contains("js/app.js"));
}

#[test]
fn test_detail_format() {
    let (_temp_dir, project_root) = create_controlled_test_project();

    let output = run_code_stats(&[project_root.to_str().unwrap(), "--detail"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());

    // Should show individual file details
    assert_contains_all(
        &stdout,
        &[
            "file1.rs",
            "file2.rs",
            "script.py",
            "Functions:",
            "Structs/Classes:",
            "Language Summary:",
        ],
    );
}

#[test]
fn test_empty_directory() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let empty_dir = temp_dir.path().join("empty");
    fs::create_dir(&empty_dir).unwrap();

    let output = run_code_stats(&[empty_dir.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Total: 0 functions, 0 structs/classes in 0 files"));
}

#[test]
fn test_nested_directories() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create deeply nested structure
    let deep_path = root.join("level1/level2/level3/level4");
    fs::create_dir_all(&deep_path).unwrap();

    create_test_file(
        &deep_path.join("deep.rs"),
        r#"
fn deep_function() {}
struct DeepStruct {}
"#,
    );

    create_test_file(
        &root.join("shallow.rs"),
        r#"
fn shallow_function() {}
"#,
    );

    let output = run_code_stats(&[root.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("2 functions"));
    assert!(stdout.contains("1 structs/classes"));
}

#[test]
fn test_mixed_file_types() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create various file types
    create_test_file(&root.join("code.rs"), "fn test() {}");
    create_test_file(&root.join("readme.txt"), "This is not code");
    create_test_file(&root.join("data.json"), r#"{"key": "value"}"#);
    create_test_file(&root.join("script.sh"), "#!/bin/bash\necho 'hello'");

    let output = run_code_stats(&[root.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    // Should only count the Rust file
    assert!(stdout.contains("Rust:"));
    assert!(stdout.contains("1 functions"));
    assert!(stdout.contains("in 1 files"));
}

#[test]
#[cfg(unix)]
fn test_follow_links_option() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create a source directory
    let source_dir = root.join("source");
    fs::create_dir(&source_dir).unwrap();
    create_test_file(&source_dir.join("linked.rs"), "fn linked_function() {}");

    // Create a symlink
    let link_path = root.join("link_to_source");
    create_symlink(&source_dir, &link_path);

    // Without --follow-links, symlink should be ignored
    let output_no_follow = run_code_stats(&[root.to_str().unwrap()]);
    let stdout_no_follow = String::from_utf8_lossy(&output_no_follow.stdout);

    assert!(output_no_follow.status.success());
    assert!(stdout_no_follow.contains("1 functions"));

    // With --follow-links, symlink should be followed
    let output_follow = run_code_stats(&[root.to_str().unwrap(), "--follow-links"]);
    let stdout_follow = String::from_utf8_lossy(&output_follow.stdout);

    assert!(output_follow.status.success());
    // Should count the file twice (once in source, once through link)
    assert!(stdout_follow.contains("2 functions"));
}

#[test]
fn test_directory_with_errors() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create a valid file
    create_test_file(&root.join("valid.rs"), "fn valid() {}");

    // Create a file with invalid UTF-8 (simulate corrupted file)
    let invalid_file = root.join("invalid.rs");
    fs::write(&invalid_file, [0xFF, 0xFE, 0xFF, 0xFF]).unwrap();

    let output = run_code_stats(&[root.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should still succeed and process the valid file
    assert!(output.status.success());
    assert!(stdout.contains("1 functions"));
}

#[test]
fn test_directory_not_found() {
    let output = run_code_stats(&["/nonexistent/directory/path"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(
        stderr.contains("Error:") && stderr.contains("directory"),
        "Expected directory error in stderr: {}",
        stderr
    );
}

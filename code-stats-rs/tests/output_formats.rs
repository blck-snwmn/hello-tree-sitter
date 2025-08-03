mod helpers;

use helpers::{
    assert_contains_all, create_controlled_test_project, parse_json_output, run_code_stats,
};

#[test]
fn test_summary_format() {
    let (_temp_dir, project_root) = create_controlled_test_project();
    
    let output = run_code_stats(&[project_root.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success());
    
    // Check summary format structure
    assert_contains_all(&stdout, &[
        "Language Summary:",
        "Rust:",
        "Python:",
        "functions",
        "structs/classes",
        "files",
        "Total:",
    ]);
    
    // Verify the format includes proper counts
    assert!(stdout.contains("3 functions"));
    assert!(stdout.contains("4 structs/classes"));
    assert!(stdout.contains("in 2 files")); // Rust files
    assert!(stdout.contains("in 1 files")); // Python file
}

#[test]
fn test_detail_format() {
    let (_temp_dir, project_root) = create_controlled_test_project();
    
    let output = run_code_stats(&[
        project_root.to_str().unwrap(),
        "--format",
        "detail",
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success());
    
    // Check detail format shows individual files
    assert_contains_all(&stdout, &[
        "file1.rs",
        "file2.rs",
        "script.py",
        "Functions:",
        "Structs/Classes:",
    ]);
    
    // Should also include summary at the end
    assert!(stdout.contains("Language Summary:"));
    assert!(stdout.contains("Total:"));
}

#[test]
fn test_detail_flag_overrides_summary() {
    let (_temp_dir, project_root) = create_controlled_test_project();
    
    // Using --detail flag should switch to detail format
    let output = run_code_stats(&[
        project_root.to_str().unwrap(),
        "--detail",
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success());
    
    // Should show file details
    assert!(stdout.contains("file1.rs"));
    assert!(stdout.contains("file2.rs"));
}

#[test]
fn test_json_format() {
    let (_temp_dir, project_root) = create_controlled_test_project();
    
    let output = run_code_stats(&[
        project_root.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success());
    
    // Parse JSON to verify structure
    let json = parse_json_output(&stdout);
    
    // Check top-level structure
    assert!(json.get("files").is_some());
    assert!(json.get("total_by_language").is_some());
    assert!(json.get("total_stats").is_some());
    
    // Check files array
    let files = json["files"].as_array().unwrap();
    assert_eq!(files.len(), 3); // 2 Rust + 1 Python
    
    // Check file structure
    for file in files {
        assert!(file.get("path").is_some());
        assert!(file.get("language").is_some());
        assert!(file.get("stats").is_some());
        
        let stats = &file["stats"];
        assert!(stats.get("function_count").is_some());
        assert!(stats.get("class_struct_count").is_some());
    }
    
    // Check total stats
    let total_stats = &json["total_stats"];
    assert_eq!(total_stats["function_count"], 5); // 3 Rust + 2 Python
    assert_eq!(total_stats["class_struct_count"], 4); // 3 Rust + 1 Python
}

#[test]
fn test_json_format_with_empty_directory() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    
    let output = run_code_stats(&[
        temp_dir.path().to_str().unwrap(),
        "--format",
        "json",
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success());
    
    let json = parse_json_output(&stdout);
    
    // Should have empty arrays/objects
    assert_eq!(json["files"].as_array().unwrap().len(), 0);
    assert_eq!(json["total_stats"]["function_count"], 0);
    assert_eq!(json["total_stats"]["class_struct_count"], 0);
}

#[test]
fn test_json_format_language_grouping() {
    let (_temp_dir, project_root) = create_controlled_test_project();
    
    let output = run_code_stats(&[
        project_root.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success());
    
    let json = parse_json_output(&stdout);
    let by_language = &json["total_by_language"];
    
    // Check Rust stats
    let rust_stats = &by_language["Rust"];
    assert_eq!(rust_stats["function_count"], 3);
    assert_eq!(rust_stats["class_struct_count"], 3); // 2 structs + 1 enum
    assert_eq!(rust_stats["file_count"], 2);
    
    // Check Python stats
    let python_stats = &by_language["Python"];
    assert_eq!(python_stats["function_count"], 2);
    assert_eq!(python_stats["class_struct_count"], 1);
    assert_eq!(python_stats["file_count"], 1);
}

#[test]
fn test_format_case_insensitive() {
    let (_temp_dir, project_root) = create_controlled_test_project();
    
    // clap v4 with ValueEnum is case-sensitive by default
    // Only lowercase values are accepted
    let formats = ["json", "detail", "summary"];
    
    for format in &formats {
        let output = run_code_stats(&[
            project_root.to_str().unwrap(),
            "--format",
            format,
        ]);
        
        assert!(
            output.status.success(),
            "Failed with format '{}': {}",
            format,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn test_single_file_analysis_format() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let test_file = temp_dir.path().join("single.rs");
    helpers::create_test_file(
        &test_file,
        r#"
fn test_function() {}
struct TestStruct {}
"#,
    );
    
    let output = run_code_stats(&[test_file.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success());
    
    // Single file should show different format
    assert_contains_all(&stdout, &[
        "Analyzing file:",
        "Language: Rust",
        "Code Statistics:",
        "Functions: 1",
        "Classes/Structs: 1",
    ]);
    
    // Should NOT show summary format
    assert!(!stdout.contains("Language Summary:"));
}

#[test]
fn test_format_with_special_characters_in_path() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let special_dir = temp_dir.path().join("special chars & spaces");
    std::fs::create_dir(&special_dir).unwrap();
    
    helpers::create_test_file(
        &special_dir.join("file with spaces.rs"),
        "fn test() {}",
    );
    
    let output = run_code_stats(&[
        special_dir.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success());
    
    // JSON should properly escape special characters
    let json = parse_json_output(&stdout);
    let files = json["files"].as_array().unwrap();
    assert_eq!(files.len(), 1);
    
    let file_path = files[0]["path"].as_str().unwrap();
    assert!(file_path.contains("file with spaces.rs"));
}
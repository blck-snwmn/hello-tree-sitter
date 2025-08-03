use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Create a test project with multiple language files in a temporary directory
pub fn create_test_project() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_path_buf();

    // Create directory structure
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("tests")).unwrap();
    fs::create_dir_all(root.join("js")).unwrap();
    fs::create_dir_all(root.join("python")).unwrap();

    // Create Rust files
    create_test_file(
        &root.join("src/main.rs"),
        r#"
fn main() {
    println!("Hello, world!");
    let result = calculate(5, 3);
    println!("Result: {}", result);
}

fn calculate(a: i32, b: i32) -> i32 {
    a + b
}

struct Config {
    name: String,
    value: i32,
}

impl Config {
    fn new(name: String, value: i32) -> Self {
        Config { name, value }
    }
}
"#,
    );

    create_test_file(
        &root.join("src/lib.rs"),
        r#"
pub fn public_function() -> String {
    "public".to_string()
}

fn private_function() {
    println!("private");
}

pub struct PublicStruct {
    field: String,
}

enum Status {
    Active,
    Inactive,
}
"#,
    );

    // Create Python files
    create_test_file(
        &root.join("python/main.py"),
        r#"
def main():
    print("Hello from Python")
    result = calculate(10, 20)
    print(f"Result: {result}")

def calculate(a, b):
    return a + b

class Calculator:
    def __init__(self):
        self.value = 0
    
    def add(self, x):
        self.value += x
        return self.value

if __name__ == "__main__":
    main()
"#,
    );

    // Create JavaScript files
    create_test_file(
        &root.join("js/app.js"),
        r#"
function main() {
    console.log("Hello from JavaScript");
    const result = calculate(15, 25);
    console.log(`Result: ${result}`);
}

const calculate = (a, b) => a + b;

function processData(data) {
    return data.map(item => item * 2);
}

class DataProcessor {
    constructor() {
        this.data = [];
    }

    process(item) {
        this.data.push(item);
    }
}

main();
"#,
    );

    // Create TypeScript file
    create_test_file(
        &root.join("js/types.ts"),
        r#"
interface User {
    id: number;
    name: string;
}

function getUser(id: number): User {
    return { id, name: "Test User" };
}

const updateUser = (user: User): void => {
    console.log(`Updating user: ${user.name}`);
};

class UserService {
    private users: User[] = [];

    addUser(user: User): void {
        this.users.push(user);
    }

    getUsers(): User[] {
        return this.users;
    }
}

export { User, UserService, getUser, updateUser };
"#,
    );

    // Create Go file
    create_test_file(
        &root.join("main.go"),
        r#"
package main

import "fmt"

func main() {
    fmt.Println("Hello from Go")
    result := calculate(7, 8)
    fmt.Printf("Result: %d\n", result)
}

func calculate(a, b int) int {
    return a + b
}

type Config struct {
    Name  string
    Value int
}

func (c *Config) Update(value int) {
    c.Value = value
}
"#,
    );

    // Create Java file
    create_test_file(
        &root.join("Main.java"),
        r#"
public class Main {
    public static void main(String[] args) {
        System.out.println("Hello from Java");
        Calculator calc = new Calculator();
        int result = calc.add(5, 10);
        System.out.println("Result: " + result);
    }

    private static void helperMethod() {
        System.out.println("Helper");
    }
}

class Calculator {
    private int value;

    public Calculator() {
        this.value = 0;
    }

    public int add(int a, int b) {
        return a + b;
    }

    private void reset() {
        this.value = 0;
    }
}

interface Operation {
    int execute(int a, int b);
}
"#,
    );

    // Create a non-code file for testing ignore functionality
    create_test_file(
        &root.join("README.md"),
        r#"
# Test Project

This is a test project for code-stats-rs.
"#,
    );

    // Create a hidden directory with code (for testing ignore patterns)
    fs::create_dir_all(root.join(".git")).unwrap();
    create_test_file(
        &root.join(".git/config.rs"),
        r#"
fn should_be_ignored() {
    println!("This file should be ignored");
}
"#,
    );

    (temp_dir, root)
}

/// Create a single test file with the given content
pub fn create_test_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

/// Parse JSON output and verify it's valid
pub fn parse_json_output(output: &str) -> serde_json::Value {
    serde_json::from_str(output).expect("Failed to parse JSON output")
}

/// Create a test directory with specific file counts for each language
pub fn create_controlled_test_project() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_path_buf();

    // Create exactly 2 Rust files with known counts
    create_test_file(
        &root.join("file1.rs"),
        r#"
fn function_one() {}
fn function_two() {}
struct StructOne {}
"#,
    );

    create_test_file(
        &root.join("file2.rs"),
        r#"
fn function_three() {}
struct StructTwo {}
enum EnumOne {}
"#,
    );

    // Create exactly 1 Python file
    create_test_file(
        &root.join("script.py"),
        r#"
def function_one():
    pass

def function_two():
    pass

class ClassOne:
    pass
"#,
    );

    (temp_dir, root)
}

/// Helper to run the code-stats-rs binary with args
pub fn run_code_stats(args: &[&str]) -> std::process::Output {
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_code-stats-rs"));
    cmd.args(args).output().expect("Failed to run code-stats-rs")
}

/// Assert that a string contains all of the given substrings
pub fn assert_contains_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            haystack.contains(needle),
            "Expected to find '{}' in:\n{}",
            needle,
            haystack
        );
    }
}

/// Create a symbolic link for testing --follow-links option
#[cfg(unix)]
pub fn create_symlink(src: &Path, dst: &Path) {
    std::os::unix::fs::symlink(src, dst).unwrap();
}

#[cfg(windows)]
pub fn create_symlink(src: &Path, dst: &Path) {
    // On Windows, we'll just copy the directory for testing purposes
    // Real symlink creation requires admin privileges
    copy_dir_all(src, dst).unwrap();
}

#[cfg(windows)]
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
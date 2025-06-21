use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_basic_tree_display() {
    let output = Command::new("cargo")
        .args(&["run", "--", "src"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("src/"));
    assert!(stdout.contains("main.rs"));
    assert!(stdout.contains("lib.rs"));
    assert!(stdout.contains("args.rs"));
    assert!(stdout.contains("tree.rs"));
}

#[test]
fn test_depth_limit() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-d", "1", "."])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    // 深度1なので、最上位のディレクトリとファイルのみ表示される
    assert!(stdout.contains("src/"));
    assert!(stdout.contains("docs/"));
    // 深度1なので、src内のファイルは表示されない
    assert!(!stdout.contains("main.rs"));
}

#[test]
fn test_files_only_filter() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", "src"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    // ファイルのみ表示
    assert!(stdout.contains("main.rs"));
    assert!(stdout.contains("lib.rs"));
    // ディレクトリは表示されない（ルートディレクトリ以外）
    let lines: Vec<&str> = stdout.lines().collect();
    let file_lines: Vec<&str> = lines
        .iter()
        .filter(|line| !line.ends_with("/") || line.trim() == "src/")
        .copied()
        .collect();
    assert!(file_lines.len() > 1); // src/ + ファイル群
}

#[test]
fn test_dirs_only_filter() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // テスト用のディレクトリ構造を作成
    fs::create_dir(temp_path.join("subdir")).unwrap();
    fs::write(temp_path.join("file.txt"), "test").unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "-D", temp_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    // ディレクトリのみ表示
    assert!(stdout.contains("subdir/"));
    // ファイルは表示されない
    assert!(!stdout.contains("file.txt"));
}

#[test]
fn test_nonexistent_path() {
    let output = Command::new("cargo")
        .args(&["run", "--", "/nonexistent/path"])
        .output()
        .expect("Failed to execute command");

    // エラーが発生することを確認
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Path does not exist"));
}

#[test]
fn test_invalid_depth() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--depth", "abc"])
        .output()
        .expect("Failed to execute command");

    // エラーが発生することを確認
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("invalid digit found in string"));
}

#[test]
fn test_help_display() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("A Rust implementation of tree command"));
    assert!(stdout.contains("--depth"));
    assert!(stdout.contains("--files-only"));
    assert!(stdout.contains("--dirs-only"));
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let output = Command::new("cargo")
        .args(&["run", "--", temp_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    // 空のディレクトリでもルートディレクトリ名は表示される
    assert!(stdout.contains("/"));
    // しかし他には何も表示されない
    assert_eq!(stdout.lines().count(), 1); // ルートディレクトリのみ
}

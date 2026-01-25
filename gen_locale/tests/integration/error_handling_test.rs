use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_file_not_found_error_message() {
  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("gen_locale");
  cmd
    .arg("--input")
    .arg("nonexistent.xlsx")
    .arg("--dir")
    .arg(output_dir);

  let assert = cmd.assert().failure();
  let stderr = String::from_utf8_lossy(&assert.get_output().stderr);

  assert!(stderr.contains("Error: Failed to parse input file"));
  assert!(stderr.contains("Cannot open file 'nonexistent.xlsx'"));
}

#[test]
fn test_missing_id_column_error_message() {
  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  // Create a minimal CSV missing the "ID" column.
  let csv_path = temp_dir.path().join("missing_id.csv");
  fs::write(&csv_path, "zh-CN,en-US\n标题,Title\n").unwrap();

  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("gen_locale");
  cmd
    .arg("--input")
    .arg(csv_path.to_str().unwrap())
    .arg("--dir")
    .arg(output_dir)
    .arg("--ext")
    .arg("ts");

  let assert = cmd.assert().failure();
  let stderr = String::from_utf8_lossy(&assert.get_output().stderr);

  assert!(stderr.contains("Error: Failed to parse input file"));
  assert!(stderr.contains("Missing required column: ID"));
}

#[test]
fn test_sheet_not_found_error_message() {
  // Requires a real Excel fixture.
  if !Path::new("test.xlsx").exists() {
    println!("Skipping test: test.xlsx not found");
    return;
  }

  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("gen_locale");
  cmd
    .arg("--input")
    .arg("test.xlsx")
    .arg("--dir")
    .arg(output_dir)
    .arg("--sheet")
    .arg("THIS_SHEET_DOES_NOT_EXIST")
    .arg("--ext")
    .arg("ts");

  let assert = cmd.assert().failure();
  let stderr = String::from_utf8_lossy(&assert.get_output().stderr);

  assert!(stderr.contains("Error: Failed to parse input file"));
  assert!(stderr.contains("Sheet 'THIS_SHEET_DOES_NOT_EXIST' not found."));
}

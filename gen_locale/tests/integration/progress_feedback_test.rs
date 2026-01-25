use std::fs;
use tempfile::TempDir;

#[test]
fn test_progress_feedback_large_file_1000_rows_or_more() {
  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  // Create a CSV with 1001 data rows (progress enabled when > 1000).
  let csv_path = temp_dir.path().join("big.csv");
  let mut csv = String::from("ID,zh-CN,en-US\n");
  for i in 0..1001 {
    csv.push_str(&format!("k{i},值{i},Value{i}\n"));
  }
  fs::write(&csv_path, csv).unwrap();

  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("gen_locale");
  cmd
    .arg("--input")
    .arg(csv_path.to_str().unwrap())
    .arg("--dir")
    .arg(output_dir)
    .arg("--ext")
    .arg("ts");

  let assert = cmd.assert().success();
  let stderr = String::from_utf8_lossy(&assert.get_output().stderr);

  // ProgressReporter prints to stderr
  assert!(stderr.contains("Processing:"));
}

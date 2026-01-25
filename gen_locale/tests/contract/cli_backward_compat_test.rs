use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_cli_backward_compat_default_args() {
  if !Path::new("test.xlsx").exists() {
    println!("Skipping test: test.xlsx not found");
    return;
  }

  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  // Old behavior: default output should be TypeScript when --ext is omitted.
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("gen_locale");
  cmd
    .arg("--input")
    .arg("test.xlsx")
    .arg("--dir")
    .arg(output_dir)
    .arg("--sheet")
    .arg("Sheet1");

  cmd.assert().success();
  assert!(Path::new(&format!("{}/zh-CN.ts", output_dir)).exists());
}

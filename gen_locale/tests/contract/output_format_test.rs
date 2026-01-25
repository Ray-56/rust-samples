use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_output_format_typescript_backward_compat() {
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
    .arg("Sheet1")
    .arg("--ext")
    .arg("ts");

  cmd.assert().success();

  let zh_cn_file = format!("{}/zh-CN.ts", output_dir);
  assert!(Path::new(&zh_cn_file).exists());

  let content = fs::read_to_string(&zh_cn_file).unwrap();
  assert!(content.starts_with("export default {"));
  assert!(content.ends_with("}"));
}

#[test]
fn test_output_format_json_backward_compat() {
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
    .arg("Sheet1")
    .arg("--ext")
    .arg("json");

  cmd.assert().success();

  let zh_cn_path = format!("{}/zh-CN.json", output_dir);
  assert!(Path::new(&zh_cn_path).exists());

  let content = fs::read_to_string(&zh_cn_path).unwrap();
  let _: serde_json::Value = serde_json::from_str(&content).unwrap();
}

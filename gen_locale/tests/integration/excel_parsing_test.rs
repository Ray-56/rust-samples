use gen_locale::application::dto::GenerationConfig;
use gen_locale::application::use_cases::GenerateLocalesUseCase;
use gen_locale::domain::enums::OutputFormat;
use gen_locale::domain::services::ValidationService;
use gen_locale::infrastructure::parsers::ExcelParser;
use gen_locale::infrastructure::writers::{JsonWriter, TypeScriptWriter};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_full_flow_excel_to_typescript() {
  // This test requires a real Excel file
  let input_path = "test.xlsx";

  if !Path::new(input_path).exists() {
    println!("Skipping test: {} not found", input_path);
    return;
  }

  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  let config = GenerationConfig::new(
    input_path.to_string(),
    output_dir.to_string(),
    Some("Sheet1".to_string()),
    OutputFormat::TypeScript,
  );

  let reader = ExcelParser::new();
  let writer = TypeScriptWriter::new();
  let validator = ValidationService::new();

  let use_case = GenerateLocalesUseCase::new(reader, writer, validator);
  let result = use_case.execute(config);

  assert!(result.is_ok());
  let result = result.unwrap();

  assert!(result.files_generated > 0);
  assert!(result.rows_processed > 0);

  // Check that files were created
  let zh_cn_file = format!("{}/zh-CN.ts", output_dir);
  assert!(Path::new(&zh_cn_file).exists());

  // Verify file content
  let content = fs::read_to_string(&zh_cn_file).unwrap();
  assert!(content.starts_with("export default {"));
  assert!(content.ends_with("}"));
}

#[test]
fn test_full_flow_excel_to_json() {
  let input_path = "test.xlsx";

  if !Path::new(input_path).exists() {
    println!("Skipping test: {} not found", input_path);
    return;
  }

  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  let config = GenerationConfig::new(
    input_path.to_string(),
    output_dir.to_string(),
    Some("Sheet1".to_string()),
    OutputFormat::Json,
  );

  let reader = ExcelParser::new();
  let writer = JsonWriter::new();
  let validator = ValidationService::new();

  let use_case = GenerateLocalesUseCase::new(reader, writer, validator);
  let result = use_case.execute(config);

  assert!(result.is_ok());

  // Check JSON file exists and is valid JSON
  let zh_cn_file = format!("{}/zh-CN.json", output_dir);
  assert!(Path::new(&zh_cn_file).exists());

  let content = fs::read_to_string(&zh_cn_file).unwrap();
  let _: serde_json::Value = serde_json::from_str(&content).unwrap();
}

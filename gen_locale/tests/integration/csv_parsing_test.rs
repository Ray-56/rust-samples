use gen_locale::application::dto::GenerationConfig;
use gen_locale::application::use_cases::GenerateLocalesUseCase;
use gen_locale::domain::enums::OutputFormat;
use gen_locale::domain::services::ValidationService;
use gen_locale::infrastructure::parsers::CsvParser;
use gen_locale::infrastructure::writers::{JsonWriter, TypeScriptWriter};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_full_flow_csv_to_typescript() {
  let input_path = "test.csv";
  if !Path::new(input_path).exists() {
    println!("Skipping test: {} not found", input_path);
    return;
  }

  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  let config = GenerationConfig::new(
    input_path.to_string(),
    output_dir.to_string(),
    None,
    OutputFormat::TypeScript,
  );

  let reader = CsvParser::new();
  let writer = TypeScriptWriter::new();
  let validator = ValidationService::new();
  let use_case = GenerateLocalesUseCase::new(reader, writer, validator);

  let result = use_case.execute(config).unwrap();
  assert!(result.files_generated > 0);
  assert!(result.rows_processed > 0);

  // Spot-check at least one file exists and looks like TS.
  let zh_cn_file = format!("{}/zh-CN.ts", output_dir);
  assert!(Path::new(&zh_cn_file).exists());
  let content = fs::read_to_string(&zh_cn_file).unwrap();
  assert!(content.starts_with("export default {"));
  assert!(content.ends_with("}"));
}

#[test]
fn test_full_flow_csv_to_json() {
  let input_path = "test.csv";
  if !Path::new(input_path).exists() {
    println!("Skipping test: {} not found", input_path);
    return;
  }

  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  let config = GenerationConfig::new(
    input_path.to_string(),
    output_dir.to_string(),
    None,
    OutputFormat::Json,
  );

  let reader = CsvParser::new();
  let writer = JsonWriter::new();
  let validator = ValidationService::new();
  let use_case = GenerateLocalesUseCase::new(reader, writer, validator);

  let result = use_case.execute(config).unwrap();
  assert!(result.files_generated > 0);
  assert!(result.rows_processed > 0);

  let zh_cn_file = format!("{}/zh-CN.json", output_dir);
  assert!(Path::new(&zh_cn_file).exists());

  let content = fs::read_to_string(&zh_cn_file).unwrap();
  let _: serde_json::Value = serde_json::from_str(&content).unwrap();
}

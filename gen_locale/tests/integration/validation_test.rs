use gen_locale::application::dto::GenerationConfig;
use gen_locale::application::use_cases::GenerateLocalesUseCase;
use gen_locale::domain::enums::OutputFormat;
use gen_locale::domain::errors::Warning;
use gen_locale::domain::services::ValidationService;
use gen_locale::infrastructure::parsers::CsvParser;
use gen_locale::infrastructure::writers::TypeScriptWriter;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_integration_validation_flow_generate_warnings_and_continue_output() {
  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().join("out");
  let output_dir_str = output_dir.to_str().unwrap();

  // Craft a CSV that triggers multiple validation warnings
  // - DuplicateKey: same ID appears twice
  // - EmptyTranslation: empty zh-CN value
  // - PlaceholderMimatch: {name} present in zh-CN but not in en-US
  // - NonStandardKeyFormat: leading space in key
  let csv_path = temp_dir.path().join("validation.csv");
  let csv = [
    "ID,zh-CN,en-US",
    "msg.welcome,欢迎 {name}!,Welcome!",
    "dup.key,重复,Duplicate",
    "dup.key,重复2,Duplicate2",
    "empty.zh,,Not empty",
    " bad.key,值,Value",
    "",
  ]
  .join("\n");
  fs::write(&csv_path, csv).unwrap();

  let config = GenerationConfig::new(
    csv_path.to_str().unwrap().to_string(),
    output_dir_str.to_string(),
    None,
    OutputFormat::TypeScript,
  );

  let reader = CsvParser::new();
  let writer = TypeScriptWriter::new();
  let validator = ValidationService::new();
  let use_case = GenerateLocalesUseCase::new(reader, writer, validator);

  let result = use_case.execute(config).unwrap();
  assert!(!result.warnings.is_empty());

  let has_duplicate = result
    .warnings
    .iter()
    .any(|w| matches!(w, Warning::DuplicateKey { key, .. } if key == "dup.key"));
  assert!(has_duplicate);

  let has_empty = result
    .warnings.iter().any(|w| {
      matches!(w, Warning::EmptyTranslation { key, language, .. } if key == "empty.zh" && language == "zh-CN") 
    });
  assert!(has_empty);

  let has_placeholder_mismatch = result
    .warnings
    .iter()
    .any(|w| matches!(w, Warning::PlaceholderMismatch { key, .. } if key == "msg.welcome"));
  assert!(has_placeholder_mismatch);

  // Ensure output is still produced even with warnings.
  assert!(Path::new(&format!("{}/zh-CN.ts", output_dir_str)).exists());
  assert!(Path::new(&format!("{}/en-US.ts", output_dir_str)).exists());
}

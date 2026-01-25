use gen_locale::application::dto::GenerationConfig;
use gen_locale::application::use_cases::GenerateLocalesUseCase;
use gen_locale::domain::enums::OutputFormat;
use gen_locale::domain::errors::Warning;
use gen_locale::domain::services::ValidationService;
use gen_locale::infrastructure::parsers::CsvParser;
use gen_locale::infrastructure::writers::TypeScriptWriter;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_data_quality_regression_empty_duplicate_placeholder_mismatch() {
  let input_path = "tests/regression/fixtures/data_quality_issues.csv";
  assert!(Path::new(input_path).exists());

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
}

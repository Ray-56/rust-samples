use gen_locale::application::dto::GenerationConfig;
use gen_locale::application::use_cases::GenerateLocalesUseCase;
use gen_locale::domain::enums::OutputFormat;
use gen_locale::domain::services::ValidationService;
use gen_locale::infrastructure::parsers::CsvParser;
use gen_locale::infrastructure::writers::TypeScriptWriter;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_verify_output_consistency_with_real_files() {
  // Use a committed CSV fixture so this test stays stable.
  let input_path = "test.csv";
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
  assert!(result.files_generated >= 5);

  // Assert expected files exist (based on test.csv header languages)
  for lang in ["zh-CN", "en-US", "es-ES", "id-ID", "pt-BR"] {
    assert!(Path::new(&format!("{}/{}.ts", output_dir, lang)).exists());
  }

  // Spot-check a known key/value stays stable
  let zh_cn = fs::read_to_string(format!("{}/zh-CN.ts", output_dir)).unwrap();
  assert!(zh_cn.contains("\"common.antd.dataEntry.input.plh\": \"请输入\""));

  let en_us = fs::read_to_string(format!("{}/en-US.ts", output_dir)).unwrap();
  assert!(en_us.contains("\"common.antd.dataEntry.input.plh\": \"Please enter\""));
}

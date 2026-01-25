use gen_locale::application::ports::FileWriter;
use gen_locale::domain::entities::{LocaleDocument, LocaleEntry};
use gen_locale::domain::enums::OutputFormat;
use gen_locale::domain::value_objects::{LanguageCode, TranslationKey, TranslationValue};
use gen_locale::infrastructure::writers::{JsonWriter, TypeScriptWriter};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_generate_typescript_file_format() {
  let document = create_document();
  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  let writer = TypeScriptWriter::new();
  writer
    .write(&document, output_dir, OutputFormat::TypeScript)
    .unwrap();

  let zh_cn_file = format!("{}/zh-CN.ts", output_dir);
  assert!(Path::new(&zh_cn_file).exists());

  let content = fs::read_to_string(&zh_cn_file).unwrap();
  assert!(content.starts_with("export default {"));
  assert!(content.contains("\"common.date\": \"日期\""));
  assert!(content.ends_with("}"));
}

#[test]
fn test_generate_json_file_format() {
  let document = create_document();
  let temp_dir = TempDir::new().unwrap();
  let output_dir = temp_dir.path().to_str().unwrap();

  let writer = JsonWriter::new();
  writer
    .write(&document, output_dir, OutputFormat::Json)
    .unwrap();

  let zh_cn_file = format!("{}/zh-CN.json", output_dir);
  assert!(Path::new(&zh_cn_file).exists());

  let content = fs::read_to_string(&zh_cn_file).unwrap();
  let json: serde_json::Value = serde_json::from_str(&content).unwrap();
  assert_eq!(json["common.date"], "日期");
}

fn create_document() -> LocaleDocument {
  let entries = vec![
    LocaleEntry::new(
      LanguageCode::new("zh-CN").unwrap(),
      TranslationKey::new("common.date").unwrap(),
      TranslationValue::new("日期"),
      1,
    ),
    LocaleEntry::new(
      LanguageCode::new("en-US").unwrap(),
      TranslationKey::new("common.date").unwrap(),
      TranslationValue::new("Date"),
      1,
    ),
  ];

  LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap()
}

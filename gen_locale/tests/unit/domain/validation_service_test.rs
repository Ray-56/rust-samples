use gen_locale::domain::entities::{LocaleDocument, LocaleEntry};
use gen_locale::domain::services::ValidationService;
use gen_locale::domain::value_objects::{LanguageCode, TranslationKey, TranslationValue};

#[test]
fn test_validate_document_without_warnings() {
  let validator = ValidationService::new();

  let entries = vec![
    create_entry("zh-CN", "app.title", "标题", 1),
    create_entry("en-US", "app.title", "Title", 1),
  ];

  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();
  let result = validator.validate(&document);

  assert!(!result.has_warnings());
}

#[test]
fn test_detect_empty_translations() {
  let validator = ValidationService::new();

  let entries =
    vec![create_entry("zh-CN", "app.title", "", 1), create_entry("en-US", "app.title", "Title", 1)];

  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();
  let result = validator.validate(&document);

  assert!(result.has_warnings());
  assert_eq!(result.warnings.len(), 1);
}

#[test]
fn test_detect_duplicate_keys() {
  let validator = ValidationService::new();

  let entries = vec![
    create_entry("zh-CN", "app.title", "标题", 1),
    create_entry("en-US", "app.title", "Title", 1),
    create_entry("zh-CN", "app.title", "标题", 2),
  ];

  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();
  let result = validator.validate(&document);

  assert!(result.has_warnings());
  // Should have at least 1 duplicate warning
  let duplicate_warnings: Vec<_> = result
    .warnings
    .iter()
    .filter(|w| matches!(w, gen_locale::domain::errors::Warning::DuplicateKey { .. }))
    .collect();
  assert!(!duplicate_warnings.is_empty());
}

#[test]
fn test_detect_non_standard_key_format() {
  let validator = ValidationService::new();

  let entries = vec![create_entry("zh-CN", "Invalid Key!", "标题", 1)];

  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();
  let result = validator.validate(&document);

  assert!(result.has_warnings());
}

#[test]
fn test_detect_unusually_long_values() {
  let validator = ValidationService::new();

  let long_value = "a".repeat(2001);
  let entries = vec![create_entry("zh-CN", "app.title", &long_value, 1)];

  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();
  let result = validator.validate(&document);

  assert!(result.has_warnings());
}

#[test]
fn test_detect_placeholder_mismatches() {
  let validator = ValidationService::new();

  let entries = vec![
    create_entry("zh-CN", "msg.welcome", "欢迎 {name}!", 1),
    create_entry("en-US", "msg.welcome", "Welcome!", 1),
  ];

  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();
  let result = validator.validate(&document);

  assert!(result.has_warnings());
}

// Helper function
fn create_entry(lang: &str, key: &str, value: &str, row: usize) -> LocaleEntry {
  LocaleEntry::new(
    LanguageCode::new(lang).unwrap(),
    TranslationKey::new(key).unwrap(),
    TranslationValue::new(value),
    row,
  )
}

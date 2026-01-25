use gen_locale::domain::entities::{LocaleDocument, LocaleEntry};
use gen_locale::domain::errors::DomainError;
use gen_locale::domain::value_objects::{LanguageCode, TranslationKey, TranslationValue};

#[test]
fn test_reject_empty_document() {
  let err = LocaleDocument::new(vec![], "test.xlsx".to_string()).unwrap_err();
  assert!(matches!(err, DomainError::EmptyDocument));
}

#[test]
fn test_language_set_deduplication() {
  let entries = vec![
    create_entry("zh-CN", "app.title", "标题", 1),
    create_entry("zh-CN", "app.desc", "描述", 2),
    create_entry("en-US", "app.title", "Title", 1),
  ];

  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();

  let zh = LanguageCode::new("zh-CN").unwrap();
  let en = LanguageCode::new("en-US").unwrap();

  assert_eq!(document.languages().len(), 2);
  assert!(document.languages().contains(&zh));
  assert!(document.languages().contains(&en));
}

#[test]
fn test_returns_all_entries() {
  let entries = vec![
    create_entry("zh-CN", "app.title", "标题", 1),
    create_entry("en-US", "app.title", "Title", 1),
  ];
  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();

  let all = document.entries();
  assert_eq!(all.len(), 2);
  assert_eq!(all[0].language().as_str(), "zh-CN");
  assert_eq!(all[0].key().as_str(), "app.title");
  assert_eq!(all[0].value().raw(), "标题");
  assert_eq!(all[0].source_row(), 1);
  assert_eq!(all[1].language().as_str(), "en-US");
  assert_eq!(all[1].key().as_str(), "app.title");
  assert_eq!(all[1].value().raw(), "Title");
  assert_eq!(all[1].source_row(), 1);
}

#[test]
fn test_entries_by_language_grouped_by_language() {
  let entries = vec![
    create_entry("zh-CN", "app.title", "标题", 1),
    create_entry("en-US", "app.title", "Title", 1),
    create_entry("zh-CN", "app.desc", "描述", 2),
  ];
  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();

  let grouped = document.entries_by_language();
  assert_eq!(grouped.len(), 2);

  let zh = LanguageCode::new("zh-CN").unwrap();
  let en = LanguageCode::new("en-US").unwrap();

  let zh_entries = grouped.get(&zh).unwrap();
  assert_eq!(zh_entries.len(), 2);
  assert!(zh_entries.iter().all(|e| e.language().as_str() == "zh-CN"));
  let zh_keys: Vec<_> = zh_entries.iter().map(|e| e.key().as_str()).collect();
  assert!(zh_keys.contains(&"app.title"));
  assert!(zh_keys.contains(&"app.desc"));

  let en_entries = grouped.get(&en).unwrap();
  assert_eq!(en_entries.len(), 1);
  assert_eq!(en_entries[0].language().as_str(), "en-US");
  assert_eq!(en_entries[0].key().as_str(), "app.title");
}

#[test]
fn test_entries_for_language_returns_entries_for_specific_language() {
  let entries = vec![
    create_entry("zh-CN", "app.title", "标题", 1),
    create_entry("en-US", "app.title", "Title", 1),
    create_entry("zh-CN", "app.desc", "描述", 2),
  ];
  let document = LocaleDocument::new(entries, "test.xlsx".to_string()).unwrap();

  let zh = LanguageCode::new("zh-CN").unwrap();
  let zh_entries = document.entries_for_language(&zh);

  assert_eq!(zh_entries.len(), 2);
  assert!(zh_entries.iter().all(|e| e.language().as_str() == "zh-CN"));
}

#[test]
fn test_source_file_returns_source_file_path() {
  let entries = vec![create_entry("zh-CN", "app.title", "标题", 1)];
  let document = LocaleDocument::new(entries, "path/to/file.xlsx".to_string()).unwrap();

  assert_eq!(document.source_file(), "path/to/file.xlsx");
}

fn create_entry(language: &str, key: &str, value: &str, row: usize) -> LocaleEntry {
  LocaleEntry::new(
    LanguageCode::new(language).unwrap(),
    TranslationKey::new(key).unwrap(),
    TranslationValue::new(value),
    row,
  )
}

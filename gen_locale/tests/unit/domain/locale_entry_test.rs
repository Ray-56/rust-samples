use gen_locale::domain::entities::LocaleEntry;
use gen_locale::domain::value_objects::{LanguageCode, TranslationKey, TranslationValue};

#[test]
fn test_create_locale_entry() {
  let lang = LanguageCode::new("zh-CN").unwrap();
  let key = TranslationKey::new("app.title").unwrap();
  let value = TranslationValue::new("应用标题");

  let entry = LocaleEntry::new(lang, key, value, 42);

  assert_eq!(entry.language().as_str(), "zh-CN");
  assert_eq!(entry.key().as_str(), "app.title");
  assert_eq!(entry.value().raw(), "应用标题");
  assert_eq!(entry.source_row(), 42);
}

#[test]
fn test_entry_can_be_cloned() {
  let lang = LanguageCode::new("en-US").unwrap();
  let key = TranslationKey::new("button.ok").unwrap();
  let value = TranslationValue::new("OK");

  let entry = LocaleEntry::new(lang, key, value, 1);
  let cloned = entry.clone();

  assert_eq!(entry.key().as_str(), cloned.key().as_str());
  assert_eq!(entry.language().as_str(), cloned.language().as_str());
}

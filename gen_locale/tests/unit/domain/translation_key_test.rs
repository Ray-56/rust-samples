use gen_locale::domain::value_objects::TranslationKey;

#[test]
fn test_create_valid_translation_key() {
  let key = TranslationKey::new("app.title.main");
  assert!(key.is_ok());
  assert_eq!(key.unwrap().as_str(), "app.title.main");
}

#[test]
fn test_reject_empty_translation_key() {
  let key = TranslationKey::new("");
  assert!(key.is_err());
}

#[test]
fn test_reject_translation_key_too_long() {
  let long_key = "a".repeat(201);
  let key = TranslationKey::new(long_key);
  assert!(key.is_err());
}

#[test]
fn test_recommended_format_check_valid_format() {
  let key = TranslationKey::new("app.title.main_button").unwrap();
  assert!(key.is_recommended_format());

  let key2 = TranslationKey::new("button-ok").unwrap();
  assert!(key2.is_recommended_format());

  let key3 = TranslationKey::new("item_123").unwrap();
  assert!(key3.is_recommended_format());

  let key4 = TranslationKey::new("Taking a break").unwrap();
  assert!(key4.is_recommended_format());

  let key5 = TranslationKey::new("common.button ok").unwrap();
  assert!(key5.is_recommended_format());

  let key6 = TranslationKey::new("version+info").unwrap();
  assert!(key6.is_recommended_format());
}

#[test]
fn test_recommended_format_check_invalid_format() {
  let key = TranslationKey::new(" app.title").unwrap();
  assert!(!key.is_recommended_format());

  let key2 = TranslationKey::new("app@title").unwrap();
  assert!(!key2.is_recommended_format());

  let key3 = TranslationKey::new("\tapp.title").unwrap();
  assert!(!key3.is_recommended_format());

  // Trailing space is not allowed
  let key4 = TranslationKey::new("app.title ").unwrap();
  assert!(!key4.is_recommended_format());

  // Trailing tab is not allowed
  let key5 = TranslationKey::new("app.title\t").unwrap();
  assert!(!key5.is_recommended_format());
}

#[test]
fn test_translation_key_equality() {
  let key1 = TranslationKey::new("app.title").unwrap();
  let key2 = TranslationKey::new("app.title").unwrap();
  assert_eq!(key1, key2);
}

#[test]
fn test_translation_key_hash() {
  use std::collections::HashSet;

  let key1 = TranslationKey::new("app.title").unwrap();
  let key2 = TranslationKey::new("app.button").unwrap();
  let key3 = TranslationKey::new("app.title").unwrap();

  let mut set = HashSet::new();
  set.insert(key1);
  set.insert(key2);
  set.insert(key3);

  // key1 and key3 are the same, so set should have 2 elements
  assert_eq!(set.len(), 2);
}

#[test]
fn test_edge_case_exactly_200_characters() {
  let key_str = "a".repeat(200);
  let key = TranslationKey::new(key_str);
  assert!(key.is_ok());
}

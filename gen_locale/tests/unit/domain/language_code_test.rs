use gen_locale::domain::value_objects::LanguageCode;

#[test]
fn test_create_valid_language_code() {
  let code = LanguageCode::new("zh-CN");
  assert!(code.is_ok());
  assert_eq!(code.unwrap().as_str(), "zh-CN");
}

#[test]
fn test_language_code_case_insensitive_preserves_original_case() {
  let code = LanguageCode::new("en-US").unwrap();
  assert_eq!(code.as_str(), "en-US");

  let code2 = LanguageCode::new("EN-us").unwrap();
  assert_eq!(code2.as_str(), "EN-us");
}

#[test]
fn test_reject_empty_language_code() {
  let code = LanguageCode::new("");
  assert!(code.is_err());
}

#[test]
fn test_reject_language_code_too_long() {
  let code = LanguageCode::new("very-long-invalid-code");
  assert!(code.is_err());
}

#[test]
fn test_reject_language_code_with_invalid_characters() {
  let code = LanguageCode::new("zh@CN");
  assert!(code.is_err());

  let code2 = LanguageCode::new("zh CN");
  assert!(code2.is_err());
}

#[test]
fn test_language_code_equality() {
  let code = LanguageCode::new("zh-CN").unwrap();
  let code2 = LanguageCode::new("zh-CN").unwrap();
  assert_eq!(code, code2);
}

#[test]
fn test_language_code_hash() {
  use std::collections::HashSet;

  let code1 = LanguageCode::new("zh-CN").unwrap();
  let code2 = LanguageCode::new("en-US").unwrap();
  let code3 = LanguageCode::new("zh-CN").unwrap();

  let mut set = HashSet::new();
  set.insert(code1);
  set.insert(code2);
  set.insert(code3);

  // code1 and code3 are the same, so set should have 2 elements
  assert_eq!(set.len(), 2);
}

#[test]
fn test_language_code_display() {
  let code = LanguageCode::new("zh-CN").unwrap();
  assert_eq!(format!("{}", code), "zh-CN");
}

#[test]
fn test_edge_case_shortest_valid_code() {
  let code = LanguageCode::new("ab");
  assert!(code.is_ok());
}

#[test]
fn test_edge_case_longest_valid_code() {
  let code = LanguageCode::new("1234567890");
  assert!(code.is_ok());
}

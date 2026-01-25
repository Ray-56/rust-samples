use gen_locale::domain::value_objects::TranslationValue;

#[test]
fn test_create_translation_value() {
  let value = TranslationValue::new("Hello World");
  assert_eq!(value.raw(), "Hello World");
  assert_eq!(value.escaped(), "Hello World");
}

#[test]
fn test_escape_newline() {
  let value = TranslationValue::new("Line 1\nLine 2");
  assert_eq!(value.raw(), "Line 1\nLine 2");
  assert_eq!(value.escaped(), "Line 1\\nLine 2");
}

#[test]
fn test_escape_quotes() {
  let value = TranslationValue::new("She said, \"Hello\"");
  assert_eq!(value.raw(), "She said, \"Hello\"");
  assert_eq!(value.escaped(), "She said, \\\"Hello\\\"");
}

#[test]
fn test_escape_backslash() {
  let value = TranslationValue::new("Path: C:\\Users\\");
  assert_eq!(value.raw(), "Path: C:\\Users\\");
  assert_eq!(value.escaped(), "Path: C:\\\\Users\\\\");
}

#[test]
fn test_multiple_escapes() {
  let value = TranslationValue::new("Line 1\n\"Quote\" and \\");
  assert_eq!(value.escaped(), "Line 1\\n\\\"Quote\\\" and \\\\");
}

#[test]
fn test_empty_value_detection() {
  let value = TranslationValue::new("");
  assert!(value.is_empty());
  assert_eq!(value.len(), 0);

  let value2 = TranslationValue::new("Not empty");
  assert!(!value2.is_empty());
  assert_eq!(value2.len(), 9);
}

#[test]
fn test_unusually_long_detection() {
  let short_value = TranslationValue::new("Short");
  assert!(!short_value.is_unusually_long());

  let long_value = TranslationValue::new("a".repeat(2001));
  assert!(long_value.is_unusually_long());

  let boundary_value = TranslationValue::new("a".repeat(2000));
  assert!(!boundary_value.is_unusually_long());
}

#[test]
fn test_value_equality() {
  let value1 = TranslationValue::new("Hello");
  let value2 = TranslationValue::new("Hello");
  assert_eq!(value1, value2);
}

#[test]
fn test_display_escaped_value() {
  let value = TranslationValue::new("Hello\nWorld");
  assert_eq!(format!("{}", value), "Hello\\nWorld");
}

#[test]
fn test_unicode_characters_not_escaped() {
  let value = TranslationValue::new("Hello🌍");
  assert_eq!(value.raw(), "Hello🌍");
  assert_eq!(value.escaped(), "Hello🌍");
}

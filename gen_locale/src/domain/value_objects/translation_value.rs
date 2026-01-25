/// Value object representing a translation value with escaping logic
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslationValue {
  raw_value: String,
  escaped_value: String,
}

impl TranslationValue {
  /// Creates a new translation value with automatic escaping
  pub fn new(raw_value: impl Into<String>) -> Self {
    let raw_value = raw_value.into();
    let escaped_value = Self::escape(&raw_value);

    Self { raw_value, escaped_value }
  }

  /// Escapes special characters for output
  fn escape(value: &str) -> String {
    value
      .replace('\\', "\\\\")
      .replace('\n', "\\n")
      .replace('"', "\\\"")
  }

  /// Gets the raw (unescaped) value
  pub fn raw(&self) -> &str {
    &self.raw_value
  }

  /// Gets the escaped value for output
  pub fn escaped(&self) -> &str {
    &self.escaped_value
  }

  /// Checks if the value is empty
  pub fn is_empty(&self) -> bool {
    self.raw_value.is_empty()
  }

  /// Gets the length of the raw value
  pub fn len(&self) -> usize {
    self.raw_value.len()
  }

  /// Checks if the value is unusually long (potential data issue)
  pub fn is_unusually_long(&self) -> bool {
    self.raw_value.len() > 2000
  }
}

impl std::fmt::Display for TranslationValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.escaped_value)
  }
}

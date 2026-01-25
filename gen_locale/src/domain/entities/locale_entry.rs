use crate::domain::value_objects::{LanguageCode, TranslationKey, TranslationValue};

/// Represents a single translation entry
#[derive(Debug, Clone)]
pub struct LocaleEntry {
  language: LanguageCode,
  key: TranslationKey,
  value: TranslationValue,
  source_row: usize,
}

impl LocaleEntry {
  /// Creates a new locale entry with validation
  pub fn new(
    language: LanguageCode,
    key: TranslationKey,
    value: TranslationValue,
    source_row: usize,
  ) -> Self {
    Self { language, key, value, source_row }
  }

  /// Gets the language code
  pub fn language(&self) -> &LanguageCode {
    &self.language
  }

  /// Gets the translation key
  pub fn key(&self) -> &TranslationKey {
    &self.key
  }

  /// Gets the translation value
  pub fn value(&self) -> &TranslationValue {
    &self.value
  }

  /// Gets the source row
  pub fn source_row(&self) -> usize {
    self.source_row
  }
}

/// Domain layer errors
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DomainError {
  InvalidLanguageCode(String),

  InvalidTranslationKey(String),

  UnsupportedFileFormat(String),

  InvalidOutputFormat(String),

  EmptyDocument,

  MissingRequiredColumn(String),

  DuplicateKey { key: String, language: String, row: usize },
}

impl std::fmt::Display for DomainError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DomainError::InvalidLanguageCode(s) => write!(f, "Invalid language code: {s}"),
      DomainError::InvalidTranslationKey(s) => write!(f, "Invalid translation key: {s}"),
      DomainError::UnsupportedFileFormat(s) => write!(f, "Unsupported file format: {s}"),
      DomainError::InvalidOutputFormat(s) => write!(f, "Invalid output format: {s}"),
      DomainError::EmptyDocument => write!(f, "Document cannot be empty"),
      DomainError::MissingRequiredColumn(s) => write!(f, "Missing required column: {s}"),
      DomainError::DuplicateKey { key, language, row } => write!(
        f,
        "Duplicate key: {key} in language {language} at row {row}"
      ),
    }
  }
}

impl std::error::Error for DomainError {}

/// Non-fatal warnings during processing
#[derive(Debug, Clone)]
pub enum Warning {
  /// Duplicate key found
  DuplicateKey { key: String, language: String, row: usize },

  /// Empty translation value
  EmptyTranslation { key: String, language: String, row: usize },

  /// Key doesn't follow recommended format
  NonStandardKeyFormat { key: String, row: usize },

  /// Placeholder count mismatch between languages
  PlaceholderMismatch {
    key: String,
    language1: String,
    count1: usize,
    language2: String,
    count2: usize,
  },

  /// Unusually long translation value
  UnusuallyLongTranslationValue { key: String, language: String, length: usize, row: usize },
}

impl Warning {
  /// Formats the warning as a user-friendly message
  pub fn to_message(&self) -> String {
    match self {
      Warning::DuplicateKey { key, language, row } => {
        format!("Duplicate key: '{key}' in language '{language}' at row {row}")
      }
      Warning::EmptyTranslation { key, language, row } => {
        format!("Empty translation for key '{key}' in language '{language}' at row {row}")
      }
      Warning::NonStandardKeyFormat { key, row } => {
        format!("Key '{key}' at row {row} doesn't follow recommended format (no leading/trailing spaces, only alphanumeric and ._+- chars allowed)")
      }
      Warning::PlaceholderMismatch { key, language1, count1, language2, count2 } => {
        format!("Placeholder count mismatch for key '{key}': {language1} has {count1} placeholders, {language2} has {count2}")
      }
      Warning::UnusuallyLongTranslationValue { key, language, length, row } => {
        format!("Unusually long translation ({length} characters for key '{key}' in language '{language}' at row {row})")
      }
    }
  }
}

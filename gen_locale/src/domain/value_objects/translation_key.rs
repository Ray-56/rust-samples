use crate::domain::errors::DomainError;

/// Value object representing a translation key (e.g., "app.title.main")
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationKey {
  key: String,
}

impl TranslationKey {
  const MAX_LENGTH: usize = 200;

  /// Creates a new translation key with validation
  pub fn new(key: impl Into<String>) -> Result<Self, DomainError> {
    let key = key.into();

    if key.is_empty() {
      return Err(DomainError::InvalidTranslationKey(
        "Key cannot be empty".into(),
      ));
    }

    if key.len() > Self::MAX_LENGTH {
      return Err(DomainError::InvalidTranslationKey(format!(
        "Key too long (max {} characters",
        Self::MAX_LENGTH
      )));
    }

    Ok(Self { key })
  }

  /// Checks if the key follows recommended format
  pub fn is_recommended_format(&self) -> bool {
    // Check if key starts or ends with whitespace (not recommended)
    if self.key.starts_with(' ')
      || self.key.starts_with('\t')
      || self.key.ends_with(' ')
      || self.key.ends_with('\t')
    {
      return false;
    }

    // Recommended format: [a-zA-Z0-9._+- ]+ (spaces allowed in middle)
    self
      .key
      .chars()
      .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-' || c == '+' || c == ' ')
  }

  /// Gets the key as a string
  pub fn as_str(&self) -> &str {
    &self.key
  }
}

impl std::fmt::Display for TranslationKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.key)
  }
}

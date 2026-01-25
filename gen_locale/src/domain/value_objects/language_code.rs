use crate::domain::errors::DomainError;

/// Value object representing a language code (e.g., "zh-CN", "en-US")
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageCode {
  code: String,
}

impl LanguageCode {
  /// Creates a new language code with validation
  pub fn new(code: impl Into<String>) -> Result<Self, DomainError> {
    let code = code.into();

    if code.is_empty() {
      return Err(DomainError::InvalidLanguageCode(
        "Language code cannot be empty".into(),
      ));
    }

    if code.len() > 10 {
      return Err(DomainError::InvalidLanguageCode(
        "Language code too long".into(),
      ));
    }

    // Basic format check: alphanumeric and hyphens only
    if !code.chars().all(|c| c.is_alphanumeric() || c == '-') {
      return Err(DomainError::InvalidLanguageCode(format!(
        "Invalid characters in language code: {code}"
      )));
    }

    Ok(Self { code })
  }

  /// Gets the language code as a string
  pub fn as_str(&self) -> &str {
    &self.code
  }
}

impl std::fmt::Display for LanguageCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.code)
  }
}

use crate::domain::errors::DomainError;

/// Represents the output file format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
  TypeScript,
  Json,
}

impl OutputFormat {
  /// Gets the file extension for this format
  pub fn extension(&self) -> &'static str {
    match self {
      OutputFormat::TypeScript => "ts",
      OutputFormat::Json => "json",
    }
  }

  /// Creates format from string
  pub fn from_string(s: &str) -> Result<Self, DomainError> {
    match s.to_lowercase().as_str() {
      "ts" => Ok(OutputFormat::TypeScript),
      "json" => Ok(OutputFormat::Json),
      _ => Err(DomainError::InvalidOutputFormat(s.to_string())),
    }
  }
}

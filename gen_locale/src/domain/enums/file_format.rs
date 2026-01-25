use crate::domain::errors::DomainError;

/// Represents the input file format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
  Excel,
  Csv,
}

impl FileFormat {
  /// Detects file format from file extension
  pub fn from_extension(path: &str) -> Result<Self, DomainError> {
    if path.ends_with(".xlsx") {
      Ok(Self::Excel)
    } else if path.ends_with(".csv") {
      Ok(Self::Csv)
    } else {
      Err(DomainError::UnsupportedFileFormat(path.to_string()))
    }
  }
}

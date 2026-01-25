use crate::domain::errors::Warning;

/// Result of validation, containing warnings
#[derive(Debug)]
pub struct ValidationResult {
  pub warnings: Vec<Warning>,
}

impl ValidationResult {
  pub fn new(warnings: Vec<Warning>) -> Self {
    Self { warnings }
  }

  pub fn no_warnings(self) -> Self {
    Self { warnings: Vec::new() }
  }

  pub fn has_warnings(&self) -> bool {
    !self.warnings.is_empty()
  }
}

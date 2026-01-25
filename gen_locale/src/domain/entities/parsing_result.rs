use crate::domain::entities::LocaleDocument;
use crate::domain::errors::Warning;

/// Result of parsing a file, including warnings
#[derive(Debug)]
pub struct ParsingResult {
  pub document: LocaleDocument,
  pub warnings: Vec<Warning>,
}

impl ParsingResult {
  pub fn new(document: LocaleDocument, warnings: Vec<Warning>) -> Self {
    Self { document, warnings }
  }

  pub fn without_warnings(document: LocaleDocument) -> Self {
    Self { document, warnings: Vec::new() }
  }

  pub fn has_warnings(&self) -> bool {
    !self.warnings.is_empty()
  }
}

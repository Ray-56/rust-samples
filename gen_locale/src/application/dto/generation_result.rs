use crate::domain::errors::Warning;

/// Result of locale generation
#[derive(Debug)]
pub struct GenerationResult {
  pub files_generated: usize,
  pub rows_processed: usize,
  pub warnings: Vec<Warning>,
}

impl GenerationResult {
  pub fn new(files_generated: usize, rows_processed: usize, warnings: Vec<Warning>) -> Self {
    Self { files_generated, rows_processed, warnings }
  }
}

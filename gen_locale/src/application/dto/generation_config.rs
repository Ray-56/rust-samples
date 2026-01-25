use crate::domain::enums::OutputFormat;

/// Configuration for locale generation
#[derive(Debug, Clone)]
pub struct GenerationConfig {
  pub input_path: String,
  pub output_dir: String,
  pub sheet_name: Option<String>,
  pub output_format: OutputFormat,
}

impl GenerationConfig {
  pub fn new(
    input_path: String,
    output_dir: String,
    sheet_name: Option<String>,
    output_format: OutputFormat,
  ) -> Self {
    Self { input_path, output_dir, sheet_name, output_format }
  }
}

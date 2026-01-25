use crate::application::ports::FileWriter;
use crate::domain::entities::LocaleDocument;
use crate::domain::enums::OutputFormat;
use crate::infrastructure::writers::TypeScriptWriter;
use anyhow::Result;

/// JSON file writer implementaion (delegates to TypeScriptWriter)
pub struct JsonWriter {
  inner: TypeScriptWriter,
}

impl JsonWriter {
  pub fn new() -> Self {
    Self { inner: TypeScriptWriter::new() }
  }
}

impl Default for JsonWriter {
  fn default() -> Self {
    Self::new()
  }
}

impl FileWriter for JsonWriter {
  fn write(&self, document: &LocaleDocument, output_dir: &str, format: OutputFormat) -> Result<()> {
    self.inner.write(document, output_dir, format)
  }
}

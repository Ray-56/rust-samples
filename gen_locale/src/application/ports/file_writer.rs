use crate::domain::entities::LocaleDocument;
use crate::domain::enums::OutputFormat;

/// Port trait for writing output files
pub trait FileWriter {
  fn write(
    &self,
    document: &LocaleDocument,
    output_dir: &str,
    format: OutputFormat,
  ) -> anyhow::Result<()>;
}

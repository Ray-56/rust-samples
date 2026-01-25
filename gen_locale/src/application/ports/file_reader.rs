use crate::domain::entities::ParsingResult;

/// Port trait for reading and parsing input files
pub trait FileReader {
  fn read(&self, path: &str, sheet_name: Option<&str>) -> anyhow::Result<ParsingResult>;
}

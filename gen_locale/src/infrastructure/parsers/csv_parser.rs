use crate::application::ports::FileReader;
use crate::domain::entities::{LocaleDocument, LocaleEntry, ParsingResult};
use crate::domain::value_objects::{TranslationKey, TranslationValue};
use crate::infrastructure::parsers::common::parse_first_row;
use crate::infrastructure::ProgressReporter;
use anyhow::{Context, Result};
use csv::ReaderBuilder;

/// CSV file parser implementaion
pub struct CsvParser;

impl CsvParser {
  pub fn new() -> Self {
    Self
  }
}

impl Default for CsvParser {
  fn default() -> Self {
    Self::new()
  }
}

impl FileReader for CsvParser {
  fn read(&self, path: &str, _sheet_name: Option<&str>) -> Result<ParsingResult> {
    // Count total data rows (excluding header) to enable progress reporting for large files.
    // This is a second pass over the file, but keeps the main parsing loop simple.
    let total_rows = ReaderBuilder::new()
      .has_headers(true)
      .flexible(true)
      .from_path(path)
      .with_context(|| {
        format!("Cannot open file '{path}'. Please check file path and permissions.")
      })?
      .records()
      .count();
    let progress = ProgressReporter::new(total_rows);

    let mut reader = ReaderBuilder::new()
      .has_headers(true)
      .flexible(true)
      .from_path(path)
      .with_context(|| {
        format!("Cannot open file '{path}'. Please check file path and permissions.")
      })?;

    let mut entries = Vec::new();

    // Get headers
    let headers = reader.headers()?.clone();
    let header_row: Vec<String> = headers.iter().map(|h| h.to_string()).collect();

    let (id_col, language_columns) = parse_first_row(&header_row)?;

    // Parse data rows
    for (row_idx, result) in reader.records().enumerate() {
      let record = result?;
      let row_num = row_idx + 2; // +2 because 0-indexed header row

      // Get ID (translation key)
      let id_str = record.get(id_col).unwrap_or("").to_string();
      if id_str.is_empty() {
        continue; // Skip rows with empty ID
      }

      let key = TranslationKey::new(id_str)?;

      // Parse each language column
      for (col_idx, lang_code) in &language_columns {
        let value_str = record.get(*col_idx).unwrap_or("").to_string();
        let value = TranslationValue::new(value_str);

        let entry = LocaleEntry::new(lang_code.clone(), key.clone(), value, row_num);
        entries.push(entry);
      }

      progress.increment();
    }

    progress.finish();

    let document = LocaleDocument::new(entries, path.to_string())?;
    Ok(ParsingResult::without_warnings(document))
  }
}

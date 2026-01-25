use crate::application::ports::FileReader;
use crate::domain::entities::{LocaleDocument, LocaleEntry, ParsingResult};
use crate::domain::value_objects::{TranslationKey, TranslationValue};
use crate::infrastructure::parsers::common::parse_first_row;
use crate::infrastructure::ProgressReporter;
use anyhow::{Context, Result};
use calamine::{open_workbook, Reader, Xlsx};

/// Excel file parser implementaion
pub struct ExcelParser;

impl ExcelParser {
  pub fn new() -> Self {
    Self
  }
}

impl Default for ExcelParser {
  fn default() -> Self {
    Self::new()
  }
}

impl FileReader for ExcelParser {
  fn read(&self, path: &str, sheet_name: Option<&str>) -> Result<ParsingResult> {
    let mut workbook: Xlsx<_> = open_workbook(path).with_context(|| {
      format!("Cannot open file '{path}'. Please check file path and permissions.")
    })?;

    let sheet_name = sheet_name.unwrap_or("Sheet1");
    let range = workbook
      .worksheet_range(sheet_name)
      .ok_or_else(|| {
        let available_sheets: Vec<_> = workbook.sheet_names().to_vec();
        anyhow::anyhow!("Sheet '{sheet_name}' not found. Avaliable sheets: {available_sheets:?}")
      })?
      .context("Failed to read worksheet")?;

    let mut entries = Vec::new();
    let mut rows = range.rows();

    // Parse first row to get column structure
    let first_row: Vec<String> = rows
      .next()
      .ok_or_else(|| anyhow::anyhow!("Empty file"))?
      .iter()
      .map(|cell| cell.to_string())
      .collect();

    let (id_col, language_cloumns) = parse_first_row(&first_row)?;

    // Progress reporting for large sheets (exclude header row).
    let total_rows = range.height().saturating_sub(1);
    let progress = ProgressReporter::new(total_rows);

    // Parse data rows
    for (row_idx, row) in rows.enumerate() {
      let row_num = row_idx + 2; // +2 because 0-indexed header row

      // Get ID (translation key)
      let id_str = row.get(id_col).map(|c| c.to_string()).unwrap_or_default();
      if id_str.is_empty() {
        continue; // Skip rows with empty ID
      }

      let key = TranslationKey::new(id_str)?;

      // Parse each language column
      for (col_idx, lang_code) in &language_cloumns {
        let value_str = row.get(*col_idx).map(|c| c.to_string()).unwrap_or_default();
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

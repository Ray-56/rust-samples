use crate::domain::errors::DomainError;
use crate::domain::value_objects::LanguageCode;

/// Parses the first row to detect column structure
pub fn parse_first_row(row: &[String]) -> Result<(usize, Vec<(usize, LanguageCode)>), DomainError> {
  // Find ID column
  let id_col = row
    .iter()
    .position(|cell| cell == "ID")
    .ok_or_else(|| DomainError::MissingRequiredColumn("ID".to_string()))?;

  // Find zh-CN column (marks start of language columns)
  let zh_cn_col = row
    .iter()
    .position(|cell| cell == "zh-CN")
    .ok_or_else(|| DomainError::MissingRequiredColumn("zh-CN".to_string()))?;

  // Collect all language columns (from zh-CN onwards)
  let mut language_columns = Vec::new();
  for (idx, cell) in row.iter().enumerate().skip(zh_cn_col) {
    if !cell.is_empty() {
      let lang_code = LanguageCode::new(cell.clone())?;
      language_columns.push((idx, lang_code));
    }
  }

  Ok((id_col, language_columns))
}

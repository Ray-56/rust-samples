use crate::domain::entities::LocaleEntry;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::LanguageCode;
use std::collections::{HashMap, HashSet};

/// Aggregate root representing a locale document
#[derive(Debug)]
pub struct LocaleDocument {
  entries: Vec<LocaleEntry>,
  languages: HashSet<LanguageCode>,
  source_file: String,
}

impl LocaleDocument {
  /// Creates a new locale document
  pub fn new(entries: Vec<LocaleEntry>, source_file: String) -> Result<Self, DomainError> {
    if entries.is_empty() {
      return Err(DomainError::EmptyDocument);
    }

    let languages = entries.iter().map(|e| e.language().clone()).collect();

    Ok(Self { entries, languages, source_file })
  }

  /// Gets all entries
  pub fn entries(&self) -> &[LocaleEntry] {
    &self.entries
  }

  /// Gets all languages in the document
  pub fn languages(&self) -> &HashSet<LanguageCode> {
    &self.languages
  }

  /// Groups entries by language
  pub fn entries_by_language(&self) -> HashMap<LanguageCode, Vec<&LocaleEntry>> {
    let mut grouped = HashMap::new();
    for entry in &self.entries {
      grouped
        .entry(entry.language().clone())
        .or_insert_with(Vec::new)
        .push(entry);
    }
    grouped
  }

  /// Gets entries for a specific language
  pub fn entries_for_language(&self, lang: &LanguageCode) -> Vec<&LocaleEntry> {
    self
      .entries
      .iter()
      .filter(|e| e.language() == lang)
      .collect()
  }

  /// Gets the source file path
  pub fn source_file(&self) -> &String {
    &self.source_file
  }
}

use crate::domain::entities::LocaleDocument;
use crate::domain::errors::Warning;
use std::collections::HashMap;

/// Service for detecting duplicate keys
pub struct DuplicateDetector;

impl DuplicateDetector {
  pub fn new() -> Self {
    Self
  }

  /// Detects duplicate keys within each language
  pub fn detect(&self, document: &LocaleDocument) -> Vec<Warning> {
    let mut warnings = Vec::new();
    let entries_by_lang = document.entries_by_language();

    for (lang_code, entries) in entries_by_lang {
      let mut seen_keys: HashMap<String, usize> = HashMap::new();

      for entry in entries {
        let key_str = entry.key().as_str().to_string();

        if let Some(&_first_row) = seen_keys.get(&key_str) {
          // Duplicate found
          warnings.push(Warning::DuplicateKey {
            key: key_str.clone(),
            language: lang_code.as_str().to_string(),
            row: entry.source_row(),
          });
        } else {
          seen_keys.insert(key_str, entry.source_row());
        }
      }
    }

    warnings
  }
}

impl Default for DuplicateDetector {
  fn default() -> Self {
    Self::new()
  }
}

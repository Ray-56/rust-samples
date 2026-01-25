use crate::domain::entities::LocaleDocument;
use crate::domain::errors::Warning;
use std::collections::HashMap;

/// Service for detecting placeholder in translations
pub struct PlaceholderDetector;

impl PlaceholderDetector {
  pub fn new() -> Self {
    Self
  }

  /// Detects placeholder mismatches across languages for the same key
  pub fn detect_mismatches(&self, document: &LocaleDocument) -> Vec<Warning> {
    let mut warnings = Vec::new();

    // Group entries by key
    let mut entries_by_key: HashMap<String, Vec<_>> = HashMap::new();
    for entry in document.entries() {
      entries_by_key
        .entry(entry.key().as_str().to_string())
        .or_default()
        .push(entry);
    }

    // Check each key across languages
    for (key, entries) in entries_by_key {
      if entries.len() < 2 {
        continue; // Need at least 2 languages to compare
      }

      // Count placeholders in each language
      let counts: Vec<(String, usize)> = entries
        .iter()
        .map(|e| {
          (
            e.language().as_str().to_string(),
            self.count_placeholders(e.value().raw()),
          )
        })
        .collect();

      // Check for mismatches
      if counts.len() > 1 {
        let first_count = counts[0].1;
        for i in 1..counts.len() {
          if counts[i].1 != first_count {
            warnings.push(Warning::PlaceholderMismatch {
              key: key.clone(),
              language1: counts[0].0.clone(),
              count1: first_count,
              language2: counts[i].0.clone(),
              count2: counts[i].1,
            });
          }
        }
      }
    }

    warnings
  }

  /// Counts placeholders in a string
  /// Supports: {x}, {0}, {{x}}, %s, %d, %f
  fn count_placeholders(&self, text: &str) -> usize {
    let mut count = 0;

    // Count {x} style (but not {{x}})
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
      if chars[i] == '{' {
        if i + 1 < chars.len() && chars[i + 1] != '{' {
          // Single { - count it
          count += 1;
        } else if i + 1 < chars.len() && chars[i + 1] == '{' {
          // {{ - also count as a placeholder
          count += 1;
          i += 1; // Skip next {
        }
      } else if chars[i] == '%' && i + 1 < chars.len() {
        // Check for %s, %d, %f, etc.
        let next = chars[i + 1];
        if next.is_alphabetic() {
          count += 1;
        }
      }
      i += 1;
    }

    count
  }
}

impl Default for PlaceholderDetector {
  fn default() -> Self {
    Self::new()
  }
}

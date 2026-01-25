use crate::domain::entities::{LocaleDocument, ValidationResult};
use crate::domain::errors::Warning;
use crate::domain::services::{DuplicateDetector, PlaceholderDetector};

/// Domain service for data quality validation
pub struct ValidationService {
  duplicate_detector: DuplicateDetector,
  placeholder_detector: PlaceholderDetector,
}

impl ValidationService {
  pub fn new() -> Self {
    Self {
      duplicate_detector: DuplicateDetector::new(),
      placeholder_detector: PlaceholderDetector::new(),
    }
  }

  /// Validates the entire document
  pub fn validate(&self, document: &LocaleDocument) -> ValidationResult {
    let mut warnings = Vec::new();

    // 1. Check for duplicate keys
    warnings.extend(self.check_duplicates(document));

    // 2. Check for empty translations
    warnings.extend(self.check_empty_translations(document));

    // 3. Check key formats
    warnings.extend(self.check_key_formats(document));

    // 4. Check placeholder consistency
    warnings.extend(self.check_placeholder_consistency(document));

    // 5. Check unusual lengths
    warnings.extend(self.check_unusual_lengths(document));

    ValidationResult::new(warnings)
  }

  fn check_duplicates(&self, document: &LocaleDocument) -> Vec<Warning> {
    self.duplicate_detector.detect(document)
  }

  fn check_empty_translations(&self, document: &LocaleDocument) -> Vec<Warning> {
    let mut warnings = Vec::new();

    for entry in document.entries() {
      if entry.value().is_empty() {
        warnings.push(Warning::EmptyTranslation {
          key: entry.key().as_str().to_string(),
          language: entry.language().as_str().to_string(),
          row: entry.source_row(),
        });
      }
    }

    warnings
  }

  fn check_key_formats(&self, document: &LocaleDocument) -> Vec<Warning> {
    let mut warnings = Vec::new();
    let mut checked_keys = std::collections::HashSet::new();

    for entry in document.entries() {
      let key_str = entry.key().as_str();

      // Only check each key once
      if checked_keys.contains(key_str) {
        continue;
      }
      checked_keys.insert(key_str.to_string());

      if !entry.key().is_recommended_format() {
        warnings.push(Warning::NonStandardKeyFormat {
          key: key_str.to_string(),
          row: entry.source_row(),
        });
      }
    }

    warnings
  }

  fn check_placeholder_consistency(&self, document: &LocaleDocument) -> Vec<Warning> {
    self.placeholder_detector.detect_mismatches(document)
  }

  fn check_unusual_lengths(&self, document: &LocaleDocument) -> Vec<Warning> {
    let mut warnings = Vec::new();

    for entry in document.entries() {
      if entry.value().is_unusually_long() {
        warnings.push(Warning::UnusuallyLongTranslationValue {
          key: entry.key().as_str().to_string(),
          language: entry.language().as_str().to_string(),
          length: entry.value().len(),
          row: entry.source_row(),
        });
      }
    }

    warnings
  }
}

impl Default for ValidationService {
  fn default() -> Self {
    Self::new()
  }
}

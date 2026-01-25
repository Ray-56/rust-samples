use crate::application::ports::FileWriter;
use crate::domain::entities::LocaleDocument;
use crate::domain::enums::OutputFormat;
use crate::infrastructure::adapters::FileSystemAdapter;
use anyhow::{Context, Result};
use serde_json;
use std::collections::HashMap;

/// TypeScript file writer implementaion
pub struct TypeScriptWriter {
  fs_adapter: FileSystemAdapter,
}

impl TypeScriptWriter {
  pub fn new() -> Self {
    Self { fs_adapter: FileSystemAdapter::new() }
  }
}

impl Default for TypeScriptWriter {
  fn default() -> Self {
    Self::new()
  }
}

impl FileWriter for TypeScriptWriter {
  fn write(&self, document: &LocaleDocument, output_dir: &str, format: OutputFormat) -> Result<()> {
    // Create output directory if it doesn't exist
    self.fs_adapter.create_dir_all(output_dir)?;

    // Group entries by language
    let entries_by_lang = document.entries_by_language();

    // Write a file for each language
    for (lang_code, entries) in entries_by_lang {
      let file_name = format!("{}.{}", lang_code.as_str(), format.extension());
      let file_path = format!("{output_dir}/{file_name}");

      // Build content based on format
      let content = match format {
        OutputFormat::TypeScript => {
          let mut lines = vec!["export default {".to_string()];
          let mut sorted_entries: Vec<_> = entries.iter().collect();
          sorted_entries.sort_by_key(|e| e.key().as_str());

          for (idx, entry) in sorted_entries.iter().enumerate() {
            let comma = if idx < sorted_entries.len() - 1 {
              ","
            } else {
              ""
            };
            lines.push(format!(
              "\t\"{}\": \"{}\"{}",
              entry.key().as_str(),
              entry.value().escaped(),
              comma
            ));
          }
          lines.push("}".to_string());
          lines.join("\n")
        }
        OutputFormat::Json => {
          let mut map = HashMap::new();
          for entry in entries {
            map.insert(entry.key().as_str(), entry.value().escaped());
          }
          serde_json::to_string_pretty(&map).context("Failed to serialize JSON")?
        }
      };

      self
        .fs_adapter
        .write_file(&file_path, &content)
        .with_context(|| format!("Failed to write file '{file_path}'"))?;
    }

    Ok(())
  }
}

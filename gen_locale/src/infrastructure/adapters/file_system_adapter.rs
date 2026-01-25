use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Adapter for file system operations
pub struct FileSystemAdapter;

impl FileSystemAdapter {
  pub fn new() -> Self {
    Self
  }

  /// Creates a directory and all parent directories if they don't exist
  pub fn create_dir_all(&self, path: &str) -> Result<()> {
    fs::create_dir_all(path).with_context(|| format!("Failed to create directory '{path}'"))
  }

  /// Writes content to a file
  pub fn write_file(&self, path: &str, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("Failed to write file '{path}'"))
  }

  /// Checks if a file exists
  pub fn file_exists(&self, path: &str) -> bool {
    Path::new(path).exists()
  }
}

impl Default for FileSystemAdapter {
  fn default() -> Self {
    Self::new()
  }
}

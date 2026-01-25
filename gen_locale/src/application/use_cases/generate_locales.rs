use crate::application::dto::{GenerationConfig, GenerationResult};
use crate::application::ports::{FileReader, FileWriter};
use crate::domain::services::ValidationService;
use anyhow::{Context, Result};

/// Use case for generating locale files
pub struct GenerateLocalesUseCase<R, W>
where
  R: FileReader,
  W: FileWriter,
{
  reader: R,
  writer: W,
  validator: ValidationService,
}

impl<R, W> GenerateLocalesUseCase<R, W>
where
  R: FileReader,
  W: FileWriter,
{
  pub fn new(reader: R, writer: W, validator: ValidationService) -> Self {
    Self { reader, writer, validator }
  }

  pub fn execute(&self, config: GenerationConfig) -> Result<GenerationResult> {
    // 1. Read and parse file
    let parsing_result = self
      .reader
      .read(&config.input_path, config.sheet_name.as_deref())
      .context("Failed to parse input file")?;

    // 2. Validate document
    let validation_result = self.validator.validate(&parsing_result.document);

    // 3. Write output files
    self
      .writer
      .write(
        &parsing_result.document,
        &config.output_dir,
        config.output_format,
      )
      .context("Failed to write output files")?;

    // 4. Combine warnings from parsing and validation
    let mut all_warnings = parsing_result.warnings;
    all_warnings.extend(validation_result.warnings);

    // 5. Return result
    Ok(GenerationResult::new(
      parsing_result.document.entries().len(),
      parsing_result.document.entries().len(),
      all_warnings,
    ))
  }
}

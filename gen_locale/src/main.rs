use clap::Parser;
use gen_locale::application::dto::GenerationConfig;
use gen_locale::application::use_cases::GenerateLocalesUseCase;
use gen_locale::domain::enums::{FileFormat, OutputFormat};
use gen_locale::domain::services::ValidationService;
use gen_locale::infrastructure::parsers::{CsvParser, ExcelParser};
use gen_locale::infrastructure::writers::TypeScriptWriter;
use gen_locale::interface::cli::args::Cli;
use gen_locale::interface::cli::feedback;
use std::time::Instant;

fn main() {
  let start = Instant::now();
  let cli = Cli::parse();

  // Run the application
  if let Err(e) = run(cli, start) {
    feedback::display_error(&e);
    std::process::exit(1);
  }
}

fn run(cli: Cli, start: Instant) -> anyhow::Result<()> {
  // Detect file format
  let file_format = FileFormat::from_extension(&cli.input)?;
  let output_format = OutputFormat::from_string(&cli.ext)?;

  // Create configuration
  let config = GenerationConfig::new(
    cli.input.clone(),
    cli.dir.clone(),
    Some(cli.sheet.clone()),
    output_format,
  );

  // Create dependencies based on file format
  let validator = ValidationService::new();
  let writer = TypeScriptWriter::new();

  // Execute use case
  let result = match file_format {
    FileFormat::Excel => {
      let reader = ExcelParser::new();
      let use_case = GenerateLocalesUseCase::new(reader, writer, validator);
      use_case.execute(config)?
    }
    FileFormat::Csv => {
      let reader = CsvParser::new();
      let use_case = GenerateLocalesUseCase::new(reader, writer, validator);
      use_case.execute(config)?
    }
  };

  // Display success message
  let elapsed = start.elapsed().as_millis();
  feedback::display_success(&result, &cli.dir, elapsed);

  // Return success
  Ok(())
}

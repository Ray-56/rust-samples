use crate::application::dto::GenerationResult;
use std::path::Path;

/// Displays success message with statistics
pub fn display_success(result: &GenerationResult, output_dir: &str, elapsed_ms: u128) {
  let abs_path = Path::new(output_dir)
    .canonicalize()
    .unwrap_or_else(|_| Path::new(output_dir).to_path_buf());

  println!("SUCCESS: The target files is generated.");
  println!(" Files are stored in \"{}\".", abs_path.display());

  if result.rows_processed > 1000 {
    println!(
      " Processed: {} rows, Generated: {} files.",
      result.rows_processed, result.files_generated
    );
  }

  if elapsed_ms >= 1000 {
    println!(" Time elapsed is: {:.2}s", elapsed_ms as f64 / 1000.0);
  } else {
    println!(" Time elapsed is: {elapsed_ms}ms");
  }

  // Display warnings if any
  if !result.warnings.is_empty() {
    eprintln!("\nWarnings:");
    for warning in &result.warnings {
      eprintln!(" {}", warning.to_message());
    }
  }
}

/// Displays error message
pub fn display_error(error: &anyhow::Error) {
  eprintln!("Error: {error}");
  for cause in error.chain().skip(1) {
    eprintln!(" Caused by: {cause}");
  }
}

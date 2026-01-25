use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Progress reporter for large file processing
pub struct ProgressReporter {
  total_rows: usize,
  processed_rows: Arc<AtomicUsize>,
  last_update: Arc<Mutex<Instant>>,
  update_interval: Duration,
  enabled: bool,
}

impl ProgressReporter {
  /// Creates a new progress reporter
  /// Only enabled for files with more than 1000 rows
  pub fn new(total_rows: usize) -> Self {
    let enabled = total_rows > 1000;
    Self {
      total_rows,
      processed_rows: Arc::new(AtomicUsize::new(0)),
      last_update: Arc::new(Mutex::new(Instant::now())),
      update_interval: Duration::from_secs(1),
      enabled,
    }
  }

  /// Increaments the processed row count and displays progress if needed
  pub fn increment(&self) {
    if !self.enabled {
      return;
    }

    let count = self.processed_rows.fetch_add(1, Ordering::Relaxed) + 1;

    // Update every 1000 rows or every second
    if count % 1000 == 0 || self.should_update() {
      self.print_progress(count);
    }
  }

  /// Checks if enough time has elapsed to update progress
  fn should_update(&self) -> bool {
    if let Ok(last_update) = self.last_update.lock() {
      last_update.elapsed() >= self.update_interval
    } else {
      false
    }
  }

  /// Prints the current progress
  fn print_progress(&self, count: usize) {
    if let Ok(mut last_update) = self.last_update.lock() {
      *last_update = Instant::now();
      let percentage = (count * 100) / self.total_rows;
      eprint!(
        "\rProcessing: {}/{} rows ({}%)",
        count, self.total_rows, percentage
      );
    }
  }

  /// Finalizes the progress display
  pub fn finish(&self) {
    if self.enabled {
      eprintln!(); // New line after progress
    }
  }
}

impl Clone for ProgressReporter {
  fn clone(&self) -> Self {
    Self {
      total_rows: self.total_rows,
      processed_rows: Arc::clone(&self.processed_rows),
      last_update: Arc::clone(&self.last_update),
      update_interval: self.update_interval,
      enabled: self.enabled,
    }
  }
}

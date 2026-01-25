use std::fs;
use std::path::{Path, PathBuf};

fn collect_rs_files(dir: &Path) -> Vec<PathBuf> {
  let mut out = Vec::new();
  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        out.extend(collect_rs_files(&path));
      } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
        out.push(path);
      }
    }
  }
  out
}

fn read_to_string(path: &Path) -> String {
  fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {:?}: {}", path, e))
}

#[test]
fn test_domain_layer_no_external_crate_dependencies() {
  let root = Path::new(env!("CARGO_MANIFEST_DIR"));
  let domain_dir = root.join("src").join("domain");

  for file in collect_rs_files(&domain_dir) {
    let content = read_to_string(&file);

    // Disallow any direct uses of external crates (heuristic)
    // - `use <crate>::...` where <crate> is not one of:
    //   crate/super/self/std/core/alloc
    for (idx, line) in content.lines().enumerate() {
      let l = line.trim_start();
      if !l.starts_with("use ") {
        continue;
      }
      let after_use = l.trim_start_matches("use ").trim_start();
      let first_seg = after_use
        .split("::")
        .next()
        .unwrap_or("")
        .trim()
        .trim_end_matches(';');

      let allowed = matches!(
        first_seg,
        "crate" | "super" | "self" | "std" | "core" | "alloc"
      );
      if !allowed {
        panic!(
          "Domain file {:?} line {} uses external crate '{}': {}",
          file,
          idx + 1,
          first_seg,
          line
        );
      }
    }
  }
}

#[test]
fn test_dependency_direction_correct() {
  let root = Path::new(env!("CARGO_MANIFEST_DIR"));
  let domain_dir = root.join("src").join("domain");
  let application_dir = root.join("src").join("application");
  let infrastructure_dir = root.join("src").join("infrastructure");
  let interface_dir = root.join("src").join("interface");

  // Domain must not depend on outer layers.
  for file in collect_rs_files(&domain_dir) {
    let content = read_to_string(&file);
    for forbidden in ["crate::application::", "crate::infrastructure::", "crate::interface::"] {
      assert!(
        !content.contains(forbidden),
        "Domain file {:?} must not reference {}",
        file,
        forbidden
      );
    }
  }

  // Application must not depend on infrastructure/interface implementations.
  for file in collect_rs_files(&application_dir) {
    let content = read_to_string(&file);
    for forbidden in ["crate::infrastructure::", "crate::interface::"] {
      assert!(
        !content.contains(forbidden),
        "Application file {:?} must not reference {}",
        file,
        forbidden
      );
    }
  }

  // Infrastructure must not depend on interface.
  for file in collect_rs_files(&infrastructure_dir) {
    let content = read_to_string(&file);
    assert!(
      !content.contains("crate::interface::"),
      "Infrastructure file {:?} must not reference crate::interface::",
      file
    );
  }

  // Interface must not depend on infrastructure.
  for file in collect_rs_files(&interface_dir) {
    let content = read_to_string(&file);
    assert!(
      !content.contains("crate::infrastructure::"),
      "Interface file {:?} must not reference crate::infrastructure::",
      file
    );
  }
}

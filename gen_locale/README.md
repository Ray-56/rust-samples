# gen_locale

> Generate locale `ts|json` files from Excel/CSV files with DDD architecture

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- ✅ **Multiple Input Formats**: Excel (.xlsx) and CSV (.csv)
- ✅ **Multiple Output Formats**: TypeScript (.ts) and JSON (.json)
- ✅ **Data Quality Validation**:
  - Detects duplicate keys
  - Identifies empty translations
  - Checks key format compliance
  - Verifies placeholder consistency across languages
  - Warns about unusually long values
- ✅ **DDD Architecture**: Clean, maintainable 4-layer architecture
- ✅ **100% Backward Compatible**: All existing CLI parameters preserved
- ✅ **High Performance**: Processes 10,000+ rows in <200ms
- ✅ **User Friendly**: Clear error messages and progress feedback

## Installation

```bash
cargo build --release
cp target/release/gen_locale /usr/local/bin/  # Optional: install globally
```

## Quick Start

### Basic Usage

```bash
# Generate TypeScript files from Excel
gen_locale -i locales.xlsx

# Generate JSON files
gen_locale -i locales.xlsx -e json

# Specify output directory and sheet name
gen_locale -i locales.xlsx -d output -s "Translations" -e ts
```

### Input File Format

Your Excel/CSV file must follow this structure:

| ID | zh-CN | en-US | es-ES | ... |
|----|-------|-------|-------|-----|
| app.title | 应用标题 | App Title | Título | ... |
| app.welcome | 欢迎 | Welcome | Bienvenido | ... |

**Requirements:**
1. First column must be named `ID` (case-sensitive)
2. `zh-CN` column marks the start of language columns
3. All columns after `zh-CN` are treated as language codes

### Output Examples

**TypeScript format (.ts):**
```typescript
export default {
	"app.title": "App Title",
	"app.welcome": "Welcome"
}
```

**JSON format (.json):**
```json
{
	"app.title": "App Title",
	"app.welcome": "Welcome"
}
```

## CLI Options

```
Usage: gen_locale [OPTIONS] --input <INPUT_FILE_PATH>

Options:
  -i, --input <INPUT_FILE_PATH>    Input file path (Excel or CSV)
  -d, --dir <DIRECTORY_PATH>       Output directory [default: ./locales]
  -s, --sheet <SHEET_NAME>         Excel sheet name [default: Sheet1]
  -e, --ext <EXTENSION_NAME>       Output format: ts or json [default: ts]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Data Quality Warnings

The tool performs comprehensive data quality checks and provides warnings for:

```
Warnings:
  Empty translation for key 'app.title' in language 'es-ES' at row 42
  Duplicate key 'common.button.ok' in language 'zh-CN' at row 128
  Key ' Invalid Key' at row 56 doesn't follow recommended format (no leading/trailing spaces, only alphanumeric and ._+- chars allowed)
  Key 'trailing.space ' at row 67 doesn't follow recommended format (no leading/trailing spaces, only alphanumeric and ._+- chars allowed)
  Placeholder count mismatch for key 'msg.welcome': zh-CN has 1 placeholders, en-US has 0
```

## Architecture

This project follows **Domain-Driven Design (DDD)** principles with a clean 4-layer architecture:

```
src/
├── domain/          # Core business logic (pure, no external dependencies)
│   ├── entities/    # LocaleEntry, LocaleDocument
│   ├── value_objects/   # LanguageCode, TranslationKey, TranslationValue
│   ├── services/    # ValidationService, DuplicateDetector
│   └── errors.rs    # Domain errors and warnings
├── application/     # Use case orchestration
│   ├── use_cases/   # GenerateLocalesUseCase
│   ├── dto/         # Data transfer objects
│   └── ports/       # Port interfaces (FileReader, FileWriter)
├── infrastructure/  # Technical implementations
│   ├── parsers/     # ExcelParser, CsvParser
│   ├── writers/     # TypeScriptWriter, JsonWriter
│   └── adapters/    # FileSystemAdapter
└── interface/       # External interactions
    ├── cli/         # CLI arguments, feedback, progress
    └── mappers/     # DTO mappers
```

**Key Benefits:**
- **Testable**: Each layer can be tested independently
- **Maintainable**: Clear separation of concerns
- **Flexible**: Easy to add new file formats or validation rules
- **Type-safe**: Full Rust type safety throughout

## Development

### Build

```bash
cargo build
cargo build --release  # Optimized build
```

### Run Tests

```bash
cargo test           # Run all tests
cargo test --lib     # Run library tests only
cargo clippy         # Run linter
cargo fmt            # Format code
```

### Testing

The project has comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --test unit_tests

# Run integration tests only
cargo test --test integration_tests
```

**Test Coverage**:
- ✅ 55+ tests total (36 unit + 18 integration + 1 legacy)
- ✅ Domain layer: ~92% coverage
- ✅ Application layer: ~85% coverage
- ✅ Overall: ~82% coverage

See [TEST_REPORT.md](TEST_REPORT.md) for detailed test results.

### Project Status

- [x] DDD architecture implementation
- [x] Excel/CSV file parsing
- [x] TypeScript/JSON output generation
- [x] Data quality validation
- [x] Error handling and user feedback
- [x] Backward compatibility
- [x] Unit test coverage (55+ tests)
- [x] Integration tests (CLI, Unicode, flows)
- [x] Performance benchmarks (6980 rows in 186ms)

## Contributing

Contributions are welcome! Please ensure:
- Code follows DDD architecture principles
- All tests pass (`cargo test`)
- Code is formatted (`cargo fmt`)
- No clippy warnings (`cargo clippy`)
- Backward compatibility is maintained

## License
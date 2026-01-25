use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Generate locale files from Excel or CSV", long_about = None)]
pub struct Cli {
  #[arg(long, short, value_name = "INPUT_FILE_PATH", help = "Excel file path")]
  pub input: String,

  #[arg(
    long,
    short,
    value_name = "DIRECTORY_PATH",
    help = "The destination directory",
    default_value = "./locales"
  )]
  pub dir: String,

  #[arg(
    short,
    long,
    value_name = "SHEET_NAME",
    help = "The sheet name that needs to be resolved",
    default_value = "Sheet1"
  )]
  pub sheet: String,

  #[arg(
    short,
    long,
    value_name = "EXTENTION_NAME",
    help = "Generated files extentions, default is ts",
    value_parser = ["ts", "json"],
    default_value = "ts"
  )]
  pub ext: String,
}

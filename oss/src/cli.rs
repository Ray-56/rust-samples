use clap::{ArgGroup, Parser};

#[derive(Parser, Debug, Clone)]
#[command(version, about = "aliyun oss actions", long_about = None)]
#[clap(group = ArgGroup::new("input").required(true).multiple(false))]
pub struct Cli {
    #[arg(group = "input", long, short = 'i', value_name = "INPUT_ROOT", help = "The input directory")]
    pub input_root: Option<String>,
    #[arg(group = "input", long, short = 'p', value_name = "INPUT_PATTERN", help = "The input pattern")]
    pub input_pattern: Option<String>,
    /// The destination directory
    #[arg(long, short = 'd', value_name = "DESTINATION_DIRECTORY", help = "The destination directory")]
    pub dest_dir: String,
    #[arg(long, value_name = "HTML_LAST", help = "The html file is last upload", default_value = "false")]
    pub html_last: bool,
}

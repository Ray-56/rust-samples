use clap::{arg, command};

pub fn command() -> clap::Command {
    command!()
        .arg(
            arg!(input: -i [INPUT_FILE_PATH] "Excel file path")
                .long("input")
                .required(true),
        )
        .arg(
            arg!(dir: -d <DIRECTORY_PATH> "The destination directory")
                .long("dir")
                .default_value("./dist"),
        )
        .arg(
            arg!(sheet: -s <SHEET_NAME> "The sheet name that needs to be resolved")
                .long("sheet")
                .default_value("Sheet1"),
        )
        .arg(
            arg!(ext: -e <EXTENTION_NAME> "Generated files extentions, default is ts")
                .long("ext")
                .value_parser(["ts", "json"])
                .default_value("ts"),
        )
}

#[test]
fn verify_cmd() {
    command().debug_assert();
}

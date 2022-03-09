use glob::{glob_with, MatchOptions};
use regex::Regex;
use std::{env, fs, process};
use walkdir::DirEntry;

fn _is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn main() -> Result<(), walkdir::Error> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // let current_dir = env::current_dir().unwrap();
    // println!("{:?}", current_dir);

    // 使用 walkdir 获取
    // let walker = WalkDir::new("test").into_iter();
    // for entry in walker.filter_map(|e| e.ok()) {
    //     let path = entry.path().display().to_string();
    //     println!("{}, is file {}", path, Path::new(&path).is_file());
    // }
    // let total_size = WalkDir::new("test")
    //     .min_depth(1)
    //     .max_depth(3)
    //     .into_iter()
    //     .filter_map(|entry| entry.ok());
    //     // .filter_map(|entry| entry.metadata().ok());

    // for entry in total_size {
    //     println!("{:?}", entry.into_path())
    // }

    // 使用 glob 获取
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    let mut records = glob_with(&config.pattern, options)
        .expect("Failed to read glob pattern")
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .map(|path| {
            let path_string = path.display().to_string();
            let text = fs::read_to_string(&path_string).unwrap();
            let lines = text.lines().fold(0u32, |acc, _| acc + 1);
            RecordByPath {
                path: path_string,
                lines,
                chars: text.len() as u32,
            }
        })
        .collect::<Vec<RecordByPath>>();

    let total = summary(&records);

    if config.extension {
        let mut exist_names: Vec<String> = Vec::new();
        let extension_names = &records
            .iter()
            .map(|r| get_extension_name(r))
            .filter(|name| {
                if exist_names.contains(name) {
                    return false;
                }
                exist_names.push(name.clone());
                true
            })
            .collect::<Vec<String>>();
        let extensions = extension_list(&records, extension_names);
        records.splice(0..1, extensions);
    }

    records.insert(0, total);

    print_table(&records);

    Ok(())
}

#[derive(Debug)]
struct Config {
    /// Unix shell style patterns
    pattern: String,
    /// 是否以扩展名类型展示
    extension: bool,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        let count = args.len();
        println!("{}", count);
        if count < 2 {
            return Err("The current program supports only one parameter");
        }
        let pattern = args[1].clone();
        let extension = args.contains(&String::from("--extension"));

        Ok(Config { pattern, extension })
    }
}

#[derive(Debug)]
struct RecordByPath {
    path: String,
    lines: u32,
    chars: u32,
}

fn summary<'a>(records: &'a Vec<RecordByPath>) -> RecordByPath {
    let mut total = RecordByPath {
        path: "Total".to_string(),
        lines: 0,
        chars: 0,
    };
    for record in records.into_iter() {
        total.lines += record.lines;
        total.chars += record.chars;
    }

    total
}

fn extension_list<'a>(records: &'a Vec<RecordByPath>, names: &'a Vec<String>) -> Vec<RecordByPath> {
    let mut ret = names
        .iter()
        .map(|name| RecordByPath {
            path: name.to_string(),
            lines: 0,
            chars: 0,
        })
        .collect::<Vec<RecordByPath>>();

    records.iter().for_each(|record| {
        let name = get_extension_name(record);
        ret.iter_mut().for_each(|r| {
            if r.path == name {
                r.lines += record.lines;
                r.chars += record.chars;
            }
        })
    });

    ret
}

fn get_extension_name(record: &RecordByPath) -> String {
    let re = Regex::new(r"\w+\.(\w+)$").unwrap();
    let caps = re.captures(&record.path.as_str());
    let extension_name = match caps {
        Some(list) => {
            let mut v: Vec<char> = list[1].to_string().chars().collect();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            v.into_iter().collect()
        }
        _ => String::from("Not Found"),
    };
    extension_name
}

fn print_table<'a>(records: &'a Vec<RecordByPath>) {
    let max = &records.iter().map(|r| r.path.len().clone()).max();
    let max = max.unwrap() + 2;
    println!("{0: >max$} | {1: >10} | {2: >10}", "Path", "Lines", "Chars");
    records.iter().for_each(|record| {
        println!(
            "{0: >max$} | {1: >10} | {2: >10}",
            record.path, record.lines, record.chars
        )
    });
}

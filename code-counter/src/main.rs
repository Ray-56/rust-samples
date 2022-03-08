use glob::{glob_with, MatchOptions};
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
    let records = glob_with(&config.pattern, options)
        .expect("Failed to read glob pattern")
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .map(|path| {
            let text = fs::read_to_string(path.display().to_string()).unwrap();
            let lines = text.lines().fold(0u32, |acc, _| acc + 1);
            RecordByPath {
                path: path.display().to_string(),
                lines,
                chars: text.len() as u32,
            }
        });

    // for record in records {
    //     println!("{:?}", record);
    // }
    let record = records.fold(
        RecordByPath {
            path: String::from("ALL"),
            lines: 0,
            chars: 0,
        },
        |acc, cur| {
            println!(
                "The `{}` contains {} lines or {} characters",
                cur.path, cur.lines, cur.chars
            );
            return RecordByPath {
                path: acc.path,
                lines: acc.lines + cur.lines,
                chars: acc.chars + cur.chars,
            };
        },
    );
    println!(
        "All contains {} lines or {} characters",
        record.lines, record.chars
    );
    // for entry in glob_with(&config.pattern, options).expect("Failed to read glob pattern") {
    //     match entry {
    //         Ok(path) => {
    //             if path.is_file() {
    //                 let suffix = Path::new(path.as_os_str()).extension().unwrap();
    //                 // println!("suffix is {:?}", suffix);
    //                 let text = fs::read_to_string(path.display().to_string()).unwrap();
    //                 let line_counter = text.lines().fold(0u32, |acc, _| acc + 1);

    //                 println!(
    //                     "The `{}` contains {} lines or {} characters",
    //                     path.display(),
    //                     line_counter,
    //                     text.len()
    //                 );
    //             }
    //         }
    //         Err(e) => println!("error: {:?}", e),
    //     }
    // }

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
        let extension = args.contains(&String::from("-extension"));

        Ok(Config { pattern, extension })
    }
}

#[derive(Debug)]
struct RecordByPath {
    path: String,
    lines: u32,
    chars: u32,
}

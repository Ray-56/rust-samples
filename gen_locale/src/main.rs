use std::env::current_dir;
use std::io::Result;
use std::{collections::HashMap, time::Instant};

use calamine::{open_workbook, Reader, Xlsx};
use gen_locale::{command, parse_first_row, parse_row, write_locales};

fn main() -> Result<()> {
    let start = Instant::now();
    let matches = command().get_matches();
    let input_path = matches
        .get_one::<String>("input")
        .expect("INPUT_FILE_PATH is required");
    let dir_dist = matches
        .get_one::<String>("dir")
        .expect("The App has set default value for DIRECTORY_PATH");
    let sheet_name = matches
        .get_one::<String>("sheet")
        .expect("The App has set default value for SHEET_NAME");
    let extension_name = matches
        .get_one::<String>("ext")
        .expect("The App has set default value for EXTENTION_NAME");

    let mut excel: Xlsx<_> = open_workbook(input_path).unwrap();
    if let Some(Ok(r)) = excel.worksheet_range(sheet_name) {
        let mut list = r.rows();

        let first = list.next().unwrap();
        let locale_index_conf = parse_first_row(first);

        let mut locales_column: HashMap<String, Vec<String>> = HashMap::new();
        for locale in locale_index_conf.iter().clone() {
            match &locale.lang {
                None => {}
                Some(lang) => {
                    locales_column.insert(lang.clone(), Vec::new());
                }
            }
        }

        let mut lang_key_list: Vec<String> = Vec::new();

        for row in list {
            let row_parsed = parse_row(row, &locale_index_conf)?;
            for parsed in row_parsed {
                let lang_key = format!("{}-{}", parsed.lang, parsed.key);
                // println!("lang_key={:?}", lang_key);
                if lang_key_list.contains(&lang_key) {
                    println!("WARNING: The key {:?} is duplicated.", parsed.key);
                } else {
                    lang_key_list.push(lang_key);
                }

                let mut new_row = locales_column.get(&parsed.lang).unwrap().to_owned();
                let item = if parsed.key.is_empty() && parsed.value.is_empty() {
                    "\n".to_string()
                } else {
                    let prefix = if new_row.is_empty() { "" } else { ",\n" };
                    format!("{}\t\"{}\": \"{}\"", prefix, parsed.key, parsed.value)
                };
                new_row.push(item);
                locales_column.insert(parsed.lang, new_row);
            }
        }
        for locale in locales_column {
            write_locales(locale.0, locale.1, dir_dist, extension_name)?;
        }

        let duration = start.elapsed();
        let path = current_dir()?.join(dir_dist);
        println!("SUCCESS: The target files is generated. \n Files are stored in {:?}. \n Time elapsed is: {:?}", path.display(), duration);
    } else {
        println!("ERROR: The {:?} sheet name is not found.", sheet_name);
    }

    Ok(())
}

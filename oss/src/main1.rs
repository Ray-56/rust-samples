use clap::Parser;
use glob::{glob_with, MatchOptions};
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

use oss::Cli;
use sdk::request::RequestBuilder;
use sdk::{error::OssError, oss::OSS};

// TODO: html 最后上传  ✅
// TODO: 记录上传文件数量  ✅
// TODO: 如果 input 是文件夹，那么上传文件夹下所有文件。如果是文件，那么上传文件
//       改为把路径的第一个文件夹移除  ✅

#[derive(Debug, Clone)]
struct Entity {
    pub path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), OssError> {
    let start = Instant::now();
    let cli = Cli::parse();
    println!("{:?}", cli);
    dotenvy::dotenv().ok();
    let mut entry_list = Vec::<Entity>::new();

    if let Some(input_pattern) = &cli.input_pattern {
        let options = MatchOptions {
            case_sensitive: true,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        match glob_with(input_pattern, options) {
            Ok(glob) => {
                entry_list = glob.filter_map(|r| r.ok()).filter(|e| e.is_file()).map(|e| Entity { path: e }).collect::<Vec<_>>();
            }
            Err(e) => {
                return Err(OssError::Err(format!("glob error: {}", e)));
            }
        }
    } else if let Some(input_root) = &cli.input_root {
        let walk = WalkDir::new(input_root);
        entry_list = walk.into_iter().filter_map(|r| r.ok()).filter(|e| e.file_type().is_file()).map(|e| Entity { path: e.path().to_path_buf() }).collect::<Vec<_>>();
    }
    println!("need files length is: {}", entry_list.len());

    let mut html_files_to_upload = vec![];
    if cli.html_last {
        entry_list.retain(|e| {
            if e.path.extension().and_then(|ext| ext.to_str()) == Some("html") {
                html_files_to_upload.push(e.clone());
                false
            } else {
                true
            }
        });
        println!("html_files_to_upload: {:?}", html_files_to_upload);
        println!("other entry_list: {:?}", entry_list);
    }

    let oss = OSS::from_env();
    oss.open_debug();
    let mut uploaded_len = 0;

    for e in &entry_list {
        let r = upload_file(&e, &cli.dest_dir, &oss).await;
        if let Ok(_) = r {
            uploaded_len += 1;
        }

        if uploaded_len == entry_list.len() {
            println!("start upload html files");
            for e in &html_files_to_upload {
                let r = upload_file(&e, &cli.dest_dir, &oss).await;
                if let Ok(_) = r {
                    uploaded_len += 1;
                }
            }
        }
    }

    if uploaded_len == entry_list.len() + html_files_to_upload.len() {
        println!("all files uploaded");
    } else {
        println!("uploaded files: {}/{}", uploaded_len, entry_list.len() + html_files_to_upload.len());
    }

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}.", duration);

    Ok(())
}

fn get_first_folder(path: &Path) -> Option<&str> {
    let first_component = path.iter().next().and_then(|os_str| os_str.to_str());
    if let Some(component) = first_component {
        let component_path = Path::new(component);
        if component_path.is_dir() {
            Some(component)
        } else {
            None
        }
    } else {
        None
    }
}

async fn upload_file(e: &Entity, dest: &String, oss: &OSS) -> Result<(), OssError> {
    let file_path = e.path.display().to_string();
    let parent = get_first_folder(&e.path).unwrap_or("");
    let dest_path = if let Ok(relative_path) = e.path.strip_prefix(&parent) {
        Path::new(dest).join(relative_path).display().to_string()
    } else {
        Path::new(dest).join(&e.path).display().to_string()
    };
    let ext = e.path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let mime = mime_guess::from_ext(ext).first_or_octet_stream();
    let builder = RequestBuilder::new().with_expire(60).with_content_type(mime);
    oss.put_object_from_file(dest_path, file_path, builder).await
}

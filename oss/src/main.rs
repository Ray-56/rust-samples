use clap::Parser;
use futures::future::join_all;
use glob::{glob_with, MatchOptions};
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;
use tokio::task;
use walkdir::WalkDir;

use oss::Cli;
use sdk::request::RequestBuilder;
use sdk::{error::OssError, oss::OSS};

#[derive(Debug, Clone)]
struct Entity {
    pub path: PathBuf,
}

async fn upload_file(e: &Entity, dest: &String, oss: &OSS) -> Result<(), OssError> {
    let file_path = e.path.display().to_string();
    let parent = get_first_folder(&e.path).unwrap_or("");
    let dest_path = if let Ok(relative_path) = e.path.strip_prefix(&parent) {
        Path::new(dest).join(relative_path).display().to_string()
    } else {
        Path::new(dest).join(&e.path).display().to_string()
    };
    let ext = e
        .path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    let mime = mime_guess::from_ext(ext).first_or_octet_stream();
    let builder = RequestBuilder::new()
        .with_expire(60)
        .with_content_type(mime);
    oss.put_object_from_file(dest_path, file_path, builder)
        .await
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

#[tokio::main]
async fn main() -> Result<(), OssError> {
    let start = Instant::now();
    let cli = Cli::parse();
    match dotenvy::dotenv() {
        Err(_) => {
            dotenvy::from_filename(".env.local").ok();
        }
        Ok(_) => {}
    }
    let mut entry_list = Vec::<Entity>::new();

    if let Some(input_pattern) = &cli.input_pattern {
        let options = MatchOptions {
            case_sensitive: true,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        match glob_with(input_pattern, options) {
            Ok(glob) => {
                entry_list = glob
                    .filter_map(|r| r.ok())
                    .filter(|e| e.is_file())
                    .map(|e| Entity { path: e })
                    .collect::<Vec<_>>();
            }
            Err(e) => {
                return Err(OssError::Err(format!("glob error: {}", e)));
            }
        }
    } else if let Some(input_root) = &cli.input_root {
        let walk = WalkDir::new(input_root);
        entry_list = walk
            .into_iter()
            .filter_map(|r| r.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| Entity {
                path: e.path().to_path_buf(),
            })
            .collect::<Vec<_>>();
    }
    let total_len = entry_list.len();
    println!("need files length is: {}", total_len);
    let mut uploaded_len = 0;

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
    }

    let oss = OSS::from_env();
    // oss.open_debug();

    let upload_tasks: Vec<_> = entry_list
        .clone()
        .into_iter()
        .map(|entity| {
            let oss = oss.clone();
            let dest = cli.dest_dir.clone();
            task::spawn(async move { upload_file(&entity, &dest, &oss).await })
        })
        .collect();
    let results = join_all(upload_tasks).await;
    for result in results {
        match result {
            Ok(Ok(())) => {
                uploaded_len += 1;
                eprintln!("uploaded files: {}/{}", uploaded_len, total_len);
            }
            Ok(Err(e)) => eprintln!("Upload failed: {:?}", e),
            Err(e) => eprintln!("Task failed: {:?}", e),
        }
    }

    if html_files_to_upload.len() > 0 {
        let upload_tasks: Vec<_> = html_files_to_upload
            .clone()
            .into_iter()
            .map(|entity| {
                let oss = oss.clone();
                let dest = cli.dest_dir.clone();
                task::spawn(async move { upload_file(&entity, &dest, &oss).await })
            })
            .collect();
        let results = join_all(upload_tasks).await;
        for result in results {
            match result {
                Ok(Ok(())) => {
                    uploaded_len += 1;
                    eprintln!("uploaded files: {}/{}", uploaded_len, total_len);
                }
                Ok(Err(e)) => eprintln!("HTML Upload failed: {:?}", e),
                Err(e) => eprintln!("HTML Task failed: {:?}", e),
            }
        }
    }

    if uploaded_len == total_len {
        println!("all files uploaded");
    } else {
        println!(
            "something wrong. already uploaded files: {}/{}",
            uploaded_len, total_len
        );
    }
    println!("Elapsed time: {:?}", start.elapsed());
    Ok(())
}

use std::fs::create_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

pub fn write_locales(
    file_name: String,
    content: Vec<String>,
    dir_dist: &String,
    ext: &String,
) -> Result<()> {
    if !Path::new(&dir_dist).exists() {
        create_dir(dir_dist.clone())?;
    }

    let path = format!("{}/{}.{}", dir_dist, file_name, ext);

    let mut file = File::create(path)?;
    let new_content = if ext == "ts" {
        format!("export default {{\n{}\n}}", content.join(""))
    } else {
        format!("{{\n{}\n}}", content.join(""))
    };
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

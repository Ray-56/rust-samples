use std::fs::create_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

pub fn write_locales(file_name: String, content: Vec<String>, dir_dist: &String) -> Result<()> {
    if !Path::new(&dir_dist).exists() {
        create_dir(dir_dist.clone())?;
    }

    let path = format!("{}/{}.ts", dir_dist, file_name);

    let mut file = File::create(path)?;
    let new_content = format!("export default {{ \n{}}}", content.join(""));
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

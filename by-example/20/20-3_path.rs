use std::path::Path;

fn main() {
    // Create a `Path` from `&'static str`
    let path = Path::new(".");

    // The `display` method returns a showable structure
    let display = path.display();

    // `join` merges paths into bytes containers using operating system specific delimiters and returns the new path
    let new_path = path.join("a").join("b");

    // Convert path into a character slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
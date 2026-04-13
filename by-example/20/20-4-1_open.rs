use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the required files
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returning `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string describing the error.
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string and return `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope and the `hello.txt` file will be closed.
}
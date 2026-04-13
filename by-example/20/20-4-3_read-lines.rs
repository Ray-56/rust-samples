use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // The file host must exist in the current path before output can be generated
    if let Ok(lines) = read_lines("./hosts") {
        // Using an iterator, return an (optional) string
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

// Output is wrapped in Result to allow matching errors
// Reader that returns an iterator to the file lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
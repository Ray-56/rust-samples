// Global variables are declared outside all other scopes
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constants in normal functions
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constants in the main function (main function)
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
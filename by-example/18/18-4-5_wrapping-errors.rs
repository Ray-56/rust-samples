use std::error;
use std::fmt;
use std::num::ParseIntError;

// Aliases `Box<error::Error>`
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // In this error type, we use the implementation of the `Err` part of the error type of `parse`
    // If you want to provide more information, you need to add more data to this type
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // This is a wrapper that uses internal types' implementations of `fmt`.
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            // The reason lies within the implementation of the wrong type. It is implicitly converted to the trait object `&error::Error`
            // This works because the inner type already implements the `Error` trait
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

// Implement conversion from `ParseIntError` to `DoubleError`
// It will be called automatically when using `?`, or when a `ParseIntError` needs to be converted to `DoubleError`
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

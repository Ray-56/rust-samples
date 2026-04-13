use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
// Define our error type, which can be customized according to the actual situation of error handling
// We can completely customize the error type, or we can completely adopt the underlying error implementation in the type
// It can also be somewhere in between
struct DoubleError;

// The generation of an error has absolutely nothing to do with how it appears. No need to worry about complex logic leading to confusing displays
// 
// Note that we do not store any additional information about the error, that is, without modifying our error type definition
// It is impossible to specify which string parsing failed
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// Implement the `Error` trait for `DoubleError` so that other errors can wrap this error type
impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, internal cause not documented
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // Change errors to our new type
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // Also change to new type here
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
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
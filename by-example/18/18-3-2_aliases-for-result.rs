use std::num::ParseIntError;

// Define a generic alias for `Result` with error type `ParseIntError`
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // In this case the correct answer will still be given
    let twenty = multiply("10", "2");
    print(twenty);

    // In this case a more useful error message will be provided
    let tt = multiply("t", "2");
    print(tt);
}

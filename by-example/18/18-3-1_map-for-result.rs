use std::num::ParseIntError;

// Just like `Option`, we can use combinatorial operators like `map()`
// Except for the writing method, this function is exactly the same as the one above. Its function is:
// If the value is legal, calculate its product, otherwise return an error
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
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

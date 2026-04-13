fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Build error 1
    2 * first.parse::<i32>().unwrap() // Build error 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    println!("The first doubled is {}", double_first(empty));
    // Error 1!: Input vecotr is empty

    println!("The first doubled is {}", double_first(strings));
    // Mistake 2!: This element cannot be parsed as a number
}
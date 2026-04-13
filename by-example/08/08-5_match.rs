fn main() {
    let number = 13;
    // Try ^ assigning different values ​​to `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One"),
        // Match multiple values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // Try it ^ Add 13 to the list of prime numbers
        // Matches a closed range
        13..=19 => println!("a teen"),
        // Handle other situations,
        _ => println!("Ain't special"),
        // Try it ^ Comment out this umbrella branch
    }

    let boolean = true;
    // match is also an expression
    let binary = match boolean {
        // The match branch must cover all possible values
        false => 0,
        true => 1,
        // Try it ^ Comment out one of the branches
    };

    println!("{} -> {}", boolean, binary);
}
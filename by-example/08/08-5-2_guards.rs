fn main() {
    let pair = (2, -2);
    // Try it ^ assign different values ​​to `pair`

    println!("Tell me about: {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // ^ The conditional part of `if` is a guard statement
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
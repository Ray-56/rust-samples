fn main() {
    let triple = (0, -2, 3);
    // Try ^ assigning different values ​​to `triple`

    println!("Tell me about {:?}", triple);

    // match can deconstruct a tuple
    match triple {
        // Deconstruct the first and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _ => println!("It does't matter what they are"),
        // `_` means not binding the value to the variable
    }
}
// The `age` function returns a `u32` value
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // One could directly match (`match`) 1..= 12, but then how old would the child be?
        // Instead, bind the matching value to `n` in the 1..= 12 branch. Age can now be read
        n @ 1 ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // If it does not meet the above range, the result will be returned.
        n => println!("I'm an old person of age {:?}", n),
    }

    
    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        // Get the mutable type `Some`, matching if its value (bound to `n`) is equal to 42
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number
        Some(n) => println!("Not interesting... {}", n),
        // Matches any other value (`None` mutable type)
        _ => (),
    }
}
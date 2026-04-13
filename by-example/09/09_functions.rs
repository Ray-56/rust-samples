fn main() {
    // We can use a function here and define it later
    fizzbuzz_to(100);
}

// a function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Boundary case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, you can do without the `return` keyword
    lhs % rhs == 0
}

// A function that does not return a value. Actually returns a unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the function signature can omit the return type
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
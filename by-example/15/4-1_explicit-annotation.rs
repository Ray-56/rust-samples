// `print_refs` accepts two `i32` references, which have different lifetimes `'a` and `'b`
// Both lifetimes must be at least as long as the `print_refs` function
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function without parameters, but with a lifetime parameter `'a`
fn failed_borrow<'a>() {
    let _x = 12;

    // Report an error! The life cycle of `_x` is not long enough
    // let y: &'a i32 = &_x;
    // Using lifetime `'a` as an explicit type annotation inside a function will cause failure
    // Because the life cycle of `&_x` is shorter than that of `y`. Short life cycle cannot be forced to convert to long life cycle
}

fn main() {
    // Create variables for later borrowing
    let (four, nine) = (4, 9);

    // Borrowing (`&`) of both variables is passed into the function
    print_refs(&four, &nine);
    // Any borrowed input must outlive the borrower
    // In other words, the life cycles of `four` and `nine` must be longer than `print_refs`

    failed_borrow();
    // `failed_borrow` does not contain a reference, so `a` is not required to outlast the lifetime of the function
    // But `a` does have a longer lifespan. Because the life cycle has never been constrained, it defaults to `'static`
}
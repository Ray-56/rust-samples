// Define a function that can accept a generic `F` parameter qualified by `Fn` and call it
fn call_me<F: Fn()>(f: F) {
    f()
}

// Define a wrapper function that satisfies the `Fn` constraint
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure that satisfies the `Fn` constraint
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
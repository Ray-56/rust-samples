fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize a binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Report an error! Uninitialized binding used
    println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
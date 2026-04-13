fn main() {
    // Get a reference of type `i32`. `&` means taking a reference
    let reference = &4;

    match reference {
        // If you use the pattern `&val` to match `reference`, it is equivalent to doing such a comparison.
        // `&i32` `&val` We see that if the matching `&` is removed, `i32` should be assigned to `val`
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // If you don’t want to use `&`, you need to dereference it before matching.
    match *reference {
        val => println!("Got a value via destructuring: {:?}", val),
    }

    // What would happen if there were no references in the first place? `reference` is of type `&` because the assignment statement
    // The right side of is already a reference. The one below is not a quote, because the one on the right is not
    let _not_a_reference = 3;

    // Rust provides `ref` for this situation. It changes the assignment behavior so that references can be created to concrete values
    // The following line will get a reference
    let ref _is_reference = 3;

    // Correspondingly, define two non-reference variables, whose references can still be obtained through `ref` and `ref mut`
    let value = 5;
    let mut mut_value = 6;

    // Use the `ref` keyword to create a reference
    match value {
        ref r => println!("Got a reference to value: {:?}", r),
    }

    // Similarly use `ref mut`
    match mut_value {
        ref mut m => {
            // And after obtaining a reference to `mut_value`, you must first dereference it before you can change its value.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}
// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32 type
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // Create an i32 type that lives on the heap, and an i32 type that lives on the stack
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // The contents of the box are borrowed without ownership, so the contents of the box can be borrowed again later
    // Note: Please note that the function itself is a scope, so the references temporarily created in the function after the following two functions are completed will no longer exist.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Get a reference to the data in the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Report an error! When the value inside `boxed_i32` is borrowed in a subsequent scope, it cannot be destroyed.
        // eat_box_i32(boxed_i32);
        // Correction ^ Comment out this line

        // After the value in `_ref_to_i32` is destroyed, try to borrow `_ref_to_i32`
        // (Note: In the previous line of code, eat_box_i32(boxed_i32) can destroy `boxed_i32` if no borrowing is done here)
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed
    }

    // `boxed_i32` can now give ownership to `eat_i32` and destroy it
    // (Note: It can be destroyed because there is no reference to `boxed_i32` anymore)
    eat_box_i32(boxed_i32);
}
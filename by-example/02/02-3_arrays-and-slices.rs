use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-length array (type tag is redundant)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Subscripts start from 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Data is allocated on the stack
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed called slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice can point to a part of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out-of-bounds subscripts will cause a panic.
    // println!("{}", xs[5]);
}
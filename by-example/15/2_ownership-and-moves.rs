// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory is freed
}

fn main() {
    // stack allocated integer
    let x = 5u32;

    // *Copy* `x` to `y` -- no resource movement
    let y = x;

    // Both values ​​can be used independently
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a heap allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *move* `a` to `b`
    let b = a;
    // Copies the pointer address (not the data) of `a` to `b`. Now both point to the same heap allocated data, but now `b` owns it

    // Report an error! `a` cannot access the data because it no longer owns that portion of the heap memory
    // println!("a conatins: {}", a);

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // At this time, the heap memory has been released. This operation will cause dereference of the released memory, which is prohibited by the compiler.
    // Report an error! The same reason as the previous error
    // println!("b conatins: {}", b);
}
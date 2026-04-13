// raii.rs
fn create_box() {
    // Allocate an integer data on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here and the memory is released
}

fn main() {
    // Allocate an integer data on the heap
    let _box2 = Box::new(5i32);

    // Nested scope
    {
        // Allocate an integer flash on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here and the memory is freed
    }

    // Create a bunch of boxes (just for fun)
    // No need to manually release memory at all
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here and the memory is released
}
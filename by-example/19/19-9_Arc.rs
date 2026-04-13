use std::sync::Arc;
use std::thread;

fn main() {
    // This variable is declared where its value is specified.
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // There is no numerical specification here because it is a pointer to a reference in the memory heap
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // Due to the use of Arc, the thread can use the value assigned at the pointer position of the `Arc` variable to generate
            println!("{:?}", apple);
        });
    }
}
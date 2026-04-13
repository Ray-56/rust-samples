use std::thread;

static NTHREADS: i32 = 10;

// This is the main (`main`) thread
fn main() {
    // Provide a vector to store the created child threads (children)
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to end. return a result
        let _ = child.join();
    }
}
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // The channel has two endpoints: `Sender<T>` and `Receiver<T>`, where `T` is the message type to be sent (the type annotation is optional)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // The sender side can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id through the channel
        thread::spawn(move || {
            // The created thread takes ownership of `thread_tx`
            // Each thread places the message in the channel's message queue.
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation and the thread will continue immediately after sending the message.
            println!("thread {} finished", id);
        });
    }

    // All messages are collected here
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method gets a message from the channel
        // `recv` will block the current thread if no messages are available
        ids.push(rx.recv());
    }

    // Show the order in which messages were sent
    println!("{:?}", ids);
}
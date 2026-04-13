fn main() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert a new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Report an error! Immutable vector cannot grow
    // collected_iterator.push(0);

    // The `len` method gets the current size of a vector
    println!("Vector size: {}", xs.len());

    // Subscripts are expressed using square brackets (starting from 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element of vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Exceeding the index range will throw a panic
    // println!("Fourth element: {}", xs[3]);

    // Throwing a `Vector` is easy
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // You can use an independent variable (`i`) to record the number of iterations while iterating `Vector`
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // Thanks to `iter_mut`, a mutable `Vector` has each value modified while iterating
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}
use std::thread;

// This is the `main` thread
fn main() {
        // This is the data we want to work with.
    // We will implement the map-reduce algorithm through threads to calculate the sum of each bit
    // Each whitespace-delimited block is assigned to a separate thread for processing
    //
    // Give it a try: insert spaces and see how the output changes!
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Create a loudspeaker to store the sub-thread to be created
    let mut children = vec![];

    /*************************************************************************
     * "Map" stage
     *
     * Segment the data and perform initialization processing
     ************************************************************************/

    // Divide the data into segments, each segment will be calculated separately
    // Each segment is a reference (&str) to the complete data
    let chunked_data = data.split_whitespace();

    // Iterate over segmented data
    // .enumerate() will combine the current iteration count and the iterated element into a tuple (index, element)
    // return in the form. Deconstruct the tuple into two variables by immediately using "destructuring assignment"
    // `i` and `data_segment`
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Use a separate thread to process each piece of data
        //
        // spawn() returns the handle of the new thread, we must have the handle,
        // To get the return value of the thread.
        //
        // The 'move || -> u32' syntax represents this closure:
        // * No parameters ('||')
        // * Will take ownership of the captured variable ('move')
        // * Returns an unsigned 32-bit integer ('->u32')
        //
        // Rust can infer '-> u32' from the contents of the closure, so we don't need to write it.
        //
        // Give it a try: remove 'move' and see what happens
        children.push(thread::spawn(move || -> u32 {
            // Calculate the sum of each bit of the segment:
            let result = data_segment
                        // Iterate over the characters in this segment..
                        .chars()
                        // ..Convert characters to numbers..
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        // ..sums the returned iterator of numeric types
                        .sum();

            // println! will lock the standard output so that the contents printed by each thread are not interleaved.
            println!("processed segment {}, result={}", i, result);

            // There is no need for "return" because Rust is an "expression language" and in each block of code
            // The last evaluated expression is the value of the code block.
            result

        }));
    }

    /*************************************************************************
     * "Reduce" phase
     *
     * Collect intermediate results to get the final result
     ************************************************************************/

    // Collect the intermediate results produced by each thread into a new vector
    let mut intermediate_sums = vec![];
    for child in children {
        // Collect the return value of each child thread
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // Add up all intermediate results to get the final result
    //
    // We use the "turbo fish" notation::<> to provide type hints for sum().
    //
    // Give it a try: Instead of using the turbine fish notation, explicitly specify the type of intermediate_sums
    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Final sum result: {}", final_result)
}
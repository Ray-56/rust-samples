struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`
// The `Iterator` trait simply defines a method that returns the `next` element
impl Iterator for Fibonacci {
    type Item = u32;

    // We come here to use `.curr` and `.next` to define the sequence (sequence)
    // The return type is `Option<T>`
    // * When `Iterator` ends, return `None`
    // * In other cases, return the next value wrapped by `Some`
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there is no end point in the Fibonacci sequence, `Iterator` will never return `None`, but will always return `Some`
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    // `0..3` is an `Iterator` that produces: 0, 1 and 2
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` iterates over `Iterator` until it returns `None`
    // And each `Some` value is unwrapped (unwrap) and then bound to a variable (here `i`)
    println!("Iterate throuth 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method extracts the first `n` items of `Iterator`,
    println!("The first four terms of the Fibonacci sequence are:");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method generates an `Iterator` for the array/slice
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
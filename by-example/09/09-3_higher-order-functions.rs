fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of the squared odd numbers under 1000");
    let upper = 1000;

    // How to write imperative
    // Declare accumulator variable
    let mut acc = 0;
    // Iteration: 0, 1, 2,... to infinity
    for n in 0.. {
        // square of number
        let n_squared = n * n;

        if n_squared >= upper {
            // If it is greater than the upper limit, exit the loop
            break;
        } else if is_odd(n_squared) {
            // If it's an odd number, count
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional writing
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                // Square all natural numbers
            .take_while(|&n| n < upper)     // Take less than the upper limit
            .filter(|&n| is_odd(n))         // Take an odd number
            .fold(0, |sum, i| sum + i);     // Add up at the end
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
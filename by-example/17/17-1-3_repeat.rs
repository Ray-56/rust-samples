// `min!` will find the minimum value of any number of arguments
macro_rules! find_min {
    // base case
    ($x: expr) => ($x);
    // `$x` is followed by at least one `$y,`
    ($x: expr, $($y: expr), +) => (
        // Call `find_min!` on the `$y` following `$x`
        std::cmp::min($x, find_min!($($y), +))
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
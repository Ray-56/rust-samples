// Depending on how you call it, `test!` will compare `$left` and `$right` differently
macro_rules! test {
    // Parameters do not need to be separated by commas
    // Parameters can be combined in any way
    ($left: expr; and $right: expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        );
    };
    // ^ Each branch must end with a semicolon
    ($left: expr; or $right: expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right,
        );
    }
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
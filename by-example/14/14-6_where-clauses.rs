use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// A `where` clause is required here, otherwise it will be expressed as `T:Debug`
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // We're going to use `Option<T>: Debug` as a constraint because that's what's going to be printed
    // Otherwise we would be given wrong constraints
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}


fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
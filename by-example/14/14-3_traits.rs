// non-copyable type
struct Empty;
struct Null;

// Generic trait for `T`
trait DoubleDrop<T> {
    // Define a caller method that accepts an extra parameter `T` but does nothing with it
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for the generic caller type `U` and any generic type `T`
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of the two incoming parameters and releases them
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // Release `empty` and `null`
    empty.double_drop(null);


    // empty;
    // null;
}
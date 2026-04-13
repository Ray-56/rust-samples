use std::fmt; // Import the `fmt` module

// Define a struct `List` containing a single `Vec`
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Get the value using the subscript of the tuple and create a reference to `vec`
        let vec = &self.0;

        write!(f, "[")?;

        // Use `v` to iterate over `vec` and use `count` to record the number of iterations
        for (count, v) in vec.iter().enumerate() {
            // Add a comma to every element except the first
            // Use `?` or `try!` to return an error
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // Add matching square brackets and return an fmt::Result value
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
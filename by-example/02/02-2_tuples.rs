use std::fmt;

// Tuples can serve as parameters and return values ​​of functions
fn reverse(pair:(i32, bool)) -> (bool, i32) {
    // You can use `let` to bind members of a tuple to some variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

// The following structure will be used in the "Try it Yourselves" exercise
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // Contains tuples of various types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Access specific values ​​via tuple subscripts
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can also serve as elements of tuples
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // Tuples can be printed
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But very long tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // Try it ^ Uncomment the two lines above and read the error message given by the compiler
    // Printing tuples is currently implemented using a macro that only works up to 12 elements
    // ^ from stackoverflow, https://stackoverflow.com/questions/51846320/why-is-tuple-formatting-limited-to-12-items-in-rust

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // Creating single-element tuples requires an extra comma to distinguish them from literals enclosed in parentheses.
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // Tuples can be deconstructed to bind values ​​to variables
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    // The following is the homework part

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "( {} {} )", self.0, self.1)?;
            write!(f, "\n")?;
            write!(f, "( {} {} )", self.2, self.3)
        }
    }
    println!("Matrix: \n{}", matrix);

    fn transpose(matrix: Matrix) -> Matrix {

        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }
    println!("Transpose: \n{}", transpose(matrix));
}
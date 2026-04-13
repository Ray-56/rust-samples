// (Use `use`) Import the `fmt` module to make `fmt:Display` available
use std::fmt;

// Define a structure and we will implement `fmt::Display` for it. The following is a simple tuple structure
// `Structure`, containing an `i32` element
struct Structure(i32);

// In order to use the `{}` tag, the `fmt::Display` trait must be manually implemented for the type
impl fmt::Display for Structure {
    // This trait requires `fmt` to use the exact same function signature as the following function
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Writes only the first element of self to the given output stream `f`. Returns `fmt::Result`, this
        // The result indicates the success or failure of the operation. Note that the usage of `write!` is very similar to `println!`
        write!(f, "{}", self.0)
    }
}

// A structure with two numbers. Derive `Debug` for comparison with the output of `Display`
#[derive(Debug)]
struct MinMax(i64, i64);

// `Display` that implements `MinMax`
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to represent each data
        write!(f, "({}, {})", self.0, self.1)
    }
}

// For comparison, consider a struct with named fields
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize the format so that only the values ​​of `x` and `y` are displayed
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // In the above code, `Point2D` is implemented using `fmt:Binary`
    println!("What does Point2D look like in binary: {:b}?", point);

    // Homework section
    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Compare complexs:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
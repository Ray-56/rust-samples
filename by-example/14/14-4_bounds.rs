// This trait is used to implement printing tags: `{:?}`
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

// Generic `T` must implement `Debug`. As long as this is met, the following function can work normally no matter what type it is.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any instance of a generic that conforms to this constraint can access the `area` function of `HasArea`
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, heigth: 4.0 };
    let _triangle = Triangle { length: 3.0, heigth: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    
}
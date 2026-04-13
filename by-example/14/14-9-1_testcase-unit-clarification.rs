use std::ops::Add;
use std::marker::PhantomData;

/// Create an empty enum type to represent units
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` is a type with an ordinal type parameter `Unit`
/// And `Length` is not generic for types that represent length (i.e. `f641)
/// 
/// `f64` already implements the `Clone` and `Copy` traits
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// The `Add` trait defines the behavior of the `+` operator
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // add() returns a new `Length` structure containing the sum
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` calls the `Add` implementation for the `f64` type
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // Specify that `one_foot` has a virtual type parameter `Inch`
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` has virtual type parameter `Mm`
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // `+` calls the `add()` method we implemented on `Length<Unit>`
    // 
    // Since `Length` implements `Copy`, `add()` does not consume `one_foot` and `one_meter`, but copies them as `self` and `rhs`
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition is executed normally
    println!("one foot + one foot = {:?} in", two_feet.0);
    println!("one meter + one meter = {:?} in", two_meters.0);
}
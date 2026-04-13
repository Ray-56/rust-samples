// `Centimeters`, a tuple structure that can be directly compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a printable tuple structure
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple structure without additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Report an error! `Seconds` cannot be printed; it does not implement the `Debug` trait
    // Report an error! `Seconds` cannot be compared; it does not implement the `PartialEq` trait

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter", cmp);
}
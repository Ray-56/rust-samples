// Deriving the `fmt::Debug` implementation of `Structure`
// `Structure` is a structure containing a single `i32`
#[derive(Debug)]
struct Structure(i32);

// Put `Structure` into the structure `Deep`. Then enable `Deep` to also print
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Persion<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // Using `{:?}` to print is similar to using `{}`
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` can also be printed
    println!("Now {:?} will print!", Structure(3));

    // One problem with using `derive` is that you cannot control the form of the output.
    // What should I do if I just want to show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Ray";
    let age = 29;
    let ray = Persion { name, age };

    // Beautify printing
    println!("{:#?}", ray);
}

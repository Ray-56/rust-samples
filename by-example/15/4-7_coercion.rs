// Here, Rust derives a lifetime as short as possible
// Then both references are forced to convert to this life cycle
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` is read as the declaration period `'a` is at least as long as `'b`
// Here we accept a `&'a i32` type and return a `&'b i32` type
// This is the result of casting
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Long life cycle

    {
        let second = 3; // Short life cycle
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}
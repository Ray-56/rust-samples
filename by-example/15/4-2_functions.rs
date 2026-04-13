// An input reference with a lifetime of `'a`, where `'a`'s inventory time
// At least as long as the function
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references may also have lifetimes
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifecycles. For the following situation, it is no problem even if the two have the same life cycle `'a`,
// But for some more complex situations, different life cycles may be needed.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// It is also possible to return the reference passed in
// But must return the correct life cycle
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

// fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// The above code is invalid: the inventory time of `'a` must be longer than the function
// Here `&String::from("foo")` will create a `String` type and then take a reference to it
// The data is deleted when it leaves the scope, returning a reference to the invalid data.

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(&z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
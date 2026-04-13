fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // Deconstructing members of a structure
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {}, y = {}", a, b, y);

    // Structures can be destructured and variables renamed, the order of members does not matter

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // You can also ignore certain variables
    let Foo { y, .. } = foo;
    println!("y = {}", y);
}
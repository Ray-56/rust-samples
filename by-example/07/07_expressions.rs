fn main() {
    // variable binding
    let x = 5;

    // expression
    x;
    x + 1;
    15;

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // Assign subexpression to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon ends the expression, so `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
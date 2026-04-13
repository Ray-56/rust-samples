fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is small number, increase ten-fold");

            // This expression returns an `i32` type
            10 * n
        } else {
            println!(", and is a big number, half the number");

            // This expression must also return an `i32` type
            n / 2
            // Try ^ Try ending the expression with a semicolon
        };
    // ^ Don’t forget to add a semicolon here! All `let` bindings require it
}
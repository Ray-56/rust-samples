fn main() {
    // Set `optional` to type `Option<i32>`
    let mut optional = Some(0);

    /* // Repeat this test
    loop {
        match optional {
            // If `optional` is deconstructed successfully, execute the following statement block
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            // Exit the loop when destructuring fails
            _ => { break; }
        }
    } */

    // This reads: When `let` deconstructs `optional` into `Some(i)`, the statement block (`{}`) is executed. Otherwise, `break`
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
    // ^ `if let` has optional `else`/`else if` clauses
    // while `while let` does not
}
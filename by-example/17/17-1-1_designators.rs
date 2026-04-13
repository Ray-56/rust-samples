macro_rules! create_function {
    // This macro accepts an argument represented by the `ident` directive and creates a function named `$func_name`
    // The `ident` directive is used for variable or function names
    ($func_name: ident) => {
        fn $func_name() {
            // The `stringify!` macro converts `ident` into a string
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}

// Create functions named `foo` and `bar` with the help of the above macros
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression,)
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that code blocks are also expressions
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

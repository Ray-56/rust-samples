macro_rules! calculate {
    (eval $e: expr) => {{
        {
            let val: usize = $e; // Force type to integer
            println!("{} = {}", stringify!($e), val);
        }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2 // You see, `eval` is not a keyword in Rust
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
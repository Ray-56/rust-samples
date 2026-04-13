// `F` must implement `Fn` for a closure with no input parameters and no return value, which is exactly the same requirement for `print`
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement `Fn` for it
    // Store closure into `print`
    let print = || println!("{}", x);

    apply(print);
}

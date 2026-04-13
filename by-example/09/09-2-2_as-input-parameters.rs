// The function takes the closure as a parameter and calls it
fn apply<F>(f: F)
where
    F: FnOnce(),
    // ^ Try it: Replace `FnOnce` with `Fn` or `FnMut`
{
    f();
}

// Input closure, return an `i32` integer function
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // non-copyable type
    // `to_owned` creates owned data from borrowed data
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference, `farewell` by value
    let diary = || {
        // `greeting` is captured by reference, so the closure needs to be `Fn`
        println!("I said {}.", greeting);

        // The following changes `farewell`, thus requiring the closure to capture it via a mutable reference
        // Now requires `FnMut`
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzzz");

        // Manually calling drop also requires the closure to obtain `farewell` by value
        // Now requires `FnOnce`
        mem::drop(farewell);
    };

    // Call the function `apply` with the closure as parameter
    apply(diary);

    // The closure `double` satisfies the trait constraint of `apply_to_3`
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}

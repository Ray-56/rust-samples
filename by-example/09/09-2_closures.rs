fn main() {
    // Implement self-increment through closures and functions respectively

    // Use functions to implement
    fn function(i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we bind them to references
    // Type annotations are the same as for functions, but type annotations and the use of `{}` to surround the function body are optional.
    // These nameless functions are assigned to appropriately named variables
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // Calling functions and closures
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure with no parameters, returning an `i32` type
    // The return type is automatically deduced
    let one = || 1;
    println!("closure returning one: {}", one());
}
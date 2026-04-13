// This is a simple macro called `say_hello`
macro_rules! say_hello {
    // `()` means this macro does not accept any parameters
    () => {
        // This macro will expand into the contents of this code block
        println!("Hello!");
    };
}

fn main() {
    // This call will expand to `println!("Hello!")`
    say_hello!();
}
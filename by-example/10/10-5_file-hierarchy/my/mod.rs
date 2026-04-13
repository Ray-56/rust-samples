// Similarly `mod inaccessible` and `mod nested` will find `nested.rs` and
// `inaccessible.rs` files and put them into their respective modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that \n> ");

    private_function();
}
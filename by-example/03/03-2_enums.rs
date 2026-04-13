// This attribute is used to hide warnings about unused code
#![allow(dead_code)]

// Create an `enum` to classify web events. Note that the variable name and type both specify `enum`
// Type of value: `PageLoad` is not equal to `PageUnload`, `KeyPress(char)` is not equal to `Paste(String)`. Each value is different and independent of each other
enum WebEvent {
    // An `enum` can be a unit structure (called `unit-like` or `unit`)
    PageLoad,
    PageUnload,
    // or a tuple structure
    KeyPress(char),
    Paste(String),
    // Or an ordinary structure
    Click { x: i64, y: i64 }
}

// This function takes a `WebEvent`enum as a parameter and has no return value
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Deconstruct `c` from `enum`
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Deconstruct `Click` into `x`and`y`
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a slice of strings
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types that implement the corresponding trait
// In fact these traits are empty internally, but that doesn't matter
fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // Due to constraints, `red()` cannot be applied to blue_jay and vice versa.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue_jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));
}
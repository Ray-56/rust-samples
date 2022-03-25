struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 这些函数只对实现了相应的 trait 的类型有效
// 事实上这些 trait 内部是空的，但这没有关系
fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // 由于约束，`red()`不能作用于 blue_jay，反过来也一样
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue_jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));
}
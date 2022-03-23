// 该属性用于隐藏对未使用代码的警告
#![allow(dead_code)]

// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显示辨别值（explicit discriminator）的enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enum`可以转成整型
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
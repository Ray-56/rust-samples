fn main() {
    let triple = (0, -2, 3);
    // 试一试 ^ 将不同的值赋给`triple`

    println!("Tell me about {:?}", triple);

    // match 可以解构一个元组
    match triple {
        // 解构出第一个和第三个元素
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // `..` 可用来忽略元组的其余部分
        _ => println!("It does't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}
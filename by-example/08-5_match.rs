fn main() {
    let number = 13;
    // 试一试 ^ 将不同的值赋给`number`

    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // 试一试 ^ 将 13 添加到质数列表中
        // 匹配一个闭区间范围
        13..=19 => println!("a teen"),
        // 处理其他情况、
        _ => println!("Ain't special"),
        // 试一试 ^ 注释掉这个总括性的分支
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // 试一试 ^ 将其中一条分支注释掉
    };

    println!("{} -> {}", boolean, binary);
}
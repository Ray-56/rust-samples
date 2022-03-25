fn main() {
    // 将`optional`设为`Option<i32>`类型
    let mut optional = Some(0);

    /* // 重复运行这个测试
    loop {
        match optional {
            // 如果`optional`解构成功，就执行下面语句块
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            // 解构失败时退出循环
            _ => { break; }
        }
    } */

    // 这读作：当`let`将`optional`解构成`Some(i)`时，就执行语句块（`{}`）。否则就`break`
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
    // ^ `if let` 有可选的 `else`/`else if` 分句
    // 而`while let`没有
}
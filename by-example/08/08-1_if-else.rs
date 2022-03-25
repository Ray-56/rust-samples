fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is small number, increase ten-fold");

            // 这个表达式返回一个`i32`类型
            10 * n
        } else {
            println!(", and is a big number, half the number");

            // 这个表达式也必须返回一个`i32`类型
            n / 2
            // 试一试 ^ 试着加上一个分号来结束这条表达式
        };
    //   ^ 不要忘记这里加上一个分号！所有的`let`绑定都需要它
}